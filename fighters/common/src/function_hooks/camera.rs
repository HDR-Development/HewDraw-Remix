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
    pub unk_0x19bdd14296: f32,
    pub others: [f32; 2],
    pub camera_offset_y_min_distance: f32,
    pub camera_offset_y_min: f32,
    pub camera_offset_y_max_distance: f32,
    pub camera_offset_y_max: f32,
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

pub static DEFAULT_TARGET_INTERPOLATION_RATE: f32 = 0.775;

#[derive(Copy, Clone)]
pub struct ReducedCameraTrackingSpeed {
    pub target_interpolation_rate: f32,
    pub normalize_increment: f32,
}

pub static mut REDUCED_CAMERA_TRACKING_SPEED: [ReducedCameraTrackingSpeed; 8] = [
    ReducedCameraTrackingSpeed{target_interpolation_rate: DEFAULT_TARGET_INTERPOLATION_RATE, normalize_increment: 0.0};
    8
];

// Doubles camera speed
#[skyline::hook(offset = 0x4fdc10)]
unsafe fn normal_camera(ptr: u64, float: f32) {
    call_original!(ptr, float);
    call_original!(ptr, float);
}

#[skyline::hook(offset = 0x2621490)]
pub fn parse_stprm_active_camera_params(param_obj: u64, params: &mut NormalCameraParams) {
    call_original!(param_obj, params);
    let fov = params.normal_camera_fov.to_degrees();
    params.normal_camera_min_distance = (0.2 * fov.powf(2.0)) - (17.0 * fov) + 455.0;
    params.normal_camera_min_distance_2 = (0.2 * fov.powf(2.0)) - (17.0 * fov) + 455.0;
    params.swing_rate_x = 0.0;
    params.swing_rate_y = 0.0;
    params.normal_camera_vertical_angle = params.normal_camera_vertical_angle.max(-5.0_f32.to_radians());
    params.target_interpolation_rate = DEFAULT_TARGET_INTERPOLATION_RATE;
}

// The following function hook handles Unrestricted Camera
#[skyline::hook(offset = 0x2623350)]
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

#[skyline::hook(offset = 0x4fddcc, inline)]
unsafe fn normal_camera_get_active_camera_params(ctx: &mut skyline::hooks::InlineCtx) {
    let ptr = ctx.registers[19].x();
    let normal_camera_params = *((ptr + 0x1e0) as *const u64);
    let current_tir = *((normal_camera_params + 0x38) as *mut f32);

    let target_interpolation_rate: f32 = if REDUCED_CAMERA_TRACKING_SPEED.iter().any(|&x| x.target_interpolation_rate < DEFAULT_TARGET_INTERPOLATION_RATE) {
        let (idx, min) = REDUCED_CAMERA_TRACKING_SPEED
            .iter()
            .copied()
            .enumerate()
            .fold((0, REDUCED_CAMERA_TRACKING_SPEED[0]), |acc, x| {
                let (best_idx, best) = acc;
                let (i, val) = x;

                if val.target_interpolation_rate < best.target_interpolation_rate {
                    (i, val)
                } else {
                    (best_idx, best)
                }
        });
        let normalize_increment = min.normalize_increment;
        let new_tir = (min.target_interpolation_rate + normalize_increment).min(DEFAULT_TARGET_INTERPOLATION_RATE);
        REDUCED_CAMERA_TRACKING_SPEED[idx].target_interpolation_rate = new_tir;
        new_tir
    } else {
        (current_tir + 0.01).min(DEFAULT_TARGET_INTERPOLATION_RATE)
    };

    *((normal_camera_params + 0x38) as *mut f32) = target_interpolation_rate;
}

// Runs immediately before CameraModule::req_quake is called
// within the routine that determines on-hit screenshake strength
#[skyline::hook(offset = 0x6c2590, inline)]
unsafe fn damage_fly_req_quake(ctx: &mut skyline::hooks::InlineCtx) {
    let boma = &mut *(ctx.registers[19].x() as *mut BattleObjectModuleAccessor);
    let kb = DamageModule::reaction(boma, 0);

    // Ignore global screenshake reduction on hits above 170 knockback units
    if kb > 170.0 {
        VarModule::on_flag(boma.object(), vars::common::instance::IGNORE_REDUCED_SCREENSHAKE);
    }
}

#[skyline::hook(offset = 0x3ebe20)]
unsafe fn camera_module__req_quake(camera_module: *const u64, quake_kind: i32) {
    let boma = *(camera_module as *mut *mut BattleObjectModuleAccessor).add(1);

    if VarModule::has_var_module((*boma).object())
    && VarModule::is_flag((*boma).object(), vars::common::instance::IGNORE_REDUCED_SCREENSHAKE) {
        VarModule::off_flag((*boma).object(), vars::common::instance::IGNORE_REDUCED_SCREENSHAKE);
        // Use vanilla's screenshake strength for this instance
        return call_original!(camera_module, quake_kind);
    }

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
        normal_camera_get_active_camera_params,
        damage_fly_req_quake,
        camera_module__req_quake,
    );
}