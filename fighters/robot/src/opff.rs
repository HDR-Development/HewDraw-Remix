// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn gyro_dash_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
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

unsafe fn uspecial_cancels(boma: &mut BattleObjectModuleAccessor, situation_kind: i32, frame: f32) {
    if WorkModule::is_flag(boma, *FIGHTER_ROBOT_STATUS_BURNER_FLAG_PUSH_B_BUTTON)
    && situation_kind == *SITUATION_KIND_AIR {
        if frame > 14.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ATTACK, false);
            }
        }
        
        if frame > 19.0 {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) {
                WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
                ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, false);
                //let vec = Vector3f{x: 0.0, y: -0.2, z: 0.0};
                //KineticModule::add_speed(boma, &vec);
            } else if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_ATTACK, false);
            }
        }
    }   
}

// JC grounded sideb on hit
unsafe fn jc_sideb(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64) {
    if ([*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_S_END, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_S_ATTACK].contains(&status_kind))
    && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
    && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                boma.check_jump_cancel(false);
    }
}

// Dair only bounces once per airtime
unsafe fn dair_boost_reset(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    if boma.is_situation(*SITUATION_KIND_GROUND)
    || boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD,
                                  *FIGHTER_STATUS_KIND_REBIRTH,
                                  *FIGHTER_STATUS_KIND_WIN,
                                  *FIGHTER_STATUS_KIND_LOSE,
                                  *FIGHTER_STATUS_KIND_ENTRY]){
        WorkModule::on_flag(boma, vars::robot::instance::AIRTIME_DAIR);
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

// Neutral Special Cancels
unsafe fn neutral_special_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag()) {
            boma.check_jump_cancel(false);
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

unsafe fn fuel_indicator_effect(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor){
    let max_fuel = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("energy_max_frame"));
    let current_fuel = WorkModule::get_float(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);

    // Bools to hold which fuel level we're at
    let mut low_fuel = false;
    let mut mid_fuel = false;
    let mut high_fuel = false;

    // Fuel level thresholds
    let high_fuel_threshold = max_fuel * 0.75;
    let mid_fuel_threshold = max_fuel * 0.66;
    let low_fuel_threshold = max_fuel * 0.33;

    // Bool to hold whether or not our fuel threshold has changed
    let mut is_fuel_threshold_changed = false;
    let prev_fuel_threshold = VarModule::get_int(boma.object(), vars::robot::instance::PREV_FUEL_THRESHOLD);
    if current_fuel < low_fuel_threshold{
        if VarModule::get_int(boma.object(), vars::robot::instance::PREV_FUEL_THRESHOLD) != 1 {
            is_fuel_threshold_changed = true;
        }
        VarModule::set_int(boma.object(), vars::robot::instance::PREV_FUEL_THRESHOLD, 1);
        low_fuel = true;
    }
    else if current_fuel < mid_fuel_threshold{
        if VarModule::get_int(boma.object(), vars::robot::instance::PREV_FUEL_THRESHOLD) != 2 {
            is_fuel_threshold_changed = true;
        }
        VarModule::set_int(boma.object(), vars::robot::instance::PREV_FUEL_THRESHOLD, 2);
        mid_fuel = true;
    }
    else{
        if VarModule::get_int(boma.object(), vars::robot::instance::PREV_FUEL_THRESHOLD) != 3 {
            is_fuel_threshold_changed = true;
        }
        VarModule::set_int(boma.object(), vars::robot::instance::PREV_FUEL_THRESHOLD, 3);
        high_fuel = true;
    }

    // Respawn the effect if it's been removed
    let fuel_effect_indicator_handle = VarModule::get_int(boma.object(), vars::robot::instance::PASSIVE_FUEL_INDICATOR_EFFECT_HANDLE);
    if !EffectModule::is_exist_effect(boma, fuel_effect_indicator_handle as u32) && !high_fuel { // Don't spawn the effect at high fuel
        //EFFECT_FOLLOW(fighter, Hash40::new("robot_lamp_l"), Hash40::new("waist2"), 5.0, 0, 0, 0, 0, 0, 2.0, true);
        let new_fuel_effect_indicator_handle = EffectModule::req_follow(
            boma,
            Hash40::new("robot_lamp_l"),
            Hash40::new("waist1"),
            &Vector3f::zero(),
            &Vector3f::zero(),
            2.0,
            true,
            0,
            0,
            0,
            0,
            0,
            true,
            true
        ) as u32;
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        //VarModule::set_int(boma.object(), vars::robot::instance::PASSIVE_FUEL_INDICATOR_EFFECT_HANDLE, EffectModule::get_last_handle(boma) as i32);
        VarModule::set_int(boma.object(), vars::robot::instance::PASSIVE_FUEL_INDICATOR_EFFECT_HANDLE, new_fuel_effect_indicator_handle as i32);
    }

    // Kill the old effect and spawn the effect again with a new color if the fuel threshold has changed
    let current_fuel_effect_indicator_handle = VarModule::get_int(boma.object(), vars::robot::instance::PASSIVE_FUEL_INDICATOR_EFFECT_HANDLE) as u32;
    if is_fuel_threshold_changed{
        EffectModule::kill(boma, current_fuel_effect_indicator_handle as u32, true, true);
        //EFFECT_FOLLOW(fighter, Hash40::new("robot_lamp_l"), Hash40::new("waist2"), 5.0, 0, 0, 0, 0, 0, 2.0, true);
        if !high_fuel{ // Don't spawn the effect if high fuel
            let new_fuel_effect_indicator_handle = EffectModule::req_follow(
                boma,
                Hash40::new("robot_lamp_l"),
                Hash40::new("waist1"),
                &Vector3f::new(0.0, 0.0, 0.0),
                &Vector3f::zero(),
                2.0,
                true,
                0,
                0,
                0,
                0,
                0,
                true,
                true
            ) as u32;
            LAST_EFFECT_SET_RATE(fighter, 0.5);
            //VarModule::set_int(boma.object(), vars::robot::instance::PASSIVE_FUEL_INDICATOR_EFFECT_HANDLE, EffectModule::get_last_handle(boma) as i32);
            VarModule::set_int(boma.object(), vars::robot::instance::PASSIVE_FUEL_INDICATOR_EFFECT_HANDLE, new_fuel_effect_indicator_handle as i32);
            if low_fuel {
                // Don't change the effect color since it's already red
            }
            else if mid_fuel {
                // Yellow fuel indicator
                LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.5, 0.1);
            }
            /*
            // High fuel
            else {
                // Blue fuel indicator
                LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.1, 1.2);
            }
            */
        }
    }
    
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //gyro_dash_cancel(boma, status_kind, situation_kind, cat[0], frame);
    //neutral_special_cancels(boma, status_kind, situation_kind, cat[0]);
    //jc_sideb(boma, cat[0], status_kind, situation_kind, motion_kind);
    dspecial_cancels(boma, status_kind, situation_kind, cat[0]);
    uspecial_cancels(boma, situation_kind, frame);
    dair_boost_reset(boma, status_kind, situation_kind);
    bair_boost_reset(boma, status_kind, situation_kind);
    bair_boost_detection(boma);
    fuel_indicator_effect(fighter, boma);
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