use super::*;
use globals::*;
// status script import

mod special_hi;

/// Prevents down b being reused
unsafe extern "C" fn should_use_special_lw_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER) > 0 {
        false.into()
    } else {
        true.into()
    }
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::USE_SPECIAL_LW_CALLBACK].assign(&L2CValue::Ptr(should_use_special_lw_callback as *const () as _));
    VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
    VarModule::off_flag(fighter.battle_object, vars::rosetta::instance::SPECIAL_LW_TICO_UNAVAILABLE);
    VarModule::off_flag(fighter.battle_object, vars::rosetta::status::SPECIAL_LW_INVALID_WARP);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    special_hi::install(agent);
}
