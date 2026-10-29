use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU64, Ordering};
use std::time::{Duration, Instant};

pub const BLENDSHAPE_COUNT: usize = 61;

/// Shared tracking state between UDP receiver and OSC sender.
///
/// Protected by `RwLock` - receiver writes, sender reads.
pub struct TrackingState {
    /// The 61 ARKit blendshape values (0.0-1.0 for expressions, radians for head/eye pose).
    pub blendshapes: [f32; BLENDSHAPE_COUNT],
    /// Device identifier from the LiveLink packet header.
    pub device_id: String,
    /// Subject name from the LiveLink packet header.
    pub subject_name: String,
    /// Frame number from the most recent packet.
    pub frame_number: u32,
    /// Timestamp of the last received packet.
    pub last_packet_time: Option<Instant>,
    /// Total packets received (for stats).
    pub packets_received: u64,
}

/// Connection status as an atomic - no lock needed to read/write.
pub static CONNECTED: AtomicBool = AtomicBool::new(false);

/// Global shutdown flag, set by Ctrl+C handler or GUI close.
pub static SHUTDOWN: AtomicBool = AtomicBool::new(false);

/// Total OSC bundles sent (atomic so sender can increment without write lock).
pub static BUNDLES_SENT: AtomicU64 = AtomicU64::new(0);

/// VRChat's `/avatar/avatarfullfacetracking` flag for the current avatar,
/// encoded as 0 = unknown, 1 = false, 2 = true. Read through `avatar_native()`.
static AVATAR_NATIVE: AtomicU8 = AtomicU8::new(0);

/// Set on `/avatar/change`; the sender clears its change cache and resends everything.
pub static RESEND_ALL: AtomicBool = AtomicBool::new(false);

/// Whether the current avatar uses native face tracking. `None` until VRChat says.
pub fn avatar_native() -> Option<bool> {
    match AVATAR_NATIVE.load(Ordering::Relaxed) {
        1 => Some(false),
        2 => Some(true),
        _ => None,
    }
}

pub fn set_avatar_native(flag: Option<bool>) {
    let v = match flag {
        None => 0,
        Some(false) => 1,
        Some(true) => 2,
    };
    AVATAR_NATIVE.store(v, Ordering::Relaxed);
}

impl TrackingState {
    pub fn new() -> Self {
        Self {
            blendshapes: [0.0; BLENDSHAPE_COUNT],
            device_id: String::new(),
            subject_name: String::new(),
            frame_number: 0,
            last_packet_time: None,
            packets_received: 0,
        }
    }

    /// Check timeout and update CONNECTED atomic. Returns true if state changed.
    pub fn check_timeout(&self, timeout: Duration) -> bool {
        let was_connected = CONNECTED.load(Ordering::Relaxed);
        let is_connected = self
            .last_packet_time
            .is_some_and(|t| t.elapsed() <= timeout);
        CONNECTED.store(is_connected, Ordering::Relaxed);
        was_connected != is_connected
    }

    /// Mark as connected (called by receiver on successful parse).
    pub fn mark_connected(&mut self) {
        self.last_packet_time = Some(Instant::now());
        self.packets_received += 1;
        CONNECTED.store(true, Ordering::Relaxed);
    }
}
