use super::*;
use crate::consts::*;
use crate::consts::globals::*;
use std::ffi::c_float;

#[skyline::hook(offset = 0x6d2498, inline)]
unsafe fn hitstun_gravity_1(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = ctx.registers[19].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_accel_y = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
    let hitstun_gravity_min = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_min");
    let hitstun_gravity_max = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_max");

    let hitstun_gravity = air_accel_y.clamp(hitstun_gravity_min, hitstun_gravity_max);

    ctx.registers_f[0].set_s(hitstun_gravity)
}

#[skyline::hook(offset = 0x6d24c0, inline)]
unsafe fn hitstun_fall_speed_1(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = ctx.registers[19].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_speed_y_stable = WorkModule::get_param_float(boma, hash40("air_speed_y_stable"), 0);

    ctx.registers_f[0].set_s(air_speed_y_stable)
}

#[skyline::hook(offset = 0x6c399c, inline)]
unsafe fn hitstun_gravity_2(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = ctx.registers[23].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_accel_y = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
    let hitstun_gravity_min = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_min");
    let hitstun_gravity_max = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_max");

    let hitstun_gravity = air_accel_y.clamp(hitstun_gravity_min, hitstun_gravity_max);

    ctx.registers_f[0].set_s(hitstun_gravity);
}

#[skyline::hook(offset = 0x6c39c4, inline)]
unsafe fn hitstun_fall_speed_2(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = ctx.registers[23].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_speed_y_stable = WorkModule::get_param_float(boma, hash40("air_speed_y_stable"), 0);

    ctx.registers_f[0].set_s(air_speed_y_stable);
}

#[skyline::hook(offset = 0x6d5920, inline)]
unsafe fn hitstun_gravity_3(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = ctx.registers[20].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_accel_y = WorkModule::get_param_float(boma, hash40("air_accel_y"), 0);
    let hitstun_gravity_min = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_min");
    let hitstun_gravity_max = ParamModule::get_float((*boma).object(), ParamType::Common, "hitstun_gravity_max");

    let hitstun_gravity = air_accel_y.clamp(hitstun_gravity_min, hitstun_gravity_max);

    ctx.registers_f[0].set_s(hitstun_gravity)
}

#[skyline::hook(offset = 0x6d5948, inline)]
unsafe fn hitstun_fall_speed_3(ctx: &mut skyline::hooks::InlineCtx) {
    let work_module = ctx.registers[20].x();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let air_speed_y_stable = WorkModule::get_param_float(boma, hash40("air_speed_y_stable"), 0);
    
    ctx.registers_f[0].set_s(air_speed_y_stable)
}

#[skyline::hook(offset = 0x13e90a0)]
unsafe fn trajectory_manager_hook(
    main_obj: *mut u8,
    unused_arg: u64,
    boma_ptr: *mut u8,
    knockback_info: *mut f32, 
) {
    // Exit if trajectory display is disabled
    // Derived from original function
    let ptr_1 = *(main_obj.add(8) as *const *const u8);
    let ptr_2 = *(ptr_1.add(0x80) as *const *const u8);
    let show_trajectory_flag = *ptr_2.add(0xb49);
    if show_trajectory_flag == 0 {
        *(main_obj.add(0x2a9) as *mut u8) = 1;
        return;
    }

    // Check if hit is valid for this
    let kb_bytes = knockback_info as *const u8;
    let is_valid_hit = *kb_bytes.add(0x111);
    let flag_b = *kb_bytes.add(0x44); // Unsure what this flag is, but it's checked as well
    if is_valid_hit != 0 || flag_b != 0 {
        *(main_obj.add(0x2a9) as *mut u8) = 1;
        return;
    }

    *(main_obj.add(0x2a9) as *mut u8) = 0;

    // calculate three knockback vectors: normal, min DI, max DI
    let boma = boma_ptr as *mut BattleObjectModuleAccessor;
    let di_angle = WorkModule::get_param_float(boma, hash40("common"), hash40("damage_fly_correction_max"));
    let launch_speed = Vector2f::new(*knockback_info.add(4), *knockback_info.add(5));
    let launch_speed_min = knockback_util::rotate_vector2f(
        launch_speed,
        -di_angle
    );
    let launch_speed_max = knockback_util::rotate_vector2f(
        launch_speed,
        di_angle
    );

    // store the three vectors in the knockback info
    let red_line = 40;
    *knockback_info.add(red_line) = launch_speed_max.x;
    *knockback_info.add(red_line + 1) = launch_speed_max.y;
    let green_line = 40 + (1 * 8);
    *knockback_info.add(green_line) = launch_speed.x;
    *knockback_info.add(green_line + 1) = launch_speed.y;
    let blue_line = 40 + (2 * 8);
    *knockback_info.add(blue_line) = launch_speed_min.x;
    *knockback_info.add(blue_line + 1) = launch_speed_min.y;

    // force validity flags to true to make sure the line draws correctly
    // this may have unintended consequences...?
    let red_line_is_valid_ptr = (knockback_info.add(red_line + 6)) as *mut u8;
    *red_line_is_valid_ptr = 1;
    let green_line_is_valid_ptr = (knockback_info.add(green_line + 6)) as *mut u8;
    *green_line_is_valid_ptr = 1;
    let blue_line_is_valid_ptr = (knockback_info.add(red_line + 6)) as *mut u8;
    *blue_line_is_valid_ptr = 1;

    let hitstun = *knockback_info.add(0x48 / 4) as u64;

    let mut buffer_0: [u64; 2] = [0; 2];
    buffer_0[0] = *(main_obj.add(0x2b0) as *mut u64);
    buffer_0[1] = 0;
    let result_0 = calculate_knockback(0, boma_ptr, knockback_info, buffer_0.as_mut_ptr(), hitstun);
    set_buffer(buffer_0[0] as *mut u8, hitstun);

    let bit_0 = (result_0 & 1) as u8;
    *(main_obj.add(0x2aa) as *mut u8) |= bit_0;
    *(main_obj.add(0x2a9) as *mut u8) |= bit_0;

    let mut buffer_1: [u64; 2] = [0; 2];
    buffer_1[0] = *(main_obj.add(0x2c8) as *mut u64);
    buffer_1[1] = 0;
    let result_1 = calculate_knockback(1, boma_ptr, knockback_info, buffer_1.as_mut_ptr(), hitstun);
    set_buffer(buffer_1[0] as *mut u8, hitstun);

    let bit_1 = (result_1 & 1) as u8;
    *(main_obj.add(0x2aa) as *mut u8) |= bit_1;
    *(main_obj.add(0x2a9) as *mut u8) |= bit_1;

    let mut buffer_2: [u64; 2] = [0; 2];
    buffer_2[0] = *(main_obj.add(0x2e0) as *mut u64);
    buffer_2[1] = 0;
    let result_2 = calculate_knockback(2, boma_ptr, knockback_info, buffer_2.as_mut_ptr(), hitstun);
    set_buffer(buffer_2[0] as *mut u8, hitstun);

    let bit_2 = (result_2 & 1) as u8;
    *(main_obj.add(0x2aa) as *mut u8) |= bit_2;
    *(main_obj.add(0x2a9) as *mut u8) |= bit_2;
}

unsafe fn set_buffer(ptr: *mut u8, duration_frames: u64) {
    if duration_frames == 0 {
        return;
    }

    let last_vector_ptr = (ptr as *mut u8).add((duration_frames - 1) as usize * 16) as *mut f32;
    let last_vector = Vector2f::new(*last_vector_ptr, *last_vector_ptr.add(1));

    for i in duration_frames - 1 .. 300 {
        let current_vector = ptr.add(i as usize * 16) as *mut f32;

        *current_vector = last_vector.x;
        *current_vector.add(1) = last_vector.y;
    }
}

#[skyline::from_offset(0x6c33f0)]
unsafe fn calculate_knockback(
    line_index: u64, 
    boma_ptr: *mut u8, 
    knockback_info: *mut f32, 
    knockback_values: *mut u64, // maybe?
    duration_frames: u64) -> u32;

pub fn install() {
    unsafe {
        // Stubs damage_fly_top_air_accel_y and
        // damage_fly_top_speed_y_stable param pulls
        skyline::patching::Patch::in_text(0x6d2498).nop();
        skyline::patching::Patch::in_text(0x6d24c0).nop();
        skyline::patching::Patch::in_text(0x6c399c).nop();
        skyline::patching::Patch::in_text(0x6c39c4).nop();
        skyline::patching::Patch::in_text(0x6d5920).nop();
        skyline::patching::Patch::in_text(0x6d5948).nop();

        // Allows custom hitstun gravity to apply to all knockback angles
        // rather than just vertical angles
        skyline::patching::Patch::in_text(0x6d1f20).nop();
        skyline::patching::Patch::in_text(0x6d1f24).nop();
        skyline::patching::Patch::in_text(0x6d1f48).nop();
        skyline::patching::Patch::in_text(0x6d1f4c).nop();
        skyline::patching::Patch::in_text(0x6cf574).data(0x320003F8);
        skyline::patching::Patch::in_text(0x6d1624).data(0x320003F8);
        skyline::patching::Patch::in_text(0x6c3568).data(0x320003E8);
        skyline::patching::Patch::in_text(0x6c3764).data(0x320003E8);
    }
    skyline::install_hooks!(
        hitstun_gravity_1,
        hitstun_fall_speed_1,
        hitstun_gravity_2,
        hitstun_fall_speed_2,
        hitstun_gravity_3,
        hitstun_fall_speed_3,
        trajectory_manager_hook
    );
}