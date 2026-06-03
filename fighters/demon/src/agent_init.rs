use super::*;
use globals::*;

unsafe extern "C" fn check_attack_3_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    if [
        *FIGHTER_STATUS_KIND_SQUAT_WAIT,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_LW3_CANCEL,
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_1,
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_2,
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_3,
        *FIGHTER_DEMON_STATUS_KIND_SQUAT_TURN_AUTO,
    ].contains(&status) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_SQUAT) {
            if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
                fighter.change_status(FIGHTER_DEMON_STATUS_KIND_ATTACK_SQUAT_2.into(), true.into());
                return 1.into();
            }
        }
    }
    0.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Bool(false));
    fighter.global_table[0x55].assign(&L2CValue::Ptr(check_attack_3_uniq as *const () as _));
    fighter.global_table[0x57].assign(&L2CValue::Bool(false));
    fighter.global_table[0x58].assign(&L2CValue::Bool(false));
    fighter.global_table[0x59].assign(&L2CValue::Bool(false));
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);
}