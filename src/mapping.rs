use std::borrow::Cow;

use crate::state::BLENDSHAPE_COUNT;

/// An OSC parameter with its address and computed value.
pub struct OscParam<'a> {
    /// Full OSC address, e.g. "/avatar/parameters/FT/v2/JawOpen".
    /// Borrowed from `LegacyParams` on the hot path.
    pub address: Cow<'a, str>,
    /// Parameter value.
    pub value: OscValue,
}

#[derive(Clone, Copy)]
pub enum OscValue {
    Float(f32),
    Bool(bool),
}

// ARKit blendshape indices for readability.
mod idx {
    pub const EYE_BLINK_LEFT: usize = 0;
    pub const EYE_LOOK_DOWN_LEFT: usize = 1;
    pub const EYE_LOOK_IN_LEFT: usize = 2;
    pub const EYE_LOOK_OUT_LEFT: usize = 3;
    pub const EYE_LOOK_UP_LEFT: usize = 4;
    pub const EYE_SQUINT_LEFT: usize = 5;
    pub const EYE_WIDE_LEFT: usize = 6;
    pub const EYE_BLINK_RIGHT: usize = 7;
    pub const EYE_LOOK_DOWN_RIGHT: usize = 8;
    pub const EYE_LOOK_IN_RIGHT: usize = 9;
    pub const EYE_LOOK_OUT_RIGHT: usize = 10;
    pub const EYE_LOOK_UP_RIGHT: usize = 11;
    pub const EYE_SQUINT_RIGHT: usize = 12;
    pub const EYE_WIDE_RIGHT: usize = 13;
    pub const JAW_FORWARD: usize = 14;
    pub const JAW_LEFT: usize = 15;
    pub const JAW_RIGHT: usize = 16;
    pub const JAW_OPEN: usize = 17;
    pub const MOUTH_CLOSE: usize = 18;
    pub const MOUTH_FUNNEL: usize = 19;
    pub const MOUTH_PUCKER: usize = 20;
    pub const MOUTH_LEFT: usize = 21;
    pub const MOUTH_RIGHT: usize = 22;
    pub const MOUTH_SMILE_LEFT: usize = 23;
    pub const MOUTH_SMILE_RIGHT: usize = 24;
    pub const MOUTH_FROWN_LEFT: usize = 25;
    pub const MOUTH_FROWN_RIGHT: usize = 26;
    pub const MOUTH_DIMPLE_LEFT: usize = 27;
    pub const MOUTH_DIMPLE_RIGHT: usize = 28;
    pub const MOUTH_STRETCH_LEFT: usize = 29;
    pub const MOUTH_STRETCH_RIGHT: usize = 30;
    pub const MOUTH_ROLL_LOWER: usize = 31;
    pub const MOUTH_ROLL_UPPER: usize = 32;
    pub const MOUTH_SHRUG_LOWER: usize = 33;
    pub const MOUTH_SHRUG_UPPER: usize = 34;
    pub const MOUTH_PRESS_LEFT: usize = 35;
    pub const MOUTH_PRESS_RIGHT: usize = 36;
    pub const MOUTH_LOWER_DOWN_LEFT: usize = 37;
    pub const MOUTH_LOWER_DOWN_RIGHT: usize = 38;
    pub const MOUTH_UPPER_UP_LEFT: usize = 39;
    pub const MOUTH_UPPER_UP_RIGHT: usize = 40;
    pub const BROW_DOWN_LEFT: usize = 41;
    pub const BROW_DOWN_RIGHT: usize = 42;
    pub const BROW_INNER_UP: usize = 43;
    pub const BROW_OUTER_UP_LEFT: usize = 44;
    pub const BROW_OUTER_UP_RIGHT: usize = 45;
    pub const CHEEK_PUFF: usize = 46;
    pub const CHEEK_SQUINT_LEFT: usize = 47;
    pub const CHEEK_SQUINT_RIGHT: usize = 48;
    pub const NOSE_SNEER_LEFT: usize = 49;
    pub const NOSE_SNEER_RIGHT: usize = 50;
    pub const TONGUE_OUT: usize = 51;
    pub const HEAD_YAW: usize = 52;
    pub const HEAD_PITCH: usize = 53;
    pub const HEAD_ROLL: usize = 54;
    pub const EYE_YAW_LEFT: usize = 55;
    pub const EYE_PITCH_LEFT: usize = 56;
    pub const EYE_YAW_RIGHT: usize = 58;
    pub const EYE_PITCH_RIGHT: usize = 59;
}

fn clamp01(v: f32) -> f32 {
    v.clamp(0.0, 1.0)
}

/// Unified Expressions v2 shapes. The discriminant is the shape's ordinal in
/// VRCFaceTracking's `UnifiedExpressions` enum, which fixes its byte in the
/// native blob. Shapes ARKit cannot produce exist only to hold their slot.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ue {
    EyeSquintRight = 0,
    EyeSquintLeft = 1,
    EyeWideRight = 2,
    EyeWideLeft = 3,
    BrowPinchRight = 4,
    BrowPinchLeft = 5,
    BrowLowererRight = 6,
    BrowLowererLeft = 7,
    BrowInnerUpRight = 8,
    BrowInnerUpLeft = 9,
    BrowOuterUpRight = 10,
    BrowOuterUpLeft = 11,
    NasalDilationRight = 12,
    NasalDilationLeft = 13,
    NasalConstrictRight = 14,
    NasalConstrictLeft = 15,
    CheekSquintRight = 16,
    CheekSquintLeft = 17,
    CheekPuffRight = 18,
    CheekPuffLeft = 19,
    CheekSuckRight = 20,
    CheekSuckLeft = 21,
    JawOpen = 22,
    JawRight = 23,
    JawLeft = 24,
    JawForward = 25,
    JawBackward = 26,
    JawClench = 27,
    JawMandibleRaise = 28,
    MouthClosed = 29,
    LipSuckUpperRight = 30,
    LipSuckUpperLeft = 31,
    LipSuckLowerRight = 32,
    LipSuckLowerLeft = 33,
    LipSuckCornerRight = 34,
    LipSuckCornerLeft = 35,
    LipFunnelUpperRight = 36,
    LipFunnelUpperLeft = 37,
    LipFunnelLowerRight = 38,
    LipFunnelLowerLeft = 39,
    LipPuckerUpperRight = 40,
    LipPuckerUpperLeft = 41,
    LipPuckerLowerRight = 42,
    LipPuckerLowerLeft = 43,
    MouthUpperUpRight = 44,
    MouthUpperUpLeft = 45,
    MouthUpperDeepenRight = 46,
    MouthUpperDeepenLeft = 47,
    NoseSneerRight = 48,
    NoseSneerLeft = 49,
    MouthLowerDownRight = 50,
    MouthLowerDownLeft = 51,
    MouthUpperRight = 52,
    MouthUpperLeft = 53,
    MouthLowerRight = 54,
    MouthLowerLeft = 55,
    MouthCornerPullRight = 56,
    MouthCornerPullLeft = 57,
    MouthCornerSlantRight = 58,
    MouthCornerSlantLeft = 59,
    MouthFrownRight = 60,
    MouthFrownLeft = 61,
    MouthStretchRight = 62,
    MouthStretchLeft = 63,
    MouthDimpleRight = 64,
    MouthDimpleLeft = 65,
    MouthRaiserUpper = 66,
    MouthRaiserLower = 67,
    MouthPressRight = 68,
    MouthPressLeft = 69,
    MouthTightenerRight = 70,
    MouthTightenerLeft = 71,
    TongueOut = 72,
    TongueUp = 73,
    TongueDown = 74,
    TongueRight = 75,
    TongueLeft = 76,
    TongueRoll = 77,
    TongueBendDown = 78,
    TongueCurlUp = 79,
    TongueSquish = 80,
    TongueFlat = 81,
    TongueTwistRight = 82,
    TongueTwistLeft = 83,
    SoftPalateClose = 84,
    ThroatSwallow = 85,
    NeckFlexRight = 86,
    NeckFlexLeft = 87,
}

pub const UE_COUNT: usize = 88;

/// ARKit blendshape → Unified Expressions shape. One ARKit source may feed
/// several shapes (e.g. mouthFunnel drives all four LipFunnel quadrants).
const ARKIT_TO_UE: &[(usize, Ue)] = &[
    (idx::EYE_SQUINT_LEFT, Ue::EyeSquintLeft),
    (idx::EYE_SQUINT_RIGHT, Ue::EyeSquintRight),
    (idx::EYE_WIDE_LEFT, Ue::EyeWideLeft),
    (idx::EYE_WIDE_RIGHT, Ue::EyeWideRight),
    (idx::JAW_FORWARD, Ue::JawForward),
    (idx::JAW_LEFT, Ue::JawLeft),
    (idx::JAW_RIGHT, Ue::JawRight),
    (idx::JAW_OPEN, Ue::JawOpen),
    (idx::MOUTH_CLOSE, Ue::MouthClosed),
    (idx::MOUTH_FROWN_LEFT, Ue::MouthFrownLeft),
    (idx::MOUTH_FROWN_RIGHT, Ue::MouthFrownRight),
    (idx::MOUTH_DIMPLE_LEFT, Ue::MouthDimpleLeft),
    (idx::MOUTH_DIMPLE_RIGHT, Ue::MouthDimpleRight),
    (idx::MOUTH_STRETCH_LEFT, Ue::MouthStretchLeft),
    (idx::MOUTH_STRETCH_RIGHT, Ue::MouthStretchRight),
    (idx::MOUTH_SHRUG_LOWER, Ue::MouthRaiserLower),
    (idx::MOUTH_SHRUG_UPPER, Ue::MouthRaiserUpper),
    (idx::MOUTH_PRESS_LEFT, Ue::MouthPressLeft),
    (idx::MOUTH_PRESS_RIGHT, Ue::MouthPressRight),
    (idx::MOUTH_LOWER_DOWN_LEFT, Ue::MouthLowerDownLeft),
    (idx::MOUTH_LOWER_DOWN_RIGHT, Ue::MouthLowerDownRight),
    (idx::MOUTH_UPPER_UP_LEFT, Ue::MouthUpperUpLeft),
    (idx::MOUTH_UPPER_UP_RIGHT, Ue::MouthUpperUpRight),
    (idx::BROW_OUTER_UP_LEFT, Ue::BrowOuterUpLeft),
    (idx::BROW_OUTER_UP_RIGHT, Ue::BrowOuterUpRight),
    (idx::CHEEK_SQUINT_LEFT, Ue::CheekSquintLeft),
    (idx::CHEEK_SQUINT_RIGHT, Ue::CheekSquintRight),
    (idx::NOSE_SNEER_LEFT, Ue::NoseSneerLeft),
    (idx::NOSE_SNEER_RIGHT, Ue::NoseSneerRight),
    (idx::TONGUE_OUT, Ue::TongueOut),
    (idx::MOUTH_FUNNEL, Ue::LipFunnelUpperLeft),
    (idx::MOUTH_FUNNEL, Ue::LipFunnelUpperRight),
    (idx::MOUTH_FUNNEL, Ue::LipFunnelLowerLeft),
    (idx::MOUTH_FUNNEL, Ue::LipFunnelLowerRight),
    (idx::MOUTH_PUCKER, Ue::LipPuckerUpperLeft),
    (idx::MOUTH_PUCKER, Ue::LipPuckerUpperRight),
    (idx::MOUTH_PUCKER, Ue::LipPuckerLowerLeft),
    (idx::MOUTH_PUCKER, Ue::LipPuckerLowerRight),
    (idx::MOUTH_LEFT, Ue::MouthUpperLeft),
    (idx::MOUTH_LEFT, Ue::MouthLowerLeft),
    (idx::MOUTH_RIGHT, Ue::MouthUpperRight),
    (idx::MOUTH_RIGHT, Ue::MouthLowerRight),
    (idx::MOUTH_SMILE_LEFT, Ue::MouthCornerPullLeft),
    (idx::MOUTH_SMILE_LEFT, Ue::MouthCornerSlantLeft),
    (idx::MOUTH_SMILE_RIGHT, Ue::MouthCornerPullRight),
    (idx::MOUTH_SMILE_RIGHT, Ue::MouthCornerSlantRight),
    (idx::MOUTH_ROLL_LOWER, Ue::LipSuckLowerLeft),
    (idx::MOUTH_ROLL_LOWER, Ue::LipSuckLowerRight),
    (idx::MOUTH_ROLL_UPPER, Ue::LipSuckUpperLeft),
    (idx::MOUTH_ROLL_UPPER, Ue::LipSuckUpperRight),
    (idx::BROW_DOWN_LEFT, Ue::BrowLowererLeft),
    (idx::BROW_DOWN_LEFT, Ue::BrowPinchLeft),
    (idx::BROW_DOWN_RIGHT, Ue::BrowLowererRight),
    (idx::BROW_DOWN_RIGHT, Ue::BrowPinchRight),
    (idx::BROW_INNER_UP, Ue::BrowInnerUpLeft),
    (idx::BROW_INNER_UP, Ue::BrowInnerUpRight),
    (idx::CHEEK_PUFF, Ue::CheekPuffLeft),
    (idx::CHEEK_PUFF, Ue::CheekPuffRight),
];

/// Eye openness and gaze blendshapes, clamped to 0..1.
#[derive(Clone, Copy, Default)]
pub struct EyeShapes {
    pub look_out_left: f32,
    pub look_in_left: f32,
    pub look_up_left: f32,
    pub look_down_left: f32,
    pub look_out_right: f32,
    pub look_in_right: f32,
    pub look_up_right: f32,
    pub look_down_right: f32,
    pub closed_left: f32,
    pub closed_right: f32,
}

/// Raw head and eye rotation in radians, unclamped.
#[derive(Clone, Copy, Default)]
pub struct Pose {
    pub head_yaw: f32,
    pub head_pitch: f32,
    pub head_roll: f32,
    pub eye_yaw_left: f32,
    pub eye_pitch_left: f32,
    pub eye_yaw_right: f32,
    pub eye_pitch_right: f32,
}

/// One frame of face data in Unified Expressions terms. Both the legacy float
/// params and the native blob read from this, so they cannot disagree.
pub struct UnifiedFace {
    /// Indexed by `Ue` ordinal, clamped to 0..1. Shapes with no ARKit source stay 0.
    pub shapes: [f32; UE_COUNT],
    pub eye: EyeShapes,
    pub pose: Pose,
}

impl UnifiedFace {
    pub fn from_arkit(bs: &[f32; BLENDSHAPE_COUNT]) -> Self {
        let mut shapes = [0.0; UE_COUNT];
        for &(src, ue) in ARKIT_TO_UE {
            shapes[ue as usize] = clamp01(bs[src]);
        }
        let eye = EyeShapes {
            look_out_left: clamp01(bs[idx::EYE_LOOK_OUT_LEFT]),
            look_in_left: clamp01(bs[idx::EYE_LOOK_IN_LEFT]),
            look_up_left: clamp01(bs[idx::EYE_LOOK_UP_LEFT]),
            look_down_left: clamp01(bs[idx::EYE_LOOK_DOWN_LEFT]),
            look_out_right: clamp01(bs[idx::EYE_LOOK_OUT_RIGHT]),
            look_in_right: clamp01(bs[idx::EYE_LOOK_IN_RIGHT]),
            look_up_right: clamp01(bs[idx::EYE_LOOK_UP_RIGHT]),
            look_down_right: clamp01(bs[idx::EYE_LOOK_DOWN_RIGHT]),
            closed_left: clamp01(bs[idx::EYE_BLINK_LEFT]),
            closed_right: clamp01(bs[idx::EYE_BLINK_RIGHT]),
        };
        let pose = Pose {
            head_yaw: bs[idx::HEAD_YAW],
            head_pitch: bs[idx::HEAD_PITCH],
            head_roll: bs[idx::HEAD_ROLL],
            eye_yaw_left: bs[idx::EYE_YAW_LEFT],
            eye_pitch_left: bs[idx::EYE_PITCH_LEFT],
            eye_yaw_right: bs[idx::EYE_YAW_RIGHT],
            eye_pitch_right: bs[idx::EYE_PITCH_RIGHT],
        };
        Self { shapes, eye, pose }
    }

    pub fn shape(&self, ue: Ue) -> f32 {
        self.shapes[ue as usize]
    }
}

/// Length VRChat requires for `/tracking/face/v1`; it drops any other size.
pub const NATIVE_BLOB_LEN: usize = 102;
/// Shapes start after the 14 eye bytes, at `NATIVE_SHAPES_OFFSET + ordinal`.
const NATIVE_SHAPES_OFFSET: usize = 14;

/// Matches VRCFT's C# `(byte)(x * 255)`: clamp, scale, truncate.
fn to_byte(v: f32) -> u8 {
    (clamp01(v) * 255.0) as u8
}

/// Build the `/tracking/face/v1` payload, mirroring VRCFT's `UnifiedData.CopyTo`.
///
/// Layout:
///   0-3   EyeLook Out/In/Up/Down Right
///   4-7   EyeLook Out/In/Up/Down Left
///   8-9   EyeClosed Right, Left
///   10-13 EyeDilation R/L, EyeConstrict R/L (ARKit has none: always 0)
///   14-101 the 88 Unified Expressions shapes by ordinal
pub fn native_face_blob(face: &UnifiedFace) -> [u8; NATIVE_BLOB_LEN] {
    let e = &face.eye;
    let mut blob = [0u8; NATIVE_BLOB_LEN];
    let eye_bytes = [
        e.look_out_right,
        e.look_in_right,
        e.look_up_right,
        e.look_down_right,
        e.look_out_left,
        e.look_in_left,
        e.look_up_left,
        e.look_down_left,
        e.closed_right,
        e.closed_left,
    ];
    for (b, v) in blob.iter_mut().zip(eye_bytes) {
        *b = to_byte(v);
    }
    for (b, &v) in blob[NATIVE_SHAPES_OFFSET..].iter_mut().zip(&face.shapes) {
        *b = to_byte(v);
    }
    blob
}

/// Legacy float params emitted 1:1 from a shape, in their historical send order.
const LEGACY_DIRECT: &[(Ue, &str)] = &[
    (Ue::EyeSquintLeft, "EyeSquintLeft"),
    (Ue::EyeWideLeft, "EyeWideLeft"),
    (Ue::EyeSquintRight, "EyeSquintRight"),
    (Ue::EyeWideRight, "EyeWideRight"),
    (Ue::JawForward, "JawForward"),
    (Ue::JawLeft, "JawLeft"),
    (Ue::JawRight, "JawRight"),
    (Ue::JawOpen, "JawOpen"),
    (Ue::MouthClosed, "MouthClosed"),
    (Ue::MouthFrownLeft, "MouthFrownLeft"),
    (Ue::MouthFrownRight, "MouthFrownRight"),
    (Ue::MouthDimpleLeft, "MouthDimpleLeft"),
    (Ue::MouthDimpleRight, "MouthDimpleRight"),
    (Ue::MouthStretchLeft, "MouthStretchLeft"),
    (Ue::MouthStretchRight, "MouthStretchRight"),
    (Ue::MouthRaiserLower, "MouthRaiserLower"),
    (Ue::MouthRaiserUpper, "MouthRaiserUpper"),
    (Ue::MouthPressLeft, "MouthPressLeft"),
    (Ue::MouthPressRight, "MouthPressRight"),
    (Ue::MouthLowerDownLeft, "MouthLowerDownLeft"),
    (Ue::MouthLowerDownRight, "MouthLowerDownRight"),
    (Ue::MouthUpperUpLeft, "MouthUpperUpLeft"),
    (Ue::MouthUpperUpRight, "MouthUpperUpRight"),
    (Ue::BrowOuterUpLeft, "BrowOuterUpLeft"),
    (Ue::BrowOuterUpRight, "BrowOuterUpRight"),
    (Ue::CheekSquintLeft, "CheekSquintLeft"),
    (Ue::CheekSquintRight, "CheekSquintRight"),
    (Ue::NoseSneerLeft, "NoseSneerLeft"),
    (Ue::NoseSneerRight, "NoseSneerRight"),
    (Ue::TongueOut, "TongueOut"),
];

/// Second run of 1:1 shape params, sent after the eye-look params.
const LEGACY_FANOUT: &[(Ue, &str)] = &[
    (Ue::LipFunnelUpperLeft, "LipFunnelUpperLeft"),
    (Ue::LipFunnelUpperRight, "LipFunnelUpperRight"),
    (Ue::LipFunnelLowerLeft, "LipFunnelLowerLeft"),
    (Ue::LipFunnelLowerRight, "LipFunnelLowerRight"),
    (Ue::LipPuckerUpperLeft, "LipPuckerUpperLeft"),
    (Ue::LipPuckerUpperRight, "LipPuckerUpperRight"),
    (Ue::LipPuckerLowerLeft, "LipPuckerLowerLeft"),
    (Ue::LipPuckerLowerRight, "LipPuckerLowerRight"),
    (Ue::MouthUpperLeft, "MouthUpperLeft"),
    (Ue::MouthLowerLeft, "MouthLowerLeft"),
    (Ue::MouthUpperRight, "MouthUpperRight"),
    (Ue::MouthLowerRight, "MouthLowerRight"),
    (Ue::MouthCornerPullLeft, "MouthCornerPullLeft"),
    (Ue::MouthCornerSlantLeft, "MouthCornerSlantLeft"),
    (Ue::MouthCornerPullRight, "MouthCornerPullRight"),
    (Ue::MouthCornerSlantRight, "MouthCornerSlantRight"),
    (Ue::LipSuckLowerLeft, "LipSuckLowerLeft"),
    (Ue::LipSuckLowerRight, "LipSuckLowerRight"),
    (Ue::LipSuckUpperLeft, "LipSuckUpperLeft"),
    (Ue::LipSuckUpperRight, "LipSuckUpperRight"),
    (Ue::BrowLowererLeft, "BrowLowererLeft"),
    (Ue::BrowPinchLeft, "BrowPinchLeft"),
    (Ue::BrowLowererRight, "BrowLowererRight"),
    (Ue::BrowPinchRight, "BrowPinchRight"),
    (Ue::BrowInnerUpLeft, "BrowInnerUpLeft"),
    (Ue::BrowInnerUpRight, "BrowInnerUpRight"),
    (Ue::CheekPuffLeft, "CheekPuffLeft"),
    (Ue::CheekPuffRight, "CheekPuffRight"),
];

/// Build the full list of OSC parameters from 61 ARKit blendshapes, owning
/// their addresses. Runs the same path as production.
/// `prefix` is the base path, e.g. "/avatar/parameters/FT/v2" or "/avatar/parameters/v2".
#[cfg(test)]
pub fn map_blendshapes(bs: &[f32; BLENDSHAPE_COUNT], prefix: &str) -> Vec<OscParam<'static>> {
    LegacyParams::new(prefix)
        .params(&UnifiedFace::from_arkit(bs))
        .into_iter()
        .map(|p| OscParam {
            address: Cow::Owned(p.address.into_owned()),
            value: p.value,
        })
        .collect()
}

/// Legacy param addresses, formatted once from the prefix so a tick only
/// computes values.
pub struct LegacyParams {
    addresses: Vec<String>,
}

impl LegacyParams {
    pub fn new(prefix: &str) -> Self {
        let mut addresses = Vec::with_capacity(128);
        // The emit sequence never depends on values, so a blank face yields
        // the same names in the same order as every later tick.
        legacy_values(&UnifiedFace::from_arkit(&[0.0; BLENDSHAPE_COUNT]), &mut |name, _| {
            // Status params carry absolute addresses; the rest hang off the prefix.
            addresses.push(if name.starts_with('/') {
                name.to_string()
            } else {
                format!("{prefix}/{name}")
            });
        });
        Self { addresses }
    }

    /// The legacy VRCFT-style float params plus the status bools, in send order.
    pub fn params(&self, face: &UnifiedFace) -> Vec<OscParam<'_>> {
        let mut out = Vec::with_capacity(self.addresses.len());
        let mut addresses = self.addresses.iter();
        legacy_values(face, &mut |name, value| {
            let address = addresses.next().expect("legacy param count changed");
            debug_assert!(address.ends_with(name), "{address} is not {name}");
            out.push(OscParam {
                address: Cow::Borrowed(address),
                value,
            });
        });
        out
    }
}

/// Emit every legacy param as (name, value) in a fixed order. Names are
/// relative to the prefix, except status params, which are absolute.
fn legacy_values(face: &UnifiedFace, emit: &mut impl FnMut(&'static str, OscValue)) {
    let mut float = |name: &'static str, value: f32| emit(name, OscValue::Float(value));
    let s = |ue: Ue| face.shape(ue);
    let eye = &face.eye;
    let pose = &face.pose;

    for &(ue, name) in LEGACY_DIRECT {
        float(name, s(ue));
    }

    float("EyeLookDownLeft", eye.look_down_left);
    float("EyeLookInLeft", eye.look_in_left);
    float("EyeLookOutLeft", eye.look_out_left);
    float("EyeLookUpLeft", eye.look_up_left);
    float("EyeLookDownRight", eye.look_down_right);
    float("EyeLookInRight", eye.look_in_right);
    float("EyeLookOutRight", eye.look_out_right);
    float("EyeLookUpRight", eye.look_up_right);

    for &(ue, name) in LEGACY_FANOUT {
        float(name, s(ue));
    }

    // --- Derived/combined parameters ---
    let eye_blink_l = eye.closed_left;
    let eye_blink_r = eye.closed_right;
    let eye_open_l = 1.0 - eye_blink_l;
    let eye_open_r = 1.0 - eye_blink_r;

    float("EyeOpenLeft", eye_open_l);
    float("EyeOpenRight", eye_open_r);
    float("EyeOpen", (eye_open_l + eye_open_r) * 0.5);
    float("EyeClosedLeft", eye_blink_l);
    float("EyeClosedRight", eye_blink_r);
    float("EyeClosed", (eye_blink_l + eye_blink_r) * 0.5);

    // Eye lid (75% openness + 25% wide, matching VRCFT)
    let wide_l = s(Ue::EyeWideLeft);
    let wide_r = s(Ue::EyeWideRight);
    float("EyeLidLeft", eye_open_l * 0.75 + wide_l * 0.25);
    float("EyeLidRight", eye_open_r * 0.75 + wide_r * 0.25);
    float("EyeLid", ((eye_open_l + eye_open_r) * 0.5) * 0.75 + ((wide_l + wide_r) * 0.5) * 0.25);

    // Eye gaze (radians, not clamped)
    float("EyeLeftX", pose.eye_yaw_left);
    float("EyeLeftY", -pose.eye_pitch_left);
    float("EyeRightX", pose.eye_yaw_right);
    float("EyeRightY", -pose.eye_pitch_right);
    // Combined eye Y (average of left and right)
    float("EyeY", -(pose.eye_pitch_left + pose.eye_pitch_right) * 0.5);

    // Head pose (radians, not clamped)
    float("HeadYaw", pose.head_yaw);
    float("HeadPitch", pose.head_pitch);
    float("HeadRoll", pose.head_roll);

    // Aggregates
    float("EyeSquint", s(Ue::EyeSquintLeft).max(s(Ue::EyeSquintRight)));
    float("EyeWide", wide_l.max(wide_r));

    // Jaw derived
    float("JawX", s(Ue::JawRight) - s(Ue::JawLeft));
    float("JawZ", s(Ue::JawForward));

    // Mouth derived
    let mouth_left = s(Ue::MouthUpperLeft);
    let mouth_right = s(Ue::MouthUpperRight);
    float("MouthX", mouth_right - mouth_left);

    let smile_l = s(Ue::MouthCornerPullLeft);
    let smile_r = s(Ue::MouthCornerPullRight);
    let frown_l = s(Ue::MouthFrownLeft);
    let frown_r = s(Ue::MouthFrownRight);
    let smile_avg = (smile_l + smile_r) * 0.5;
    let frown_avg = (frown_l + frown_r) * 0.5;
    float("SmileFrown", smile_avg - frown_avg);
    float("SmileFrownLeft", smile_l - frown_l);
    float("SmileFrownRight", smile_r - frown_r);

    let roll_lower = s(Ue::LipSuckLowerLeft);
    let roll_upper = s(Ue::LipSuckUpperLeft);
    float("LipFunnel", s(Ue::LipFunnelUpperLeft));
    float("LipPucker", s(Ue::LipPuckerUpperLeft));
    float("LipSuck", (roll_lower + roll_upper) * 0.5);
    float("LipSuckUpper", roll_upper);
    float("LipSuckLower", roll_lower);

    let cheek_puff = s(Ue::CheekPuffLeft);
    float("CheekPuffSuck", cheek_puff);
    float("CheekPuffSuckLeft", cheek_puff);
    float("CheekPuffSuckRight", cheek_puff);

    float("NoseSneer", (s(Ue::NoseSneerLeft) + s(Ue::NoseSneerRight)) * 0.5);

    let brow_inner = s(Ue::BrowInnerUpLeft);
    let brow_outer_l = s(Ue::BrowOuterUpLeft);
    let brow_outer_r = s(Ue::BrowOuterUpRight);
    let brow_down_l = s(Ue::BrowLowererLeft);
    let brow_down_r = s(Ue::BrowLowererRight);
    float("BrowInnerUp", brow_inner);
    float("BrowOuterUp", (brow_outer_l + brow_outer_r) * 0.5);
    float("BrowDown", (brow_down_l + brow_down_r) * 0.5);

    // BrowExpression: (innerUp + outerUp) * 0.5 - browDown (matching VRCFT)
    let brow_up_l = brow_inner * 0.5 + brow_outer_l * 0.5;
    let brow_up_r = brow_inner * 0.5 + brow_outer_r * 0.5;
    float("BrowExpressionLeft", brow_up_l.min(1.0) - (brow_down_l * 0.75 + brow_down_l * 0.25));
    float("BrowExpressionRight", brow_up_r.min(1.0) - (brow_down_r * 0.75 + brow_down_r * 0.25));

    // MouthOpen, MouthUpperUp, MouthLowerDown, MouthPress (combined)
    let lower_down_l = s(Ue::MouthLowerDownLeft);
    let lower_down_r = s(Ue::MouthLowerDownRight);
    let upper_up_l = s(Ue::MouthUpperUpLeft);
    let upper_up_r = s(Ue::MouthUpperUpRight);
    float("MouthOpen", (upper_up_l + upper_up_r + lower_down_l + lower_down_r) * 0.25);
    float("MouthUpperUp", (upper_up_l + upper_up_r) * 0.5);
    float("MouthLowerDown", (lower_down_l + lower_down_r) * 0.5);
    float("MouthPress", (s(Ue::MouthPressLeft) + s(Ue::MouthPressRight)) * 0.5);
    float("MouthStretch", (s(Ue::MouthStretchLeft) + s(Ue::MouthStretchRight)) * 0.5);

    // --- Status parameters (no prefix, these are standard) ---
    // Always report tracking as active so VRChat doesn't disable face tracking
    // on brief disconnects. Data simply freezes until packets resume.
    emit("/avatar/parameters/EyeTrackingActive", OscValue::Bool(true));
    emit("/avatar/parameters/ExpressionTrackingActive", OscValue::Bool(true));
    emit("/avatar/parameters/LipTrackingActive", OscValue::Bool(true));
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_PREFIX: &str = "/avatar/parameters/FT/v2";

    fn find_float(params: &[OscParam], suffix: &str) -> f32 {
        params
            .iter()
            .find(|p| p.address.ends_with(suffix))
            .map(|p| match p.value {
                OscValue::Float(v) => v,
                _ => panic!("expected float for {suffix}"),
            })
            .unwrap_or_else(|| panic!("param {suffix} not found"))
    }

    #[test]
    fn mapping_produces_params() {
        let bs = [0.0f32; BLENDSHAPE_COUNT];
        let params = map_blendshapes(&bs, DEFAULT_PREFIX);
        assert!(params.len() > 90, "expected many params, got {}", params.len());
    }

    #[test]
    fn eye_open_inverse_of_blink() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::EYE_BLINK_LEFT] = 0.7;
        bs[idx::EYE_BLINK_RIGHT] = 0.3;
        let params = map_blendshapes(&bs, DEFAULT_PREFIX);

        assert!((find_float(&params, "EyeOpenLeft") - 0.3).abs() < f32::EPSILON);
        assert!((find_float(&params, "EyeOpenRight") - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn jaw_x_is_right_minus_left() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::JAW_RIGHT] = 0.8;
        bs[idx::JAW_LEFT] = 0.3;
        let params = map_blendshapes(&bs, DEFAULT_PREFIX);
        assert!((find_float(&params, "JawX") - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn status_params_use_no_prefix() {
        let bs = [0.0f32; BLENDSHAPE_COUNT];
        let params = map_blendshapes(&bs, DEFAULT_PREFIX);
        let s = params.iter().find(|p| p.address.contains("EyeTrackingActive")).unwrap();
        assert_eq!(s.address, "/avatar/parameters/EyeTrackingActive");
        assert!(matches!(s.value, OscValue::Bool(true)));
    }

    #[test]
    fn status_params_always_true() {
        let bs = [0.0f32; BLENDSHAPE_COUNT];
        let params = map_blendshapes(&bs, DEFAULT_PREFIX);
        let s = params.iter().find(|p| p.address.contains("LipTrackingActive")).unwrap();
        assert!(matches!(s.value, OscValue::Bool(true)));
    }

    #[test]
    fn mouth_funnel_fans_out_to_four() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::MOUTH_FUNNEL] = 0.6;
        let params = map_blendshapes(&bs, DEFAULT_PREFIX);
        let funnel_params: Vec<_> = params
            .iter()
            .filter(|p| {
                let name = p.address.rsplit('/').next().unwrap_or("");
                name.starts_with("LipFunnel") && name != "LipFunnel"
            })
            .collect();
        assert_eq!(funnel_params.len(), 4);
        for p in &funnel_params {
            match p.value {
                OscValue::Float(v) => assert!((v - 0.6).abs() < f32::EPSILON),
                _ => panic!("expected float"),
            }
        }
    }

    #[test]
    fn smile_frown_derived() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::MOUTH_SMILE_LEFT] = 0.8;
        bs[idx::MOUTH_SMILE_RIGHT] = 0.6;
        bs[idx::MOUTH_FROWN_LEFT] = 0.1;
        bs[idx::MOUTH_FROWN_RIGHT] = 0.2;
        let params = map_blendshapes(&bs, DEFAULT_PREFIX);

        assert!((find_float(&params, "SmileFrown") - 0.55).abs() < 1e-6);
        assert!((find_float(&params, "SmileFrownLeft") - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn eye_gaze_negated_pitch() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::EYE_PITCH_LEFT] = 0.3;
        bs[idx::EYE_PITCH_RIGHT] = -0.2;
        let params = map_blendshapes(&bs, DEFAULT_PREFIX);

        assert!((find_float(&params, "EyeLeftY") - (-0.3)).abs() < f32::EPSILON);
        assert!((find_float(&params, "EyeRightY") - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn clamping_caps_above_one() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::JAW_OPEN] = 1.05;
        let params = map_blendshapes(&bs, DEFAULT_PREFIX);
        assert!((find_float(&params, "/JawOpen") - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn clamping_floors_below_zero() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::JAW_OPEN] = -0.01;
        let params = map_blendshapes(&bs, DEFAULT_PREFIX);
        assert!((find_float(&params, "/JawOpen")).abs() < f32::EPSILON);
    }

    #[test]
    fn eye_look_params_present() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::EYE_LOOK_DOWN_LEFT] = 0.4;
        bs[idx::EYE_LOOK_IN_RIGHT] = 0.6;
        let params = map_blendshapes(&bs, DEFAULT_PREFIX);

        assert!((find_float(&params, "EyeLookDownLeft") - 0.4).abs() < f32::EPSILON);
        assert!((find_float(&params, "EyeLookInRight") - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn head_pose_params_present() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::HEAD_YAW] = 0.3;
        bs[idx::HEAD_PITCH] = -0.2;
        bs[idx::HEAD_ROLL] = 0.1;
        let params = map_blendshapes(&bs, DEFAULT_PREFIX);

        assert!((find_float(&params, "HeadYaw") - 0.3).abs() < f32::EPSILON);
        assert!((find_float(&params, "HeadPitch") - (-0.2)).abs() < f32::EPSILON);
        assert!((find_float(&params, "HeadRoll") - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn configurable_prefix() {
        let bs = [0.0f32; BLENDSHAPE_COUNT];
        let params = map_blendshapes(&bs, "/avatar/parameters/v2");
        let jaw = params.iter().find(|p| p.address.contains("JawOpen")).unwrap();
        assert!(jaw.address.starts_with("/avatar/parameters/v2/"));

        let params2 = map_blendshapes(&bs, "/avatar/parameters/FT/v2");
        let jaw2 = params2.iter().find(|p| p.address.contains("JawOpen")).unwrap();
        assert!(jaw2.address.starts_with("/avatar/parameters/FT/v2/"));
    }

    /// Deterministic xorshift32 so golden inputs never depend on a crate or platform.
    fn xorshift(state: &mut u32) -> u32 {
        *state ^= *state << 13;
        *state ^= *state >> 17;
        *state ^= *state << 5;
        *state
    }

    /// Golden inputs: edge-value fills plus pseudo-random frames. Expression slots
    /// span -0.3..1.4 to exercise clamping; pose slots (52..61) span -1.5..1.5 rad.
    fn golden_inputs() -> Vec<([f32; BLENDSHAPE_COUNT], &'static str)> {
        let mut cases = vec![
            ([0.0; BLENDSHAPE_COUNT], DEFAULT_PREFIX),
            ([-0.2; BLENDSHAPE_COUNT], DEFAULT_PREFIX),
            ([1.3; BLENDSHAPE_COUNT], DEFAULT_PREFIX),
            ([0.5; BLENDSHAPE_COUNT], "/avatar/parameters/v2"),
        ];
        let mut rng = 0x1234_5678u32;
        for _ in 0..12 {
            let mut bs = [0.0f32; BLENDSHAPE_COUNT];
            for (i, v) in bs.iter_mut().enumerate() {
                let unit = (xorshift(&mut rng) >> 8) as f32 / (1u32 << 24) as f32;
                *v = if i >= idx::HEAD_YAW { unit * 3.0 - 1.5 } else { unit * 1.7 - 0.3 };
            }
            cases.push((bs, DEFAULT_PREFIX));
        }
        cases
    }

    /// One line per param: address and exact f32 bit pattern (or bool), so the
    /// comparison is bit-exact rather than epsilon-based.
    fn render_golden() -> String {
        let mut out = String::new();
        for (n, (bs, prefix)) in golden_inputs().iter().enumerate() {
            out.push_str(&format!("# case {n}\n"));
            for p in map_blendshapes(bs, prefix) {
                let v = match p.value {
                    OscValue::Float(f) => format!("f {:08x} {f}", f.to_bits()),
                    OscValue::Bool(b) => format!("b {b}"),
                };
                out.push_str(&format!("{} {v}\n", p.address));
            }
        }
        out
    }

    /// Guards the legacy float output against any change in address, order, or value.
    /// Fixture was generated from the pre-refactor mapping. Regenerate only on an
    /// intentional output change: `LITELINK_BLESS_GOLDEN=1 cargo nextest run golden`.
    #[test]
    fn legacy_output_matches_golden() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/legacy_params.golden");
        let actual = render_golden();
        if std::env::var_os("LITELINK_BLESS_GOLDEN").is_some() {
            std::fs::create_dir_all(std::path::Path::new(path).parent().unwrap()).unwrap();
            std::fs::write(path, &actual).unwrap();
        }
        let expected = std::fs::read_to_string(path).expect("missing golden fixture");
        for (i, (a, e)) in actual.lines().zip(expected.lines()).enumerate() {
            assert_eq!(a, e, "golden mismatch at line {}", i + 1);
        }
        assert_eq!(actual.lines().count(), expected.lines().count(), "golden line count differs");
    }

    fn blob_of(bs: &[f32; BLENDSHAPE_COUNT]) -> [u8; NATIVE_BLOB_LEN] {
        native_face_blob(&UnifiedFace::from_arkit(bs))
    }

    #[test]
    fn blob_is_102_bytes() {
        assert_eq!(blob_of(&[0.0; BLENDSHAPE_COUNT]).len(), 102);
    }

    #[test]
    fn blob_places_shapes_at_ordinal() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::JAW_OPEN] = 0.85;
        bs[idx::MOUTH_SMILE_RIGHT] = 0.5;
        let blob = blob_of(&bs);
        assert_eq!(blob[14 + 22], 216, "JawOpen: (0.85 * 255) truncated");
        assert_eq!(blob[14 + 56], 127, "MouthCornerPullRight");
        assert_eq!(blob[14 + 58], 127, "MouthCornerSlantRight");
        let nonzero = blob.iter().filter(|&&b| b != 0).count();
        assert_eq!(nonzero, 3, "nothing else should be set");
    }

    #[test]
    fn blob_fans_out_like_legacy() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::MOUTH_FUNNEL] = 1.0;
        bs[idx::BROW_DOWN_LEFT] = 1.0;
        bs[idx::MOUTH_SHRUG_UPPER] = 1.0;
        let blob = blob_of(&bs);
        for ordinal in [36, 37, 38, 39, 7, 5, 66] {
            assert_eq!(blob[14 + ordinal], 255, "ordinal {ordinal}");
        }
    }

    #[test]
    fn blob_eye_byte_order() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        // Distinct value per source, so a swapped byte shows up as a wrong value.
        let sources = [
            idx::EYE_LOOK_OUT_RIGHT,
            idx::EYE_LOOK_IN_RIGHT,
            idx::EYE_LOOK_UP_RIGHT,
            idx::EYE_LOOK_DOWN_RIGHT,
            idx::EYE_LOOK_OUT_LEFT,
            idx::EYE_LOOK_IN_LEFT,
            idx::EYE_LOOK_UP_LEFT,
            idx::EYE_LOOK_DOWN_LEFT,
            idx::EYE_BLINK_RIGHT,
            idx::EYE_BLINK_LEFT,
        ];
        for (n, &src) in sources.iter().enumerate() {
            bs[src] = (n + 1) as f32 * 0.05;
        }
        let blob = blob_of(&bs);
        for (n, _) in sources.iter().enumerate() {
            let expected = ((n + 1) as f32 * 0.05 * 255.0) as u8;
            assert_eq!(blob[n], expected, "eye byte {n}");
        }
        assert_eq!(&blob[10..14], &[0, 0, 0, 0], "dilation/constrict unsupported");
    }

    #[test]
    fn blob_clamps() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::JAW_OPEN] = 1.3;
        bs[idx::TONGUE_OUT] = -0.2;
        bs[idx::EYE_BLINK_LEFT] = 7.0;
        let blob = blob_of(&bs);
        assert_eq!(blob[14 + 22], 255);
        assert_eq!(blob[14 + 72], 0);
        assert_eq!(blob[9], 255);
    }

    #[test]
    fn blob_unsupported_shapes_stay_zero() {
        // Every ARKit expression at 1.0; pose radians at an extreme too.
        let bs = [1.0f32; BLENDSHAPE_COUNT];
        let blob = blob_of(&bs);
        let mut unsupported = vec![12, 13, 14, 15, 20, 21, 26, 27, 28, 34, 35, 46, 47, 70, 71];
        unsupported.extend(73..=87);
        for ordinal in 0..UE_COUNT {
            let expected = if unsupported.contains(&ordinal) { 0 } else { 255 };
            assert_eq!(blob[14 + ordinal], expected, "ordinal {ordinal}");
        }
        assert_eq!(&blob[..10], &[255; 10]);
        assert_eq!(&blob[10..14], &[0; 4]);
    }

    #[test]
    fn eye_lid_combined() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::EYE_BLINK_LEFT] = 0.2; // openness = 0.8
        bs[idx::EYE_WIDE_LEFT] = 0.4;
        let params = map_blendshapes(&bs, DEFAULT_PREFIX);
        // EyeLidLeft = 0.8 * 0.75 + 0.4 * 0.25 = 0.6 + 0.1 = 0.7
        assert!((find_float(&params, "EyeLidLeft") - 0.7).abs() < 1e-6);
    }
}
