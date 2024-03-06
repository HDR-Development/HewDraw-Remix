use super::*;

pub unsafe fn run(fighter: &mut L2CFighterCommon, situation_kind: i32) {
    if situation_kind != *SITUATION_KIND_AIR {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_FLOAT);
    }
}
