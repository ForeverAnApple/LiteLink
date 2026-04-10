use crate::state::BLENDSHAPE_COUNT;

/// An OSC parameter with its address and computed value.
pub struct OscParam {
    /// Full OSC address, e.g. "/avatar/parameters/FT/v2/JawOpen"
    pub address: String,
    /// Parameter value.
    pub value: OscValue,
}

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

/// Build the full list of OSC parameters from 61 ARKit blendshapes.
/// `prefix` is the base path, e.g. "/avatar/parameters/FT/v2" or "/avatar/parameters/v2".
pub fn map_blendshapes(bs: &[f32; BLENDSHAPE_COUNT], connected: bool, prefix: &str) -> Vec<OscParam> {
    let mut params = Vec::with_capacity(128);

    // Helper closures that capture `prefix`
    let v2 = |name: &str| -> String { format!("{prefix}/{name}") };
    let float = |name: &str, value: f32| -> OscParam {
        OscParam {
            address: format!("{prefix}/{name}"),
            value: OscValue::Float(value),
        }
    };

    // --- Direct 1:1 expression parameters ---
    let direct_singles: &[(usize, &str)] = &[
        (idx::EYE_SQUINT_LEFT, "EyeSquintLeft"),
        (idx::EYE_WIDE_LEFT, "EyeWideLeft"),
        (idx::EYE_SQUINT_RIGHT, "EyeSquintRight"),
        (idx::EYE_WIDE_RIGHT, "EyeWideRight"),
        (idx::JAW_FORWARD, "JawForward"),
        (idx::JAW_LEFT, "JawLeft"),
        (idx::JAW_RIGHT, "JawRight"),
        (idx::JAW_OPEN, "JawOpen"),
        (idx::MOUTH_CLOSE, "MouthClosed"),
        (idx::MOUTH_FROWN_LEFT, "MouthFrownLeft"),
        (idx::MOUTH_FROWN_RIGHT, "MouthFrownRight"),
        (idx::MOUTH_DIMPLE_LEFT, "MouthDimpleLeft"),
        (idx::MOUTH_DIMPLE_RIGHT, "MouthDimpleRight"),
        (idx::MOUTH_STRETCH_LEFT, "MouthStretchLeft"),
        (idx::MOUTH_STRETCH_RIGHT, "MouthStretchRight"),
        (idx::MOUTH_SHRUG_LOWER, "MouthRaiserLower"),
        (idx::MOUTH_SHRUG_UPPER, "MouthRaiserUpper"),
        (idx::MOUTH_PRESS_LEFT, "MouthPressLeft"),
        (idx::MOUTH_PRESS_RIGHT, "MouthPressRight"),
        (idx::MOUTH_LOWER_DOWN_LEFT, "MouthLowerDownLeft"),
        (idx::MOUTH_LOWER_DOWN_RIGHT, "MouthLowerDownRight"),
        (idx::MOUTH_UPPER_UP_LEFT, "MouthUpperUpLeft"),
        (idx::MOUTH_UPPER_UP_RIGHT, "MouthUpperUpRight"),
        (idx::BROW_OUTER_UP_LEFT, "BrowOuterUpLeft"),
        (idx::BROW_OUTER_UP_RIGHT, "BrowOuterUpRight"),
        (idx::CHEEK_SQUINT_LEFT, "CheekSquintLeft"),
        (idx::CHEEK_SQUINT_RIGHT, "CheekSquintRight"),
        (idx::NOSE_SNEER_LEFT, "NoseSneerLeft"),
        (idx::NOSE_SNEER_RIGHT, "NoseSneerRight"),
        (idx::TONGUE_OUT, "TongueOut"),
    ];

    for &(i, name) in direct_singles {
        params.push(float(name, clamp01(bs[i])));
    }

    // --- Eye look direction (indices 1-4, 8-11) ---
    params.push(float("EyeLookDownLeft", clamp01(bs[idx::EYE_LOOK_DOWN_LEFT])));
    params.push(float("EyeLookInLeft", clamp01(bs[idx::EYE_LOOK_IN_LEFT])));
    params.push(float("EyeLookOutLeft", clamp01(bs[idx::EYE_LOOK_OUT_LEFT])));
    params.push(float("EyeLookUpLeft", clamp01(bs[idx::EYE_LOOK_UP_LEFT])));
    params.push(float("EyeLookDownRight", clamp01(bs[idx::EYE_LOOK_DOWN_RIGHT])));
    params.push(float("EyeLookInRight", clamp01(bs[idx::EYE_LOOK_IN_RIGHT])));
    params.push(float("EyeLookOutRight", clamp01(bs[idx::EYE_LOOK_OUT_RIGHT])));
    params.push(float("EyeLookUpRight", clamp01(bs[idx::EYE_LOOK_UP_RIGHT])));

    // --- One-to-many direct mappings ---
    let funnel = clamp01(bs[idx::MOUTH_FUNNEL]);
    params.push(float("LipFunnelUpperLeft", funnel));
    params.push(float("LipFunnelUpperRight", funnel));
    params.push(float("LipFunnelLowerLeft", funnel));
    params.push(float("LipFunnelLowerRight", funnel));

    let pucker = clamp01(bs[idx::MOUTH_PUCKER]);
    params.push(float("LipPuckerUpperLeft", pucker));
    params.push(float("LipPuckerUpperRight", pucker));
    params.push(float("LipPuckerLowerLeft", pucker));
    params.push(float("LipPuckerLowerRight", pucker));

    let mouth_left = clamp01(bs[idx::MOUTH_LEFT]);
    params.push(float("MouthUpperLeft", mouth_left));
    params.push(float("MouthLowerLeft", mouth_left));

    let mouth_right = clamp01(bs[idx::MOUTH_RIGHT]);
    params.push(float("MouthUpperRight", mouth_right));
    params.push(float("MouthLowerRight", mouth_right));

    let smile_l = clamp01(bs[idx::MOUTH_SMILE_LEFT]);
    params.push(float("MouthCornerPullLeft", smile_l));
    params.push(float("MouthCornerSlantLeft", smile_l));

    let smile_r = clamp01(bs[idx::MOUTH_SMILE_RIGHT]);
    params.push(float("MouthCornerPullRight", smile_r));
    params.push(float("MouthCornerSlantRight", smile_r));

    let roll_lower = clamp01(bs[idx::MOUTH_ROLL_LOWER]);
    params.push(float("LipSuckLowerLeft", roll_lower));
    params.push(float("LipSuckLowerRight", roll_lower));

    let roll_upper = clamp01(bs[idx::MOUTH_ROLL_UPPER]);
    params.push(float("LipSuckUpperLeft", roll_upper));
    params.push(float("LipSuckUpperRight", roll_upper));

    let brow_down_l = clamp01(bs[idx::BROW_DOWN_LEFT]);
    params.push(float("BrowLowererLeft", brow_down_l));
    params.push(float("BrowPinchLeft", brow_down_l));

    let brow_down_r = clamp01(bs[idx::BROW_DOWN_RIGHT]);
    params.push(float("BrowLowererRight", brow_down_r));
    params.push(float("BrowPinchRight", brow_down_r));

    let brow_inner = clamp01(bs[idx::BROW_INNER_UP]);
    params.push(float("BrowInnerUpLeft", brow_inner));
    params.push(float("BrowInnerUpRight", brow_inner));

    let cheek_puff = clamp01(bs[idx::CHEEK_PUFF]);
    params.push(float("CheekPuffLeft", cheek_puff));
    params.push(float("CheekPuffRight", cheek_puff));

    // --- Derived/combined parameters ---
    let eye_blink_l = clamp01(bs[idx::EYE_BLINK_LEFT]);
    let eye_blink_r = clamp01(bs[idx::EYE_BLINK_RIGHT]);
    let eye_open_l = 1.0 - eye_blink_l;
    let eye_open_r = 1.0 - eye_blink_r;

    params.push(float("EyeOpenLeft", eye_open_l));
    params.push(float("EyeOpenRight", eye_open_r));
    params.push(float("EyeOpen", (eye_open_l + eye_open_r) * 0.5));
    params.push(float("EyeClosedLeft", eye_blink_l));
    params.push(float("EyeClosedRight", eye_blink_r));
    params.push(float("EyeClosed", (eye_blink_l + eye_blink_r) * 0.5));

    // Eye lid (75% openness + 25% wide, matching VRCFT)
    let wide_l = clamp01(bs[idx::EYE_WIDE_LEFT]);
    let wide_r = clamp01(bs[idx::EYE_WIDE_RIGHT]);
    params.push(float("EyeLidLeft", eye_open_l * 0.75 + wide_l * 0.25));
    params.push(float("EyeLidRight", eye_open_r * 0.75 + wide_r * 0.25));
    params.push(float("EyeLid", ((eye_open_l + eye_open_r) * 0.5) * 0.75 + ((wide_l + wide_r) * 0.5) * 0.25));

    // Eye gaze (radians, not clamped)
    params.push(float("EyeLeftX", bs[idx::EYE_YAW_LEFT]));
    params.push(float("EyeLeftY", -bs[idx::EYE_PITCH_LEFT]));
    params.push(float("EyeRightX", bs[idx::EYE_YAW_RIGHT]));
    params.push(float("EyeRightY", -bs[idx::EYE_PITCH_RIGHT]));
    // Combined eye Y (average of left and right)
    params.push(float("EyeY", -(bs[idx::EYE_PITCH_LEFT] + bs[idx::EYE_PITCH_RIGHT]) * 0.5));

    // Head pose (radians, not clamped)
    params.push(float("HeadYaw", bs[idx::HEAD_YAW]));
    params.push(float("HeadPitch", bs[idx::HEAD_PITCH]));
    params.push(float("HeadRoll", bs[idx::HEAD_ROLL]));

    // Aggregates
    let squint_l = clamp01(bs[idx::EYE_SQUINT_LEFT]);
    let squint_r = clamp01(bs[idx::EYE_SQUINT_RIGHT]);
    params.push(float("EyeSquint", squint_l.max(squint_r)));

    params.push(float("EyeWide", wide_l.max(wide_r)));

    // Jaw derived
    params.push(float("JawX", clamp01(bs[idx::JAW_RIGHT]) - clamp01(bs[idx::JAW_LEFT])));
    params.push(float("JawZ", clamp01(bs[idx::JAW_FORWARD])));

    // Mouth derived
    params.push(float("MouthX", mouth_right - mouth_left));

    let frown_l = clamp01(bs[idx::MOUTH_FROWN_LEFT]);
    let frown_r = clamp01(bs[idx::MOUTH_FROWN_RIGHT]);
    let smile_avg = (smile_l + smile_r) * 0.5;
    let frown_avg = (frown_l + frown_r) * 0.5;
    params.push(float("SmileFrown", smile_avg - frown_avg));
    params.push(float("SmileFrownLeft", smile_l - frown_l));
    params.push(float("SmileFrownRight", smile_r - frown_r));

    params.push(float("LipFunnel", funnel));
    params.push(float("LipPucker", pucker));
    params.push(float("LipSuck", (roll_lower + roll_upper) * 0.5));
    params.push(float("LipSuckUpper", roll_upper));
    params.push(float("LipSuckLower", roll_lower));

    params.push(float("CheekPuffSuck", cheek_puff));
    params.push(float("CheekPuffSuckLeft", cheek_puff));
    params.push(float("CheekPuffSuckRight", cheek_puff));

    let sneer_l = clamp01(bs[idx::NOSE_SNEER_LEFT]);
    let sneer_r = clamp01(bs[idx::NOSE_SNEER_RIGHT]);
    params.push(float("NoseSneer", (sneer_l + sneer_r) * 0.5));

    params.push(float("BrowInnerUp", brow_inner));
    let brow_outer_l = clamp01(bs[idx::BROW_OUTER_UP_LEFT]);
    let brow_outer_r = clamp01(bs[idx::BROW_OUTER_UP_RIGHT]);
    params.push(float("BrowOuterUp", (brow_outer_l + brow_outer_r) * 0.5));
    params.push(float("BrowDown", (brow_down_l + brow_down_r) * 0.5));

    // BrowExpression: (innerUp + outerUp) * 0.5 - browDown (matching VRCFT)
    let brow_up_l = brow_inner * 0.5 + brow_outer_l * 0.5;
    let brow_up_r = brow_inner * 0.5 + brow_outer_r * 0.5;
    params.push(float("BrowExpressionLeft", brow_up_l.min(1.0) - (brow_down_l * 0.75 + brow_down_l * 0.25)));
    params.push(float("BrowExpressionRight", brow_up_r.min(1.0) - (brow_down_r * 0.75 + brow_down_r * 0.25)));

    // MouthOpen, MouthUpperUp, MouthLowerDown, MouthPress (combined)
    let lower_down_l = clamp01(bs[idx::MOUTH_LOWER_DOWN_LEFT]);
    let lower_down_r = clamp01(bs[idx::MOUTH_LOWER_DOWN_RIGHT]);
    let upper_up_l = clamp01(bs[idx::MOUTH_UPPER_UP_LEFT]);
    let upper_up_r = clamp01(bs[idx::MOUTH_UPPER_UP_RIGHT]);
    params.push(float("MouthOpen", (upper_up_l + upper_up_r + lower_down_l + lower_down_r) * 0.25));
    params.push(float("MouthUpperUp", (upper_up_l + upper_up_r) * 0.5));
    params.push(float("MouthLowerDown", (lower_down_l + lower_down_r) * 0.5));
    params.push(float("MouthPress", (clamp01(bs[idx::MOUTH_PRESS_LEFT]) + clamp01(bs[idx::MOUTH_PRESS_RIGHT])) * 0.5));
    params.push(float("MouthStretch", (clamp01(bs[idx::MOUTH_STRETCH_LEFT]) + clamp01(bs[idx::MOUTH_STRETCH_RIGHT])) * 0.5));

    // --- Status parameters (no prefix, these are standard) ---
    params.push(OscParam {
        address: "/avatar/parameters/EyeTrackingActive".to_string(),
        value: OscValue::Bool(connected),
    });
    params.push(OscParam {
        address: "/avatar/parameters/ExpressionTrackingActive".to_string(),
        value: OscValue::Bool(connected),
    });
    params.push(OscParam {
        address: "/avatar/parameters/LipTrackingActive".to_string(),
        value: OscValue::Bool(connected),
    });

    // Suppress unused variable warning
    let _ = v2("_");

    params
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
        let params = map_blendshapes(&bs, true, DEFAULT_PREFIX);
        assert!(params.len() > 90, "expected many params, got {}", params.len());
    }

    #[test]
    fn eye_open_inverse_of_blink() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::EYE_BLINK_LEFT] = 0.7;
        bs[idx::EYE_BLINK_RIGHT] = 0.3;
        let params = map_blendshapes(&bs, true, DEFAULT_PREFIX);

        assert!((find_float(&params, "EyeOpenLeft") - 0.3).abs() < f32::EPSILON);
        assert!((find_float(&params, "EyeOpenRight") - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn jaw_x_is_right_minus_left() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::JAW_RIGHT] = 0.8;
        bs[idx::JAW_LEFT] = 0.3;
        let params = map_blendshapes(&bs, true, DEFAULT_PREFIX);
        assert!((find_float(&params, "JawX") - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn status_params_use_no_prefix() {
        let bs = [0.0f32; BLENDSHAPE_COUNT];
        let params = map_blendshapes(&bs, true, DEFAULT_PREFIX);
        let s = params.iter().find(|p| p.address.contains("EyeTrackingActive")).unwrap();
        assert_eq!(s.address, "/avatar/parameters/EyeTrackingActive");
        assert!(matches!(s.value, OscValue::Bool(true)));
    }

    #[test]
    fn status_params_false_when_disconnected() {
        let bs = [0.0f32; BLENDSHAPE_COUNT];
        let params = map_blendshapes(&bs, false, DEFAULT_PREFIX);
        let s = params.iter().find(|p| p.address.contains("LipTrackingActive")).unwrap();
        assert!(matches!(s.value, OscValue::Bool(false)));
    }

    #[test]
    fn mouth_funnel_fans_out_to_four() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::MOUTH_FUNNEL] = 0.6;
        let params = map_blendshapes(&bs, true, DEFAULT_PREFIX);
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
        let params = map_blendshapes(&bs, true, DEFAULT_PREFIX);

        assert!((find_float(&params, "SmileFrown") - 0.55).abs() < 1e-6);
        assert!((find_float(&params, "SmileFrownLeft") - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn eye_gaze_negated_pitch() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::EYE_PITCH_LEFT] = 0.3;
        bs[idx::EYE_PITCH_RIGHT] = -0.2;
        let params = map_blendshapes(&bs, true, DEFAULT_PREFIX);

        assert!((find_float(&params, "EyeLeftY") - (-0.3)).abs() < f32::EPSILON);
        assert!((find_float(&params, "EyeRightY") - 0.2).abs() < f32::EPSILON);
    }

    #[test]
    fn clamping_caps_above_one() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::JAW_OPEN] = 1.05;
        let params = map_blendshapes(&bs, true, DEFAULT_PREFIX);
        assert!((find_float(&params, "/JawOpen") - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn clamping_floors_below_zero() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::JAW_OPEN] = -0.01;
        let params = map_blendshapes(&bs, true, DEFAULT_PREFIX);
        assert!((find_float(&params, "/JawOpen")).abs() < f32::EPSILON);
    }

    #[test]
    fn eye_look_params_present() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::EYE_LOOK_DOWN_LEFT] = 0.4;
        bs[idx::EYE_LOOK_IN_RIGHT] = 0.6;
        let params = map_blendshapes(&bs, true, DEFAULT_PREFIX);

        assert!((find_float(&params, "EyeLookDownLeft") - 0.4).abs() < f32::EPSILON);
        assert!((find_float(&params, "EyeLookInRight") - 0.6).abs() < f32::EPSILON);
    }

    #[test]
    fn head_pose_params_present() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::HEAD_YAW] = 0.3;
        bs[idx::HEAD_PITCH] = -0.2;
        bs[idx::HEAD_ROLL] = 0.1;
        let params = map_blendshapes(&bs, true, DEFAULT_PREFIX);

        assert!((find_float(&params, "HeadYaw") - 0.3).abs() < f32::EPSILON);
        assert!((find_float(&params, "HeadPitch") - (-0.2)).abs() < f32::EPSILON);
        assert!((find_float(&params, "HeadRoll") - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn configurable_prefix() {
        let bs = [0.0f32; BLENDSHAPE_COUNT];
        let params = map_blendshapes(&bs, true, "/avatar/parameters/v2");
        let jaw = params.iter().find(|p| p.address.contains("JawOpen")).unwrap();
        assert!(jaw.address.starts_with("/avatar/parameters/v2/"));

        let params2 = map_blendshapes(&bs, true, "/avatar/parameters/FT/v2");
        let jaw2 = params2.iter().find(|p| p.address.contains("JawOpen")).unwrap();
        assert!(jaw2.address.starts_with("/avatar/parameters/FT/v2/"));
    }

    #[test]
    fn eye_lid_combined() {
        let mut bs = [0.0f32; BLENDSHAPE_COUNT];
        bs[idx::EYE_BLINK_LEFT] = 0.2; // openness = 0.8
        bs[idx::EYE_WIDE_LEFT] = 0.4;
        let params = map_blendshapes(&bs, true, DEFAULT_PREFIX);
        // EyeLidLeft = 0.8 * 0.75 + 0.4 * 0.25 = 0.6 + 0.1 = 0.7
        assert!((find_float(&params, "EyeLidLeft") - 0.7).abs() < 1e-6);
    }
}
