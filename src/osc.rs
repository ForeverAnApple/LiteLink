use std::collections::HashMap;
use std::net::UdpSocket;

use rosc::encoder;
use rosc::{OscBundle, OscMessage, OscPacket, OscTime, OscType};

use crate::mapping::{OscParam, OscValue};

/// Epsilon for float change detection. Values changing less than this are skipped.
const FLOAT_EPSILON: f32 = 0.001;

/// VRChat's native face tracking input (102-byte blob).
pub const NATIVE_FACE_ADDR: &str = "/tracking/face/v1";

pub struct OscSender {
    socket: UdpSocket,
    target: std::net::SocketAddr,
    /// Cache of last-sent values for change detection.
    last_sent: HashMap<String, CachedValue>,
    // Buffers reused across sends, so a steady tick allocates only for the
    // params that changed. rosc wants owned messages, so `send` lends these
    // to a bundle and takes them back after encoding.
    content: Vec<OscPacket>,
    blob_msg: Option<OscMessage>,
    encoded: Vec<u8>,
}

#[derive(Clone, Copy, PartialEq)]
enum CachedValue {
    Float(f32),
    Bool(bool),
}

impl CachedValue {
    fn differs_from(self, old: CachedValue) -> bool {
        match (self, old) {
            (CachedValue::Float(new), CachedValue::Float(old)) => (new - old).abs() > FLOAT_EPSILON,
            (new, old) => new != old,
        }
    }
}

impl OscSender {
    pub fn new(target: std::net::SocketAddr) -> std::io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        Ok(Self {
            socket,
            target,
            last_sent: HashMap::with_capacity(128),
            content: Vec::with_capacity(128),
            blob_msg: None,
            encoded: Vec::with_capacity(4096),
        })
    }

    /// Forget everything sent so far, so the next send carries every param.
    pub fn reset(&mut self) {
        self.last_sent.clear();
    }

    /// Send one bundle holding the params that changed since last send, plus
    /// the native face blob (if given) unconditionally.
    /// Returns the number of messages sent.
    pub fn send(&mut self, params: &[OscParam<'_>], blob: Option<&[u8]>) -> std::io::Result<usize> {
        let mut content = std::mem::take(&mut self.content);

        if let Some(blob) = blob {
            let mut msg = self.blob_msg.take().unwrap_or_else(|| OscMessage {
                addr: NATIVE_FACE_ADDR.to_string(),
                args: vec![OscType::Blob(Vec::with_capacity(blob.len()))],
            });
            if let Some(OscType::Blob(buf)) = msg.args.first_mut() {
                buf.clear();
                buf.extend_from_slice(blob);
            }
            content.push(OscPacket::Message(msg));
        }

        for p in params {
            let new_val = match p.value {
                OscValue::Float(v) => CachedValue::Float(v),
                OscValue::Bool(v) => CachedValue::Bool(v),
            };

            let changed = match self.last_sent.get_mut(p.address.as_ref()) {
                Some(old) => {
                    let changed = new_val.differs_from(*old);
                    if changed {
                        *old = new_val;
                    }
                    changed
                }
                None => {
                    self.last_sent.insert(p.address.to_string(), new_val);
                    true
                }
            };

            if changed {
                let args = match p.value {
                    OscValue::Float(v) => vec![OscType::Float(v)],
                    OscValue::Bool(v) => vec![OscType::Bool(v)],
                };
                content.push(OscPacket::Message(OscMessage {
                    addr: p.address.to_string(),
                    args,
                }));
            }
        }

        if content.is_empty() {
            self.content = content;
            return Ok(0);
        }

        let count = content.len();
        let bundle = OscPacket::Bundle(OscBundle {
            timetag: OscTime {
                seconds: 0,
                fractional: 1,
            },
            content,
        });

        self.encoded.clear();
        // Encoding into a Vec cannot fail.
        let Ok(_) = encoder::encode_into(&bundle, &mut self.encoded);

        // Take the buffers back. The blob message, if any, is always first.
        if let OscPacket::Bundle(b) = bundle {
            self.content = b.content;
        }
        let mut sent = self.content.drain(..);
        if blob.is_some()
            && let Some(OscPacket::Message(m)) = sent.next()
        {
            self.blob_msg = Some(m);
        }
        drop(sent);

        self.socket.send_to(&self.encoded, self.target)?;
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mapping::map_blendshapes;
    use crate::state::BLENDSHAPE_COUNT;

    #[test]
    fn osc_messages_have_valid_addresses() {
        let bs = [0.5f32; BLENDSHAPE_COUNT];
        let params = map_blendshapes(&bs, "/avatar/parameters/FT/v2");

        for p in &params {
            assert!(p.address.starts_with('/'), "bad addr: {}", p.address);
            assert!(!p.address.contains(' '), "space in addr: {}", p.address);
        }
    }

    #[test]
    fn osc_bundle_encodes_successfully() {
        let bs = [0.0f32; BLENDSHAPE_COUNT];
        let params = map_blendshapes(&bs, "/avatar/parameters/FT/v2");

        let content: Vec<OscPacket> = params
            .iter()
            .map(|p| {
                let args = match &p.value {
                    OscValue::Float(v) => vec![OscType::Float(*v)],
                    OscValue::Bool(v) => vec![OscType::Bool(*v)],
                };
                OscPacket::Message(OscMessage {
                    addr: p.address.to_string(),
                    args,
                })
            })
            .collect();

        let bundle = OscPacket::Bundle(OscBundle {
            timetag: OscTime {
                seconds: 0,
                fractional: 1,
            },
            content,
        });

        let encoded = encoder::encode(&bundle);
        assert!(encoded.is_ok(), "encoding failed: {:?}", encoded.err());
        let bytes = encoded.unwrap();
        assert!(
            bytes.len() > 100,
            "bundle suspiciously small: {} bytes",
            bytes.len()
        );
    }

    #[test]
    fn sender_binds_successfully() {
        let target: std::net::SocketAddr = "127.0.0.1:9000".parse().unwrap();
        let sender = OscSender::new(target);
        assert!(sender.is_ok());
    }

    #[test]
    fn change_detection_skips_identical_values() {
        // Bind a real receiver so send_to has a valid target
        let recv = UdpSocket::bind("127.0.0.1:0").unwrap();
        let target = recv.local_addr().unwrap();
        let mut sender = OscSender::new(target).unwrap();

        let bs = [0.5f32; BLENDSHAPE_COUNT];
        let params = map_blendshapes(&bs, "/avatar/parameters/FT/v2");

        // First send: all params are new, all should be sent
        let count1 = sender.send(&params, None).unwrap();
        assert!(count1 > 90, "first send should send all params, got {count1}");

        // Second send with identical values: nothing should be sent
        let params2 = map_blendshapes(&bs, "/avatar/parameters/FT/v2");
        let count2 = sender.send(&params2, None).unwrap();
        assert_eq!(count2, 0, "identical values should send nothing");
    }

    #[test]
    fn change_detection_sends_changed_values() {
        // Bind a real receiver so send_to has a valid target
        let recv = UdpSocket::bind("127.0.0.1:0").unwrap();
        let target = recv.local_addr().unwrap();
        let mut sender = OscSender::new(target).unwrap();

        let bs = [0.0f32; BLENDSHAPE_COUNT];
        let params = map_blendshapes(&bs, "/avatar/parameters/FT/v2");
        sender.send(&params, None).unwrap();

        // Change one blendshape significantly
        let mut bs2 = [0.0f32; BLENDSHAPE_COUNT];
        bs2[17] = 0.8; // JawOpen
        let params2 = map_blendshapes(&bs2, "/avatar/parameters/FT/v2");
        let count = sender.send(&params2, None).unwrap();
        // Should send JawOpen + derived params that depend on it, but not all
        assert!(count > 0, "changed values should be sent");
        assert!(count < 90, "only changed params should be sent, got {count}");
    }

    #[test]
    fn change_detection_ignores_tiny_changes() {
        // Bind a real receiver so send_to has a valid target
        let recv = UdpSocket::bind("127.0.0.1:0").unwrap();
        let target = recv.local_addr().unwrap();
        let mut sender = OscSender::new(target).unwrap();

        let bs = [0.5f32; BLENDSHAPE_COUNT];
        let params = map_blendshapes(&bs, "/avatar/parameters/FT/v2");
        sender.send(&params, None).unwrap();

        // Tiny change below epsilon
        let mut bs2 = [0.5f32; BLENDSHAPE_COUNT];
        bs2[17] = 0.5005; // below FLOAT_EPSILON of 0.001
        let params2 = map_blendshapes(&bs2, "/avatar/parameters/FT/v2");
        let count = sender.send(&params2, None).unwrap();
        assert_eq!(count, 0, "sub-epsilon changes should be skipped");
    }

    fn recv_messages(recv: &UdpSocket) -> Vec<OscMessage> {
        let mut buf = [0u8; 16384];
        let (len, _) = recv.recv_from(&mut buf).unwrap();
        match rosc::decoder::decode_udp(&buf[..len]).unwrap().1 {
            OscPacket::Bundle(b) => b
                .content
                .into_iter()
                .filter_map(|p| match p {
                    OscPacket::Message(m) => Some(m),
                    _ => None,
                })
                .collect(),
            OscPacket::Message(m) => vec![m],
        }
    }

    #[test]
    fn blob_rides_in_same_bundle_every_send() {
        let recv = UdpSocket::bind("127.0.0.1:0").unwrap();
        recv.set_read_timeout(Some(std::time::Duration::from_secs(1))).unwrap();
        let mut sender = OscSender::new(recv.local_addr().unwrap()).unwrap();
        let bs = [0.5f32; BLENDSHAPE_COUNT];
        let params = map_blendshapes(&bs, "/avatar/parameters/FT/v2");
        let blob = [7u8; 102];

        sender.send(&params, Some(&blob)).unwrap();
        let first = recv_messages(&recv);
        assert_eq!(first.len(), params.len() + 1, "floats and blob in one packet");
        assert_eq!(first[0].addr, NATIVE_FACE_ADDR);
        assert!(matches!(&first[0].args[..], [OscType::Blob(b)] if b.len() == 102));

        // Nothing changed: the blob still goes out alone.
        sender.send(&params, Some(&blob)).unwrap();
        let second = recv_messages(&recv);
        assert_eq!(second.len(), 1);
        assert_eq!(second[0].addr, NATIVE_FACE_ADDR);
    }

    #[test]
    fn reset_resends_everything() {
        let recv = UdpSocket::bind("127.0.0.1:0").unwrap();
        let mut sender = OscSender::new(recv.local_addr().unwrap()).unwrap();
        let bs = [0.5f32; BLENDSHAPE_COUNT];
        let params = map_blendshapes(&bs, "/avatar/parameters/FT/v2");
        let all = sender.send(&params, None).unwrap();
        assert_eq!(sender.send(&params, None).unwrap(), 0);
        sender.reset();
        assert_eq!(sender.send(&params, None).unwrap(), all);
    }
}
