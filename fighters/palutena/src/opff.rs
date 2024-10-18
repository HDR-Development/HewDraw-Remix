// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3,
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

unsafe fn teleport_logic(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if !fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3]) {
        return;
    }
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        if StatusModule::is_changing(boma) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                VarModule::on_flag(boma.object(), vars::palutena::instance::SPECIAL_HI_TELEPORT_GROUND_START);
            }
            else {
                VarModule::off_flag(boma.object(), vars::palutena::instance::SPECIAL_HI_TELEPORT_GROUND_START);
            }
        }
    }
    if fighter.is_status(*FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2) {
        if StatusModule::is_changing(boma) {
            if !VarModule::is_flag(boma.object(), vars::palutena::instance::SPECIAL_HI_TELEPORT_GROUND_START)
            && !(boma.get_num_used_jumps() < boma.get_jump_count_max()) {
                VarModule::on_flag(fighter.battle_object, vars::palutena::status::SPECIAL_HI_TELEPORT_AIR_START);
            }
        }
        // Allow turnaround based on stick position when reappearing
        if MotionModule::is_end(boma) {
            PostureModule::set_stick_lr(boma, 0.0);
            PostureModule::update_rot_y_lr(boma);
        }
        // Prevent actionability toggle when touching ground during the travel
        if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_DOWN as u32)
        && !VarModule::is_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_ENABLE_FREEFALL)
        && VarModule::is_flag(boma.object(), vars::palutena::status::SPECIAL_HI_TELEPORT_AIR_START) {
            VarModule::on_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_ENABLE_FREEFALL);
        }
    }
    if fighter.is_status(*FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3) {
        if StatusModule::is_changing(boma) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                VarModule::on_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_ENABLE_FREEFALL);
            }
            else {
                // Actionability when double jump isn't burned
                if boma.get_num_used_jumps() < boma.get_jump_count_max()
                && !VarModule::is_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_ENABLE_FREEFALL) {
                    VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
                    CancelModule::enable_cancel(boma);
                    // Consume double jump, except when Teleport is initiated on ground
                    if !VarModule::is_flag(boma.object(), vars::palutena::instance::SPECIAL_HI_TELEPORT_GROUND_START) {
                        WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                    }
                }
            }
        }
    }
}

unsafe fn dj_upB_jump_refresh(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL) {
        // If first 3 frames of dj
        if fighter.status_frame() <= 3 {
            VarModule::on_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_JUMP_REFRESH);
        }
        else {
            VarModule::off_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_JUMP_REFRESH);
        }
    }
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && fighter.is_prev_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL)
    && VarModule::is_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_JUMP_REFRESH) {
        // Grants 1 extra jump if all jumps used up
        if fighter.get_num_used_jumps() == fighter.get_jump_count_max() {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
        VarModule::off_flag(fighter.battle_object, vars::palutena::instance::SPECIAL_HI_JUMP_REFRESH);
    }
}

pub unsafe fn palutena_teleport_wall_ride(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32) {
    // Wall Ride momentum fixes
    if boma.is_status(*FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2) {
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
    else if boma.is_status(*FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3) {
        if GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_SIDE as u32) {
            if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
                let wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
                KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            }
        }
        if StatusModule::is_changing(boma) && boma.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::mul_speed(boma, &Vector3f::new(0.6, 1.0, 1.0), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        }
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

// Power Board Death Reset
unsafe fn var_reset(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) {
        VarModule::on_flag(fighter.object(), vars::palutena::instance::SPECIAL_N_FLUSH_BOARD);
    }

    if [*FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) || !sv_information::is_ready_go() {
        VarModule::on_flag(fighter.object(), vars::palutena::instance::SPECIAL_N_FLUSH_BOARD);
    }
}

// Training Mode color charging
unsafe fn training_mode_taunts(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if is_training_mode() {
        if (fighter.is_motion(Hash40::new("appeal_s_r")) || fighter.is_motion(Hash40::new("appeal_s_l")))
        && fighter.motion_frame() == 2.0 {
            VarModule::set_int(fighter.object(), vars::palutena::instance::SPECIAL_N_GAINED_COLOR, 3);
        }
        if (fighter.is_motion(Hash40::new("appeal_hi_r")) || fighter.is_motion(Hash40::new("appeal_hi_l")))
        && fighter.motion_frame() == 2.0 {
            VarModule::set_int(fighter.object(), vars::palutena::instance::SPECIAL_N_GAINED_COLOR, 2);
        }
        if (fighter.is_motion(Hash40::new("appeal_lw_r")) || fighter.is_motion(Hash40::new("appeal_lw_l")))
        && fighter.motion_frame() == 2.0 {
            VarModule::set_int(fighter.object(), vars::palutena::instance::SPECIAL_N_GAINED_COLOR, 1);
        }
    }
}

// sets set_color var, controlling when a color is charged
unsafe fn color_charge(fighter: &mut L2CFighterCommon) {
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && !AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_PARRY)
    && VarModule::is_flag(fighter.object(), vars::palutena::status::ENABLE_COLOR_INCREMENT) {
        VarModule::off_flag(fighter.object(), vars::palutena::status::ENABLE_COLOR_INCREMENT);
        // yellow moves: side
        if fighter.is_motion(Hash40::new("attack_s3_s"))
        || fighter.is_motion(Hash40::new("attack_s4_s"))
        || fighter.is_motion(Hash40::new("attack_air_f"))
        || fighter.is_motion(Hash40::new("attack_air_b")) {
            VarModule::set_int(fighter.object(), vars::palutena::instance::SPECIAL_N_GAINED_COLOR, 3);
        }

        // blue moves: up
        else if fighter.is_motion(Hash40::new("attack_hi3"))
        || fighter.is_motion(Hash40::new("attack_hi4"))
        || fighter.is_motion(Hash40::new("attack_air_hi")) {
            VarModule::set_int(fighter.object(), vars::palutena::instance::SPECIAL_N_GAINED_COLOR, 2);
        }

        // red moves: down
        else if fighter.is_motion(Hash40::new("attack_lw3"))
        || fighter.is_motion(Hash40::new("attack_lw4"))
        || fighter.is_motion(Hash40::new("attack_air_lw")) {
            VarModule::set_int(fighter.object(), vars::palutena::instance::SPECIAL_N_GAINED_COLOR, 1);
        }
    }
}

// handles the color charges
unsafe fn power_board(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    // check if we should gain a color
    if VarModule::get_int(fighter.object(), vars::palutena::instance::SPECIAL_N_GAINED_COLOR) != 0 {
        // set slot 2 to old slot 1, slot 1 becomes new color; fill up 1 stock if possible
        VarModule::set_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2, VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1));
        VarModule::set_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1, VarModule::get_int(fighter.object(), vars::palutena::instance::SPECIAL_N_GAINED_COLOR));
        VarModule::set_int(boma.object(), vars::palutena::instance::SPECIAL_N_GAINED_COLOR, 0);
        utils::ui::UiManager::change_power_board_color(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1),
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2)
        );
    }

    // check if we should flush our power board
    if VarModule::is_flag(fighter.object(), vars::palutena::instance::SPECIAL_N_FLUSH_BOARD) {
        // set each slot to 0
        VarModule::set_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2, 0);
        VarModule::set_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1, 0);
        VarModule::off_flag(fighter.object(), vars::palutena::instance::SPECIAL_N_FLUSH_BOARD);
        utils::ui::UiManager::change_power_board_color(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1),
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2)
        );

        VarModule::on_flag(fighter.object(), vars::palutena::status::POWER_BOARD_FLUSHED);
    }
    
    utils::ui::UiManager::change_power_board_color(
        fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
        VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1),
        VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2)
    );
    
}

pub extern "C" fn palu_power_board(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        if !sv_information::is_ready_go() && fighter.status_frame() < 1 {
            return;
        }
        utils::ui::UiManager::set_power_board_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_power_board_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1),
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2)
        );
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    palutena_teleport_wall_ride(fighter, boma, id, status_kind, situation_kind, cat[0]);
    teleport_logic(fighter, boma);
    var_reset(fighter, id, status_kind);
    training_mode_taunts(fighter, id, status_kind);
    dj_upB_jump_refresh(fighter);
    power_board(fighter, boma, status_kind);
    color_charge(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn palutena_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		palutena_frame(fighter)
    }
}

pub unsafe fn palutena_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, palutena_frame_wrapper);
    agent.on_line(Main, palu_power_board);
}