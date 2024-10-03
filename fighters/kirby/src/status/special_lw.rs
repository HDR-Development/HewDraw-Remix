use super::*;

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    special_lw_off_mtrans(fighter);
    fighter.on_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
    fighter.off_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
    fighter.set_int64(hash40("special_lw") as i64, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_GROUND_MOT);
    fighter.set_int64(hash40("special_air_lw") as i64, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_AIR_MOT);
    ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, false, -1);
    ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_STONE, false, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    }
    else {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    }
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    fighter.main_shift(special_lw_main_loop)
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !special_lw_check_mtrans_1(fighter) {
        if fighter.is_situation(*SITUATION_KIND_GROUND) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            fighter.on_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_ESCAPE);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
            let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
            let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
            sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * 0.5, 0.0);
            sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x_mul * 0.5);
            sv_kinetic_energy!(controller_set_accel_x_add, fighter, air_accel_x_add * 0.5);
            fighter.on_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        }
        special_lw_change_motion(fighter);
    }
    if special_lw_check_mtrans_2(fighter) {
        special_lw_off_mtrans(fighter);
    }
    let effect_onoff = fighter.is_flag(*FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_EFFECT_ONOFF);
    let prev_effect_onoff = fighter.is_flag(*FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_PREV_EFFECT_ONOFF);
    if effect_onoff {         
        fighter.on_flag(*FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_PREV_EFFECT_ONOFF);
    }
    else {
        fighter.off_flag(*FIGHTER_KIRBY_STATUS_WORK_ID_FLAG_STONE_PREV_EFFECT_ONOFF);
    }
    if effect_onoff != prev_effect_onoff {
        if !effect_onoff {
            let handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("kirby_stone_s"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, false, 0, 0, 0, 0, 0, false, false);
            fighter.set_int(handle as i32, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_STONE_EFFECT_HANDLE);
        }
        else {
            let handle = fighter.get_int(*FIGHTER_KIRBY_STATUS_WORK_ID_INT_STONE_EFFECT_HANDLE);
            if handle != 0 {
                fighter.clear_lua_stack();
                fighter.push_lua_stack(&mut L2CValue::I32(*MA_MSC_EFFECT_REMOVE));
                fighter.push_lua_stack(&mut L2CValue::I32(handle));
                sv_module_access::effect(fighter.lua_state_agent);
                fighter.pop_lua_stack(1);
                fighter.set_int(0, *FIGHTER_KIRBY_STATUS_WORK_ID_INT_STONE_EFFECT_HANDLE);
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_KIRBY_STATUS_KIND_STONE_STONE.into(), false.into());
        return 1.into();
    }

    return 0.into();
}

//FUN_71001a6740
unsafe extern "C" fn special_lw_off_mtrans(fighter: &mut L2CFighterCommon) {
    fighter.off_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR);
    fighter.off_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND);
    fighter.off_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END);
}

//FUN_7100229ec0
unsafe extern "C" fn special_lw_check_mtrans_1(fighter: &mut L2CFighterCommon) -> bool {
    if !fighter.is_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR) {
        if !fighter.is_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND) {
            if !fighter.is_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END) {
                return false;
            }
        }
    }

    return true;
}

//FUN_7100229610
unsafe extern "C" fn special_lw_change_motion(fighter: &mut L2CFighterCommon) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        let motion = fighter.get_int64(*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_GROUND_MOT);
        if fighter.is_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion.as_hash40(), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(fighter.module_accessor, motion.as_hash40(), 0.0, 1.0, false, 0.0, false, false);
        fighter.on_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
    }
    else {
        let motion = fighter.get_int64(*FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_AIR_MOT);
        if fighter.is_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, motion.as_hash40(), -1.0, 1.0, 0.0, false, false);
        }
        MotionModule::change_motion(fighter.module_accessor, motion.as_hash40(), 0.0, 1.0, false, 0.0, false, false);
        fighter.on_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MOT_FRAME_INHERIT);
    }
}

//FUN_710022a090
unsafe extern "C" fn special_lw_check_mtrans_2(fighter: &mut L2CFighterCommon) -> bool {
    if (fighter.is_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR) && fighter.is_situation(*SITUATION_KIND_GROUND))
    || !fighter.is_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR) {
        if fighter.is_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                return true;
            }
        }
        if fighter.is_flag(*FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END) {
            return MotionModule::is_end(fighter.module_accessor);
        }
        else {
            return false;
        }
    }

    return true;
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
}