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
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
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
            if fighter.motion_frame() <= 1.0 {
                KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(0.0, 0.0, 0.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            if fighter.motion_frame() > 1.0 && fighter.motion_frame() <= 3.0 {
                KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(0.0, 0.0, 0.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            if fighter.motion_frame() > 3.0 {
                smash::app::lua_bind::FighterKineticEnergyGravity::set_accel(fighter_gravity, -0.02666667);
            }
        }
        if fighter.motion_frame() > 3.0 {
            if ((fighter.is_status (*FIGHTER_STATUS_KIND_SPECIAL_LW) && fighter.motion_frame() > 3.0)  // Allows for jump cancel on frame 5 in game
            || fighter.is_status_one_of(&[
                *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT,
                *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP,
                *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END]))
            && !fighter.is_in_hitlag()
            {
                fighter.check_jump_cancel();
            }
    
        }   
    }
}

// Falco Phantasm Shortens
unsafe fn phantasm_shorten(boma: &mut BattleObjectModuleAccessor, id: usize, motion_kind: u64, frame: f32) {
    /*
    if [hash40("special_s"), hash40("special_air_s")].contains(&motion_kind) {
        if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) {
            let motion_vec = Vector3f{x: 0.1, y: 1.0, z: 1.0};
            WorkModule::on_flag(boma, *FIGHTER_FALCO_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END);
            KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        }
    }
    */

    if motion_kind == hash40("special_s") || motion_kind == hash40("special_air_s") {
        if frame <= 1.0 {
            VarModule::off_flag(boma.object(), vars::falco::status::ILLUSION_SHORTEN);
            VarModule::off_flag(boma.object(), vars::falco::status::ILLUSION_SHORTENED);
        }
        if VarModule::is_flag(boma.object(), vars::falco::status::ILLUSION_SHORTEN) &&  !VarModule::is_flag(boma.object(), vars::falco::status::ILLUSION_SHORTENED) {
            KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            VarModule::on_flag(boma.object(), vars::falco::status::ILLUSION_SHORTENED);
        }

        if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) &&  !VarModule::is_flag(boma.object(), vars::falco::status::ILLUSION_SHORTENED) {
            VarModule::on_flag(boma.object(), vars::falco::status::ILLUSION_SHORTEN);
            WorkModule::on_flag(boma, *FIGHTER_FALCO_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END);
        }
    }
}

unsafe fn firebird_startup_ledgegrab(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        // allows ledgegrab during Firebird startup
        fighter.sub_transition_group_check_air_cliff();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {

    laser_ff_land_cancel(boma, status_kind, situation_kind, cat[1], stick_y);
    shine_jc_turnaround(fighter);
    phantasm_shorten(boma, id, motion_kind, frame);
    firebird_startup_ledgegrab(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_FALCO )]
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