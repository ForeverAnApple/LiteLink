//! Output mode selection and the VRChat avatar listener that drives `auto` mode.

use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::Ordering;
use std::time::Duration;

use clap::ValueEnum;
use log::{debug, info, warn};
use rosc::{decoder, OscPacket, OscType};

use crate::state::{set_avatar_native, RESEND_ALL, SHUTDOWN};

const AVATAR_CHANGE_ADDR: &str = "/avatar/change";
const FULL_FACE_TRACKING_ADDR: &str = "/avatar/avatarfullfacetracking";

/// Which OSC outputs to send.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum OutputMode {
    /// Follow the avatar's native face tracking flag, like VRCFaceTracking
    Auto,
    /// Legacy `/avatar/parameters/*` floats only
    Legacy,
    /// Native `/tracking/face/v1` blob only
    Native,
    /// Blob and floats
    Both,
}

/// What to send this tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Outputs {
    pub floats: bool,
    pub blob: bool,
}

/// Decide outputs from the mode and the avatar's native flag.
///
/// Auto mirrors VRCFT: a native avatar gets only the blob (every
/// `/avatar/parameters/*` is suppressed, status bools included); a legacy
/// avatar gets only floats; before VRChat reports the flag, send both so
/// either kind of avatar animates from the start.
pub fn select_outputs(mode: OutputMode, avatar_native: Option<bool>) -> Outputs {
    let (floats, blob) = match mode {
        OutputMode::Legacy => (true, false),
        OutputMode::Native => (false, true),
        OutputMode::Both => (true, true),
        OutputMode::Auto => match avatar_native {
            None => (true, true),
            Some(true) => (false, true),
            Some(false) => (true, false),
        },
    };
    Outputs { floats, blob }
}

/// Something VRChat told us about the current avatar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarEvent {
    Changed,
    FullFaceTracking(bool),
}

/// Collect avatar events from a packet, descending into nested bundles.
pub fn collect_events(packet: &OscPacket, out: &mut Vec<AvatarEvent>) {
    match packet {
        OscPacket::Bundle(bundle) => {
            for p in &bundle.content {
                collect_events(p, out);
            }
        }
        OscPacket::Message(msg) => match msg.addr.as_str() {
            AVATAR_CHANGE_ADDR => out.push(AvatarEvent::Changed),
            FULL_FACE_TRACKING_ADDR => {
                if let Some(OscType::Bool(b)) = msg.args.first() {
                    out.push(AvatarEvent::FullFaceTracking(*b));
                }
            }
            _ => {}
        },
    }
}

fn apply(event: AvatarEvent) {
    match event {
        AvatarEvent::Changed => {
            info!("avatar changed: native face tracking unknown, resending all params");
            set_avatar_native(None);
            RESEND_ALL.store(true, Ordering::Relaxed);
        }
        AvatarEvent::FullFaceTracking(native) => {
            info!("avatar native face tracking: {native}");
            set_avatar_native(Some(native));
        }
    }
}

/// Listen for VRChat's OSC output until shutdown. A bind failure is not fatal:
/// the flag stays unknown and auto mode keeps sending both outputs.
pub fn run_listener(addr: SocketAddr) {
    let socket = match UdpSocket::bind(addr) {
        Ok(s) => s,
        Err(e) => {
            warn!(
                "cannot listen for VRChat OSC on {addr} ({e}); port {} is likely taken. \
                 Auto mode will send both native and legacy output.",
                addr.port()
            );
            return;
        }
    };
    socket.set_read_timeout(Some(Duration::from_millis(100))).ok();

    // Max UDP payload, so large VRChat bundles are never truncated.
    let mut buf = vec![0u8; 65536];
    let mut events = Vec::new();

    while !SHUTDOWN.load(Ordering::Relaxed) {
        let len = match socket.recv_from(&mut buf) {
            Ok((len, _)) => len,
            Err(e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut =>
            {
                continue;
            }
            Err(e) => {
                debug!("VRChat OSC recv error: {e}");
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }
        };

        match decoder::decode_udp(&buf[..len]) {
            Ok((_, packet)) => {
                events.clear();
                collect_events(&packet, &mut events);
                events.iter().copied().for_each(apply);
            }
            Err(e) => debug!("undecodable {len}-byte OSC packet: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rosc::{encoder, OscBundle, OscMessage, OscTime};

    fn msg(addr: &str, args: Vec<OscType>) -> OscPacket {
        OscPacket::Message(OscMessage { addr: addr.into(), args })
    }

    fn bundle(content: Vec<OscPacket>) -> OscPacket {
        OscPacket::Bundle(OscBundle {
            timetag: OscTime { seconds: 0, fractional: 1 },
            content,
        })
    }

    /// Round-trip through the wire format, as the listener sees it.
    fn events_of(packet: &OscPacket) -> Vec<AvatarEvent> {
        let bytes = encoder::encode(packet).unwrap();
        let (_, decoded) = decoder::decode_udp(&bytes).unwrap();
        let mut out = Vec::new();
        collect_events(&decoded, &mut out);
        out
    }

    #[test]
    fn mode_selection() {
        use OutputMode::*;
        let both = Outputs { floats: true, blob: true };
        let floats = Outputs { floats: true, blob: false };
        let blob = Outputs { floats: false, blob: true };
        for flag in [None, Some(false), Some(true)] {
            assert_eq!(select_outputs(Legacy, flag), floats);
            assert_eq!(select_outputs(Native, flag), blob);
            assert_eq!(select_outputs(Both, flag), both);
        }
        assert_eq!(select_outputs(Auto, None), both);
        assert_eq!(select_outputs(Auto, Some(true)), blob);
        assert_eq!(select_outputs(Auto, Some(false)), floats);
    }

    #[test]
    fn parses_full_face_tracking_flag() {
        let t = msg(FULL_FACE_TRACKING_ADDR, vec![OscType::Bool(true)]);
        let f = msg(FULL_FACE_TRACKING_ADDR, vec![OscType::Bool(false)]);
        assert_eq!(events_of(&t), vec![AvatarEvent::FullFaceTracking(true)]);
        assert_eq!(events_of(&f), vec![AvatarEvent::FullFaceTracking(false)]);
    }

    #[test]
    fn parses_avatar_change() {
        let p = msg(AVATAR_CHANGE_ADDR, vec![OscType::String("avtr_123".into())]);
        assert_eq!(events_of(&p), vec![AvatarEvent::Changed]);
    }

    #[test]
    fn parses_nested_bundles_in_order() {
        let p = bundle(vec![
            msg(AVATAR_CHANGE_ADDR, vec![OscType::String("avtr_1".into())]),
            msg("/avatar/parameters/Other", vec![OscType::Float(0.5)]),
            bundle(vec![msg(FULL_FACE_TRACKING_ADDR, vec![OscType::Bool(true)])]),
        ]);
        assert_eq!(
            events_of(&p),
            vec![AvatarEvent::Changed, AvatarEvent::FullFaceTracking(true)]
        );
    }

    #[test]
    fn ignores_unrelated_and_malformed() {
        assert!(events_of(&msg("/avatar/parameters/Foo", vec![OscType::Bool(true)])).is_empty());
        assert!(events_of(&msg(FULL_FACE_TRACKING_ADDR, vec![OscType::Int(1)])).is_empty());
        assert!(events_of(&msg(FULL_FACE_TRACKING_ADDR, vec![])).is_empty());
    }
}
