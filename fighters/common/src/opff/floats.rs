use super::*;

pub unsafe fn run(fighter: &mut L2CFighterCommon, situation_kind: i32) {
    if situation_kind == *SITUATION_KIND_GROUND
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_GIMMICK_SPRING_JUMP]) {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_FLOATING);
        VarModule::off_flag(fighter.battle_object, vars::common::instance::DISABLE_FLOAT);
    }
}
