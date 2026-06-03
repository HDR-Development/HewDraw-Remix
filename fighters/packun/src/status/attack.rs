use super::*;

unsafe extern "C" fn attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.check_attack_mtrans();
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(smash::lua2cpp::L2CFighterCommon_check_attack_mtrans as *const () as _));
    fighter.sub_status_AttackComboCommon();
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ATTACK_WORK_INT_100_HIT_NEAR_COUNT_CLIFF_STOP);
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_main_loop as *const () as _))
}

unsafe extern "C" fn attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = fighter.status_Attack_Main();

    if fighter.is_motion(Hash40::new("attack_13"))
    && VarModule::get_int(fighter.battle_object, vars::packun::instance::CURRENT_STANCE) == 1 {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_100, false);
        return 0.into();
    }

    return ret;
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK, attack_main);
}