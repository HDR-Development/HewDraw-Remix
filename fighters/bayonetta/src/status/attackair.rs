use super::*;
use globals::*;

 
pub fn install() {
    install_status_scripts!(
        attack_air_pre,
        attack_air_f_main
    );
}

// FIGHTER_STATUS_KIND_ATTACK_AIR //

#[status_script(agent = "bayonetta", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    original!(fighter)
}

// FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F //

#[status_script(agent = "bayonetta", status = FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_air_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air();
    fair_motion(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_attack_air_f_loop as *const () as _))
}

unsafe extern "C" fn bayonetta_attack_air_f_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO) 
    && (ControlModule::get_attack_air_kind(fighter.module_accessor) == *FIGHTER_COMMAND_ATTACK_AIR_KIND_F || fighter.is_cat_flag(Cat1::Catch))
    && fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP) == 0 {
        fair_motion(fighter);
    }
    if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::FAIR_STATE) > 0
    && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && VarModule::get_int(fighter.battle_object, vars::common::instance::LAST_ATTACK_HITBOX_ID) < 6 {
        let control_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) as *mut smash::app::KineticEnergy;
        smash::app::lua_bind::KineticEnergy::mul_speed(control_energy, &Vector3f::new(0.7, 1.0, 1.0)); 
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.4);
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.048);
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
    lua_args!(fighter, MA_MSC_CMD_CANCEL_UNABLE_CANCEL);
    smash::app::sv_module_access::cancel(fighter.lua_state_agent);
    AttackModule::clear_inflict_kind_status(fighter.module_accessor);
    fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
    fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
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
        VisibilityModule::set_int64(fighter.module_accessor, hash40("0x88fc722e4") as i64, hash40("0x110ed68a57") as i64);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    false.into()
}