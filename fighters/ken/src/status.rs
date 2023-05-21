use super::*;
use globals::*;
// status script import

utils::import_noreturn!(common::shoto_status::{
    fgc_end_dashback,
    ryu_idkwhatthisis2
});

extern "Rust" {
    // from common::shoto_status
    fn ryu_kara_cancel(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn ryu_attack_main_uniq_chk(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn fgc_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn ryu_attack_main_uniq_chk4(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue;
    fn ryu_final_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue;
    fn ryu_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue;
    fn fgc_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue;
}

pub fn install() {
    install_status_scripts!(
        pre_turndash,
        main_dashback,
        end_dashback,
        main_attack,
        wait_pre,
        //wait_main,
        landing_main,
        guard,
        init_special_s,
        init_special_s_command
    );
    smashline::install_agent_init_callbacks!(ken_init);
}

// Prevents sideB from being used again if it has already been used once in the current airtime
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

// Re-enables the ability to use sideB when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD]) {
        VarModule::off_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S);
    }
    true.into()
}

#[smashline::fighter_init]
fn ken_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_KEN {
            fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}

// FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND //

#[status_script(agent = "ken", status = FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_special_s_command(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S);
    }
    original!(fighter)
}

// FIGHTER_STATUS_KIND_SPECIAL_S //

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_special_s(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::on_flag(fighter.battle_object, vars::shotos::instance::DISABLE_SPECIAL_S);
    }
    original!(fighter)
}

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_GUARD_OFF, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rate = fighter.status_GuardOff_Common().get_f32();
    if VarModule::is_flag(
        fighter.object(),
        vars::common::instance::IS_PARRY_FOR_GUARD_OFF,
    ) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("just_shield_off"),
            0.0,
            0.0,
            false,
            0.0,
            false,
            false,
        );
        app::FighterUtil::flash_eye_info(fighter.module_accessor);
        if !WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL,
        ) {
            ModelModule::enable_gold_eye(fighter.module_accessor);
            WorkModule::on_flag(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_DAMAGE_WORK_FLAG_GOLD_EYE,
            );
        }
        EffectModule::req_on_joint(
            fighter.module_accessor,
            Hash40::new_raw(0xff4f9200f),
            Hash40::new("throw"),
            &Vector3f::zero(),
            &Vector3f::zero(),
            0.5,
            &Vector3f::zero(),
            &Vector3f::zero(),
            false,
            *EFFECT_SUB_ATTRIBUTE_NONE as u32,
            *EFFECT_FLIP_NONE,
            1,
        );
        EffectModule::req_common(fighter.module_accessor, Hash40::new("just_shield"), 0.0);
        // let shield_se = app::FighterUtil::get_just_shield_se(fighter.global_table[0x2].get_i32());
        SoundModule::play_se(
            fighter.module_accessor,
            smash::phx::Hash40::new("se_item_deathscythe_swing_m"),
            true,
            false,
            false,
            false,
            app::enSEType(0),
        );
    } else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(0x97ab1c684),
            0.0,
            rate,
            false,
            0.0,
            false,
            false,
        );
    }
    fighter.main_shift(guard_main)
}

unsafe extern "C" fn guard_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOff_Main()
}
// FIGHTER_STATUS_KIND_TURN_DASH //

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_TURN_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_turndash(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = WorkModule::get_float(
        fighter.module_accessor,
        *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1,
    );
    if lr != 0.0 {
        if PostureModule::lr(fighter.module_accessor) == lr {
            if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_STATUS_KIND_TURN {
                StatusModule::set_status_kind_interrupt(
                    fighter.module_accessor,
                    *FIGHTER_RYU_STATUS_KIND_DASH_BACK,
                );
                return L2CValue::I32(1);
            }
        }
    }
    VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_SMASH_TURN);
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN);
    return 1.into();
}

// FIGHTER_RYU_STATUS_KIND_DASH_BACK //

#[status_script(agent = "ken", status = FIGHTER_RYU_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_dashback(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

#[status_script(agent = "ken", status = FIGHTER_RYU_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn end_dashback(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::shoto_status::fgc_end_dashback(fighter);
    original!(fighter)
}

// FIGHTER_STATUS_KIND_ATTACK //

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn main_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_NEAR_OPPONENT,
    ) {
        WorkModule::set_int64(
            fighter.module_accessor,
            0x10556e6036,
            *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION,
        );
        WorkModule::set_int(
            fighter.module_accessor,
            *FIGHTER_LOG_ATTACK_KIND_ATTACK_NEAR,
            *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND,
        );
    } else {
        WorkModule::set_int64(
            fighter.module_accessor,
            0xb4f4e6f8f,
            *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION,
        );
        WorkModule::set_int(
            fighter.module_accessor,
            *FIGHTER_LOG_ATTACK_KIND_ATTACK11,
            *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND,
        );
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_attack_main_uniq_chk(fighter);
    }
    fighter.global_table[SUB_STATUS3]
        .assign(&L2CValue::Ptr(ryu_attack_main_uniq_chk as *const () as _));
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_attack_main_uniq_chk4(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS]
        .assign(&L2CValue::Ptr(ryu_attack_main_uniq_chk4 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ken_attack_main_loop as *const () as _))
}

unsafe extern "C" fn ken_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL,
        ) {
            if AttackModule::is_infliction_status(
                fighter.module_accessor,
                *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT,
            ) {
                if ryu_final_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                    return 1.into();
                }
            }
        }
        if WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL,
        ) {
            if AttackModule::is_infliction_status(
                fighter.module_accessor,
                *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT,
            ) {
                if ryu_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                    return 1.into();
                }
            }
        }
    }
    if ComboModule::count(fighter.module_accessor) == 1 {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
            let attack_start_cancel_frame = WorkModule::get_param_float(
                fighter.module_accessor,
                hash40("param_private"),
                hash40("attack_start_cancel_frame"),
            );
            if current_frame < attack_start_cancel_frame {
                if ryu_kara_cancel(fighter).get_bool() {
                    return 1.into();
                }
            }
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter
            .sub_wait_ground_check_common(false.into())
            .get_bool()
        {
            return 1.into();
        }
    }
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    if [
        hash40("attack_11_w"),
        hash40("attack_11_s"),
        hash40("attack_11_near_s"),
    ]
    .contains(&mot)
    {
        if WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL,
        ) {
            if WorkModule::is_flag(
                fighter.module_accessor,
                *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER,
            ) {
                if ControlModule::check_button_off(
                    fighter.module_accessor,
                    *CONTROL_PAD_BUTTON_ATTACK,
                ) {
                    let stick_y = fighter.global_table[STICK_Y].get_f32();
                    let attack_hi3_stick_y = WorkModule::get_param_float(
                        fighter.module_accessor,
                        hash40("common"),
                        hash40("attack_hi3_stick_y"),
                    );
                    let cont;
                    if !(stick_y < attack_hi3_stick_y) {
                        cont = false;
                    } else {
                        let attack_lw3_stick_y = WorkModule::get_param_float(
                            fighter.module_accessor,
                            hash40("common"),
                            hash40("attack_lw3_stick_y"),
                        );
                        if !(attack_lw3_stick_y < stick_y) {
                            cont = false;
                        } else {
                            let stick_x = fighter.global_table[STICK_X].get_f32();
                            let attack_s3_stick_x = WorkModule::get_param_float(
                                fighter.module_accessor,
                                hash40("common"),
                                hash40("attack_s3_stick_x"),
                            );
                            cont = stick_x < attack_s3_stick_x;
                        }
                    }
                    if cont {
                        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
                        return 1.into();
                    }
                }
            }
        }
        if WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL,
        ) {
            let button_on_frame = WorkModule::get_int(
                fighter.module_accessor,
                *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME,
            );
            let attack_11_s_button_on_frame = WorkModule::get_param_float(
                fighter.module_accessor,
                hash40("param_private"),
                hash40("attack_11_s_button_on_frame"),
            );
            if attack_11_s_button_on_frame <= button_on_frame as f32 {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
                return 1.into();
            }
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    // if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME) {
    //     if !StopModule::is_stop(fighter.module_accessor) {
    //         if fighter.sub_check_button_jump().get_bool() {

    //         }
    //     }
    // }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
        if !MotionModule::is_end(fighter.module_accessor) {
            common::shoto_status::ryu_idkwhatthisis2(fighter);
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    } else {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
    }
    0.into()
}

// FIGHTER_STATUS_KIND_WAIT //

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_Wait()
}

// vanilla script
#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_wait_common();
    fighter.sub_wait_motion_mtrans();
    fighter.sub_shift_status_main(L2CValue::Ptr(fgc_wait_main_loop as *const () as _))
}

pub unsafe extern "C" fn fgc_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_Wait_Main().get_bool() {
        return 0.into();
    }
    let lr = WorkModule::get_float(
        fighter.module_accessor,
        *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1,
    );
    if lr != 0.0 && PostureModule::lr(fighter.module_accessor) != lr {
        let stick_x_corrected = fighter.global_table[STICK_X].get_f32()
            * (PostureModule::lr(fighter.module_accessor) * -1.0);
        let stick_y = fighter.global_table[STICK_Y].get_f32();
        let walk_stick_x = WorkModule::get_param_float(
            fighter.module_accessor,
            hash40("common"),
            hash40("walk_stick_x"),
        );
        let squat_stick_y = WorkModule::get_param_float(
            fighter.module_accessor,
            hash40("common"),
            hash40("squat_stick_y"),
        );

        if WorkModule::is_enable_transition_term(
            fighter.module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK,
        ) {
            if walk_stick_x <= stick_x_corrected {
                if squat_stick_y < stick_y {
                    fighter.change_status(FIGHTER_RYU_STATUS_KIND_WALK_BACK.into(), true.into());
                    return 0.into();
                }
            }
        }
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_TURN_AUTO.into(), false.into());
        return 0.into();
    }
    0.into()
}

// FIGHTER_STATUS_KIND_LANDING //

#[status_script(agent = "ken", status = FIGHTER_STATUS_KIND_LANDING, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_landing_main(fighter)
}
