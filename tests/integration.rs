use std::collections::BTreeSet;
use std::net::UdpSocket;
use std::time::{Duration, Instant};

use rosc::decoder;
use rosc::{OscPacket, OscType};

const BLENDSHAPE_COUNT: usize = 61;

/// Kills the bridge when dropped, so a failed assert never leaks a process
/// that keeps the test ports bound.
struct ChildGuard(std::process::Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        self.0.kill().ok();
        self.0.wait().ok();
    }
}

/// Build a synthetic LiveLink packet.
fn build_livelink_packet(
    device_id: &str,
    subject_name: &str,
    frame: u32,
    blendshapes: &[f32; BLENDSHAPE_COUNT],
) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(6); // version
    buf.extend_from_slice(&(device_id.len() as u32).to_be_bytes());
    buf.extend_from_slice(device_id.as_bytes());
    buf.extend_from_slice(&(subject_name.len() as u32).to_be_bytes());
    buf.extend_from_slice(subject_name.as_bytes());
    buf.extend_from_slice(&frame.to_be_bytes());
    buf.extend_from_slice(&0.0f32.to_be_bytes());
    buf.extend_from_slice(&30u32.to_be_bytes());
    buf.extend_from_slice(&1u32.to_be_bytes());
    buf.push(BLENDSHAPE_COUNT as u8);
    for &val in blendshapes {
        buf.extend_from_slice(&val.to_be_bytes());
    }
    buf
}

/// End-to-end test: send a LiveLink packet to the receiver, capture the OSC output.
///
/// This test starts the full binary with custom ports and verifies OSC output.
#[test]
fn end_to_end_packet_flow() {
    // Use high ports to avoid conflicts
    let livelink_port = 19111;
    let osc_port = 19222;

    // Bind an OSC receiver socket before starting the binary
    let osc_receiver = UdpSocket::bind(format!("127.0.0.1:{osc_port}")).unwrap();
    osc_receiver
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();

    // Start the binary as a subprocess
    let exe = env!("CARGO_BIN_EXE_litelink");
    let _child = ChildGuard(std::process::Command::new(exe)
        .args([
            "--listen-port",
            &livelink_port.to_string(),
            "--osc-target",
            &format!("127.0.0.1:{osc_port}"),
            "--send-rate",
            "50",
            // Default (auto) mode listens for VRChat; keep it off the real port 9001.
            "--osc-listen",
            "127.0.0.1:19233",
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("failed to start litelink"));

    // Give it a moment to bind
    std::thread::sleep(Duration::from_millis(200));

    // Send a LiveLink packet
    let sender = UdpSocket::bind("0.0.0.0:0").unwrap();
    let mut shapes = [0.0f32; BLENDSHAPE_COUNT];
    shapes[17] = 0.85; // JawOpen

    let packet = build_livelink_packet("TestDevice", "TestSubject", 1, &shapes);
    sender
        .send_to(&packet, format!("127.0.0.1:{livelink_port}"))
        .unwrap();

    // Read OSC bundles until we get one with our JawOpen value.
    // The sender may emit a few zero-state bundles before our LiveLink packet is processed.
    let mut buf = [0u8; 8192];
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    let mut found_jaw_open = false;
    let mut found_tracking = false;

    while std::time::Instant::now() < deadline {
        let (len, _) = match osc_receiver.recv_from(&mut buf) {
            Ok(v) => v,
            Err(_) => break,
        };
        let osc_data = &buf[..len];
        let (_, osc_packet) = decoder::decode_udp(osc_data).expect("failed to decode OSC");

        let messages = match osc_packet {
            OscPacket::Bundle(bundle) => {
                let mut msgs = Vec::new();
                for pkt in bundle.content {
                    if let OscPacket::Message(msg) = pkt {
                        msgs.push(msg);
                    }
                }
                msgs
            }
            OscPacket::Message(msg) => vec![msg],
        };

        // Check JawOpen
        if let Some(jaw_open) = messages.iter().find(|m| m.addr.ends_with("JawOpen")) {
            if let OscType::Float(v) = &jaw_open.args[0] {
                if (*v - 0.85).abs() < 0.01 {
                    found_jaw_open = true;
                }
            }
        }

        // Check tracking status
        if let Some(tracking) = messages
            .iter()
            .find(|m| m.addr.contains("ExpressionTrackingActive"))
        {
            if let OscType::Bool(true) = &tracking.args[0] {
                found_tracking = true;
            }
        }

        if found_jaw_open && found_tracking {
            break;
        }
    }

    assert!(found_jaw_open, "never received JawOpen ~0.85 in OSC output");
    assert!(
        found_tracking,
        "never received ExpressionTrackingActive=true"
    );
}

/// Flatten one OSC datagram (bundle or message) into its messages.
fn decode_messages(data: &[u8]) -> Vec<rosc::OscMessage> {
    fn flatten(packet: OscPacket, out: &mut Vec<rosc::OscMessage>) {
        match packet {
            OscPacket::Message(msg) => out.push(msg),
            OscPacket::Bundle(bundle) => bundle.content.into_iter().for_each(|p| flatten(p, out)),
        }
    }
    let (_, packet) = decoder::decode_udp(data).expect("failed to decode OSC");
    let mut out = Vec::new();
    flatten(packet, &mut out);
    out
}

/// Everything the bridge sent during `window`, one Vec of messages per datagram.
fn collect_packets(socket: &UdpSocket, window: Duration) -> Vec<Vec<rosc::OscMessage>> {
    let mut buf = [0u8; 16384];
    let mut packets = Vec::new();
    let deadline = Instant::now() + window;
    while Instant::now() < deadline {
        if let Ok((len, _)) = socket.recv_from(&mut buf) {
            packets.push(decode_messages(&buf[..len]));
        }
    }
    packets
}

fn is_blob(msg: &rosc::OscMessage) -> bool {
    msg.addr == "/tracking/face/v1"
        && matches!(&msg.args[..], [OscType::Blob(b)] if b.len() == 102)
}

fn is_param(msg: &rosc::OscMessage) -> bool {
    msg.addr.starts_with("/avatar/parameters/")
}

fn param_set<'a>(msgs: impl IntoIterator<Item = &'a rosc::OscMessage>) -> BTreeSet<String> {
    msgs.into_iter()
        .filter(|m| is_param(m))
        .map(|m| m.addr.clone())
        .collect()
}

fn send_osc(socket: &UdpSocket, target: &str, addr: &str, args: Vec<OscType>) {
    let packet = OscPacket::Message(rosc::OscMessage { addr: addr.into(), args });
    socket
        .send_to(&rosc::encoder::encode(&packet).unwrap(), target)
        .unwrap();
}

/// Bridge under test: its OSC receiver, a LiveLink frame sender, and the process.
struct Bridge {
    osc: UdpSocket,
    livelink: UdpSocket,
    livelink_target: String,
    started: Instant,
    _child: ChildGuard,
}

impl Bridge {
    /// Start litelink headless at 50Hz, sending to `osc_port`, plus `extra` args.
    fn start(livelink_port: u16, osc_port: u16, extra: &[&str]) -> Self {
        let osc = UdpSocket::bind(format!("127.0.0.1:{osc_port}")).unwrap();
        osc.set_read_timeout(Some(Duration::from_millis(50))).unwrap();

        let mut cmd = std::process::Command::new(env!("CARGO_BIN_EXE_litelink"));
        cmd.args(["--listen-port", &livelink_port.to_string()])
            .args(["--osc-target", &format!("127.0.0.1:{osc_port}")])
            .args(["--send-rate", "50"])
            .args(extra)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
        if cfg!(feature = "gui") {
            cmd.arg("--headless");
        }
        let started = Instant::now();
        let child = ChildGuard(cmd.spawn().expect("failed to start litelink"));

        Self {
            osc,
            livelink: UdpSocket::bind("127.0.0.1:0").unwrap(),
            livelink_target: format!("127.0.0.1:{livelink_port}"),
            started,
            _child: child,
        }
    }

    fn send_frame(&self, frame: u32, jaw_open: f32) {
        let mut shapes = [0.0f32; BLENDSHAPE_COUNT];
        shapes[17] = jaw_open;
        let packet = build_livelink_packet("TestDevice", "TestSubject", frame, &shapes);
        self.livelink.send_to(&packet, &self.livelink_target).unwrap();
    }

    fn collect(&self, window: Duration) -> Vec<Vec<rosc::OscMessage>> {
        collect_packets(&self.osc, window)
    }

    fn sleep_until(&self, since_start: Duration) {
        let target = self.started + since_start;
        let now = Instant::now();
        assert!(now <= target, "test fell behind its schedule at {since_start:?}");
        std::thread::sleep(target - now);
    }
}

/// Auto mode follows VRChat's avatar flag: both outputs while unknown, blob only
/// for a native avatar, and a full legacy resend (no blob) after switching to a
/// legacy avatar.
#[test]
fn auto_mode_follows_avatar_flag() {
    let listen_addr = "127.0.0.1:19333";
    let bridge = Bridge::start(19311, 19322, &["--output", "auto", "--osc-listen", listen_addr]);
    let vrchat = UdpSocket::bind("127.0.0.1:0").unwrap();

    // (a) Flag unknown: floats and blob both arrive (the blob once the phone is seen).
    std::thread::sleep(Duration::from_millis(200));
    bridge.send_frame(1, 0.85);
    let phase_a = bridge.collect(Duration::from_millis(500));
    let all_params = param_set(phase_a.iter().flatten());
    assert!(
        phase_a.iter().flatten().any(is_blob),
        "no blob before the avatar flag is known"
    );
    assert!(all_params.len() > 90, "expected full float set, got {}", all_params.len());
    assert!(all_params.contains("/avatar/parameters/LipTrackingActive"));

    // (b) Native avatar: blob only, even while the face moves.
    send_osc(&vrchat, listen_addr, "/avatar/avatarfullfacetracking", vec![OscType::Bool(true)]);
    std::thread::sleep(Duration::from_millis(200));
    bridge.collect(Duration::from_millis(100)); // drain the switch-over
    let mut phase_b = Vec::new();
    for frame in 2..=6 {
        bridge.send_frame(frame, 0.1 * (frame - 1) as f32);
        phase_b.extend(bridge.collect(Duration::from_millis(100)));
    }
    assert!(!phase_b.is_empty(), "no packets for a native avatar");
    for msgs in &phase_b {
        assert!(msgs.iter().any(is_blob), "native avatar packet without blob");
        let leaked: Vec<_> = msgs.iter().filter(|m| is_param(m)).map(|m| &m.addr).collect();
        assert!(leaked.is_empty(), "native avatar got legacy params: {leaked:?}");
    }

    // (c) Switch to a legacy avatar: every float is resent, the blob stops.
    // Periodic full resends land near multiples of 2s after start; switching at
    // 2.6s and checking a 300ms window proves the resend came from the avatar
    // change, not the timer.
    bridge.sleep_until(Duration::from_millis(2600));
    send_osc(&vrchat, listen_addr, "/avatar/change", vec![OscType::String("avtr_test".into())]);
    send_osc(&vrchat, listen_addr, "/avatar/avatarfullfacetracking", vec![OscType::Bool(false)]);
    let resent = param_set(bridge.collect(Duration::from_millis(300)).iter().flatten());
    let missing: Vec<_> = all_params.difference(&resent).collect();
    assert!(missing.is_empty(), "avatar change did not resend: {missing:?}");

    let mut phase_c = Vec::new();
    for frame in 10..=13 {
        bridge.send_frame(frame, 0.1 * (frame - 9) as f32);
        phase_c.extend(bridge.collect(Duration::from_millis(100)));
    }
    assert!(
        phase_c.iter().flatten().any(|m| m.addr.ends_with("/JawOpen")),
        "legacy avatar stopped receiving floats"
    );
    assert!(
        !phase_c.iter().flatten().any(is_blob),
        "legacy avatar still receives the blob"
    );

    // (d) Legacy to legacy: floats never paused, so only the avatar change can
    // explain a full resend. The window sits between periodic resends (~2s, ~4s).
    bridge.sleep_until(Duration::from_millis(3500));
    send_osc(&vrchat, listen_addr, "/avatar/change", vec![OscType::String("avtr_two".into())]);
    send_osc(&vrchat, listen_addr, "/avatar/avatarfullfacetracking", vec![OscType::Bool(false)]);
    let resent = param_set(bridge.collect(Duration::from_millis(300)).iter().flatten());
    let missing: Vec<_> = all_params.difference(&resent).collect();
    assert!(missing.is_empty(), "second avatar change did not resend: {missing:?}");
}

/// Constants like the status bools change once per process at most, so only a
/// periodic full resend reaches an avatar (or VRChat) that loads later.
#[test]
fn legacy_mode_resends_everything_periodically() {
    let bridge = Bridge::start(19411, 19422, &["--output", "legacy"]);

    // Idle input: only full resends produce packets.
    let packets = bridge.collect(Duration::from_millis(2600));
    let full: Vec<_> = packets.iter().map(param_set).filter(|set| set.len() > 90).collect();
    assert!(full.len() >= 2, "expected a periodic full resend, got {} full packets", full.len());
    assert_eq!(full[0], full[1], "periodic resend differs from the first full send");
    assert!(full[1].contains("/avatar/parameters/LipTrackingActive"));
}

/// The blob waits for the phone: an all-zero face would override the avatar's
/// own blinks and visemes. After the phone goes quiet the frozen face keeps going.
#[test]
fn blob_waits_for_first_livelink_packet() {
    let bridge = Bridge::start(19511, 19522, &["--output", "native", "--timeout", "0.2"]);

    let before = bridge.collect(Duration::from_millis(400));
    assert!(
        !before.iter().flatten().any(is_blob),
        "blob sent before any LiveLink packet"
    );

    bridge.send_frame(1, 0.85);
    let after = bridge.collect(Duration::from_millis(300));
    let blobs: Vec<_> = after.iter().flatten().filter(|m| is_blob(m)).collect();
    assert!(!blobs.is_empty(), "no blob after the first LiveLink packet");
    let last = blobs.last().unwrap();
    assert!(
        matches!(&last.args[..], [OscType::Blob(b)] if b[14 + 22] == 216),
        "blob does not carry JawOpen 0.85"
    );

    // Past the 0.2s timeout: still sending the frozen face.
    std::thread::sleep(Duration::from_millis(300));
    let frozen = bridge.collect(Duration::from_millis(200));
    assert!(
        frozen.iter().flatten().any(|m| is_blob(m)
            && matches!(&m.args[..], [OscType::Blob(b)] if b[14 + 22] == 216)),
        "blob stopped after disconnect"
    );
}
