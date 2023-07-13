use super::*;
use globals::*;


#[status_script(agent = "ganon", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_STATUS_ATTR_DISABLE_GROUND_FRICTION) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    0.into()
}

#[status_script(agent = "ganon", status = FIGHTER_GANON_STATUS_KIND_SPECIAL_LW_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    let ret = original!(fighter);

    let start_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_START_SITUATION);
    let end_situation = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_END_SITUATION);
    if start_situation == *SITUATION_KIND_AIR
    && end_situation == *FIGHTER_GANON_KICK_END_SITUATION_AG {
        // Allows you to slide when landing early into Wizard Kick's animation
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: x_speed.abs(), y: 0.0, z: 0.0});
    }
    else if start_situation == *SITUATION_KIND_AIR
    && end_situation == *FIGHTER_GANON_KICK_END_SITUATION_AA {
        // Allows you to slide when landing late into Wizard Kick's animation
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_KICK_START_SITUATION);
    }

    ret
}

pub fn install() {
    smashline::install_status_scripts!(
        special_lw_pre,
        special_lw_end_main
    );
}
