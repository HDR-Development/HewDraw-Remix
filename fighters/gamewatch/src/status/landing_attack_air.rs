use super::*;

#[status_script(agent = "gamewatch", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn landing_attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK as u64,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32,
        0
    );

    0.into()
}

pub fn install() {
    smashline::install_status_scripts!(
        landing_attack_air_pre
    );
}
