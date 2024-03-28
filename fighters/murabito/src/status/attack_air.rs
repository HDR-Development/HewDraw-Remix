use super::*;

// FIGHTER_STATUS_KIND_ATTACK_AIR

pub unsafe extern "C" fn attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air();
    let motion = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    if [hash40("attack_air_hi")].contains(&motion) {
        // Usually there's code in here to check for the random turnip pulls. However... we don't want that.
        // Instead, we want to force the turnip count to go in a rotation of 1 > 2 > 3 > 1 > 2 > 3 ...
        let mut turnip_num_hi = VarModule::get_int(fighter.battle_object, vars::murabito::instance::TURNIP_NUM_HI);
        // Adds 1 to the turnip count. If the new turnip count is not 1, 2, or 3, reset it back to 1.
        turnip_num_hi += 1;
        if !(1..=3).contains(&turnip_num_hi) {
            turnip_num_hi = 1;
        }
        VarModule::set_int(fighter.battle_object, vars::murabito::instance::TURNIP_NUM_HI, turnip_num_hi);
        WorkModule::set_int(fighter.module_accessor, turnip_num_hi, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM);

    } else if [hash40("attack_air_lw")].contains(&motion) {
        let mut turnip_num_lw = VarModule::get_int(fighter.battle_object, vars::murabito::instance::TURNIP_NUM_LW);
        turnip_num_lw += 1;
        if !(1..=3).contains(&turnip_num_lw) {
            turnip_num_lw = 1;
        }
        VarModule::set_int(fighter.battle_object, vars::murabito::instance::TURNIP_NUM_LW, turnip_num_lw);
        WorkModule::set_int(fighter.module_accessor, turnip_num_lw, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TURNIP_NUM);

    } else if [
        hash40("attack_air_f"),
        hash40("attack_air_b")
    ].contains(&motion) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHOOT_NUM);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_AttackAir_Main as *const () as _))
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air);
}