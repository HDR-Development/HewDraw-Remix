use super::*;
use globals::*;

 
pub fn install() {
    install_status_scripts!(
        special_s_main,
        special_s_end
    );
}

// FIGHTER_STATUS_KIND_SPECIAL_S //

#[status_script(agent = "bayonetta", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0x976c3b29b), 0.0, 1.0, false, 0.0, false, false);
    StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_special_s_main_loop as *const () as _))
}

#[status_script(agent = "bayonetta", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT_CANCEL_OK);
    original!(fighter)
}

unsafe extern "C" fn bayonetta_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND { //gr checks
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
        && VarModule::get_int(fighter.battle_object, vars::common::instance::LAST_ATTACK_HITBOX_ID) < 6 {
            if fighter.is_cat_flag(Cat1::SpecialAny | Cat1::AttackN)
            && fighter.global_table[CURRENT_FRAME].get_i32() >= 20
            && fighter.global_table[CURRENT_FRAME].get_i32() <= 35 {
                fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_S_HOLD_END.into(), false.into());
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into()); }
        if fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_WALL_CHECK) {
            let mut touch_wall = false;
            if PostureModule::lr(fighter.module_accessor) > 0.0 {
                touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT as u32);
            } else {
                touch_wall = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT as u32);
            }
            if touch_wall {fighter.change_status(FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END.into(), false.into()); }
        }
    } else { //slide-off
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            GroundModule::set_correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.45);
            sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.66);
        }
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_run_smoke"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("bayonetta_heelslide_trace"), false, false);
        if fighter.motion_frame() >= 45.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    0.into()
}