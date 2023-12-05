use super::*;
use globals::*;

pub fn install() {
    install_status_scripts!(
        attack_s4_start_main,
        attack_s4_main,
        attack_s4_hold_main,
    );
}

#[status_script(agent = "packun", status = FIGHTER_STATUS_KIND_ATTACK_S4_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_s4_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
        MotionModule::change_motion(boma, Hash40::new("attack_s4_2"), 0.0, 1.0, false, 0.0, false, false);
    }
    return fighter.status_AttackS4Start();
}

#[status_script(agent = "packun", status = FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_s4_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
        MotionModule::change_motion(boma, Hash40::new("attack_s4_hold_2"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackS4Hold as *const () as _))
}

#[status_script(agent = "packun", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
        MotionModule::change_motion(boma, Hash40::new("attack_s4_2"), 24.0, 1.0, false, 0.0, false, false);
    }
    return fighter.status_AttackS4();
}