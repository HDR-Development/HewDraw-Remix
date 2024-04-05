use super::*;
use globals::*;
// status script import

mod special_n;
mod special_s;
mod special_hi;
mod special_lw;
mod guard;

pub unsafe fn spend_ink(fighter: &mut L2CFighterCommon, ink_cost: f32) -> bool {
    let ink_current = WorkModule::get_float(fighter.module_accessor,*FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK);
    if (ink_current-ink_cost) < 0.0 {
        return false;
    }
    let fighter_ptr = fighter.global_table[0x4].get_ptr() as *mut Fighter;
    let ink_max = FighterSpecializer_Inkling::get_ink_max(fighter_ptr);
    let new_ink = (0.0 as f32).max(ink_current-ink_cost);
    WorkModule::set_float(fighter.module_accessor, new_ink, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS);

    //println!("Max: {ink_max} Current: {new_ink}");
    if ink_max <= 0.0 {
        FighterSpecializer_Inkling::lack_ink(fighter_ptr);
    }
    if new_ink <= 0.0 {
        VisibilityModule::set_status_default(fighter.module_accessor, Hash40::new_raw(0x4ad12b739), Hash40::new_raw(0xa48dd021e));
        MotionModule::add_motion_partial(fighter.module_accessor, *FIGHTER_INKLING_MOTION_PART_SET_KIND_TANK, Hash40::new_raw(0xa48dd021e), 0.0, 1.0, true, false,0.0,true,true,false);
    }
    FighterSpecializer_Inkling::change_ink(fighter_ptr, new_ink);

    return true;
}

unsafe extern "C" fn guard_on(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOn()
}

unsafe extern "C" fn guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Guard()
}

// Prevents sideB from being used again if it has already been used once in the current airtime
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::inkling::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

// Re-enables the ability to use sideB when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::inkling::instance::DISABLE_SPECIAL_S);
    }
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD])
    || !app::sv_information::is_ready_go() {
        EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
        VarModule::set_float(fighter.battle_object,vars::inkling::instance::SPECIAL_LW_CHARGE,0.0);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    special_n::install(agent);
    special_s::install(agent);
    special_lw::install(agent);
    special_hi::install(agent);
    guard::install(agent);

}
