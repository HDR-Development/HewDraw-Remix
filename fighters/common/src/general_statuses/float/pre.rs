use super::*;

#[no_mangle]
unsafe fn float_pre_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ATTACK_AIR {
        VarModule::off_flag(fighter.battle_object, vars::common::status::FLOAT_INHERIT_AERIAL);
    }
    let fs_succeeds = if VarModule::is_flag(fighter.battle_object, vars::common::status::FLOAT_INHERIT_AERIAL) {
        *FS_SUCCEEDS_KEEP_VISIBILITY | *FS_SUCCEEDS_KEEP_ATTACK | *FS_SUCCEEDS_KEEP_EFFECT
    }
    else {
        0
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        fs_succeeds
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        0,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32,
        0
    );
    0.into()
}
