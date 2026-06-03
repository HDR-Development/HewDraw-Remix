use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_S

pub unsafe extern "C" fn special_s_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::plizardon::instance::DISABLE_SPECIAL_S);
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        let speed_y = fighter.get_speed_y(*FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if speed_y < 0.0 {
            sv_kinetic_energy!(
                set_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                speed_y * 0.2
            );
        }
    }
    0.into()
}

unsafe extern "C" fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}

// FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_END

unsafe extern "C" fn special_s_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_s_rush_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_PLIZARDON_INSTANCE_WORK_ID_FLAG_SPECIAL_S_IS_HIT_SLOPE);
    fighter.off_flag(*FIGHTER_PLIZARDON_STATUS_SPECIAL_S_FLAG_IS_STATUS_CHANGE_BLOWN);
    fighter.set_int(0, *FIGHTER_PLIZARDON_INSTANCE_WORK_ID_INT_EXPLOSION_DELAY_FRAME);
    fighter.set_int(0, *FIGHTER_PLIZARDON_STATUS_SPECIAL_S_WORK_INT_SPECIAL_S_ROTATE_COUNT);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    let facing = fighter.lr();
    let speed_x = fighter.get_param_float("param_special_s", "speed_x_") * facing;
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_GROUND, speed_x, 0.0, 0.0, 0.0, 0.0);
    let brake_x = fighter.get_param_float("param_special_s", "brake_x_") * facing;
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, -brake_x, 0.0);  // the vanilla script does this, don't look at me
    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, -1.0, -1.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    else {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    let rush_damage = fighter.get_param_float("param_special_s", "rush_damage_");
    DamageModule::add_damage(fighter.module_accessor, rush_damage, 0);
    let super_armor_damage = fighter.get_param_float("param_special_s", "super_armor_damage_");
    DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER as u8}, -1.0, super_armor_damage, -1);
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("special_s"));
        let la0 = (end_frame + 1.0) / 2.0; // l90
        let rush_rotate_frame = fighter.get_param_float("param_special_s", "rush_rotate_frame_");
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, la0 / rush_rotate_frame, false, 0.0, false, false);
    }
    else {
        let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("special_air_s"));
        let la0 = (end_frame + 1.0) / 2.0; // l90
        let rush_rotate_frame = fighter.get_param_float("param_special_s", "rush_rotate_frame_");
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s"), 0.0, la0 / rush_rotate_frame, false, 0.0, false, false);
    }

    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if (fighter.lr() > 0.0 && GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32))
    || (fighter.lr() < 0.0 && GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32))
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_PARRY) {
        fighter.change_status(FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_BLOWN.into(), true.into());
        return 0.into();
    }
    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) { Hash40::new("special_s") } else { Hash40::new("special_air_s") };
    let end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, motion);
    let la0 = (end_frame + 1.0) / 2.0; // l90
    let rush_rotate_frame = fighter.get_param_float("param_special_s", "rush_rotate_frame_");
    let rate = la0 / rush_rotate_frame;
    if MotionModule::frame(fighter.module_accessor) >= end_frame - 1.0 {
        fighter.inc_int(*FIGHTER_PLIZARDON_STATUS_SPECIAL_S_WORK_INT_SPECIAL_S_ROTATE_COUNT);
        let rotate_count = fighter.get_int(*FIGHTER_PLIZARDON_STATUS_SPECIAL_S_WORK_INT_SPECIAL_S_ROTATE_COUNT);
        let rush_rotate_count = fighter.get_param_float("param_special_s", "rush_rotate_count_");
        if rush_rotate_count - 0.1 >= rotate_count as f32 {
            fighter.change_motion_inherit_frame_by_situation("special_s", "special_air_s", -1.0, 1.0, 0.0, false, false);
            MotionModule::set_frame(fighter.module_accessor, 0.0, true);
            MotionModule::set_rate(fighter.module_accessor, rate);
            return 0.into();
        }
    }
    else {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
                MotionModule::set_rate(fighter.module_accessor, rate);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
                MotionModule::set_rate(fighter.module_accessor, rate);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
        }
        return 0.into();
    }
    fighter.change_status(FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_END.into(), false.into());
    return 1.into();
}

unsafe extern "C" fn special_s_rush_check_attack(fighter: &mut L2CFighterCommon, param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("bust"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_init);
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, special_s_pre);

    agent.status(Main, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_RUSH, special_s_rush_main);
    agent.status(CheckAttack, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_RUSH, special_s_rush_check_attack);
    
    agent.status(Pre, *FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_S_END, special_s_end_pre);
}
