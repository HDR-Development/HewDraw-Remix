use super::*;

// Hardcoding these vars as hdr.xml doesn't play nice with Pocketed articles
const PULSE_LIFE: i32 = 40;
const PULSE_EFFECT_COUNT: i32 = 2;
const PULSE_STABLE_SPEED: f32 = 0.76;
const PULSE_BRAKE_FRAMES: i32 = 10;

unsafe extern "C" fn straight_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_RESET,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    
    return 0.into();
}

unsafe extern "C" fn straight_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    VarModule::set_int(weapon.battle_object, vars::miigunner_supermissile::status::PULSE_TIMER, PULSE_LIFE);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("straight"), 0.0, 1.0, false, 0.0, false, false);

    weapon.fastshift(L2CValue::Ptr(straight_main_loop as *const () as _))
}

unsafe extern "C" fn straight_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if VarModule::is_flag(weapon.battle_object, vars::miigunner_supermissile::instance::ENABLE_PULSE) {
        if VarModule::countdown_int(weapon.battle_object, vars::miigunner_supermissile::status::PULSE_TIMER, 0) {
            VarModule::on_flag(weapon.battle_object, vars::miigunner_supermissile::instance::PULSE_DETONATE);
            weapon.change_status(WEAPON_MIIGUNNER_SUPERMISSILE_STATUS_KIND_S_BURST.into(), false.into());
            return 0.into();
        }
        else {
            play_pulse_effect(weapon);
        }
    }
    else {
        if !StatusModule::is_changing(weapon.module_accessor) {
            if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_REFLECTOR | *COLLISION_KIND_MASK_PARRY) {
                let owner_boma = weapon.get_owner_boma();
                if owner_boma.kind() == *FIGHTER_KIND_MIIGUNNER {
                    // Reset missile id so Gunner can fire another when the first is reflected
                    VarModule::set_int(owner_boma.object(), vars::miigunner::instance::SPECIAL_S3_MISSILE_OBJECT_ID, -1);
                }
                AttackModule::clear_inflict_kind_status(weapon.module_accessor);
            }
            let weapon_team = TeamModule::team_no(weapon.module_accessor);
            let owner_team = TeamModule::team_no(weapon.get_owner_boma());
            if weapon_team == owner_team {
                if weapon.status_frame() >= 20
                && weapon.get_owner_boma().is_cat_flag(Cat1::SpecialS) {
                    VarModule::on_flag(weapon.battle_object, vars::miigunner_supermissile::instance::ENABLE_PULSE);
                    VarModule::set_int(weapon.battle_object, vars::miigunner_supermissile::status::PULSE_TIMER, PULSE_LIFE);
                    let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
                    let facing = PostureModule::lr(weapon.module_accessor);
        
                    // The missile will reach its intended pulsing speed of 0.4 within 10 frames based on its current speed
                    let accel_lerp = (PULSE_STABLE_SPEED - speed_x.abs()) / PULSE_BRAKE_FRAMES as f32;
                    //println!("accel_lerp: {}", accel_lerp);
                    sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, PULSE_STABLE_SPEED, 0.0);
                    if accel_lerp > 0.0 {
                        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_lerp * facing, 0.0);
                        sv_kinetic_energy!(set_brake, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
                        sv_kinetic_energy!(set_limit_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, PULSE_STABLE_SPEED, 0.0);
                    }
                    else {
                        sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
                        sv_kinetic_energy!(set_brake, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -accel_lerp, 0.0);
                    }
                    play_pulse_effect(weapon);
                }
            }
            weapon.dec_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        }
    }
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32)
    || weapon.get_int(*WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0 {
        weapon.change_status(WEAPON_MIIGUNNER_SUPERMISSILE_STATUS_KIND_S_BURST.into(), false.into());
    }
    let s_model_angle_speed = weapon.get_param_float("param_supermissile", "s_model_angle_speed");
    let rot_angle = weapon.get_float(*WEAPON_MIIGUNNER_SUPERMISSILE_INSTANCE_WORK_ID_FLOAT_MODEL_ROT_ANGLE);
    let rot_vec = Vector3f::new(0.0, 0.0, rot_angle);
    ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new("rot"), &rot_vec, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_NONE as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    weapon.set_float(rot_angle + s_model_angle_speed, *WEAPON_MIIGUNNER_SUPERMISSILE_INSTANCE_WORK_ID_FLOAT_MODEL_ROT_ANGLE);

    return 0.into();
}

unsafe extern "C" fn play_pulse_effect(weapon: &mut L2CWeaponCommon) {
    let pulse_timer = VarModule::get_int(weapon.battle_object, vars::miigunner_supermissile::status::PULSE_TIMER);
    let pulse_interval = PULSE_LIFE / PULSE_EFFECT_COUNT;
    if pulse_timer % pulse_interval == 0 {
        let h = (pulse_interval / pulse_timer) + 1;
        let facing = weapon.lr();
        EFFECT_FOLLOW(weapon, Hash40::new("sys_sp_flash"), Hash40::new("top"), -1.0 * facing, 0, 3, 0, 0, 0, 0.3 + 0.125 * h as f32, false);
        LAST_EFFECT_SET_COLOR(weapon, 0.5 + 3.75 * h as f32, 2.0, 1.0);
        LAST_EFFECT_SET_RATE(weapon, 0.6);
        let handle = SoundModule::play_se_no3d(weapon.module_accessor, Hash40::new("se_common_spirits_floor_elec_spark2"), true, true);
        SoundModule::set_se_vol(weapon.module_accessor, handle as i32, 1.25 + 0.25 * h as f32, 0);
        let handle2 = SoundModule::play_se_no3d(weapon.module_accessor, Hash40::new("se_common_spirits_floor_elec_spark1"), true, true);
        SoundModule::set_se_vol(weapon.module_accessor, handle2 as i32, 1.5 + 0.25 * h as f32, 0);
    }
}

unsafe extern "C" fn straight_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::detach_all(weapon.module_accessor, 5);
    return 0.into();
}

unsafe extern "C" fn sburst_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *WEAPON_MIIGUNNER_SUPERMISSILE_STATUS_KIND_STRAIGHT, straight_pre);
    agent.status(Main, *WEAPON_MIIGUNNER_SUPERMISSILE_STATUS_KIND_STRAIGHT, straight_main);
    agent.status(End, *WEAPON_MIIGUNNER_SUPERMISSILE_STATUS_KIND_STRAIGHT, straight_end);

    agent.status(End, *WEAPON_MIIGUNNER_SUPERMISSILE_STATUS_KIND_S_BURST, sburst_end);
}