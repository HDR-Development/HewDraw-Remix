// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn bair_boost_detection(boma: &mut BattleObjectModuleAccessor){
    if boma.get_aerial() == Some(AerialKind::Bair) 
    && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
        if boma.is_cat_flag(Cat1::AttackS4){
            VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
        } else {
            VarModule::off_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
        }
    }
}

// resets the once-per-airtime bounce for sideb/bair when landing/respawning
unsafe fn boost_reset(boma: &mut BattleObjectModuleAccessor) {
    if !VarModule::is_flag(boma.object(), vars::robot::instance::ATTACK_AIR_B_USED)
    || !VarModule::is_flag(boma.object(), vars::robot::instance::SPECIAL_S_AIR_USED) {
        if boma.is_situation(*SITUATION_KIND_GROUND)
        || boma.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_REBIRTH,
            *FIGHTER_STATUS_KIND_WIN,
            *FIGHTER_STATUS_KIND_LOSE,
            *FIGHTER_STATUS_KIND_ENTRY
        ]) {
            VarModule::on_flag(boma.object(), vars::robot::instance::ATTACK_AIR_B_USED);
            VarModule::on_flag(boma.object(), vars::robot::instance::SPECIAL_S_AIR_USED);
        }
    }
}

unsafe fn special_hi_handling(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if WorkModule::get_float(fighter.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE) <= 0.0 {
        WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
    }

    if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP {
        VarModule::set_int(fighter.battle_object, vars::robot::instance::SPECIAL_HI_CHARGE_FRAME, 0);
    }

    if StatusModule::prev_status_kind(boma, 1) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        PostureModule::set_rot(fighter.module_accessor, &Vector3f::zero(), 0);
    }
}

// PM-like down-b canceling
unsafe fn special_lw_cancels(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_ROBOT_STATUS_KIND_SPECIAL_LW_HOLD)
    && boma.is_situation(*SITUATION_KIND_AIR)
    && boma.is_cat_flag(Cat1::AirEscape) {
        WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
    }
}

unsafe fn meter_control(boma: &mut BattleObjectModuleAccessor) {
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let info = app::lua_bind::FighterManager::get_fighter_information(crate::singletons::FighterManager(), app::FighterEntryID(entry_id));
    if lua_bind::FighterManager::is_result_mode(utils::singletons::FighterManager()) {
        MeterModule::reset(boma.object());
    }
    
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE
    ]) {
        MeterModule::reset(boma.object());
    }

    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_ENTRY]) 
    || !sv_information::is_ready_go() {
        VarModule::on_flag(boma.object(), vars::robot::instance::IS_INIT_METER);
    }

    if VarModule::is_flag(boma.object(), vars::robot::instance::IS_INIT_METER) {
        MeterModule::reset(boma.object());
        MeterModule::set_meter_cap(boma.object(), 200);
        MeterModule::set_meter_per_level(boma.object(), 100.0);
        
        MeterModule::add(boma.object(), 200.0);
        VarModule::off_flag(boma.object(), vars::robot::instance::IS_INIT_METER);
    }

    if MeterModule::meter(boma.object()) < 200.0 && boma.is_situation(*SITUATION_KIND_GROUND) {
        MeterModule::add(boma.object(), 0.75);
    }

    if MeterModule::meter(boma.object()) != WorkModule::get_float(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE) 
    && !boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        let diff = WorkModule::get_float(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE) - MeterModule::meter(boma.object());

        if WorkModule::get_float(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE) == 10.0 {
            let remain = MeterModule::meter(boma.object());
            MeterModule::drain_direct(boma.object(), remain);
        } else {
            if diff >= 0.0 {
                MeterModule::add(boma.object(), diff);
            } else {
                MeterModule::drain_direct(boma.object(), diff.abs());
            }
        }
    }

    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        let charge_frame = VarModule::get_int(boma.object(), vars::robot::instance::SPECIAL_HI_CHARGE_FRAME);

        if charge_frame == 1 {
            MeterModule::drain_direct(boma.object(), 20.0);
        } else if charge_frame > 10 {
            MeterModule::drain_direct(boma.object(), 2.0);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_LW_HOLD,
        *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_LW_END
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

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    bair_boost_detection(boma);
    boost_reset(boma);
    special_hi_handling(fighter, boma);
    special_lw_cancels(boma);
    meter_control(boma);
    fastfall_specials(fighter);
}

unsafe extern "C" fn robot_meter(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        if !sv_information::is_ready_go() && fighter.status_frame() < 1 {
            return;
        }
        MeterModule::update(fighter.object(), false);
        utils::ui::UiManager::set_robot_meter_enable(fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32, true);
        utils::ui::UiManager::set_robot_meter_info(
            fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32,
            MeterModule::meter(fighter.object()),
            (MeterModule::meter_cap(fighter.object()) as f32 * MeterModule::meter_per_level(fighter.object())),
            MeterModule::meter_per_level(fighter.object())
        );
    }
}

pub extern "C" fn robot_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        robot_frame(fighter)
    }
}

pub unsafe fn robot_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, robot_frame_wrapper);
    agent.on_line(Main, robot_meter);
}
