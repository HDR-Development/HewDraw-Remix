use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
    sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);

    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR
    && !VarModule::is_flag(fighter.battle_object, vars::wiifit::instance::SPECIAL_S_DISABLE_STALL) {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);

        let heading_init_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("heading_init_speed_y"));
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, heading_init_speed_y);

        let heading_gravity = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("heading_gravity"));
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -heading_gravity);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    InputModule::set_command_life_count_max(fighter.battle_object, 3);

    return 0.into();
}

unsafe extern "C" fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP {
        VarModule::set_flag(fighter.battle_object, vars::wiifit::instance::SPECIAL_S_DISABLE_STALL, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR);
    }
    return 0.into();
}

// FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP

unsafe extern "C" fn special_s_jump_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::wiifit::instance::SPECIAL_S_DISABLE_STALL) {
        return 0.into();
    }
    smashline::original_status(Init, fighter, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP)(fighter)
}

unsafe extern "C" fn special_s_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.change_motion_by_situation("special_s_jump", "special_air_s_jump", 0.0, 1.0, false, 0.0, false, false);
    fighter.main_shift(special_s_jump_main_loop)
}

unsafe extern "C" fn special_s_jump_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_HEADING.into(), false.into());
        return 0.into();
    }
    let heading_min_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), 0x24df8311a5);
    if fighter.status_frame() > heading_min_frame {
        if fighter.is_pad_flag(PadFlag::SpecialTrigger | PadFlag::AttackTrigger) {
            fighter.change_status(FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_HEADING.into(), false.into());
            return 0.into();
        }
    }
    if fighter.status_frame() > 3 {
        if fighter.is_pad_flag(PadFlag::GuardTrigger) {
            if !VarModule::is_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL) {
                VarModule::on_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
                ControlModule::reset_trigger(fighter.module_accessor);
                ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return 0.into();
            }
            let heading_cancel_landing_frame = fighter.get_param_int("param_special_s", "heading_cancel_landing_frame");
            fighter.set_float(heading_cancel_landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
    }
    
    return 0.into();
}

unsafe extern "C" fn special_s_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_flag(fighter.battle_object, vars::wiifit::instance::SPECIAL_S_DISABLE_STALL, fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR);
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_end);

    agent.status(Init, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_init);
    agent.status(Main, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_main);
    agent.status(End, *FIGHTER_WIIFIT_STATUS_KIND_SPECIAL_S_JUMP, special_s_jump_end);
}