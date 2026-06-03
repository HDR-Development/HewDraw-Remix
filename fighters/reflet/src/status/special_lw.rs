use super::*;

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //set vars/flags/momentum
    fighter.set_int64(hash40("special_lw_start") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_GROUND);
    fighter.set_int64(hash40("special_air_lw_start") as i64, *FIGHTER_REFLET_STATUS_COMMON_INT_MOTION_KIND_AIR);
    fighter.set_int(*FIGHTER_KINETIC_TYPE_GROUND_STOP, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_GROUND);
    fighter.set_int(*FIGHTER_KINETIC_TYPE_AIR_STOP, *FIGHTER_REFLET_STATUS_COMMON_INT_KINETIC_AIR);
    fighter.set_int(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_GROUND);
    fighter.set_int(*GROUND_CORRECT_KIND_AIR, *FIGHTER_REFLET_STATUS_COMMON_INT_CORRECT_AIR);
    fighter.set_int(0x50000000,*FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_INT_OBJECT_ID);
    AttackModule::set_overlap_hit(fighter.module_accessor, true);//what is this
    mot_handler(fighter);//mot/kin
    air_start_stall(fighter);
    //update book
    let mut reflet_fighter = app::Fighter{battle_object: *(fighter.battle_object)};
    FighterSpecializer_Reflet::change_hud_kind(&mut reflet_fighter, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA);
    fighter.set_int(*FIGHTER_REFLET_MAGIC_KIND_RIZAIA, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
    if !CHECK_MAGIC(fighter) {//if empty before starting
        fighter.on_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE);
    } else {
        FighterSpecializer_Reflet::change_grimoire(fighter.module_accessor as *mut FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA);
        fighter.off_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE);
        fighter.off_flag(*FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_FLAG_MISS);
        //resource
        fighter.dec_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT);
        if fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT) <= 0 {//if using final bar 
            FighterSpecializer_Reflet::set_flag_to_table(fighter.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
        }
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, MA_MSC_CMD_GRAB_IS_GRAB, 0);
    sv_module_access::grab(fighter.lua_state_agent);
    if fighter.pop_lua_stack(1).get_bool() {
        if !FighterSpecializer_Reflet::check_special_lw_pos(fighter.module_accessor as *mut FighterModuleAccessor) {
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_CMD_GRAB_CLEAR, 0);
            sv_module_access::grab(fighter.lua_state_agent);
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_CMD_GRAB_CLEAR, 1);
            sv_module_access::grab(fighter.lua_state_agent);
        }
    }
    if fighter.get_int(*FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_WORK_INT_CATCH_STATUS) == *FIGHTER_REFLET_STATUS_SPECIAL_LW_CATCH_STATUS_CATCH_START {
        fighter.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_CAPTURE.into(), false.into())
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_END.into(), false.into())
    }
    mot_handler(fighter);//mot/kin
    false.into()
}

pub unsafe extern "C" fn air_start_stall(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        let prev_status_0 = StatusModule::prev_status_kind(fighter.module_accessor, 0);
        let prev_status_1 = StatusModule::prev_status_kind(fighter.module_accessor, 1);
        let start_x_speed_mul = fighter.get_param_float("param_special_lw", "start_x_speed_mul");
        let start_y_speed_mul = fighter.get_param_float("param_special_lw", "start_y_speed_mul");//start mul and stable mul
        let air_accel_y = fighter.get_param_float("param_special_lw", "special_lw_air_catch_fall_accel");
        let air_speed_y_stable = fighter.get_param_float("air_speed_y_stable", "0");
        KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(start_x_speed_mul, 1.0, 1.0), *FIGHTER_KINETIC_ENERGY_ID_STOP);//x stall
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);//air accel stall
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable * start_y_speed_mul);
        if prev_status_1 == statuses::reflet::FLOAT && prev_status_0 == *FIGHTER_STATUS_KIND_FALL && fighter.global_table[PREV_STATUS_FRAME].get_i32() < 10 {//unfloat grab
            //VarModule::on_flag(fighter.battle_object, vars::common::instance::STALL_PREVENTION);//if used out of float do not cut y speed
        } else {
            //hardcode set-speed stall
            if !VarModule::is_flag(fighter.battle_object, vars::common::instance::STALL_PREVENTION) {
                sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.0);
            } else {//if not first use, vanilla stall
                KineticModule::mul_speed(fighter.module_accessor, &Vector3f::new(1.0, start_y_speed_mul, 1.0), *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
        VarModule::on_flag(fighter.battle_object, vars::common::instance::STALL_PREVENTION);
    }
    0.into()
}

unsafe extern "C" fn special_lw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::reflet::instance::SPECIAL_HI_ENABLE_FREEFALL);
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_CAPTURE {
        if CatchModule::is_catch(fighter.module_accessor) {
            CatchModule::catch_cut(fighter.module_accessor, false, false);
        }
        EFFECT_DETACH_KIND(fighter, Hash40::new("reflet_rizaia"), -1);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_flash"), true, true);
    }
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_END {
        EFFECT_DETACH_KIND(fighter, Hash40::new("reflet_rizaia"), -1);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_flash"), true, true);
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_end);
}
