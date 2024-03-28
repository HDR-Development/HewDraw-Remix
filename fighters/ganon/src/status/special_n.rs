use super::*;

unsafe extern "C" fn special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_SpecialNCommon();
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
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
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N as u32,
        0
    );

    0.into()
}

unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Motion Kind change depending on situation.
    VarModule::on_flag(fighter.battle_object, vars::ganon::instance::DISABLE_SPECIAL_N);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
    fighter.sub_change_motion_by_situation(L2CValue::Hash40s("float_start"), L2CValue::Hash40s("float_air_start"), false.into());
    if fighter.global_table[globals::SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        let speed_y = {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy::get_speed_y(fighter.lua_state_agent)
        };
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y.clamp(-0.05, 0.5)
        );
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
    else {
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }
    fighter.main_shift(special_n_main_loop)
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Decided which direction Ganon should float.
    if VarModule::is_flag(fighter.battle_object, vars::ganon::status::FLOAT_GROUND_DECIDE_ANGLE) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE);
        let stick_x = fighter.global_table[globals::STICK_X].get_f32();
        let angle = (45.0 * -stick_x).to_radians();
        sv_kinetic_energy!(
            set_angle,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            angle
        );
        if angle != 0.0 {
            sv_kinetic_energy!(
                set_speed_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_MOTION,
                1.2
            );
        }
        VarModule::off_flag(fighter.battle_object, vars::ganon::status::FLOAT_GROUND_DECIDE_ANGLE);
    }
    // Increases Ganon's fall speed when this flag is enabled.
    if VarModule::is_flag(fighter.battle_object, vars::ganon::status::FLOAT_GROUND_CHANGE_KINETIC) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        let speed_y = {
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy::get_speed_y(fighter.lua_state_agent)
        };
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            speed_y.clamp(-0.05, 0.1)
        );
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
        VarModule::off_flag(fighter.battle_object, vars::ganon::status::FLOAT_GROUND_CHANGE_KINETIC);
    }
    // Make sure if you touch the ground you actually land.
    if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_MOTION_AIR
    && fighter.global_table[globals::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_LIGHT.into(), false.into());
        return 0.into();
    }
    // Only perform these actions if vars::ganon::status::FLOAT_ENABLE_ACTIONS is true.
    if VarModule::is_flag(fighter.battle_object, vars::ganon::status::FLOAT_ENABLE_ACTIONS) {
        // if the proper transition terms are enabled, these functions will check for
        // if Ganon performs an aerial, a double jump, or airdodge.
        if fighter.sub_transition_group_check_air_cliff().get_bool()
        || fighter.sub_transition_group_check_air_attack().get_bool()
        || fighter.sub_transition_group_check_air_jump_aerial().get_bool()
        || fighter.sub_transition_group_check_air_escape().get_bool() {
            return 1.into();
        }
        // If Special is pressed, enable a flag and transition into the next status.
        if fighter.global_table[globals::PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0
        || fighter.global_table[globals::STICK_Y].get_f32() <= -0.7 {
            VarModule::on_flag(fighter.battle_object, vars::ganon::status::FLOAT_CANCEL);
            // Clear the buffer here so you don't accidentally buffer a side special on cancel.
            fighter.change_status(statuses::ganon::SPECIAL_N_FLOAT.into(), true.into());
            return 0.into();
        }
    }
    // When the animation ends, transition to the next status.
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(statuses::ganon::SPECIAL_N_FLOAT.into(), false.into());
    }

    0.into()
}

unsafe extern "C" fn special_n_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_n_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_end);
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_init);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_exec);
}
