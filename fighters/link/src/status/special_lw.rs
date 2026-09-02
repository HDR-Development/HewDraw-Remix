use super::*;

unsafe extern "C" fn special_lw_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_SHOOT) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    ret
}

unsafe extern "C" fn special_lw_blast_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        0,
        0,
        0,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn item_throw_status_main(agent: &mut L2CFighterCommon) -> L2CValue {
    //is down-special input
    let is_special_lw_input;
    let stick_x = ControlModule::get_stick_x(agent.module_accessor);
    let stick_y = ControlModule::get_stick_y(agent.module_accessor);
    let special_stick_x = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_x"));
    let special_stick_y = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("special_stick_y"));
    let squat_stick_y = WorkModule::get_param_float(agent.module_accessor, hash40("common"), hash40("squat_stick_y"));
    let special_button = ControlModule::check_button_trigger(agent.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);
    let special_s = agent.global_table[CMD_CAT1].get_i32() & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S) != 0;
    let special_lw = agent.global_table[CMD_CAT1].get_i32() & (*FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW) != 0;
    let item_throw = agent.global_table[CMD_CAT3].get_i32() & (*FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_ALL|*FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_AIR_ALL) != 0;
    let is_squat = agent.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SQUAT_WAIT;
    if (
        !special_s 
        && special_lw
    ) || (
        special_button
        && is_squat 
        && stick_y <= squat_stick_y
    ) || (
        special_button
        && !is_squat 
        && stick_x.abs() <= special_stick_x
        && stick_y <= special_stick_y*-1.0 
    ) {
        is_special_lw_input = true;
    }else {
        is_special_lw_input = false;
    }
    smashline::original_status(Main, agent, *FIGHTER_STATUS_KIND_ITEM_THROW)(agent);
    //special-lw disabled & has bomb
    let bomb_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOMB_OBJECT_ID);
    if !WorkModule::is_enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW) 
    && ItemModule::get_have_item_id(agent.module_accessor, 0) == bomb_id as u64 
    && is_special_lw_input || !item_throw {
        if MotionModule::motion_kind(agent.module_accessor) == hash40("item_light_throw_f")
        || MotionModule::motion_kind(agent.module_accessor) == hash40("item_light_throw_air_f") {
            if agent.is_situation(*SITUATION_KIND_GROUND) {
                MotionModule::change_motion(agent.module_accessor, Hash40::new("special_lw_throw"), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::set_int64(agent.module_accessor, hash40("special_lw_throw") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
                WorkModule::set_int64(agent.module_accessor, hash40("special_air_lw_throw") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND_OPPOSITE);
            }else {
                MotionModule::change_motion(agent.module_accessor, Hash40::new("special_air_lw_throw"), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::set_int64(agent.module_accessor, hash40("special_air_lw_throw") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND);
                WorkModule::set_int64(agent.module_accessor, hash40("special_lw_throw") as i64, *FIGHTER_STATUS_ITEM_THROW_WORK_INT_MOTION_KIND_OPPOSITE);
            }
            MotionModule::remove_motion_partial(agent.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_HAVE_ITEM, false);
        }
    }
    0.into()
}

unsafe extern "C" fn item_throw_status_end(agent: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(End, agent, *FIGHTER_STATUS_KIND_ITEM_THROW)(agent);
    if ItemModule::is_have_item(agent.module_accessor, 0) {
        let bomb_id = WorkModule::get_int(agent.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_INT_BOMB_OBJECT_ID);
        if ItemModule::get_have_item_id(agent.module_accessor, 0) == bomb_id as u64 {
        }
    }
    0.into()
}

unsafe extern "C" fn special_lw_throw_game(agent: &mut L2CAgentBase) {
    FT_MOTION_RATE(agent, 0.4);
    frame(agent.lua_state_agent, 27.0);
    if is_excute(agent) {
        let lr = PostureModule::lr(agent.module_accessor);
        let item_id = ItemModule::get_have_item_id(agent.module_accessor,0) as u32;
        let item_boma = sv_battle_object::module_accessor(item_id);
        agent.clear_lua_stack();
        lua_args!(agent, 12, 2, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_ANGLE, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_SPEED, *ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_POWER);
        smash::app::sv_animcmd::THROW_ITEM_OFFSET(agent.lua_state_agent);
        KineticModule::add_speed(item_boma, &Vector3f{x:2.05*lr, y:-0.54, z:0.0});
    }
    frame(agent.lua_state_agent, 32.0);
    if is_excute(agent) {
    FT_MOTION_RATE(agent, 1.0);
}}

unsafe extern "C" fn special_lw_throw_exp(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 5.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(agent.lua_state_agent, 47.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
    frame(agent.lua_state_agent, 52.0);
    if is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
    }
}

unsafe extern "C" fn special_lw_blast_exp(agent: &mut L2CAgentBase) {
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(agent.lua_state_agent, 9.0);
    if is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 34.0);
    if is_excute(agent) {
        //fixing shield visibility not switching when grabbing an item
        if !ItemModule::is_have_item(agent.module_accessor, 0) {
            VisibilityModule::set_int64(agent.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ITEM_THROW, item_throw_status_main);
    agent.status(End, *FIGHTER_STATUS_KIND_ITEM_THROW, item_throw_status_end);

    agent.game_acmd("game_speciallwthrow", special_lw_throw_game, Priority::High);
    agent.game_acmd("game_specialairlwthrow", special_lw_throw_game, Priority::High);
    agent.expression_acmd("expression_speciallwthrow", special_lw_throw_exp, Priority::High);
    agent.expression_acmd("expression_specialairlwthrow", special_lw_throw_exp, Priority::High);

    agent.expression_acmd("expression_speciallwblast", special_lw_blast_exp, Priority::High);
    agent.expression_acmd("expression_specialairlwblast", special_lw_blast_exp, Priority::High);

    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_pre);
    agent.status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST, special_lw_blast_pre);
}