use super::*;

pub unsafe fn run(fighter: &mut L2CFighterCommon, status_kind: i32, situation_kind: i32) {
    if situation_kind == *SITUATION_KIND_GROUND || status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_FLOAT);
    }
}
