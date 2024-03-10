use super::*;
use globals::*;

extern "Rust" {
    #[link_name = "float_pre_common"]
    fn float_pre_common(fighter: &mut L2CFighterCommon) -> L2CValue;
    #[link_name = "float_check_aerial"]
    fn float_check_aerial(fighter: &mut L2CFighterCommon);
    #[link_name = "float_set_aerial"]
    fn float_set_aerial(fighter: &mut L2CFighterCommon);
    #[link_name = "float_end_common"]
    fn float_end_common(fighter: &mut L2CFighterCommon) -> L2CValue;
}

unsafe extern "C" fn float_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    float_pre_common(fighter)
}

unsafe extern "C" fn float_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);

    let eff_handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("reflet_catch"), Hash40::new("top"), &Vector3f{x: 0.0, y: -6.0, z: -5.3}, &Vector3f::zero(), 0.7, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::set_rgb(fighter.module_accessor, eff_handle as u32, 0.0, 1.0, 0.0);  // elwind green
    let eff_handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_aura_light"), Hash40::new("bookc"), &Vector3f::zero(), &Vector3f::zero(), 1.5, true, 0, 0, 0, 0, 0, false, false);
    EffectModule::set_rgb(fighter.module_accessor, eff_handle as u32, 0.0, 1.0, 0.078);  // elwind green

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
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);

    VarModule::set_int(fighter.battle_object, vars::common::status::FLOAT_FRAME, 30);
    VarModule::set_int(fighter.battle_object, vars::common::status::FLOAT_ENABLE_UNIQ, 1);
    VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_FLOAT);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("fall"),
        0.0,
        0.5, // purposefully run the animation slower because it might look nicer
        false,
        0.0,
        false,
        false
    );

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    let shield_stiff_mul_attack_air = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_stiff_mul_attack_air"));
    AttackModule::set_shield_stiff_mul(fighter.module_accessor, shield_stiff_mul_attack_air);
    float_check_aerial(fighter);
    // if VarModule::is_flag(fighter.battle_object, vars::common::instance::FLOAT_RAY_CHECK) {
        let pos_x = PostureModule::pos_x(fighter.module_accessor);
        let pos_y = PostureModule::pos_y(fighter.module_accessor);
        let pos_z = PostureModule::pos_z(fighter.module_accessor);
        let result = &mut Vector2f{x: 0.0, y: 0.0};
        let ray_check = GroundModule::ray_check_hit_pos(
            fighter.module_accessor,
            &Vector2f{x: pos_x, y: pos_y},
            &Vector2f{x: 0.0, y: -6.0},
            result,
            true
        );
        if ray_check {
            let ray_check_y = result.y;
            // let uniq_float_start_y = WorkModule::get_float(fighter.module_accessor, hash40("param_private"), hash40("uniq_float_start_y"));
            let uniq_float_start_y = 1.5;
            PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos_x, y: ray_check_y + uniq_float_start_y, z: pos_z});
        }
    // }

    fighter.sub_shift_status_main(L2CValue::Ptr(reflet_float_main_loop as *const () as _))
}

unsafe extern "C" fn reflet_float_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    // if StatusModule::is_changing(fighter.module_accessor)
    // && fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_AIR
    // && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
    //     FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
    //     float_set_aerial(fighter);
    //     return 0.into();
    // }

    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING) {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR.into(), false.into());
        return 0.into();
    }

    if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_MTRANS) == 2
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    && {
        let stick_y = fighter.left_stick_y();
        let jump_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y"));
        stick_y <= jump_stick_y
    } {
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(cat1));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        return 0.into();
    }

    if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_MTRANS) == 1
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    && {
        let stick_y = fighter.left_stick_y();
        let jump_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("jump_stick_y"));
        stick_y <= jump_stick_y
    } {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
        return 0.into();
    }

    if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_MTRANS) == 2
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) <= 0 {
        ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N);
        let cat1 = ControlModule::get_command_flag_cat(fighter.module_accessor, 0);
        fighter.global_table[CMD_CAT1].assign(&L2CValue::I32(cat1));
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        return 0.into();
    }

    if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_MTRANS) == 1
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) <= 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), true.into());
        return 0.into();
    }

    // Unique Down Special stuff is usually here but we don't want it

    if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_ENABLE_UNIQ) == 1 {
        if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_FRAME) > 0 {
            if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_FRAME) == 30 {
                let mut reflet_fighter = app::Fighter{battle_object: *(fighter.battle_object)};
                app::FighterSpecializer_Reflet::change_hud_kind(&mut reflet_fighter, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND);
                app::FighterSpecializer_Reflet::change_grimoire(fighter.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND);
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
            }

            VarModule::dec_int(fighter.battle_object, vars::common::status::FLOAT_FRAME);

            if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_FRAME) % 10 == 0 {
                WorkModule::dec_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
            }

            if VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_FRAME) == 0 {
                VarModule::set_int(fighter.battle_object, vars::common::status::FLOAT_FRAME, 30);
            }

            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) <= 0 {
                FighterSpecializer_Reflet::set_flag_to_table(fighter.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
                VarModule::set_int(fighter.battle_object, vars::common::status::FLOAT_ENABLE_UNIQ, 0);
            }
        }
        if WorkModule::is_enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING) {
            fighter.sub_transition_group_check_air_landing();
        }
        if WorkModule::is_enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL) {
            fighter.sub_transition_group_check_air_special();
        }

        let mut check_aerial = false;

        if !StatusModule::is_changing(fighter.module_accessor) {
            let mtrans = VarModule::get_int(fighter.battle_object, vars::common::status::FLOAT_MTRANS);

            if mtrans == 1 {
                if MotionModule::is_end(fighter.module_accessor) {
                    check_aerial = true;
                }
                if CancelModule::is_enable_cancel(fighter.module_accessor) {
                    if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
                        check_aerial = true;
                    }
                }
            }
            else if mtrans == 2 {
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
                    check_aerial = true;
                }
                if MotionModule::is_end(fighter.module_accessor) {
                    MotionModule::change_motion(
                        fighter.module_accessor,
                        Hash40::new("fall"),
                        0.0,
                        0.5, // purposefully run the animation slower because it might look nicer
                        false,
                        0.0,
                        false,
                        false
                    );
                }
            }
        }

        if check_aerial {
            float_check_aerial(fighter);
        }
    }
    0.into()
}

unsafe extern "C" fn float_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("reflet_catch"), false, true);
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_aura_light"), false, true);
    float_end_common(fighter)
}

pub fn install() {
    smashline::Agent::new("reflet")
        .status(Pre, statuses::reflet::FLOAT, float_pre)
        .status(Main, statuses::reflet::FLOAT, float_main)
        .status(End, statuses::reflet::FLOAT, float_end)
        .install();
}
