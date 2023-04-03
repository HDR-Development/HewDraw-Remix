use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        pre_jump,
        
        tantan_attack_pre,
        tantan_attack_main,

        tantan_attack_s3_pre,
        tantan_attack_s3_main,
        
        /*
        tantan_attack_s4_start_pre,
        tantan_attack_s4_start_main,
        tantan_attack_s4_hold_pre,
        tantan_attack_s4_hold_main,
        */
        
        tantan_attack_air_pre,
        tantan_attack_air_end,
        tantan_attack_landing_exec,
        
        tantan_special_n_pre,
        tantan_special_n_main,
        tantan_special_n_exec,
        tantan_landing_air_main,

        tantan_special_hi_exec,
        tantan_special_hi_air_pre,
        tantan_special_hi_air_exec,
        tantan_special_hi_air_reach_exec
    );
}

// FIGHTER_STATUS_KIND_JUMP //

#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut arg = true;
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP {
        arg = false;
    }
    if fighter.status_pre_Jump_Common_param(L2CValue::Bool(arg)).get_bool() {
        return 1.into()
    }
    else {
        if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP {
            fighter.status_pre_Jump_sub_param(
                L2CValue::I32(-1),
                L2CValue::I32(-1),
                L2CValue::I32(-1),
                L2CValue::I32(*KINETIC_TYPE_NONE),
                L2CValue::I32(*FS_SUCCEEDS_KEEP_EFFECT | *FS_SUCCEEDS_KEEP_SOUND | *FS_SUCCEEDS_KEEP_TRANSITION | *FS_SUCCEEDS_KEEP_CANCEL)
            );
        }
        else {
            fighter.status_pre_Jump_sub_param(
                L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLAG),
                L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_INT),
                L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLOAT),
                L2CValue::I32(*FIGHTER_KINETIC_TYPE_JUMP),
                L2CValue::I32(0)
            );
        }
        return 0.into()
    }
}

//ARMS land, prevents ARMDashing
#[status_script(agent = "tantan", status = FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING_LIGHT, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn tantan_attack_landing_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.motion_frame() < 2.0 {
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.5);
    }

    return 0.into();
}

// Normal Jab //
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_Attack();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.check_attack_mtrans();
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_check_attack_mtrans as *const () as _));
    fighter.sub_status_AttackComboCommon();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Attack_Main as *const () as _))
}

//Ftilt//
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_attack_s3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_AttackS3();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_attack_s3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackS3_Main();
}

//Fsmash//
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_attack_s4_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_AttackS4Start();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_attack_s4_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackS4Start();
}

#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_attack_s4_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    macros::PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
    return fighter.status_pre_AttackS4Hold();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_attack_s4_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackS4Hold as *const () as _))
}

// Aerial Attacks
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_AttackAir();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn tantan_attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_end_AttackAir();
}

// Neutral Special (Air)
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) ==*SITUATION_KIND_AIR
    {
        return fighter.status_pre_AttackAir();
    }
    else{
        return original!(fighter);
    }
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) ==*SITUATION_KIND_AIR
    {
        let fighter_log_attack_kind = *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F;
        smash_script::notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_KEEP, fighter_log_attack_kind);
        let motion_kind = Hash40::new("attack_air_f").hash;
        WorkModule::set_int64(fighter.module_accessor, motion_kind as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);

        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackAir_Main as *const () as _))
    }
    else{
        return original!(fighter);
    }
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn tantan_special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        return 0.into();
    }

    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_SHORT_R);
    return 0.into();
}

#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_landing_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_SPECIAL_N
    {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_end"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_LandingAttackAir_Main as *const () as _))
    }
    else{
        return original!(fighter);
    }
}

#[status_script(agent = "tantan", status = FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn tantan_special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (fighter.motion_frame() <= 8.0)
    && !fighter.is_button_on(Buttons::Special)
    {
        println!("convert");
        fighter.change_status_req(*FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR, false);
        return 0.into();
    }
    return original!(fighter);
}

#[status_script(agent = "tantan", status = FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_special_hi_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_prev_status(*FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND){

        return 0.into();
    }
    else
    {
        return original!(fighter);
    }
}
#[status_script(agent = "tantan", status = FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn tantan_special_hi_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}
#[status_script(agent = "tantan", status = FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR_REACH, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn tantan_special_hi_air_reach_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut angle = WorkModule::get_float(fighter.module_accessor,*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_SHIFT_ANGLE_L);
    if (fighter.motion_frame() < 2.0)
    {
        let stick = Vector2f::new(
            fighter.stick_x(),
            fighter.stick_y()        
        );
        angle = ((stick.x)*-10.0*PostureModule::lr(fighter.module_accessor))-5.0;
        WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_SHIFT_ANGLE_L);
    }
    fighter.set_joint_rotate("claviclel", Vector3f::new(0.0, angle, 0.0));
    return 0.into();
}