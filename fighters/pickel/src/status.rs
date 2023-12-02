use super::*;
use globals::*;
// status script import

pub unsafe extern "C" fn pickel_attack_que(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if !FighterSpecializer_Pickel::is_status_kind_attack(prev_status) {
        fighter.sub_GetLightItemImm(L2CValue::Void());
        if StatusModule::status_kind_que_from_script(fighter.module_accessor) as i32 != *STATUS_KIND_NONE {
            return true.into();
        }
    }
    false.into()
}

pub unsafe extern "C" fn pickel_attack_catch_item(fighter: &mut L2CFighterCommon) {
    let catch_frame_param = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        hash40("item_catch_frame_attack_3")
    }
    else {
        hash40("item_air_catch_frame")
    };
    let catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), catch_frame_param);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_ATTACK_FRAME) <= catch_frame {
        fighter.sub_GetLightItemImm(L2CValue::Void());
    }
}

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
    if pickel_attack_que(fighter).get_bool() {
        return 0.into();
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_ATTACK_FRAME);
    let mut generate = false;
    if ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_ATTACK_AIR_LW_FORBID_FRAME) <= 0
        || !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE) {
            generate = true;
        }
    }
    if !FighterSpecializer_Pickel::check_material_attack_air_lw_generate(fighter.module_accessor) {
        generate = false;
    }
    let mot = if generate {
        KineticModule::clear_speed_attr(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        Hash40::new("attack_air_lw")
    }
    else {
        Hash40::new("attack_air_lw_fail")
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        mot,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    WorkModule::set_flag(fighter.module_accessor, generate, *FIGHTER_PICKEL_STATUS_ATTACK_FLAG_IS_GENERATE_FORGE);
    if generate {
        let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        WorkModule::set_int(fighter.module_accessor, jump_count, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_JUMP_COUNT_BACKUP);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_LAMDING_RECOVER);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_FORGE_LANDING);
        fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_lw_start_main_loop as *const () as _))
    }
    else {
        fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_lw_fail_main_status_loop as *const () as _))
    }
}

unsafe extern "C" fn attack_air_lw_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
            fighter.change_status(FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_LOOP.into(), false.into());
            return 0.into();
        }
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 0.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_ATTACK_FLAG_FORGE_GENERATE_ENABLE) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_ATTACK_FLAG_FORGE_GENERATE_ENABLE);
        if FighterSpecializer_Pickel::check_material_attack_air_lw_generate(fighter.module_accessor) {
            let attack_air_lw_interval = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("attack_air_lw_interval"));
            WorkModule::set_int(fighter.module_accessor, attack_air_lw_interval, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_ATTACK_AIR_LW_FORBID_FRAME);
            let anvil_iron_count = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), 0x188e0b0db2) + 2;
            FighterSpecializer_Pickel::sub_material_num(fighter.module_accessor, *FIGHTER_PICKEL_MATERIAL_KIND_IRON, anvil_iron_count);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE, false, -1);
            if LinkModule::is_link(fighter.module_accessor, *FIGHTER_PICKEL_LINK_NO_FORGE) {
                LinkModule::send_event_parents(fighter.module_accessor, *FIGHTER_PICKEL_LINK_NO_FORGE, Hash40::new_raw(0x11d608f91f));
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    pickel_attack_catch_item(fighter);
    attack_air_lw_dead_area(fighter);
    0.into()
}

unsafe extern "C" fn attack_air_lw_dead_area(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
        ModelModule::joint_global_position(
            fighter.module_accessor,
            Hash40::new("hip"),
            pos,
            true
        );
        let check_dead = GroundUtility::check_dead_area(pos);
        if check_dead != *GROUND_DEAD_AREA_CHECK_RESULT_OUTSIDE_UP {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        }
    }
}

pub unsafe extern "C" fn attack_air_lw_fail_main_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    let trolleyArticle = ArticleModule::is_exist(fighter.module_accessor,*FIGHTER_PICKEL_GENERATE_ARTICLE_TROLLEY);
    let canCart = hasIron && !trolleyArticle;

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

    if (!fighter.is_situation(*SITUATION_KIND_AIR) && !still_SideSpecial) 
    || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
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
