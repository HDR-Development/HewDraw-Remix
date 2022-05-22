use super::*;
use globals::*;
utils::import!(common::djc::attack_air_main_status);
// status script import
 
pub fn install() {
    install_status_scripts!(
        attack_air,
        special_hi_attack
    );
}

// FIGHTER_STATUS_KIND_ATTACK_AIR //

#[status_script(agent = "lucas", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::djc::attack_air_main_status(fighter)
}

// FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK //

#[status_script(agent = "lucas", status = FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_WALL_BRAKE);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        sub_special_hi_attack(fighter);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sub_special_hi_attack as *const () as _));
    fighter.main_shift(special_hi_attack_main)
}

unsafe extern "C" fn sub_special_hi_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    // this does...something?
    if !fighter.is_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_ATTACK_FALL_START) {
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    else {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_ALL) {
        AttackModule::clear_inflict_kind_status(fighter.module_accessor);
    }
    0.into()
}

unsafe extern "C" fn special_hi_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.is_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_ATTACK_END) {
        fighter.off_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_ATTACK_END);
        let attack_end_brake = fighter.get_param_float("param_special_hi", "attack_end_brake");
        let special_hi_angle = fighter.get_float(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_WORK_FLOAT_DIR);
        let brake_x = attack_end_brake * special_hi_angle.cos() * PostureModule::lr(fighter.module_accessor);
        let brake_y = attack_end_brake * special_hi_angle.sin();

        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, brake_x, brake_y);
        app::sv_kinetic_energy::set_brake(fighter.lua_state_agent);
    }
    /*
    if fighter.is_prev_situation(*SITUATION_KIND_AIR) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            let attack_ground_speed_xmul = fighter.get_param_float("param_special_hi", "attack_ground_speed_xmul");

            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, attack_ground_speed_xmul, attack_ground_speed_xmul, attack_ground_speed_xmul);
            app::sv_kinetic_energy::mul_speed(fighter.lua_state_agent);
        }
    }
    */
    if !MotionModule::is_end(fighter.module_accessor) {
        if fighter.is_flag(*FIGHTER_LUCAS_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE) && fighter.is_situation(*SITUATION_KIND_GROUND) {
            let attack_ground_end_speed_xmul = fighter.get_param_float("param_special_hi", "attack_ground_end_speed_xmul");

            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, attack_ground_end_speed_xmul, attack_ground_end_speed_xmul, attack_ground_end_speed_xmul);
            app::sv_kinetic_energy::mul_speed(fighter.lua_state_agent);

            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
        // [insert stubbed redirection/bonk function here]
        // LOL good riddance fucker
        0.into()
    }
    else {
        fighter.set_int64(0x19ea19ce46, *FIGHTER_LUCAS_INSTANCE_WORK_ID_INT_SPECIAL_AIR_END_MOTION);
        fighter.change_status(FIGHTER_LUCAS_STATUS_KIND_SPECIAL_HI_END.into(), false.into());
        1.into()
    }
}