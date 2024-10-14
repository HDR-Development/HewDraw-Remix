// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;
 
unsafe fn teleport_logic(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) && !VarModule::is_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_GROUNDED_TELEPORT) {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            VarModule::on_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_GROUNDED_TELEPORT);
        } //touching ground at any point counts as a grounded start
    }
    if fighter.is_status(*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2) {
        if StatusModule::is_changing(boma) {
            if boma.get_num_used_jumps() < boma.get_jump_count_max() {
                VarModule::on_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_TELEPORT_CANCEL);
                if !VarModule::is_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_GROUNDED_TELEPORT) {
                    WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                    VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_FLOAT);
                }
            } else {//takes jump and float frame 0, takes away tourney winner if teleport is used 
                VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_FLOAT);
                VarModule::off_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_TELEPORT_CANCEL);
            }
            VarModule::off_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_GROUNDED_TELEPORT);
        } else if MotionModule::is_end(boma) {
            if VarModule::is_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_TELEPORT_CANCEL) {
                PostureModule::set_stick_lr(boma, 0.0);
                PostureModule::update_rot_y_lr(boma);
            }// Allows M2 to turnaround based on stick position when reappearing
        } 
    }
    if fighter.is_prev_status(*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3) {
        if StatusModule::is_changing(fighter.module_accessor) && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            VarModule::on_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_ENABLE_FREEFALL);
        }
    }
    // Actionability when double jump isn't burned
    if fighter.is_motion(Hash40::new("special_air_hi"))
    && !VarModule::is_flag(boma.object(), vars::mewtwo::instance::SPECIAL_HI_ENABLE_FREEFALL)
    && VarModule::is_flag(fighter.battle_object, vars::mewtwo::instance::SPECIAL_HI_TELEPORT_CANCEL)
    && fighter.motion_frame() > 9.0 {
        VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
        CancelModule::enable_cancel(boma);
    }
}

unsafe fn dj_upB_jump_refresh(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && fighter.is_prev_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL)
    && fighter.global_table[PREV_STATUS_FRAME].get_i32() <= 3 {
        // Grants 1 extra jump if all jumps used up
        if fighter.get_num_used_jumps() == fighter.get_jump_count_max() {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
    }
}

pub unsafe fn mewtwo_teleport_wall_ride(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, id: usize) {
    // Wall Ride momentum fixes
    if boma.is_status(*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_2) {
        let init_speed_x = VarModule::get_float(boma.object(), vars::common::status::TELEPORT_INITIAL_SPEED_X);
        let init_speed_y = VarModule::get_float(boma.object(), vars::common::status::TELEPORT_INITIAL_SPEED_Y);
        if !GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_ID_NONE as u32) {
            if !VarModule::is_flag(boma.object(), vars::common::status::IS_TELEPORT_WALL_RIDE) {
                VarModule::on_flag(boma.object(), vars::common::status::IS_TELEPORT_WALL_RIDE);
            }
        }
        if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) {
            if init_speed_y > 0.0 {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, init_speed_y);
                app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            }
        } else if VarModule::is_flag(boma.object(), vars::common::status::IS_TELEPORT_WALL_RIDE) {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, init_speed_x, init_speed_y);
            app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        }
    }
    else if boma.is_status(*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3) {
        if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) {
            if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
                let wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
                KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            }
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_MAX,
        *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3
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
    teleport_logic(fighter, boma);
    mewtwo_teleport_wall_ride(fighter, boma, status_kind, id);
    dj_upB_jump_refresh(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn mewtwo_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		mewtwo_frame(fighter)
    }
}

pub unsafe fn mewtwo_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, mewtwo_frame_wrapper);
}