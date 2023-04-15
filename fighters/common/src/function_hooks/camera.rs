use super::*;
use utils::ext::*;


// Doubles camera speed
#[skyline::hook(offset = 0x4fdbf0)]
unsafe fn normal_camera(ptr: u64, float: f32) {
    call_original!(ptr, float);
    call_original!(ptr, float);
}

// Standardizes normal_camera_min_distance for all stages
#[skyline::hook(offset = 0x26209bc, inline)]
unsafe fn parse_stprm_normal_camera_min_distance(ctx: &mut skyline::hooks::InlineCtx) {
    let normal_camera_min_distance: f32;
    asm!("fmov w20, s0", out("w20") normal_camera_min_distance);
    if normal_camera_min_distance < 125.0 {
        let normal_camera_min_distance: f32 = 125.0;
        asm!("fmov s0, w8", in("w8") normal_camera_min_distance);
    }
}

// Standardizes target_interpolation_rate for all stages
#[skyline::hook(offset = 0x2620fec, inline)]
unsafe fn parse_stprm_target_interpolation_rate(ctx: &mut skyline::hooks::InlineCtx) {
    let target_interpolation_rate: f32 = 0.9;
    asm!("fmov s1, w8", in("w8") target_interpolation_rate)
}

// The following hooks handle Unrestricted Camera
#[skyline::hook(offset = 0x262286c, inline)]
unsafe fn parse_stprm_pause_camera_min_fov(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_min_fov: f32 = 4e-44;
    asm!("fmov s2, w8", in("w8") pause_camera_min_fov)
}
#[skyline::hook(offset = 0x2622a04, inline)]
unsafe fn parse_stprm_pause_camera_max_fov(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_max_fov: f32 = 180.0;
    asm!("fmov s2, w8", in("w8") pause_camera_max_fov)
}
#[skyline::hook(offset = 0x2622acc, inline)]
unsafe fn parse_stprm_pause_camera_min_distance(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_min_distance: f32 = 0.0;
    asm!("fmov s2, w8", in("w8") pause_camera_min_distance)
}
#[skyline::hook(offset = 0x2622c54, inline)]
unsafe fn parse_stprm_pause_camera_max_distance(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_max_distance: f32 = 536870900.0;
    asm!("fmov s2, w8", in("w8") pause_camera_max_distance)
}
#[skyline::hook(offset = 0x2622d18, inline)]
unsafe fn parse_stprm_pause_camera_limit_pos_top(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_limit_pos_top: f32 = f32::NAN;
    asm!("fmov s2, w8", in("w8") pause_camera_limit_pos_top)
}
#[skyline::hook(offset = 0x2622ddc, inline)]
unsafe fn parse_stprm_pause_camera_limit_pos_bottom(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_limit_pos_bottom: f32 = f32::NAN;
    asm!("fmov s2, w8", in("w8") pause_camera_limit_pos_bottom)
}
#[skyline::hook(offset = 0x2622ea0, inline)]
unsafe fn parse_stprm_pause_camera_limit_pos_left(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_limit_pos_left: f32 = f32::NAN;
    asm!("fmov s2, w8", in("w8") pause_camera_limit_pos_left)
}
#[skyline::hook(offset = 0x2622f64, inline)]
unsafe fn parse_stprm_pause_camera_limit_pos_right(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_limit_pos_right: f32 = f32::NAN;
    asm!("fmov s2, w8", in("w8") pause_camera_limit_pos_right)
}
#[skyline::hook(offset = 0x2623028, inline)]
unsafe fn parse_stprm_pause_camera_limit_angle_up(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_limit_angle_up: f32 = f32::NAN;
    asm!("fmov s2, w8", in("w8") pause_camera_limit_angle_up)
}
#[skyline::hook(offset = 0x26230f0, inline)]
unsafe fn parse_stprm_pause_camera_limit_angle_down(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_limit_angle_down: f32 = f32::NAN;
    asm!("fmov s2, w8", in("w8") pause_camera_limit_angle_down)
}
#[skyline::hook(offset = 0x2623280, inline)]
unsafe fn parse_stprm_pause_camera_limit_angle_left(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_limit_angle_left: f32 = f32::NAN;
    asm!("fmov s2, w8", in("w8") pause_camera_limit_angle_left)
}
#[skyline::hook(offset = 0x26231b8, inline)]
unsafe fn parse_stprm_pause_camera_limit_angle_right(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_limit_angle_right: f32 = f32::NAN;
    asm!("fmov s2, w8", in("w8") pause_camera_limit_angle_right)
}
#[skyline::hook(offset = 0x2623348, inline)]
unsafe fn parse_stprm_pause_camera_gyro_limit_angle_up(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_gyro_limit_angle_up: f32 = 0.0;
    asm!("fmov s2, w8", in("w8") pause_camera_gyro_limit_angle_up)
}
#[skyline::hook(offset = 0x2623410, inline)]
unsafe fn parse_stprm_pause_camera_gyro_limit_angle_down(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_gyro_limit_angle_down: f32 = 0.0;
    asm!("fmov s2, w8", in("w8") pause_camera_gyro_limit_angle_down)
}
#[skyline::hook(offset = 0x2623598, inline)]
unsafe fn parse_stprm_pause_camera_gyro_limit_angle_left(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_gyro_limit_angle_left: f32 = 0.0;
    asm!("fmov s0, w8", in("w8") pause_camera_gyro_limit_angle_left)
}
#[skyline::hook(offset = 0x26234d8, inline)]
unsafe fn parse_stprm_pause_camera_gyro_limit_angle_right(ctx: &mut skyline::hooks::InlineCtx) {
    let pause_camera_gyro_limit_angle_right: f32 = 0.0;
    asm!("fmov s2, w8", in("w8") pause_camera_gyro_limit_angle_right)
}
#[skyline::hook(offset = 0x2621b24, inline)]
unsafe fn parse_stprm_vr_camera_position_x_min(ctx: &mut skyline::hooks::InlineCtx) {
    let vr_camera_position_x_min: f32 = f32::NAN;
    asm!("fmov s0, w8", in("w8") vr_camera_position_x_min)
}
#[skyline::hook(offset = 0x26158f8, inline)]
unsafe fn parse_stprm_vr_camera_position_x_max(ctx: &mut skyline::hooks::InlineCtx) {
    let vr_camera_position_x_max: f32 = f32::NAN;
    asm!("fmov s1, w8", in("w8") vr_camera_position_x_max)
}
#[skyline::hook(offset = 0x2621bd8, inline)]
unsafe fn parse_stprm_vr_camera_position_y_min(ctx: &mut skyline::hooks::InlineCtx) {
    let vr_camera_position_y_min: f32 = f32::NAN;
    asm!("fmov s1, w8", in("w8") vr_camera_position_y_min)
}
#[skyline::hook(offset = 0x26159b8, inline)]
unsafe fn parse_stprm_vr_camera_position_y_max(ctx: &mut skyline::hooks::InlineCtx) {
    let vr_camera_position_y_max: f32 = f32::NAN;
    asm!("fmov s2, w8", in("w8") vr_camera_position_y_max)
}
#[skyline::hook(offset = 0x2621c8c, inline)]
unsafe fn parse_stprm_vr_camera_position_z_min(ctx: &mut skyline::hooks::InlineCtx) {
    let vr_camera_position_z_min: f32 = f32::NEG_INFINITY;
    asm!("fmov s2, w8", in("w8") vr_camera_position_z_min)
}
#[skyline::hook(offset = 0x2615a78, inline)]
unsafe fn parse_stprm_vr_camera_position_z_max(ctx: &mut skyline::hooks::InlineCtx) {
    let vr_camera_position_z_max: f32 = f32::INFINITY;
    asm!("fmov s3, w8", in("w8") vr_camera_position_z_max)
}



pub fn install() {
    unsafe {
        // Stubs original target_interpolation_rate pull
        skyline::patching::Patch::in_text(0x2620fec).nop();

        // Stubs original param pulls related to Unrestricted Camera
        skyline::patching::Patch::in_text(0x262286c).nop();
        skyline::patching::Patch::in_text(0x2622a04).nop();
        skyline::patching::Patch::in_text(0x2622acc).nop();
        skyline::patching::Patch::in_text(0x2622c54).nop();
        skyline::patching::Patch::in_text(0x2622d18).nop();
        skyline::patching::Patch::in_text(0x2622ddc).nop();
        skyline::patching::Patch::in_text(0x2622ea0).nop();
        skyline::patching::Patch::in_text(0x2622f64).nop();
        skyline::patching::Patch::in_text(0x2623028).nop();
        skyline::patching::Patch::in_text(0x26230f0).nop();
        skyline::patching::Patch::in_text(0x2623280).nop();
        skyline::patching::Patch::in_text(0x26231b8).nop();
        skyline::patching::Patch::in_text(0x2623348).nop();
        skyline::patching::Patch::in_text(0x2623410).nop();
        skyline::patching::Patch::in_text(0x2623598).nop();
        skyline::patching::Patch::in_text(0x26234d8).nop();
        skyline::patching::Patch::in_text(0x2621b24).nop();
        skyline::patching::Patch::in_text(0x26158f8).nop();
        skyline::patching::Patch::in_text(0x2621bd8).nop();
        skyline::patching::Patch::in_text(0x26159b8).nop();
        skyline::patching::Patch::in_text(0x2621c8c).nop();
        skyline::patching::Patch::in_text(0x2615a78).nop();
        
    }
    skyline::install_hooks!(
        normal_camera,
        parse_stprm_normal_camera_min_distance,
        parse_stprm_target_interpolation_rate,

        parse_stprm_pause_camera_min_fov,
        parse_stprm_pause_camera_max_fov,
        parse_stprm_pause_camera_min_distance,
        parse_stprm_pause_camera_max_distance,
        parse_stprm_pause_camera_limit_pos_top,
        parse_stprm_pause_camera_limit_pos_bottom,
        parse_stprm_pause_camera_limit_pos_left,
        parse_stprm_pause_camera_limit_pos_right,
        parse_stprm_pause_camera_limit_angle_up,
        parse_stprm_pause_camera_limit_angle_down,
        parse_stprm_pause_camera_limit_angle_left,
        parse_stprm_pause_camera_limit_angle_right,
        parse_stprm_pause_camera_gyro_limit_angle_up,
        parse_stprm_pause_camera_gyro_limit_angle_down,
        parse_stprm_pause_camera_gyro_limit_angle_left,
        parse_stprm_pause_camera_gyro_limit_angle_right,
        parse_stprm_vr_camera_position_x_min,
        parse_stprm_vr_camera_position_x_max,
        parse_stprm_vr_camera_position_y_min,
        parse_stprm_vr_camera_position_y_max,
        parse_stprm_vr_camera_position_z_min,
        parse_stprm_vr_camera_position_z_max
    );
}