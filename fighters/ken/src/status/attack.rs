use super::*;

// FIGHTER_STATUS_KIND_ATTACK

unsafe extern "C" fn attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if WorkModule::is_flag(
        fighter.module_accessor,
        *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_NEAR_OPPONENT,
    ) {
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("attack_11_near_s") as i64,
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
            hash40("attack_11_s") as i64,
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
    if !fighter.is_flag(*FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
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

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK, attack_main);
}
