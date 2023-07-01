use super::*;
use utils::ext::*;
use std::arch::asm;


#[skyline::hook(offset = 0x6d2174, inline)]
unsafe fn fullhop_initial_y_speed_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(*ctx.registers[8].x.as_ref());
    let work_module = *ctx.registers[0].x.as_ref();
    let jump_y = callable(work_module, smash::hash40("jump_y"), 0);
    let gravity = callable(work_module, smash::hash40("air_accel_y"), 0);
    let initital_jump_vel = (jump_y * gravity * 2.0).sqrt() + (0.5 * gravity);
    asm!("fmov s0, w8", in("w8") initital_jump_vel)
}

#[skyline::hook(offset = 0x6ce6b8, inline)]
unsafe fn jump1_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = (*boma).left_stick_x();
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6ce6ec, inline)]
unsafe fn jump1_jump_speed_x_max_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(*ctx.registers[8].x.as_ref());
    let work_module = *ctx.registers[0].x.as_ref();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let run_speed_max = callable(work_module, smash::hash40("run_speed_max"), 0);
    let ratio = VarModule::get_float((*boma).object(), vars::common::instance::JUMP_SPEED_RATIO);
    // get the multiplier for any special mechanics that require additional jump speed max (meta quick, etc)
    let mut jump_speed_max_mul = VarModule::get_float((*boma).object(), vars::common::instance::JUMP_SPEED_MAX_MUL);
    match jump_speed_max_mul {
        // if its not between 0.1 and 3.0, it is likely not a real value and we should ignore it
        0.1..=3.0 => {},
        _ => { jump_speed_max_mul = 1.0 }
    }
    let jump_speed_x_max = run_speed_max * ratio * jump_speed_max_mul;
    asm!("fmov s0, w8", in("w8") jump_speed_x_max)
}

#[skyline::hook(offset = 0x6d19a4, inline)]
unsafe fn jump2_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = (*boma).left_stick_x();
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d19d8, inline)]
unsafe fn jump2_jump_speed_x_max_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(*ctx.registers[8].x.as_ref());
    let work_module = *ctx.registers[0].x.as_ref();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let run_speed_max = callable(work_module, smash::hash40("run_speed_max"), 0);
    let ratio = VarModule::get_float((*boma).object(), vars::common::instance::JUMP_SPEED_RATIO);
    // get the multiplier for any special mechanics that require additional jump speed max (meta quick, etc)
    let mut jump_speed_max_mul = VarModule::get_float((*boma).object(), vars::common::instance::JUMP_SPEED_MAX_MUL);
    match jump_speed_max_mul {
        // if its not between 0.1 and 3.0, it is likely not a real value and we should ignore it
        0.1..=3.0 => {},
        _ => { jump_speed_max_mul = 1.0 }
    }
    let jump_speed_x_max = run_speed_max * ratio * jump_speed_max_mul;
    asm!("fmov s0, w8", in("w8") jump_speed_x_max)
}

#[skyline::hook(offset = 0x6d1af0, inline)]
unsafe fn jump3_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = (*boma).left_stick_x();
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d1b24, inline)]
unsafe fn jump3_jump_speed_x_max_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(*ctx.registers[8].x.as_ref());
    let work_module = *ctx.registers[0].x.as_ref();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let run_speed_max = callable(work_module, smash::hash40("run_speed_max"), 0);
    let ratio = VarModule::get_float((*boma).object(), vars::common::instance::JUMP_SPEED_RATIO);
    // get the multiplier for any special mechanics that require additional jump speed max (meta quick, etc)
    let mut jump_speed_max_mul = VarModule::get_float((*boma).object(), vars::common::instance::JUMP_SPEED_MAX_MUL);
    match jump_speed_max_mul {
        // if its not between 0.1 and 3.0, it is likely not a real value and we should ignore it
        0.1..=3.0 => {},
        _ => { jump_speed_max_mul = 1.0 }
    }
    let jump_speed_x_max = run_speed_max * ratio * jump_speed_max_mul;
    asm!("fmov s0, w8", in("w8") jump_speed_x_max)
}

#[skyline::hook(offset = 0x6d0434, inline)]
unsafe fn jump4_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = (*boma).left_stick_x();
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d04c4, inline)]
unsafe fn jump4_jump_speed_x_max_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let callable: extern "C" fn(u64, u64, u64) -> f32 = std::mem::transmute(*ctx.registers[8].x.as_ref());
    let work_module = *ctx.registers[0].x.as_ref();
    let boma = *(work_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let run_speed_max = callable(work_module, smash::hash40("run_speed_max"), 0);
    let ratio = VarModule::get_float((*boma).object(), vars::common::instance::JUMP_SPEED_RATIO);
    // get the multiplier for any special mechanics that require additional jump speed max (meta quick, etc)
    let mut jump_speed_max_mul = VarModule::get_float((*boma).object(), vars::common::instance::JUMP_SPEED_MAX_MUL);
    match jump_speed_max_mul {
        // if its not between 0.1 and 3.0, it is likely not a real value and we should ignore it
        0.1..=3.0 => {},
        _ => { jump_speed_max_mul = 1.0 }
    }
    let jump_speed_x_max = run_speed_max * ratio * jump_speed_max_mul;
    asm!("fmov s0, w8", in("w8") jump_speed_x_max)
}

#[skyline::hook(replace = L2CFighterCommon_sub_check_button_jump)]
unsafe extern "C" fn sub_check_button_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    //if fighter.is_cat_flag(CatHdr::Shorthop) 
    //    && 
    //    (fighter.is_situation(*SITUATION_KIND_GROUND) && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON))
    //    || (fighter.is_situation(*SITUATION_KIND_AIR) && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON))
    //{
    //    return true.into();
    //}
    call_original!(fighter)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hook!(sub_check_button_jump);
    }
}

pub fn install() {
    unsafe {
        // stubs vanilla fullhop initial y velocity calculations
        skyline::patching::Patch::in_text(0x6d2174).nop();

        // Stubs ControlModule::get_stick_x calls when calculating horizontal jump velocity
        skyline::patching::Patch::in_text(0x6ce6b8).nop();
        skyline::patching::Patch::in_text(0x6d19a4).nop();
        skyline::patching::Patch::in_text(0x6d1af0).nop();
        skyline::patching::Patch::in_text(0x6d0434).nop();

        // Stubs vanilla initial horizontal jump speed calculations
        skyline::patching::Patch::in_text(0x6ce6ec).nop();
        skyline::patching::Patch::in_text(0x6d19d8).nop();
        skyline::patching::Patch::in_text(0x6d1b24).nop();
        skyline::patching::Patch::in_text(0x6d04c4).nop();
    }
    skyline::install_hooks!(
        fullhop_initial_y_speed_hook,
        jump1_stick_x_hook,
        jump1_jump_speed_x_max_hook,
        jump2_stick_x_hook,
        jump2_jump_speed_x_max_hook,
        jump3_stick_x_hook,
        jump3_jump_speed_x_max_hook,
        jump4_stick_x_hook,
        jump4_jump_speed_x_max_hook
    );
    skyline::nro::add_hook(nro_hook);
}