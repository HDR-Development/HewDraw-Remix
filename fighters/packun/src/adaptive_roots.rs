
use super::*;
use globals::*;

pub unsafe extern "C" fn adaptive_roots_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_NONE as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        0,
        0,
        0,
        0
    );

    0.into()
}

/// main status loop for metaquick summon
unsafe extern "C" fn adaptive_roots_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // exit if the animation is not done yet
    if !MotionModule::is_end(fighter.module_accessor) {
        if MotionModule::frame(fighter.module_accessor) >= 30.0 && !CancelModule::is_enable_cancel(fighter.module_accessor) {
            CancelModule::enable_cancel(fighter.module_accessor);
        } 
        if fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
        return 0.into();
    }

    // if the animation is over, transition to wait
    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());

    1.into()
}

/// main status for metaquick summon
pub unsafe extern "C" fn adaptive_roots_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = Hash40::new("final");

    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.5, false, 0.0, false, false);

    fighter.main_shift(adaptive_roots_main_loop)
}

/// end status for metaquick summon
pub unsafe extern "C" fn adaptive_roots_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

/// handles starting metaquick
unsafe fn handle_start_adaptive_roots(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_GUARD,
            *FIGHTER_STATUS_KIND_GUARD_ON]){
            fighter.change_to_custom_status(statuses::packun::ADAPTIVE_ROOTS, false, false);
        }
    }
}

pub unsafe fn run(fighter: &mut smash::lua2cpp::L2CFighterCommon) {

    if lua_bind::FighterManager::is_result_mode(utils::singletons::FighterManager()) {
        VarModule::set_int(fighter.object(), vars::packun::instance::CURRENT_STANCE, 0);
    }

    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && ControlModule::get_stick_y(fighter.module_accessor) < 0.3 {
        if (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
        || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L))
        && VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) != 0 {
            handle_start_adaptive_roots(fighter);
            LANDING_EFFECT(fighter, Hash40::new("sys_grass"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            VarModule::set_int(fighter.object(), vars::packun::instance::CURRENT_STANCE, 0);
        }
        else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
        && VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) != 1 {
            handle_start_adaptive_roots(fighter);
            LANDING_EFFECT(fighter, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            VarModule::set_int(fighter.object(), vars::packun::instance::CURRENT_STANCE, 1);
        }
        else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
        && VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) != 2 {
            handle_start_adaptive_roots(fighter);
            LANDING_EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            VarModule::set_int(fighter.object(), vars::packun::instance::CURRENT_STANCE, 2);
        }
    }

    stance_head(fighter);
    check_apply_speeds(fighter);

    if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 1 {
        check_reset(fighter);
        VarModule::set_float(fighter.object(), vars::common::instance::JUMP_SPEED_MAX_MUL, 0.8);
        if fighter.is_status(*FIGHTER_STATUS_KIND_DASH) {
            let motion_vec = Vector3f {
                x: -0.25 * PostureModule::lr(fighter.boma()) * (1.0 - (MotionModule::frame(fighter.boma()) / MotionModule::end_frame(fighter.boma()))),
                y: 0.0, 
                z: 0.0
            };
            KineticModule::add_speed_outside(fighter.boma(), *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
    }
    else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
        check_reset(fighter);
        VarModule::set_float(fighter.object(), vars::common::instance::JUMP_SPEED_MAX_MUL, 0.6);
        if fighter.is_status(*FIGHTER_STATUS_KIND_DASH) {
            let motion_vec = Vector3f {
                x: -0.4 * PostureModule::lr(fighter.boma()) * (1.0 - (MotionModule::frame(fighter.boma()) / MotionModule::end_frame(fighter.boma()))),
                y: 0.0, 
                z: 0.0
            };
            KineticModule::add_speed_outside(fighter.boma(), *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
    }
}

unsafe fn stance_head(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    // Enable meshes for stances
    // HeadA is the normal head
	// HeadB is the poison head
	// HeadS is the spike head
    if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 0 {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), false);
    }
    else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 1  {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), false);
    }
    else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2  {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), false);
    }
}

/// handle speed application
unsafe fn check_apply_speeds(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    
    // handle speed application once
    if VarModule::is_flag(fighter.object(), vars::packun::instance::STANCE_NEED_SET_SPEEDS) {
        if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 0 {
            apply_status_speed_mul(fighter, 1.0);
        } else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 1 {
            apply_status_speed_mul(fighter, 0.7);
        } else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
            apply_status_speed_mul(fighter, 0.5);
        }
        VarModule::off_flag(fighter.object(), vars::packun::instance::STANCE_NEED_SET_SPEEDS);
    }

    if VarModule::get_int(fighter.object(), vars::packun::instance::STANCE_STATUS) != VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) {
        //println!("Status is changing!");
        VarModule::on_flag(fighter.object(), vars::packun::instance::STANCE_NEED_SET_SPEEDS);
        VarModule::set_int(fighter.object(), vars::packun::instance::STANCE_STATUS, VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE));
        //println!("new meta quick status: {}", VarModule::get_int(fighter.object(), vars::metaknight::instance::META_QUICK_STATUS));
    }
}

/// checks if meta quick should have its state reset due to death or match end
unsafe fn check_reset(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    // we dont want meta quick *or* the ready effect to persist during these states
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]) {
            VarModule::set_int(fighter.object(), vars::packun::instance::CURRENT_STANCE, 0);
    }
}

/// applies the given multiplier to various speed stats of the given fighter. 
/// This should only be called once per status, or you will get some multiplicative effects
unsafe fn apply_status_speed_mul(fighter: &mut smash::lua2cpp::L2CFighterCommon, mul: f32) {
    // set the X motion speed multiplier (where movement is baked into an anim)
    lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter.get_motion_energy(), mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_mul( fighter.get_controller_energy(), mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_add( fighter.get_controller_energy(), mul);

    // set the X speed max multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_speed_max(fighter.get_controller_energy(), mul);
}

pub fn install() {
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_packun"),
        statuses::packun::ADAPTIVE_ROOTS,
        StatusInfo::new()
            .with_pre(adaptive_roots_pre)
            .with_main(adaptive_roots_main)
            .with_end(adaptive_roots_end)
    );
}