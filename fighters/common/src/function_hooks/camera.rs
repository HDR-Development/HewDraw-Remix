use super::*;
use utils::ext::*;

#[repr(C)]
pub struct NormalCameraParams {
    pub normal_camera_min_distance: f32,
    pub normal_camera_max_distance: f32,
    pub normal_camera_min_distance_2: f32,
    pub swing_rate_x: f32,
    pub swing_rate_y: f32,
    pub unk: [f32; 7],
    pub normal_camera_vertical_angle: f32,
    pub normal_camera_fov: f32,
    pub target_interpolation_rate: f32,
    // others...
}

#[repr(C)]
pub struct PauseCameraParams {
    pub pause_camera_min_fov: f32,
    pub pause_camera_fov: f32,
    pub pause_camera_max_fov: f32,
    pub pause_camera_min_distance: f32,
    pub pause_camera_initial_distance: f32,
    pub pause_camera_max_distance: f32,
    pub pause_camera_limit_pos_top: f32,
    pub pause_camera_limit_pos_bottom: f32,
    pub pause_camera_limit_pos_right: f32,
    pub pause_camera_limit_pos_left: f32,
    pub pause_camera_limit_angle_up: f32,
    pub pause_camera_limit_angle_down: f32,
    pub pause_camera_limit_angle_right: f32,
    pub pause_camera_limit_angle_left: f32,
    pub pause_camera_gyro_limit_angle_up: f32,
    pub pause_camera_gyro_limit_angle_down: f32,
    pub pause_camera_gyro_limit_angle_right: f32,
    pub pause_camera_gyro_limit_angle_left: f32,
    // others...
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum QuakeKind {
    None = 0x0,
    KeepSmall,
    SmallHalf,
    Small,
    KeepMiddle,
    Middle,
    KeepLarge,
    Large,
    KeepExtraLarge,
    ExtraLarge,
    PowerBlock,
    Knockout,
    DollyStage,
    ExtraExtraLarge,
    Invalid,
}


// Doubles camera speed
#[skyline::hook(offset = 0x4fdbf0)]
unsafe fn normal_camera(ptr: u64, float: f32) {
    call_original!(ptr, float);
    call_original!(ptr, float);
}

#[skyline::hook(offset = 0x26207f0)]
pub fn parse_stprm_active_camera_params(param_obj: u64, params: &mut NormalCameraParams) {
    call_original!(param_obj, params);
    params.normal_camera_min_distance = params.normal_camera_min_distance.max(140.0);
    params.normal_camera_min_distance_2 = params.normal_camera_min_distance_2.max(140.0);
    params.swing_rate_x = 0.0;
    params.swing_rate_y = 0.0;
    params.normal_camera_vertical_angle = params.normal_camera_vertical_angle.max(-5.0_f32.to_radians());
    params.target_interpolation_rate = 0.9;
}

// The following function hook handles Unrestricted Camera
#[skyline::hook(offset = 0x26226b0)]
pub fn parse_stprm_pause_camera_params(param_obj: u64, params: &mut PauseCameraParams) {
    call_original!(param_obj, params);
    params.pause_camera_min_fov = 4e-44_f32.to_radians();
    params.pause_camera_max_fov = 180.0_f32.to_radians();
    params.pause_camera_min_distance = 0.0;
    params.pause_camera_max_distance = 536870900.0;
    params.pause_camera_limit_pos_top = f32::NAN;
    params.pause_camera_limit_pos_bottom = f32::NAN;
    params.pause_camera_limit_pos_right = f32::NAN;
    params.pause_camera_limit_pos_left = f32::NAN;
    params.pause_camera_limit_angle_up = f32::NAN;
    params.pause_camera_limit_angle_down = f32::NAN;
    params.pause_camera_limit_angle_right = f32::NAN;
    params.pause_camera_limit_angle_left = f32::NAN;
    params.pause_camera_gyro_limit_angle_up = 0.0;
    params.pause_camera_gyro_limit_angle_down = 0.0;
    params.pause_camera_gyro_limit_angle_right = 0.0;
    params.pause_camera_gyro_limit_angle_left = 0.0;
}

#[skyline::hook(offset = 0x3ebe00)]
unsafe fn camera_module__req_quake(camera_module: *const u64, quake_kind: i32) {
    use QuakeKind::*;
    let mut quake_kind = std::mem::transmute(quake_kind.clone());
    let quake_kind = match quake_kind {
        Middle => { Small },
        Large => { Middle },
        ExtraLarge => { Large },
        ExtraExtraLarge => { ExtraLarge },
        _ => { quake_kind }
    };
    call_original!(camera_module, quake_kind as i32);
}

pub fn install() {
    skyline::install_hooks!(
        normal_camera,
        parse_stprm_active_camera_params,
        parse_stprm_pause_camera_params,
        camera_module__req_quake
    );
}