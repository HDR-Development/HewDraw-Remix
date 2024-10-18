use super::*;

// statuses::ganon::SPECIAL_N_FLOAT

unsafe extern "C" fn special_n_float_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_n_float_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cancel = VarModule::is_flag(fighter.battle_object, vars::ganon::status::SPECIAL_N_END);
    let frame =
    if cancel {
        59.0
    }
    else {
        0.0
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("float"),
        frame,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !cancel {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -0.015 // hardcoded value for now
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -0.05 // hardcoded value for now
        );
    }
    fighter.main_shift(special_n_float_main_loop)
}

unsafe extern "C" fn special_n_float_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Increases Ganon's fall speed when this flag is enabled.
    if VarModule::is_flag(fighter.battle_object, vars::ganon::status::SPECIAL_N_CHANGE_FALL_SPEED) {
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -0.25 // hardcoded value for now
        );
        VarModule::off_flag(fighter.battle_object, vars::ganon::status::SPECIAL_N_CHANGE_FALL_SPEED);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    // Make sure if you touch the ground you actually land.
    if fighter.global_table[globals::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_LIGHT.into(), false.into());
        return 0.into();
    }
    // Only perform these actions if vars::ganon::status::FLOAT_ENABLE_ACTIONS is true.
    if VarModule::is_flag(fighter.battle_object, vars::ganon::status::SPECIAL_N_ENABLE_ACTION) {
        // if the proper transition terms are enabled, these functions will check for
        // if Ganon performs an aerial, airdodge, or a double jump.
        if fighter.sub_transition_group_check_air_cliff().get_bool()
        || fighter.sub_transition_group_check_air_attack().get_bool()
        || fighter.sub_transition_group_check_air_jump_aerial().get_bool()
        || fighter.sub_transition_group_check_air_escape().get_bool() {
            return 1.into();
        }
        // If Special is pressed, enable a flag and transition into the next status.
        if fighter.global_table[globals::PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0
        || fighter.global_table[globals::STICK_Y].get_f32() <= -0.7 {
            VarModule::on_flag(fighter.battle_object, vars::ganon::status::SPECIAL_N_END);
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("float"),
                59.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
                0.0,
                speed_y,
                0.0,
                0.0,
                0.0
            );
            return 0.into();
        }
    }
    // Transition to Fall when the animation ends.
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }

    0.into()
}

unsafe extern "C" fn special_n_float_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, statuses::ganon::SPECIAL_N_FLOAT, special_n_float_pre);
    agent.status(Main, statuses::ganon::SPECIAL_N_FLOAT, special_n_float_main);
    agent.status(End, statuses::ganon::SPECIAL_N_FLOAT, special_n_float_end);
}
