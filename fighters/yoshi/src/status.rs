use super::*;
use globals::*;
utils::import!(common::djc::attack_air_main_status);
// status script import
 
pub fn install() {
    install_status_scripts!(
        init_guard_damage,
        guard_damage,
        exit_guard_damage,
        guard_off,
        pre_jump_aerial,
        jump_aerial,
        init_attack_air,
        attack_air,
        exec_attack_air,
        exit_attack_air
    );
}

// FIGHTER_STATUS_KIND_GUARD_DAMAGE //

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_GUARD_DAMAGE, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_guard_damage(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_initStatus_Inner();
    original!(fighter)
}

unsafe extern "C" fn init_guard_damage_uniq(fighter: &mut L2CFighterCommon) {
    let shield_radius = WorkModule::get_param_float(fighter.module_accessor, hash40("shield_radius"), 0);
    let throw_scale: Vector3f = Vector3f { x: shield_radius, y: shield_radius, z: shield_radius };

    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("throw"), &throw_scale);

    fighter.clear_lua_stack();
    lua_args!(fighter, 0x2dc1210b69i64);
    app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    fighter.pop_lua_stack(0);
}

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_GUARD_DAMAGE, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
pub unsafe fn exit_guard_damage(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_exitStatus_common();
    original!(fighter)
}

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_GUARD_DAMAGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn guard_damage(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardDamage_common(L2CValue::Bool(false));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_GuardDamage_Main as *const () as _))
}

// FIGHTER_STATUS_KIND_GUARD_OFF //

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_GUARD_OFF, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn guard_off(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rate = fighter.status_GuardOff_Common().get_f32();
    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(0x97ab1c684), 0.0, rate, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(guard_off_main as *const () as _))
}

unsafe extern "C" fn guard_off_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.status_GuardOff_Main_common().get_bool() {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(
                    L2CValue::I32(*FIGHTER_STATUS_KIND_WAIT),
                    L2CValue::Bool(false)
                );
                return 1.into();
            }
            if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_JUMP_TRIGGER != 0 {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) {
                    fighter.change_status(
                        L2CValue::I32(*FIGHTER_STATUS_KIND_ESCAPE),
                        L2CValue::Bool(false)
                    );
                    return 1.into();
                }
            }
        }
    }
    return 0.into()
}

// FIGHTER_STATUS_KIND_JUMP_AERIAL

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_jump_aerial(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_0 = fighter.status_pre_JumpAerial_sub().get_i32() == 0;
    let should_end = is_0 as i32 & 1 == 0;
    if !should_end {
        StatusModule::init_settings(
            fighter.module_accessor,
            app::SituationKind(*SITUATION_KIND_AIR),
            *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION,
            *GROUND_CORRECT_KIND_AIR as u32,
            app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
            true,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
            0
        );
        FighterStatusModuleImpl::set_fighter_status_data(
            fighter.module_accessor,
            false,
            *FIGHTER_TREADED_KIND_ENABLE,
            true,
            false,
            true,
            0,
            *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
            0,
            0
        );
    }
    return (should_end as i32).into()
}

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn jump_aerial(fighter: &mut L2CFighterCommon) -> L2CValue {
    let aerial_damage_reaction = WorkModule::get_float(fighter.module_accessor, *FIGHTER_YOSHI_INSTANCE_WORK_ID_FLOAT_AERIAL_DAMAGE_REACTION);
    DamageModule::set_no_reaction_mode_status(fighter.module_accessor, DamageNoReactionMode{_address: *DAMAGE_NO_REACTION_MODE_REACTION_VALUE as u8}, aerial_damage_reaction, -1.0, -1);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_YOSHI_INSTANCE_WORK_ID_FLAG_JUMP_AERIAL_ARMOR);
    let turn_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_aerial"), hash40("turn_cont_value"));
    if fighter.global_table[STICK_X].get_f32() * -1.0 * PostureModule::lr(fighter.module_accessor) > turn_cont_value {
        let turn_count = WorkModule::get_param_int(fighter.module_accessor, hash40("jump_aerial"), hash40("turn_count"));
        WorkModule::set_int(fighter.module_accessor, turn_count, *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_AERIAL_TURN_COUNT);
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_YOSHI_INSTANCE_WORK_ID_INT_AERIAL_TURN_COUNT);
    }
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.status_JumpAerial();
    0.into()
}

// FIGHTER_STATUS_KIND_ATTACK_AIR //

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_init();
    0.into()
}

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::djc::attack_air_main_status(fighter)
}

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
pub unsafe fn exec_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec();
    0.into()
}

#[status_script(agent = "yoshi", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
pub unsafe fn exit_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exit();
    0.into()
}