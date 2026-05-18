use super::*;

unsafe extern "C" fn special_lw1_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
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
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );

    return 0.into();
}

// Forces Grounded Earthquake punch on the ground
unsafe extern "C" fn special_lw1_ground_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw1"), 0.0, 1.0, false, 0.0, false, false);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_10) + -1);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_10) + -1);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_GROUND), false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    VarModule::set_int(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE, 0);
    VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE_DISTANCE, 0.0);

    fighter.main_shift(special_lw1_ground_main_loop)
}

unsafe extern "C" fn special_lw1_ground_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    if VarModule::is_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_HOLD) {
        let charge_end_frame = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "earthquake_fist_ground.charge_end_frame");
        
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, charge_end_frame, true, true, false);
        }
        if fighter.motion_frame() >= charge_end_frame {
            VarModule::off_flag(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_HOLD);
        }
        else {
            let charge = VarModule::get_int(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE);
            let charge_distance = VarModule::get_float(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE_DISTANCE) as f32;
            let charge_distance_mod = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "earthquake_fist_ground.charge_distance_mod");
            let max_charge_distance = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "earthquake_fist_ground.max_charge_distance");
            let mut eff_pos_offset = (charge as f32/charge_distance_mod) + charge_distance + (max_charge_distance/charge_distance_mod);
            let lr = PostureModule::lr(fighter.module_accessor);

            let is_ground = GroundModule::ray_check(
                fighter.module_accessor, 
                &Vector2f{ x: PostureModule::pos_x(fighter.module_accessor) + ((charge_distance + 12.0) * lr), y: PostureModule::pos_y(fighter.module_accessor)}, 
                &Vector2f{ x: 0.0, y: -6.0}, true
            ) == 1;
            if is_ground {
                let pos_offset = charge_distance + (max_charge_distance/charge_distance_mod);

                VarModule::set_float(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE_DISTANCE, pos_offset);
                eff_pos_offset = (10.0 - 10.0 * (charge as f32/charge_distance_mod)) + charge_distance + (max_charge_distance/charge_distance_mod);
            }

            let eff_handle = VarModule::get_int64(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_QUAKE_EFFECT_HANDLE);

            EffectModule::set_pos(fighter.module_accessor, eff_handle as u32, &Vector3f::new(0.0, 0.0, eff_pos_offset));
            VarModule::set_int64(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_QUAKE_EFFECT_HANDLE, eff_handle as u64);
            VarModule::set_int(fighter.battle_object, vars::miifighter::status::SPECIAL_LW1_CHARGE, (charge + 1) as i32);
        }
    }

    return 0.into();
}

unsafe extern "C" fn special_lw1_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    fighter.set_int(0x50000000, *FIGHTER_MIIFIGHTER_STATUS_WORK_ID_KUIUCHI_HEAD_WORK_INT_FALL_HIT_OBJECT_ID);
    if fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_KUIUCHI_HEAD_FLAG_FROM_GR) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw1_loop"), 0.0, 1.0, false, 0.0, false, false);
        fighter.on_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_KUIUCHI_HEAD_FLAG1);
        fighter.on_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_KUIUCHI_HEAD_FLAG_FROM_GR);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw1"), 0.0, 1.0, false, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    if fighter.global_table[0x10].get_int() as i32 == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_GROUND {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x20cbc92683), 1, FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND, (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_10) + -1);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), (*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_10) + -1);
    }

    fighter.main_shift(special_lw1_air_main_loop)
}

unsafe extern "C" fn special_lw1_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    if !fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_KUIUCHI_HEAD_FLAG2) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.on_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_KUIUCHI_HEAD_FLAG2);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            let lw1_fall_vy = fighter.get_param_float("param_special_lw", "lw1_fall_vy");
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, lw1_fall_vy, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw1_loop"), 0.0, 1.0, false, 0.0, false, false);
            fighter.on_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_KUIUCHI_HEAD_FLAG1);
        }
    }

    if !fighter.is_situation(*SITUATION_KIND_AIR) {
        if fighter.is_flag(*FIGHTER_MIIFIGHTER_STATUS_WORK_ID_KUIUCHI_HEAD_FLAG1) {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
            fighter.change_status(FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_LANDING.into(), false.into());
            return 0.into();
        }
        else {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            return 0.into();
        }
    }

    if fighter.is_motion(Hash40::new("special_lw1_loop")) {
        // Allows EQF to be cancelled into freefall with second B press
        if (ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || fighter.status_frame() >= 40) {
            EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_smash_flash"), Hash40::new("top"), &Vector3f::new(1.0, 7.0, 5.0), &Vector3f::zero(), 0.5, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
            SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_miifighter_final06"), 0);
            let handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("se_miifighter_appeal_h01"), true, false, false, false, enSEType(0));
            SoundModule::set_se_vol(fighter.module_accessor, handle as i32, 1.5, 0);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
        fighter.check_wall_jump_cancel();
    }
    

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_GROUND, special_lw1_pre);
    agent.status(Main, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_GROUND, special_lw1_ground_main);
    
    agent.status(Pre, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_AIR, special_lw1_pre);
    agent.status(Main, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_AIR, special_lw1_air_main);
}