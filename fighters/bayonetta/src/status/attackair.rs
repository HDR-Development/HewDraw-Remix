use super::*;
use globals::*;

 

// FIGHTER_STATUS_KIND_ATTACK_AIR //

unsafe extern "C" fn attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter)
}

// FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F //

unsafe extern "C" fn attack_air_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air();
    fair_motion(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_attack_air_f_loop as *const () as _))
}

unsafe extern "C" fn bayonetta_attack_air_f_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO) 
    && (ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F || fighter.is_cat_flag(Cat1::Catch))
    && !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION) {
        fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F.into(), false.into());
    }
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) 
    && !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION) {
        let control_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) as *mut smash::app::KineticEnergy;
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.048);
        if fighter.is_motion(Hash40::new("attack_air_f")) {
            let y_speed = fighter.get_param_float("param_private", "attack_air_f_hit_speed_y");
            smash::app::lua_bind::KineticEnergy::mul_speed(control_energy, &Vector3f::new(0.65, 1.0, 1.0)); 
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, y_speed);
        } else if fighter.is_motion(Hash40::new("attack_air_f2")) {
            let y_speed = fighter.get_param_float("param_private", "attack_air_f2_hit_speed_y");
            smash::app::lua_bind::KineticEnergy::mul_speed(control_energy, &Vector3f::new(0.65, 1.0, 1.0)); 
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, y_speed);
        }
    }
    if !fighter.status_AttackAir_Main_common().get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOPPING].get_bool() {
            fighter.sub_attack_air_uniq_process_exec_fix_pos();
        }
        return 0.into()
    }
    1.into()
}

unsafe extern "C" fn fair_motion(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fair = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::FAIR_STATE);
    if fair == 1 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_f2"), 0.0, 1.0, false, 0.0, false, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_ATTACK, FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F2);
    } else if fair == 2 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_f3"), 0.0, 1.0, false, 0.0, false, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_ATTACK, FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F3);
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_f"), 0.0, 1.0, false, 0.0, false, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_ATTACK, FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F);
    }
    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("gun_hand") as i64, hash40("gun_hand_show_all") as i64);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    false.into()
}

pub fn install() {
    smashline::Agent::new("bayonetta")
        .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, attack_air_pre)
        .status(
            Main,
            *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F,
            attack_air_f_main,
        )
        .install();
}
