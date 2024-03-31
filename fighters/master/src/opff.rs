// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn areadbhar_autocancel(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if [*FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT,
        *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
        if situation_kind == *SITUATION_KIND_AIR {
            if frame < 27.0 {
                VarModule::off_flag(boma.object(), vars::master::status::AIR_SPECIAL_S_AUTOCANCEL);
            }
            if frame >= 27.0 {
                VarModule::on_flag(boma.object(), vars::master::status::AIR_SPECIAL_S_AUTOCANCEL);
            }
        }
    }
    if status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_LANDING && VarModule::is_flag(boma.object(), vars::master::status::AIR_SPECIAL_S_AUTOCANCEL) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
    }
}

// Areadbhar Dash Cancel (Raging Storm)
unsafe fn areadbhar_dash_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if [*FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT,
        *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind) {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag()) {
            boma.check_dash_cancel();
        }
    }
}

unsafe fn specialhi_reset(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_status(*FIGHTER_STATUS_KIND_CLIFF_CATCH) {
        VarModule::off_flag(fighter.battle_object, vars::master::instance::SPECIAL_AIR_HI_CATCH);
    }
}

unsafe fn nspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_MASTER_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_MASTER_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                //ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

unsafe fn aymr_slowdown(boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status(*FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_HIT)  {
        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) && MotionModule::frame(boma) < 11.0 {
            SlowModule::set_whole(boma, 7, 100);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_TURN,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_MAX_SHOOT,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_S_FRONT_DASH,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_HI_WALL_JUMP,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_TURN,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_CANCEL
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
    //areadbhar_autocancel(boma, id, status_kind, situation_kind, frame);
    nspecial_cancels(boma, status_kind, situation_kind);
    aymr_slowdown(boma);
    specialhi_reset(fighter);
    fastfall_specials(fighter);

    // Magic Series
    //areadbhar_dash_cancel(boma, status_kind, situation_kind, cat[0]);
}

pub extern "C" fn master_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		master_frame(fighter)
    }
}

pub unsafe fn master_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, master_frame_wrapper);
}