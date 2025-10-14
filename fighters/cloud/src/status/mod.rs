use super::*;
use globals::*;
// status script import

mod special_lw;
mod special_s;
mod special_hi;

// prevents limit charge from being used while the limit charge graphic is still above cloud
// effectively this is a limit charge cooldown. the length is limit_gauge_frame in vl.prcxml
unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.get_int(*FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_LIMIT_GAUGE_REMOVE_FRAME) > 0
    && !fighter.is_flag(*FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK) {
        return false.into();
    }
    return true.into();
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Reset shine stall flag on landing or ledgegrab
    if [*SITUATION_KIND_GROUND, *SITUATION_KIND_CLIFF].contains(&fighter.global_table[SITUATION_KIND].get_i32())
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) {
        VarModule::off_flag(fighter.battle_object, vars::cloud::instance::SPECIAL_S_DISABLE_STALL);
    }
    true.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
    special_lw::install(agent);
    special_s::install(agent);
    special_hi::install(agent);
}