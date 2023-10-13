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
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_BAYONETTA_SPECIAL_S);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0x976c3b29b), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(bayonetta_special_s_main_loop as *const () as _))
}

#[status_script(agent = "bayonetta", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_BAYONETTA_STATUS_WORK_ID_SPECIAL_S_FLAG_HIT_CANCEL_OK);
    original!(fighter)
}

unsafe extern "C" fn bayonetta_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::set_correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.set_int(*FIGHTER_BAYONETTA_SHOOTING_STEP_WAIT_END, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SHOOTING_STEP);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.5);
        if fighter.motion_frame() >= 44.0 {fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into()); }
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
    0.into()
}