// status imports
use super::*;
use globals::*;

macro_rules! interrupt {
    () => { return L2CValue::I32(1); };
    ($fighter:ident, $status:expr, $repeat:expr) => {{ $fighter.change_status($status.into(), $repeat.into()); interrupt!(); }}
}

#[common_status_script(status = FIGHTER_STATUS_KIND_TURN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN,
    symbol = "_ZN7lua2cpp16L2CFighterCommon11status_TurnEv")]
unsafe fn status_turn(fighter: &mut L2CFighterCommon) -> L2CValue {
    status_pre_turncommon(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(status_turn_main as *const () as _))
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon21status_pre_TurnCommonEv")]
unsafe extern "C" fn status_pre_turncommon(fighter: &mut L2CFighterCommon) {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ITEM);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_TURN_ATTACK_S4_REV_PAD);

    let frame = if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_LATE_PIVOT) {
            6.0
        }
        else {
            0.0
        };
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("turn"), frame, 1.0, false, 0.0, false, false);
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon16status_Turn_MainEv")]
unsafe extern "C" fn status_turn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let should_end = if fighter.global_table[0x35].get_bool() {
        let custom_routine: *const extern "C" fn(&mut L2CFighterCommon) -> L2CValue = fighter.global_table[0x35].get_ptr() as _;
        if !custom_routine.is_null() {
            let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(custom_routine);
            callable(fighter).get_bool()
        } else {
            false
        }
    }
    else { false };
    if !should_end {
        let dash_stick_x: f32 = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dash_stick_x"));
        let stick_x = fighter.global_table[STICK_X].get_f32();
        let turn_work_lr: f32 = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_TURN_WORK_FLOAT_LR);

        if !status_turncommon(fighter).get_bool() {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
            && MotionModule::frame(fighter.module_accessor) >= 1.0 {
                if (VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN)
                && MotionModule::frame(fighter.module_accessor) == 1.0 
                && stick_x * -1.0 * turn_work_lr >= dash_stick_x)
                || fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0
                {
                    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
                    interrupt!(fighter, FIGHTER_STATUS_KIND_DASH, true);
                }
                if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0 {
                    interrupt!(fighter, FIGHTER_STATUS_KIND_TURN_DASH, true);
                }
            }
            return L2CValue::I32(0);
        }
    }
    L2CValue::I32(1)
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon17status_TurnCommonEv")]
unsafe extern "C" fn status_turncommon(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
        if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            if MotionModule::is_end(fighter.module_accessor) {
                interrupt!(fighter, FIGHTER_STATUS_KIND_WAIT, false);
            }
        }
        fighter.check_turn_attack_s4_pad_rev();
        if !fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
            if !fighter.sub_transition_group_check_ground_item().get_bool() {
                if !fighter.sub_transition_group_check_ground_catch().get_bool() {
                    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_TURN_FLAG_NO_TURN_TO_ESCAPE) {
                            if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_TURN_TO_ESCAPE_F == 0 {
                                if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_TURN_TO_ESCAPE_B == 0 {
                                    // label
                                    if !fighter.sub_transition_group_check_ground_escape().get_bool() {
                                        if !fighter.sub_transition_group_check_ground_guard().get_bool() {
                                            if !fighter.sub_transition_group_check_ground_special().get_bool() {
                                                if !fighter.sub_transition_group_check_ground_attack().get_bool() {
                                                    if !fighter.sub_transition_group_check_ground_jump().get_bool() {
                                                        if !fighter.sub_transition_group_check_ground(L2CValue::Bool(false)).get_bool() {
                                                            return L2CValue::Bool(false);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                else {
                                    interrupt!(fighter, FIGHTER_STATUS_KIND_ESCAPE_B, true);
                                }
                            }
                            else {
                                interrupt!(fighter, FIGHTER_STATUS_KIND_ESCAPE_F, true);
                            }
                            return L2CValue::Bool(true);
                        }
                    }
                    // label
                    if !fighter.sub_transition_group_check_ground_escape().get_bool() {
                        if !fighter.sub_transition_group_check_ground_guard().get_bool() {
                            if !fighter.sub_transition_group_check_ground_special().get_bool() {
                                if !fighter.sub_transition_group_check_ground_attack().get_bool() {
                                    if !fighter.sub_transition_group_check_ground_jump().get_bool() {
                                        if !fighter.sub_transition_group_check_ground(L2CValue::Bool(false)).get_bool() {
                                            return L2CValue::Bool(false);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

    }
    else {
        interrupt!(fighter, FIGHTER_STATUS_KIND_FALL, false);
    }
    return L2CValue::Bool(true)
}

#[common_status_script(status = FIGHTER_STATUS_KIND_TURN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END,
    symbol = "_ZN7lua2cpp16L2CFighterCommon15status_end_TurnEv")]
unsafe fn status_end_turn(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_LATE_PIVOT);
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_DASH {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::CAN_PERFECT_PIVOT);
    }
    if StatusModule::status_kind_next(fighter.module_accessor) != *FIGHTER_STATUS_KIND_TURN_DASH {
        VarModule::off_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
    }
    fighter.sub_exit_Turn();
    0.into()
}

#[hook(module = "common", symbol = "_ZN7lua2cpp16L2CFighterCommon13sub_exit_TurnEv")]
unsafe extern "C" fn sub_exit_Turn(fighter: &mut L2CFighterCommon) {
    if StatusModule::status_kind_next(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ESCAPE_F
    || StatusModule::status_kind_next(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ESCAPE_B {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }
}

pub fn install() {
    install_hooks!(
        status_pre_turncommon,
        status_turn_main,
        status_turncommon,
        sub_exit_Turn
    );

    install_status_scripts!(
        status_turn,
        status_end_turn
    );
}