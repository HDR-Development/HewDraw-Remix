use super::*;
use globals::*;
// status script import

#[smashline::status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_GUARD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Guard()
}

#[smashline::status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_GUARD_ON, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn guard_on(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOn()
}

pub fn install() {
    install_status_scripts!(
        //jumpsquat,
        //exec_stop_jumpsquat,
        //waza_jumpsquat,
        pre_jump,
        guard,
        guard_on,
        //jump

        attack_air_lw_start_main,
        special_s_pre

    );
    smashline::install_agent_init_callbacks!(steve_init);
}

#[status_script(agent = "pickel", status = FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_air_lw_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let hasIron = WorkModule::get_int(fighter.module_accessor,*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_IRON) > 0;
    let forgeArticle = ArticleModule::is_exist(fighter.module_accessor,*FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE);
    let canForge = hasIron && !forgeArticle;

    if (canForge)
    {
        return original!(fighter);
    }
    else{
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_air_lw_fail"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_attack_air_common(false.into());
        return fighter.main_shift(attack_air_lw_main_status_loop);
    }
}
pub unsafe extern "C" fn attack_air_lw_main_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.status_AttackAir_Main_common().get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOPPING].get_bool() {
            fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
        }
        0.into()
    }
    else {
        1.into()
    }
}

#[status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue{

    let hasIron = WorkModule::get_int(fighter.module_accessor,*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_IRON) > 0;
    let forgeArticle = ArticleModule::is_exist(fighter.module_accessor,*FIGHTER_PICKEL_GENERATE_ARTICLE_TROLLEY);
    let canCart = hasIron && !forgeArticle;

    if canCart {
        VarModule::on_flag(fighter.battle_object, vars::pickel::instance::DISABLE_SPECIAL_S);
    }
    return original!(fighter);
}

// Prevents sideB from being used again if it has already been used once in the current airtime
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::pickel::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

// Re-enables the ability to use sideB when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let still_SideSpecial = fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_JUMP,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_RIDE,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_DRIVE
        ]
    );
    
    let damage_statuses = &[*FIGHTER_STATUS_KIND_DAMAGE,
                            *FIGHTER_STATUS_KIND_DAMAGE_AIR,
                            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
                            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
                            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
                            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
                            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
                            *FIGHTER_STATUS_KIND_DAMAGE_FALL];

    if (!fighter.is_situation(*SITUATION_KIND_AIR) && !still_SideSpecial) 
    || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING])
    || fighter.is_status_one_of(damage_statuses) {
        VarModule::off_flag(fighter.battle_object, vars::pickel::instance::DISABLE_SPECIAL_S);
    }

    return true.into();
}

#[smashline::fighter_init]
fn steve_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_PICKEL {
            fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}

// FIGHTER_STATUS_KIND_JUMP_SQUAT //

#[status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn jumpsquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    if pickel_js_status_check(fighter).get_bool() {
        fighter.sub_shift_status_main(L2CValue::Ptr(
            smash::lua2cpp::L2CFighterCommon_status_JumpSquat_Main as *const () as _,
        ))
    } else {
        fighter.status_JumpSquat()
    }
}

#[status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STOP)]
pub unsafe fn exec_stop_jumpsquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    if pickel_js_status_check(fighter).get_bool() {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new_raw(0xad160bda8),
            -1.0,
            1.0,
            0.0,
            true,
            true,
        );
    } else {
        fighter.sub_jump_squat_uniq_process_init();
    }
    return 0.into();
}

unsafe extern "C" fn pickel_js_status_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP_SQUAT
        && fighter.global_table[PREV_STATUS_KIND]
            != FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP_SQUAT
        && fighter.global_table[PREV_STATUS_KIND]
            != FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_SQUAT
    {
        return L2CValue::Bool(false);
    } else {
        return L2CValue::Bool(true);
    }
}

// acts as exec status
// do not use; causes bugs, simply here for research purposes
#[status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_LINE_WAZA_CUSTOMIZE)]
pub unsafe fn waza_jumpsquat(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let special_stick_x = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("common"),
        hash40("special_stick_x"),
    );
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let special_stick_y = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("common"),
        hash40("special_stick_y"),
    );

    fighter.uniq_process_JumpSquat_exec_status(); //this call is causing bugs for some reason
    if WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP,
    ) {
        return 0.into();
    }
    if WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI,
    ) {
        if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            return 0.into();
        } else {
            if special_stick_x > stick_x.abs() {
                return 0.into();
            } else {
                if special_stick_y > stick_y.abs() {
                    return 0.into();
                } else {
                    WorkModule::off_flag(
                        fighter.module_accessor,
                        *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI,
                    );
                }
            }
        }
    } else {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                if stick_x.abs() >= special_stick_x {
                    return 0.into();
                } else {
                    if stick_y.abs() >= special_stick_y {
                        return 0.into();
                    } else {
                        WorkModule::on_flag(
                            fighter.module_accessor,
                            *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI,
                        );
                    }
                }
            }
        }
    }
    return 0.into();
}

// FIGHTER_STATUS_KIND_JUMP //

#[status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter
        .status_pre_Jump_Common_param(L2CValue::Bool(true))
        .get_bool()
    {
        return 1.into();
    } else {
        if pickel_jump_status_check(fighter).get_bool() {
            fighter.status_pre_Jump_sub_param(
                L2CValue::I32(-1),
                L2CValue::I32(-1),
                L2CValue::I32(-1),
                L2CValue::I32(*KINETIC_TYPE_NONE),
                L2CValue::I32(
                    *FS_SUCCEEDS_KEEP_EFFECT
                        | *FS_SUCCEEDS_KEEP_SOUND
                        | *FS_SUCCEEDS_KEEP_TRANSITION
                        | *FS_SUCCEEDS_KEEP_CANCEL,
                ),
            );
        } else {
            fighter.status_pre_Jump_sub_param(
                L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLAG),
                L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_INT),
                L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLOAT),
                L2CValue::I32(*FIGHTER_KINETIC_TYPE_JUMP),
                L2CValue::I32(0),
            );
        }
        return 0.into();
    }
}

#[status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    if pickel_jump_status_check(fighter).get_bool() {
        if !sub_jump(fighter).get_bool() {
            if MotionModule::motion_kind(fighter.module_accessor) != 0x17f0bb63e4u64 {
                if MotionModule::motion_kind(fighter.module_accessor) != 0x12e6fa5eceu64 {
                    if MotionModule::motion_kind(fighter.module_accessor) != 0x176b2a21f2u64 {
                        MotionModule::change_motion_inherit_frame(
                            fighter.module_accessor,
                            Hash40::new_raw(0x62dd02058),
                            -1.0,
                            1.0,
                            0.0,
                            true,
                            true,
                        );
                    } else {
                        MotionModule::change_motion_inherit_frame(
                            fighter.module_accessor,
                            Hash40::new_raw(0xb38c9ab48),
                            -1.0,
                            1.0,
                            0.0,
                            true,
                            true,
                        );
                    }
                } else {
                    MotionModule::change_motion_inherit_frame(
                        fighter.module_accessor,
                        Hash40::new_raw(0x62abde441),
                        -1.0,
                        1.0,
                        0.0,
                        true,
                        true,
                    );
                }
            } else {
                MotionModule::change_motion_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new_raw(0xba358e95e),
                    -1.0,
                    1.0,
                    0.0,
                    true,
                    true,
                );
            }
            if !StopModule::is_stop(fighter.module_accessor) {
                fighter.sub_fall_common_uniq(L2CValue::Bool(false));
            }
            fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(
                smash::lua2cpp::L2CFighterCommon_sub_fall_common_uniq as *const () as _,
            ));
            fighter.sub_shift_status_main(L2CValue::Ptr(
                smash::lua2cpp::L2CFighterCommon_status_Jump_Main as *const () as _,
            ))
        } else {
            return 0.into();
        }
    } else {
        if WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI,
        ) {
            let pickel_int = WorkModule::get_param_int(
                fighter.module_accessor,
                hash40("param_private"),
                0xf9b69867e,
            );
            WorkModule::set_int(
                fighter.module_accessor,
                pickel_int,
                *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_JUMP_MINI_FRAME,
            );
        }
        fighter.status_Jump()
    }
}

unsafe extern "C" fn sub_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_cancel_status_kind = WorkModule::get_int(
        fighter.module_accessor,
        *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_ATTACK_CANCEL_STATUS_KIND,
    );
    if attack_cancel_status_kind != *FIGHTER_STATUS_KIND_NONE {
        fighter.change_status(
            L2CValue::I32(attack_cancel_status_kind),
            L2CValue::Bool(true),
        );
        WorkModule::set_int(
            fighter.module_accessor,
            *FIGHTER_STATUS_KIND_NONE,
            *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_ATTACK_CANCEL_STATUS_KIND,
        );
        return L2CValue::Bool(true);
    } else {
        return L2CValue::Bool(false);
    }
}

unsafe extern "C" fn pickel_jump_status_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP
        && fighter.global_table[PREV_STATUS_KIND] != FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP
        && fighter.global_table[PREV_STATUS_KIND] != FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP
    {
        return L2CValue::Bool(false);
    } else {
        return L2CValue::Bool(true);
    }
}
