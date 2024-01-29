use super::*;
utils::import!(common::djc::attack_air_main_status);

// FIGHTER_STATUS_KIND_ATTACK_AIR //

#[status_script(agent = "trail", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    original!(fighter)
}

#[status_script(agent = "trail", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);

    /*
    fighter.sub_attack_air_kind();
    if motion_kind != smash::hash40("jump_aerial_f") as u64 {
        if motion_kind == smash::hash40("jump_aerial_b") as u64 {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
                    MotionModule::set_weight(fighter.module_accessor, 1.0, true);
                }
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            }
            fighter.sub_attack_air_uniq_process_init();
            return L2CValue::I32(0);
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        fighter.sub_attack_air_uniq_process_init();
        return L2CValue::I32(0);
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
                MotionModule::set_weight(fighter.module_accessor, 1.0, true);
            }
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        }
        fighter.sub_attack_air_uniq_process_init();
        return L2CValue::I32(0);
    }
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    fighter.sub_attack_air_uniq_process_init();
    0.into()
    */

    fighter.sub_attack_air_kind();
    if motion_kind == smash::hash40("jump_aerial_f") || motion_kind == smash::hash40("jump_aerial_b") {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION)
        && (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) 
        || MotionModule::frame_2nd(fighter.module_accessor) < 2.0) {
            MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
            MotionModule::set_weight(fighter.module_accessor, 1.0, true);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
        } else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        }
    } else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    }
    let _ = fighter.sub_attack_air_uniq_process_init();
    0.into()
}

#[status_script(agent = "trail", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::djc::attack_air_main_status(fighter)
}

// FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N //

unsafe extern "C" fn attack_air_n_change_motion(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut const_arg: LuaConst;
    let mut int_arg: i64;
    let combo_count = ComboModule::count(fighter.module_accessor);
    if combo_count != 1 {
        if combo_count != 2 {
            int_arg = hash40("attack_air_n3") as i64;
            const_arg = FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_02;
            MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, false);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_n3"), 0.0, 1.0, false, 0.0, false, false);
            MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, true);
        }
        else {
            int_arg = hash40("attack_air_n2") as i64;
            const_arg = FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_01;
            MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, false);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_n2"), 0.0, 1.0, false, 0.0, false, false);
            MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, true);
        }
    }
    else {
        int_arg = hash40("attack_air_n") as i64;
        const_arg = FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_n"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, 0x2b94de0d96i64, FIGHTER_LOG_ACTION_CATEGORY_ATTACK, const_arg);
    app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    fighter.pop_lua_stack(0);

    WorkModule::set_int64(fighter.module_accessor, int_arg, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    0.into()
}

unsafe extern "C" fn sub_attack_air_n(fighter: &mut L2CFighterCommon) {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);

    ComboModule::set(fighter.module_accessor, *FIGHTER_COMBO_KIND_AIR_N_COMBINATION);
    attack_air_n_change_motion(fighter);

    fighter.sub_attack_air_kind();
    if motion_kind != hash40("jump_aerial_f") as u64 {
        if motion_kind != hash40("jump_aerial_b") as u64 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            fighter.sub_attack_air_uniq_process_init();
            return;
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION)
    && (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) 
    || MotionModule::frame_2nd(fighter.module_accessor) < 2.0) {
        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
        MotionModule::set_weight(fighter.module_accessor, 1.0, true);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
    } else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    }
    fighter.sub_attack_air_uniq_process_init();
    return;
}

#[status_script(agent = "trail", status = FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_attack_air_n(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_attack_air_n(fighter);
    // Momentum transfer stuff
    let ratio = VarModule::get_float(fighter.object(), vars::common::instance::JUMP_SPEED_RATIO);
    let jump_speed_x_max = WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0) * ratio;

    let mut l2c_agent = smash::lib::L2CAgent::new(fighter.lua_state_agent);
    let new_speed = VarModule::get_float(fighter.object(), vars::common::instance::CURRENT_MOMENTUM).clamp(-jump_speed_x_max, jump_speed_x_max);

    if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_JUMP {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, new_speed);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }
    0.into()
}

// FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F //

unsafe extern "C" fn attack_air_f_change_motion(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut const_arg: LuaConst;
    let mut int_arg: i64;
    let combo_count = ComboModule::count(fighter.module_accessor);
    if combo_count != 1 {
        if combo_count != 2 {
            int_arg = hash40("attack_air_f3") as i64;
            const_arg = FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F3;
            MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, false);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_n3"), 0.0, 1.0, false, 0.0, false, false);
            MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, true);
        }
        else {
            int_arg = hash40("attack_air_f2") as i64;
            const_arg = FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F2;
            MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, false);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_n2"), 0.0, 1.0, false, 0.0, false, false);
            MotionModule::enable_remove_2nd_change_motion(fighter.module_accessor, true);
        }
    }
    else {
        int_arg = hash40("attack_air_f") as i64;
        const_arg = FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_n"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, 0x2b94de0d96i64, FIGHTER_LOG_ACTION_CATEGORY_ATTACK, const_arg);
    app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    fighter.pop_lua_stack(0);

    WorkModule::set_int64(fighter.module_accessor, int_arg, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    0.into()
}

unsafe extern "C" fn sub_attack_air_f(fighter: &mut L2CFighterCommon) {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);

    ComboModule::set(fighter.module_accessor, *FIGHTER_COMBO_KIND_AIR_F_COMBINATION);
    attack_air_f_change_motion(fighter);

    fighter.sub_attack_air_kind();
    if motion_kind != hash40("jump_aerial_f") as u64 {
        if motion_kind != hash40("jump_aerial_b") as u64 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            fighter.sub_attack_air_uniq_process_init();
            return;
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION)
    && (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) 
    || MotionModule::frame_2nd(fighter.module_accessor) < 2.0) {
        MotionModule::add_motion_2nd(fighter.module_accessor, Hash40::new_raw(motion_kind), frame, 1.0, false, 1.0);
        MotionModule::set_weight(fighter.module_accessor, 1.0, true);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND);
    } else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
    }
    fighter.sub_attack_air_uniq_process_init();
    return;
}

#[status_script(agent = "trail", status = FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_attack_air_f(fighter: &mut L2CFighterCommon) -> L2CValue {
    sub_attack_air_f(fighter);
    // Momentum transfer stuff
    let ratio = VarModule::get_float(fighter.object(), vars::common::instance::JUMP_SPEED_RATIO);
    let jump_speed_x_max = WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0) * ratio;

    let mut l2c_agent = smash::lib::L2CAgent::new(fighter.lua_state_agent);
    let new_speed = VarModule::get_float(fighter.object(), vars::common::instance::CURRENT_MOMENTUM).clamp(-jump_speed_x_max, jump_speed_x_max);

    if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_JUMP {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, new_speed);
        app::sv_kinetic_energy::set_speed(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        attack_air_pre,
        init_attack_air,
        attack_air,
        init_attack_air_n,
        init_attack_air_f
    );
}
