// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;
 
unsafe fn gyro_dash_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    let current_fuel = WorkModule::get_float(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
    let max_fuel = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("energy_max_frame"));
    // Use 50% fuel to dash cancel gyro
    let boost_fuel_depletion = max_fuel * 0.50;
    if status_kind == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_LW_END {
        if frame > 11.0 {
            if current_fuel > boost_fuel_depletion {
                boma.check_dash_cancel();
            }
        }
    }
}

// Bair only bounces once per airtime
unsafe fn bair_boost_reset(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if boma.is_situation(*SITUATION_KIND_GROUND)
    || boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD,
                                  *FIGHTER_STATUS_KIND_REBIRTH,
                                  *FIGHTER_STATUS_KIND_WIN,
                                  *FIGHTER_STATUS_KIND_LOSE,
                                  *FIGHTER_STATUS_KIND_ENTRY]){
        WorkModule::on_flag(boma, vars::robot::instance::AIRTIME_BAIR);
    }
}

// Sideb only bounces once per airtime
unsafe fn sideb_boost_reset(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if boma.is_situation(*SITUATION_KIND_GROUND)
    || boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD,
                                  *FIGHTER_STATUS_KIND_REBIRTH,
                                  *FIGHTER_STATUS_KIND_WIN,
                                  *FIGHTER_STATUS_KIND_LOSE,
                                  *FIGHTER_STATUS_KIND_ENTRY]){
        WorkModule::on_flag(boma, vars::robot::instance::AIRTIME_SIDEB);
    }
}

// Neutral Special Cancels
unsafe fn neutral_special_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag()) {
            boma.check_jump_cancel(false, false);
        }
    }
}

unsafe fn dspecial_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    //PM-like down-b canceling
    if status_kind == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_LW_HOLD {
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat1::AirEscape) {
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

unsafe fn bair_boost_detection(boma: &mut BattleObjectModuleAccessor){
    if boma.get_aerial() == Some(AerialKind::Bair) && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CSTICK_ON) {
        if boma.is_cat_flag(Cat1::AttackS4){
            VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
        }
        else{
            VarModule::off_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
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

unsafe fn upb_opff(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if WorkModule::get_float(fighter.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE) < 0.0 {
        WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
    } else if WorkModule::get_float(fighter.module_accessor, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE) == 0.0 {
        WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
    }

    if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP {
        VarModule::set_float(fighter.battle_object, vars::robot::instance::FRAMES_SINCE_UPB, 0.0);
        VarModule::set_float(fighter.battle_object, vars::robot::instance::FRAMES_SINCE_UPB_RISE, 0.0);
        VarModule::set_float(fighter.battle_object, vars::robot::instance::JOINT_ROT, 0.0);
        VarModule::off_flag(fighter.battle_object, vars::robot::instance::UPB_CANCEL);
        VarModule::off_flag(fighter.battle_object, vars::robot::instance::GROUNDED_UPB);
    }

    if StatusModule::prev_status_kind(boma, 1) == *FIGHTER_STATUS_KIND_SPECIAL_HI
    {
        PostureModule::set_rot(fighter.module_accessor, &Vector3f::zero(), 0);
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
        *FIGHTER_STATUS_KIND_LOSE,]) {
        MeterModule::reset(boma.object());
    }

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ENTRY,]) || !sv_information::is_ready_go() {
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
        let robotFrames = VarModule::get_float(boma.object(), vars::robot::instance::FRAMES_SINCE_UPB);

        if robotFrames == 1.0 {
            MeterModule::drain_direct(boma.object(), 20.0);
        } else if robotFrames > 10.0 {
            MeterModule::drain_direct(boma.object(), 2.0);
        }
    }

}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //gyro_dash_cancel(boma, status_kind, situation_kind, cat[0], frame);
    //neutral_special_cancels(boma, status_kind, situation_kind, cat[0]);
    dspecial_cancels(boma, status_kind, situation_kind, cat[0]);
    bair_boost_reset(boma, status_kind, situation_kind);
    sideb_boost_reset(boma, status_kind, situation_kind);
    bair_boost_detection(boma);
    fastfall_specials(fighter);
    upb_opff(fighter, boma);
    meter_control(boma);
}

#[fighter_frame( agent = FIGHTER_KIND_ROBOT )]
pub fn robot_meter(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
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

#[utils::macros::opff(FIGHTER_KIND_ROBOT )]
pub fn robot_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
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