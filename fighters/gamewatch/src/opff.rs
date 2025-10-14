// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn ff_chef_land_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_N) {
        if fighter.status_frame() == 18 {
            let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
            let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
            sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x_mul * 0.5);
            sv_kinetic_energy!(controller_set_accel_x_add, fighter, air_accel_x_add * 0.5);
        }

        if boma.is_situation(*SITUATION_KIND_AIR) {
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_REQUEST_DIVE_EFFECT) {
                WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_REQUEST_DIVE_EFFECT);
            }
            if !WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
                if KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0
                && ControlModule::get_stick_y(boma) < WorkModule::get_param_float(boma, hash40("common"), hash40("attack_lw4_stick_y")) {
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                    WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_REQUEST_DIVE_EFFECT);
                }
            }
        }

        let landing_lag = 6.0;
        boma.check_land_cancel(Some(landing_lag));

        if StatusModule::is_changing(boma) {
            let nspec_halt = Vector3f{x: 0.9, y: 1.0, z: 1.0};
            KineticModule::mul_speed(boma, &nspec_halt, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            if boma.is_situation(*SITUATION_KIND_AIR) {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

unsafe fn parachute(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, vars::gamewatch::instance::SPECIAL_HI_ENABLE_PARACHUTE) {
        if fighter.is_cat_flag(Cat1::SpecialAny) {
            if (fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR) && !CancelModule::is_enable_cancel(fighter.module_accessor))
            || fighter.is_status_one_of(&[
                *FIGHTER_STATUS_KIND_ESCAPE_AIR,
                *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE,
                *FIGHTER_STATUS_KIND_DAMAGE,
                *FIGHTER_STATUS_KIND_DAMAGE_AIR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                *FIGHTER_STATUS_KIND_DAMAGE_FALL]) {
                return;
            }
            fighter.change_status(statuses::gamewatch::SPECIAL_HI_OPEN.into(), true.into());
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_WAIT,
        *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_CATCH,
        *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_SHOOT,
        *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_REFLECT,
        *FIGHTER_GAMEWATCH_STATUS_KIND_SPECIAL_LW_WAIT_START
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

// down throw mirror
unsafe fn dthrow_reverse(boma: & mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("throw_lw")) {
        ModelModule::set_joint_rotate(boma, Hash40::new("rot"), &Vector3f{x: 0.0, y: 180.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    ff_chef_land_cancel(fighter, boma);
    parachute(fighter);
    dthrow_reverse(boma);
    fastfall_specials(fighter);
}

pub extern "C" fn gamewatch_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		gamewatch_frame(fighter)
    }
}

pub unsafe fn gamewatch_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, gamewatch_frame_wrapper);
}