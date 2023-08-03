// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn laser_ff_land_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request(boma, *FIGHTER_STATUS_KIND_LANDING, true);
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat2::FallJump)
                && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Falco Shine Jump Cancels and Turnaround
unsafe fn shine_jc_turnaround(fighter: &mut L2CFighterCommon) {
    let frame = fighter.status_frame();
    if fighter.is_status (*FIGHTER_STATUS_KIND_SPECIAL_LW) {
        let facing = PostureModule::lr(fighter.module_accessor);
        let stick_x = fighter.stick_x();
        if stick_x * facing < 0.0 {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) <= 1 {
            GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        // Momentum handling
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let fighter_gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut FighterKineticEnergyGravity;
        if fighter.is_status (*SITUATION_KIND_AIR) {
            if frame <= 3 {
                KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(0.0, 0.0, 0.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            if frame > 3 {
                smash::app::lua_bind::FighterKineticEnergyGravity::set_accel(fighter_gravity, -0.02666667);
            }
        }
        if ((fighter.is_status (*FIGHTER_STATUS_KIND_SPECIAL_LW) && frame > 2)  // Allows for jump cancel on frame 4 in game
        || fighter.is_status_one_of(&[
            *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT,
            *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP,
            *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END]))
        && !fighter.is_in_hitlag()
        {
            fighter.check_jump_cancel(false) || fighter.check_attack_hi4_cancel(false);
        }
    }
}

unsafe fn firebird_startup_ledgegrab(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        // allows ledgegrab during Firebird startup
        fighter.sub_transition_group_check_air_cliff();
    }
}

unsafe fn aim_throw_lasers(boma: &mut BattleObjectModuleAccessor) {
    let frame = boma.motion_frame();
    let lr = PostureModule::lr(boma);

    if boma.is_motion(Hash40::new("throw_hi"))
    && 13.0 <= frame
    && frame < 23.0 {
        let rot = Vector3f::new(0.0, boma.stick_x() * lr * -20.0, 0.0);
        boma.set_joint_rotate("clavicler", rot);
    }
    else if boma.is_motion(Hash40::new("throw_b"))
    && 9.0 <= frame
    && frame < 21.0 {
        let rot = Vector3f::new(0.0, boma.stick_y() * -20.0, 0.0);
        boma.set_joint_rotate("shoulderr", rot);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    laser_ff_land_cancel(boma, status_kind, situation_kind, cat[1], stick_y);
    shine_jc_turnaround(fighter);
    firebird_startup_ledgegrab(fighter);
    aim_throw_lasers(boma);
}

#[utils::macros::opff(FIGHTER_KIND_FALCO)]
pub fn falco_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		falco_frame(fighter)
    }
}

pub unsafe fn falco_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}
