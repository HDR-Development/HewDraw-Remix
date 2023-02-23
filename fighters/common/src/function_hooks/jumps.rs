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
    }
    skyline::install_hooks!(fullhop_initial_y_speed_hook);
    skyline::nro::add_hook(nro_hook);
}