use super::*;
use globals::*;

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);

    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_lw").into());
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
    if speed_y < 0.0 {
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            0.0
        );
    }

    fighter.sub_change_motion_by_situation(
        Hash40::new("special_lw").into(),
        Hash40::new("special_air_lw").into(),
        false.into()
    );

    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, -1);

    let motion = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        Hash40::new("special_lw")
    }
    else {
        Hash40::new("special_air_lw")
    };
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, motion, false, -1.0);

    VarModule::set_int(fighter.battle_object, vars::master::status::SPECIAL_LW_FALL_COUNT, 30);

    if !StopModule::is_stop(fighter.module_accessor) {
        special_lw_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_lw_substatus as *const () as _));

    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool()
    && VarModule::is_flag(fighter.battle_object, vars::master::status::SPECIAL_LW_FALLING) {
        VarModule::countdown_int(fighter.battle_object, vars::master::status::SPECIAL_LW_FALL_COUNT, 0);
    }
    0.into()
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }

    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if MotionModule::is_end(fighter.module_accessor) {
        let status = if situation == *SITUATION_KIND_GROUND {
            FIGHTER_STATUS_KIND_WAIT
        }
        else {
            FIGHTER_STATUS_KIND_FALL_AERIAL
        };
        fighter.change_status(status.into(), false.into());
        return 0.into();
    }

    special_lw_check_kinetics(fighter);

    let situation = fighter.global_table[SITUATION_KIND].get_i32();

    if VarModule::is_flag(fighter.battle_object, vars::master::status::SPECIAL_LW_FALLING) {
        if (
            situation == *SITUATION_KIND_GROUND ||
            VarModule::get_int(fighter.battle_object, vars::master::status::SPECIAL_LW_FALL_COUNT) == 0
        )
        && MotionModule::rate(fighter.module_accessor) == 0.0 {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, 1.0);
            VarModule::off_flag(fighter.battle_object, vars::master::status::SPECIAL_LW_FALLING);

            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                0.0
            );

            special_lw_motion_helper(fighter);
        }
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor)
    && !VarModule::is_flag(fighter.battle_object, vars::master::status::SPECIAL_LW_IS_JUMP) {
        special_lw_motion_helper(fighter);
    }

    0.into()
}

unsafe extern "C" fn special_lw_motion_helper(fighter: &mut L2CFighterCommon) {
    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    let (kinetic, correct, motion) = if situation == *SITUATION_KIND_GROUND {
        (*FIGHTER_KINETIC_TYPE_GROUND_STOP, *GROUND_CORRECT_KIND_GROUND, Hash40::new("special_lw"))
    }
    else {
        (*FIGHTER_KINETIC_TYPE_AIR_STOP, *GROUND_CORRECT_KIND_AIR, Hash40::new("special_air_lw"))
    };

    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    MotionModule::change_motion_inherit_frame(
        fighter.module_accessor,
        motion,
        -1.0,
        1.0,
        0.0,
        false,
        false
    );
    // ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_AXE, motion, true, -1.0);
}

unsafe extern "C" fn special_lw_check_kinetics(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, vars::master::status::SPECIAL_LW_JUMP) {
        // let you turn around the jump

        PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
        PostureModule::update_rot_y_lr(fighter.module_accessor);

        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);

        KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY, fighter.module_accessor);

        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_AIR,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );

        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            -1.0,
            -1.0
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.0,
            0.0
        );

        let lr = PostureModule::lr(fighter.module_accessor);
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let speed_add_x = stick_x * 0.8;
        let speed_add_y = stick_y * 0.5;
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            (2.5 * lr) + speed_add_x,
            3.0 + speed_add_y
        );
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            0.15,
            0.175
        );

        StatusModule::set_keep_situation_air(fighter.module_accessor, true);

        VarModule::off_flag(fighter.battle_object, vars::master::status::SPECIAL_LW_JUMP);
        VarModule::on_flag(fighter.battle_object, vars::master::status::SPECIAL_LW_IS_JUMP);
    }

    if VarModule::is_flag(fighter.battle_object, vars::master::status::SPECIAL_LW_FALL) {
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );

        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            4.0
        );
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            4.0
        );
        sv_kinetic_energy!(
            set_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -4.0
        );
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -0.5
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

        VarModule::off_flag(fighter.battle_object, vars::master::status::SPECIAL_LW_FALL);
        VarModule::off_flag(fighter.battle_object, vars::master::status::SPECIAL_LW_IS_JUMP);
        VarModule::on_flag(fighter.battle_object, vars::master::status::SPECIAL_LW_FALLING);
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
}