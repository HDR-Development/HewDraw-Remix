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

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    palutena_teleport_wall_ride(fighter, boma, id, status_kind, situation_kind, cat[0]);
    actionable_teleport_air(fighter, boma, id, status_kind, situation_kind, frame);
    var_reset(fighter, id, status_kind);
    training_mode_taunts(fighter, id, status_kind);
    dj_upB_jump_refresh(fighter);
    power_board(fighter, boma, status_kind);
    color_charge(fighter);
    power_cast(fighter);
    fastfall_specials(fighter);
}

unsafe fn actionable_teleport_air(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(fighter.module_accessor)
    && (fighter.is_situation(*SITUATION_KIND_GROUND)
        || fighter.is_situation(*SITUATION_KIND_CLIFF)
        || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]))
    {
        VarModule::off_flag(fighter.battle_object, vars::palutena::instance::UP_SPECIAL_FREEFALL);
    }

    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
    && !StatusModule::is_changing(boma)
    && boma.status_frame() == 1 {
        VarModule::off_flag(boma.object(), vars::palutena::instance::GROUNDED_TELEPORT);
        if situation_kind == *SITUATION_KIND_GROUND {
            VarModule::on_flag(boma.object(), vars::palutena::instance::GROUNDED_TELEPORT);
        }
    }
    // Allows Palutena to turnaround based on stick position when reappearing
    if status_kind == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2
    && !StatusModule::is_changing(boma)
    && MotionModule::is_end(boma) {
        if boma.get_num_used_jumps() < boma.get_jump_count_max() {
            PostureModule::set_stick_lr(boma, 0.0);
            PostureModule::update_rot_y_lr(boma);
        }
    }

    if fighter.is_prev_status(*FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3) {
        if StatusModule::is_changing(fighter.module_accessor) {
            VarModule::on_flag(fighter.battle_object, vars::palutena::instance::UP_SPECIAL_FREEFALL);
        }
    }
    
    // Actionability when double jump isn't burned
    if status_kind == *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3
    && !StatusModule::is_changing(boma)
    && situation_kind == *SITUATION_KIND_AIR
    && boma.get_num_used_jumps() < boma.get_jump_count_max() 
    && !VarModule::is_flag(fighter.battle_object, vars::palutena::instance::UP_SPECIAL_FREEFALL) {
        VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
        CancelModule::enable_cancel(boma);
        // Consume double jump, except when Teleport is initiated on ground
        if !VarModule::is_flag(boma.object(), vars::palutena::instance::GROUNDED_TELEPORT) {
            WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
    }
}

unsafe fn dj_upB_jump_refresh(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL) {
        // If first 3 frames of dj
        if fighter.status_frame() <= 3 {
            VarModule::on_flag(fighter.battle_object, vars::palutena::instance::UP_SPECIAL_JUMP_REFRESH);
        }
        else {
            VarModule::off_flag(fighter.battle_object, vars::palutena::instance::UP_SPECIAL_JUMP_REFRESH);
        }
    }
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && fighter.is_prev_status(*FIGHTER_STATUS_KIND_JUMP_AERIAL)
    && VarModule::is_flag(fighter.battle_object, vars::palutena::instance::UP_SPECIAL_JUMP_REFRESH) {
        // Grants 1 extra jump if all jumps used up
        if fighter.get_num_used_jumps() == fighter.get_jump_count_max() {
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
        VarModule::off_flag(fighter.battle_object, vars::palutena::instance::UP_SPECIAL_JUMP_REFRESH);
    }
}

pub unsafe fn palutena_teleport_wall_ride(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32) {
    // Wall Ride momentum fixes
    let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
    let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);

    if boma.is_status(*FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_2) {
        let touch_normal_y_left = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
        let touch_normal_y_right = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
        if (touch_right && touch_normal_y_right != 0.0)
        || (touch_left && touch_normal_y_left != 0.0)
        {
            let init_speed_y = VarModule::get_float(boma.object(), vars::common::status::TELEPORT_INITIAL_SPEED_Y);

            if init_speed_y > 0.0 {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, init_speed_y);
                app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
            }
        }
    }
    else if boma.is_status(*FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3) {
        if touch_right || touch_left {
            if KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0 {
                let wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
                KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            }
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
        MeterModule::drain_direct(fighter.object(), 999.0);
        VarModule::on_flag(fighter.object(), vars::palutena::instance::FLUSH);
    }

    if [*FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) || !sv_information::is_ready_go() {
        VarModule::on_flag(fighter.object(), vars::palutena::instance::FLUSH);
        MeterModule::reset(fighter.object());
    }
}

// Training Mode Aegis Reflector timer taunt reset & color charging
unsafe fn training_mode_taunts(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if is_training_mode() {
        if (fighter.is_motion(Hash40::new("appeal_s_r")) || fighter.is_motion(Hash40::new("appeal_s_l")))
        && fighter.motion_frame() == 2.0 {
            VarModule::set_int(fighter.object(), vars::palutena::instance::SET_COLOR, 3);
        }
        if (fighter.is_motion(Hash40::new("appeal_hi_r")) || fighter.is_motion(Hash40::new("appeal_hi_l")))
        && fighter.motion_frame() == 2.0 {
            VarModule::set_int(fighter.object(), vars::palutena::instance::SET_COLOR, 2);
        }
        if (fighter.is_motion(Hash40::new("appeal_lw_r")) || fighter.is_motion(Hash40::new("appeal_lw_l")))
        && fighter.motion_frame() == 2.0 {
            VarModule::set_int(fighter.object(), vars::palutena::instance::SET_COLOR, 1);
        }
    }
}

// sets set_color var, controlling when a color is charged
unsafe fn color_charge(fighter: &mut L2CFighterCommon) {
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && VarModule::is_flag(fighter.object(), vars::palutena::status::CAN_INCREASE_COLOR) {
        VarModule::off_flag(fighter.object(), vars::palutena::status::CAN_INCREASE_COLOR);
        // yellow moves: neutral/side
        if fighter.is_motion(Hash40::new("attack_dash"))
        || fighter.is_motion(Hash40::new("attack_s3_s"))
        || fighter.is_motion(Hash40::new("attack_s4_s"))
        || fighter.is_motion(Hash40::new("attack_air_f"))
        || fighter.is_motion(Hash40::new("attack_air_b")) {
            VarModule::set_int(fighter.object(), vars::palutena::instance::SET_COLOR, 3);
        }

        // blue moves: up
        else if fighter.is_motion(Hash40::new("attack_hi3"))
        || fighter.is_motion(Hash40::new("attack_hi4"))
        || fighter.is_motion(Hash40::new("attack_air_hi")) {
            VarModule::set_int(fighter.object(), vars::palutena::instance::SET_COLOR, 2);
        }

        // red moves: down
        else if fighter.is_motion(Hash40::new("attack_lw3"))
        || fighter.is_motion(Hash40::new("attack_lw4"))
        || fighter.is_motion(Hash40::new("attack_air_lw")) {
            VarModule::set_int(fighter.object(), vars::palutena::instance::SET_COLOR, 1);
        }
    }
}

// handles the color charges
unsafe fn power_board(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    // check if we should gain a color
    if VarModule::get_int(fighter.object(), vars::palutena::instance::SET_COLOR) != 0 {
        // set slot 2 to old slot 1, slot 1 becomes new color; fill up 1 stock if possible
        VarModule::set_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2, VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1));
        VarModule::set_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1, VarModule::get_int(fighter.object(), vars::palutena::instance::SET_COLOR));
        MeterModule::add(fighter.object(), 50.0);
        VarModule::set_int(boma.object(), vars::palutena::instance::SET_COLOR, 0);
        utils::ui::UiManager::change_power_board_color(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1),
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2)
        );
    }

    // check if we should flush our power board
    if VarModule::is_flag(fighter.object(), vars::palutena::instance::FLUSH) {
        // set each slot to 0
        VarModule::set_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2, 0);
        VarModule::set_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1, 0);
        VarModule::off_flag(fighter.object(), vars::palutena::instance::FLUSH);
        utils::ui::UiManager::change_power_board_color(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1),
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2)
        );
    }
    utils::ui::UiManager::change_power_board_color(
        fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
        VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1),
        VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2)
    );
    
}

// checks which color spell should be cast
unsafe fn power_cast(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N) {
        let color_1 = VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1);
        let color_2 = VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2);
        if color_1 == 1 {
            if color_2 == 2 {
                fighter.change_status(statuses::palutena::SPECIAL_N_P.into(), false.into());
                //println!("and why he ourple");

            }
            else if color_2 == 3 {
                fighter.change_status(statuses::palutena::SPECIAL_N_O.into(), false.into());
                //println!("bornana");
            }
            else {
                if VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 1 {
                    VarModule::on_flag(fighter.object(), vars::palutena::instance::POWERED);
                }
                fighter.change_status(statuses::palutena::SPECIAL_N_R.into(), false.into());
                //println!("red");
            }
        }
        else if color_1 == 2 {
            if color_2 == 1 {
                fighter.change_status(statuses::palutena::SPECIAL_N_P.into(), false.into());
                //println!("and why he ourple");
            }
            else if color_2 == 3 {
                fighter.change_status(statuses::palutena::SPECIAL_N_G.into(), false.into());
                //println!("i like cash from my hair to my ass");
            }
            else {
                if VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 2 {
                    VarModule::on_flag(fighter.object(), vars::palutena::instance::POWERED);
                }
                fighter.change_status(statuses::palutena::SPECIAL_N_B.into(), false.into());
                //println!("blud");
            }
        }
        else if color_1 == 3 {
            if color_2 == 1 {
                fighter.change_status(statuses::palutena::SPECIAL_N_O.into(), false.into());
                //println!("bornana");
            }
            else if color_2 == 2 {
                fighter.change_status(statuses::palutena::SPECIAL_N_G.into(), false.into());
                //println!("i like cash from my hair to my ass");
            }
            else {
                if VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2) == 3 {
                    VarModule::on_flag(fighter.object(), vars::palutena::instance::POWERED);
                }
                fighter.change_status(statuses::palutena::SPECIAL_N_Y.into(), false.into());
                //println!("ielo");
            }
        }
    }
}

pub extern "C" fn palu_power_board(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        if !sv_information::is_ready_go() && fighter.status_frame() < 1 {
            return;
        }
        MeterModule::update(fighter.object(), false);
        MeterModule::set_meter_cap(fighter.object(), 2);
        utils::ui::UiManager::set_power_board_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_power_board_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            MeterModule::meter(fighter.object()),
            (MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object())),
            MeterModule::meter_per_level(fighter.object()),
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_1),
            VarModule::get_int(fighter.object(), vars::palutena::instance::POWER_BOARD_SLOT_2)
        );
    }
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

pub extern "C" fn reflection_board_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        if weapon.is_status(*WEAPON_PALUTENA_REFLECTIONBOARD_STATUS_KIND_SHOOT) {
            let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let palutena = utils::util::get_battle_object_from_id(owner_id);
            let palutena_boma = &mut *(*palutena).module_accessor;
            if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_ATTACK){
                StatusModule::change_status_request_from_script(weapon.module_accessor, *WEAPON_PALUTENA_REFLECTIONBOARD_STATUS_KIND_BREAK, false);
            }
        }
    }
}

pub fn install() {
    smashline::Agent::new("palutena")
        .on_line(Main, palutena_frame_wrapper)
        .on_line(Main, palu_power_board)
        .install();
    smashline::Agent::new("palutena_reflectionboard")
        .on_line(Main, reflection_board_callback)
        .install();
}