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

#[skyline::hook(offset = 0x6d19a4, inline)]
unsafe fn jump2_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = (*boma).left_stick_x();
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d1af0, inline)]
unsafe fn jump3_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = (*boma).left_stick_x();
    asm!("fmov s0, w8", in("w8") left_stick_x)
}

#[skyline::hook(offset = 0x6d0434, inline)]
unsafe fn jump4_stick_x_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let control_module = *ctx.registers[0].x.as_ref();
    let boma = *(control_module as *mut *mut BattleObjectModuleAccessor).add(1);
    let left_stick_x = (*boma).left_stick_x();
    asm!("fmov s0, w8", in("w8") left_stick_x)
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
    }
    skyline::install_hooks!(
        fullhop_initial_y_speed_hook,
        jump1_stick_x_hook,
        jump2_stick_x_hook,
        jump3_stick_x_hook,
        jump4_stick_x_hook
    );
    skyline::nro::add_hook(nro_hook);
}