// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;


unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH) {
        let sword_scale = Vector3f{x: 0.7, y: 1.0, z: 1.0};
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("swordl1"), &sword_scale);
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("swordr1"), &sword_scale);
    }
    else {
        let long_sword_scale = Vector3f{x: 0.95, y: 1.0, z: 1.0};
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("swordl1"), &long_sword_scale);
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("swordr1"), &long_sword_scale);
    }
}

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FIGHTER_EDGE_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

unsafe fn refresh_flare(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_int(boma.object(), vars::edge::instance::FIRE_ID) != -1
    && ArticleModule::get_active_num(boma, *FIGHTER_EDGE_GENERATE_ARTICLE_FIRE) < 1 {
        VarModule::set_int(boma.object(), vars::edge::instance::FIRE_ID, -1);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CHARGE,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_SHOOT,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    sword_length(boma);
    nspecial_cancels(boma, status_kind, situation_kind, cat[1]);
    refresh_flare(boma);
    fastfall_specials(fighter);
}

pub extern "C" fn edge_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		edge_frame(fighter)
    }
}

pub unsafe fn edge_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, edge_frame_wrapper);
}