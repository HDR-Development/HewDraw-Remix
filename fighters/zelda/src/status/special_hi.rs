use super::*;

unsafe extern "C" fn special_hi2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
        false,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_STATUS_ATTR_DISABLE_DISSOLVE_CURSOR | *FIGHTER_STATUS_ATTR_HIDE_NAME_CURSOR) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
    VisibilityModule::set_whole(fighter.module_accessor, false);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
    fighter.off_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
    GroundModule::set_passable_check(fighter.module_accessor, true);
    let cliff_check = fighter.get_int(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_CLIFF_CHECK);
    fighter.sub_fighter_cliff_check(cliff_check.into());
    fighter.set_int(0, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_hi2_main_loop as *const () as _))
}

unsafe extern "C" fn special_hi2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    let move_time = fighter.get_param_int("param_special_hi", "move_time"); //time spent moving
    if move_time <= fighter.get_int(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME) {
        fighter.change_status(FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3.into(), true.into())
    } else {
        if fighter.is_cat_flag(Cat1::SpecialAny) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK);
            fighter.change_status(FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3.into(), true.into())
        }
        if StatusModule::is_changing(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            } else {
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
        } else {
            weirdness(fighter);
            if StatusModule::is_situation_changed(fighter.module_accessor) {
                if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                    GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                } else {
                    GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                }
            }
        }
    }
    //subsatus
    if !StatusModule::is_changing(fighter.module_accessor) {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        let frame: i32 = fighter.get_int(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_FRAME);
        let move_xlu: i32 = fighter.get_param_int("param_special_hi", "move_xlu"); //time ignoring platforms
        let cliff_check_frame = fighter.get_param_int("param_special_hi", "move_cliff_check");
        if frame == move_xlu {
            GroundModule::set_passable_check(fighter.module_accessor, false);
        }
        if frame == cliff_check_frame {
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        }
        if frame < 2 {fighter.on_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CHECK_GROUND);}
    }
    0.into()
}

//excludes a lot of vanilla stuff that adds wall bounce, forced straight when angled down and landing... etc makes wallride opff unnecessary
unsafe extern "C" fn weirdness(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CHECK_GROUND) {
        return 0.into()
    }
    if GroundModule::is_attach_cliff(fighter.module_accessor) {
        return 0.into()
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            GroundModule::set_attach_ground(fighter.module_accessor, true);
        }
    }
    0.into()
}

unsafe extern "C" fn special_hi2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3 {
        //re-uses waveland window logic
        let init_speed_y = VarModule::get_float(fighter.battle_object, vars::common::status::TELEPORT_INITIAL_SPEED_Y); //teleport direction
        let pos = *PostureModule::pos(fighter.module_accessor); //top bone (bottom of ecb w/o shifting)
        let bot_snap = &Vector2f::new(pos.x, pos.y - 1.0);
        let top_snap = &Vector2f::new(pos.x, pos.y + 11.0);//around chest level
        let ground_pos_any = &mut Vector2f::zero();
        let ground_pos_stage = &mut Vector2f::zero();
        let is_touch_any = GroundModule::line_segment_check(fighter.module_accessor, top_snap, bot_snap, &Vector2f::zero(), ground_pos_any, true);
        let is_touch_stage = GroundModule::line_segment_check(fighter.module_accessor, top_snap, bot_snap, &Vector2f::zero(), ground_pos_stage, false);
        let can_snap = !(is_touch_any == 0 as *const *const u64 || (is_touch_stage != 0 as *const *const u64 && init_speed_y > 0.0)); //avoid snapping to stage from below
        if can_snap {
            PostureModule::set_pos(fighter.module_accessor, &Vector3f::new(pos.x, ground_pos_any.y + 0.1, pos.z));
            GroundModule::attach_ground(fighter.module_accessor, false);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
            PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
                let lr = PostureModule::lr(fighter.module_accessor);
                let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
                let speed = Vector2f{ x: lua_bind::KineticEnergy::get_speed_x(stop_energy), y: lua_bind::KineticEnergy::get_speed_y(stop_energy)};
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed.x.abs() * lr * 1.05, speed.y); //b-reverse telecancel reverses momentum on ground
                EFFECT_FOLLOW(fighter, Hash40::new("zelda_atk"), Hash40::new("top"), 5.5 * lr, 8.0, -2.0, 0, 0, 0, 1.65, true);
                LAST_EFFECT_SET_COLOR(fighter, 0.95, 3.0, 0.6);
                LAST_EFFECT_SET_ALPHA(fighter, 0.75);
                LAST_EFFECT_SET_RATE(fighter, 1.10); //spawn gr cancel eff frame 0
            }
        }
    }
    0.into()
}

unsafe extern "C" fn special_hi3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FS_SUCCEEDS_KEEP_ATTACK | *FS_SUCCEEDS_KEEP_EFFECT) as i32 //allow effect to spawn on ledge cancel
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    return 0.into();
}

unsafe extern "C" fn special_hi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        fighter.set_int(*SITUATION_KIND_GROUND, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            MotionModule::change_motion(fighter.module_accessor, "landing_fall_special".to_hash(), 0.0, 1.0, false, 0.0, false, false);
        } else {
            MotionModule::change_motion(fighter.module_accessor, "special_hi".to_hash(), 0.0, 1.0, false, 0.0, false, false);
        }
    } else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.set_int(*SITUATION_KIND_AIR, *FIGHTER_ZELDA_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        MotionModule::change_motion(fighter.module_accessor, "special_air_hi".to_hash(), 0.0, 1.0, false, 0.0, false, false);
        let x_max = fighter.get_param_float("param_special_hi", "fall_x_mull_value");
        WorkModule::set_float(fighter.module_accessor, x_max, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    }

    fighter.main_shift(special_hi3_main_loop)
}

unsafe extern "C" fn special_hi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let control = ControlModule::get_attack_air_kind(fighter.module_accessor);
    if control == *FIGHTER_COMMAND_ATTACK_AIR_KIND_NONE {
        FighterControlModuleImpl::update_attack_air_kind(fighter.module_accessor, true);
    } //?
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
          return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(*FIGHTER_STATUS_KIND_WAIT, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
            WorkModule::set_float(fighter.module_accessor, 18.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME); // halved lag if canceled
        } else {
            let landing_lag = fighter.get_param_float("param_special_hi", "landing_frame");
            WorkModule::set_float(fighter.module_accessor, landing_lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        }
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_HEAVY_ATTACK) {
                WorkModule::set_float(fighter.module_accessor, 18.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME); // halved lag if canceled
            } else {
                let landing_lag = fighter.get_param_float("param_special_hi", "landing_frame");
                WorkModule::set_float(fighter.module_accessor, landing_lag, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            }
            return 1.into();
        } else {
            //clear buffer
            ControlModule::reset_trigger(fighter.module_accessor);
            ControlModule::clear_command(fighter.module_accessor, true);
            ControlModule::reset_special_command(fighter.module_accessor, true);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        if fighter.is_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_DIVE) {
            fighter.sub_air_check_dive();
        }  //bypass manual fastfall
        if fighter.is_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CONTROL) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            fighter.off_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_CONTROL);
        } //bypass manual drift code
        if !fighter.global_table[IS_STOPPING].get_bool() && KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_MOTION_FALL { //only runs t he capping stuff before she can drift and fastfall
            if !fighter.is_flag(*FIGHTER_ZELDA_STATUS_SPECIAL_HI_FLAG_1) {
                let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
                let speed = Vector2f{x: lua_bind::KineticEnergy::get_speed_x(stop_energy), y: lua_bind::KineticEnergy::get_speed_y(stop_energy)};//pretty sure code subtracts 1/10th of y speed every frame before normal fall is enabled
                sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, speed.x, speed.y *0.9);
            } else {
                if !KineticModule::is_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) {
                    let stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP) as *mut app::KineticEnergy;
                    let speed_y = lua_bind::KineticEnergy::get_speed_y(stop_energy);
                    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, speed_y);
                    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                } //only enables gravity once instead of every frame
                let air_speed_x_stable: f32 = fighter.get_param_float("air_speed_x_stable", "");
                let fall_x_mul: f32 = fighter.get_param_float("param_special_hi", "fall_x_mull_value");
                let mut x_cap = air_speed_x_stable * fall_x_mul;
                sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, x_cap, 0.0);
            } //bypass once per frame set_speed to x_cap
        }
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, special_hi2_pre);
    agent.status(Main, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, special_hi2_main);
    agent.status(End, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, special_hi2_end);
    agent.status(Pre, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, special_hi3_pre);
    agent.status(Main, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3, special_hi3_main);
}