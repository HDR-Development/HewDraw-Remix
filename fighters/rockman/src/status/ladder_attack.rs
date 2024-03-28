use super::*;

unsafe extern "C" fn ladder_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_LadderAttack_common();
    let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    if [
        hash40("attack_air_hi"), hash40("attack_air_lw")
    ].contains(&mot) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_SHOOT_NUM);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_LadderAttack_Main as *const () as _))
}

unsafe extern "C" fn ladder_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_LadderAttack()
}

pub fn install(agent: &mut Agent) {
    agent.status(
            Main,
            *FIGHTER_STATUS_KIND_LADDER_ATTACK,
            ladder_attack_main,
        );
    agent.status(
            End,
            *FIGHTER_STATUS_KIND_LADDER_ATTACK,
            ladder_attack_end,
        );
}
