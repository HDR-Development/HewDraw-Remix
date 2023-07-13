use super::*;
use globals::*;
// status script import
 

/// Re-enables the ability to use aerial specials when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    //remove double dragon effect
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ENTRY,*FIGHTER_STATUS_KIND_DEAD,*FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,*FIGHTER_STATUS_KIND_LOSE]) || !sv_information::is_ready_go() {
        let dragonEffect = VarModule::get_int(fighter.object(),vars::tantan::instance::DRAGONIZE_R_EFFECT_HANDLE) as u32;
        if EffectModule::is_exist_effect(fighter.module_accessor, dragonEffect){
            EffectModule::kill(fighter.module_accessor, dragonEffect, false,false);
        }
        VarModule::set_int(fighter.object(),vars::tantan::instance::DRAGONIZE_R_EFFECT_HANDLE,0);
    }
    true.into()
}

#[smashline::fighter_init]
fn tantan_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_TANTAN {                
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}

pub fn install() {
    install_agent_init_callbacks!(tantan_init);
    install_status_scripts!(
        pre_jump,
        pre_jump_squat,

        tantan_attack_landing_exec,
        tantan_attack_jump_aerial_main,

        tantan_attack_pre,
        tantan_attack_main,

        tantan_attack_s3_pre,
        tantan_attack_s3_main,
        tantan_attack_s3_exec,
        
        /*
        tantan_attack_s4_start_pre,
        tantan_attack_s4_start_main,
        tantan_attack_s4_hold_pre,
        tantan_attack_s4_hold_main,
        */
        
        tantan_attack_air_pre,
        tantan_attack_air_end,

        tantan_catch_pre,
        tantan_catch_main,
        tantan_catch_pull_main,
        
        tantan_special_n_pre,
        tantan_special_n_main,
        tantan_special_n_exec,
        tantan_landing_air_main,

        tantan_special_hi_exec,
        tantan_special_hi_air_pre,
        tantan_special_hi_air_exec,
        tantan_special_hi_air_reach_exec,
    );
}

// FIGHTER_STATUS_KIND_JUMP //
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    let arg = !(fighter.global_table[PREV_STATUS_KIND] == FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP);
    
    if fighter.status_pre_Jump_Common_param(L2CValue::Bool(arg)).get_bool() {
        return 1.into()
    }
    else {

        if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP {
            fighter.status_pre_Jump_sub_param(
                L2CValue::I32(-1),
                L2CValue::I32(-1),
                L2CValue::I32(-1),
                L2CValue::I32(*FIGHTER_KINETIC_TYPE_NONE),
                L2CValue::I32(*FS_SUCCEEDS_KEEP_EFFECT | *FS_SUCCEEDS_KEEP_SOUND | *FS_SUCCEEDS_KEEP_TRANSITION)
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

//Fixes bug related to pressing Jump and Catch at the same time results in an in-place wavedash
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_JUMP_SQUAT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_jump_squat(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_button_on(Buttons::Catch) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_CATCH, false);
        return 1.into();
    }
    return original!(fighter);
}

#[status_script(agent = "tantan", status = FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_attack_jump_aerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("tantan_jump_line_s"), false, true);
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("tantan_jump_line_l"), false, true);
    macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    
    return original!(fighter);
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
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn tantan_attack_s3_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor){
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
    else{
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
    return 0.into();
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


//Grab//
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_CATCH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_catch_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_Catch();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_CATCH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_Catch();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_catch_pull_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_CATCH {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_pull2"), 10.0, 1.0, false, 0.0, false, false);
        return fighter.status_CatchPull();
    }
    else {
        return original!(fighter);
    }
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
    if (fighter.motion_frame() <= 30.0)
    && !fighter.is_button_on(Buttons::Special)
    {
        fighter.change_status_req(*FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR, false);
        return 0.into();
    }
    return original!(fighter);
}

#[status_script(agent = "tantan", status = FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_special_hi_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_prev_status(*FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND){
        StatusModule::init_settings(
            fighter.module_accessor,
            app::SituationKind(*SITUATION_KIND_NONE),
            *FIGHTER_KINETIC_TYPE_UNIQ,
            *GROUND_CORRECT_KIND_KEEP as u32,
            app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
            true,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
            0
        );
        FighterStatusModuleImpl::set_fighter_status_data(
            fighter.module_accessor,
            false,
            *FIGHTER_TREADED_KIND_NO_REAC,
            false,
            false,
            false,
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64,
            0,
            *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
            0
        );
        return 0.into();
    }
    else
    {
        return original!(fighter);
    }
}
#[status_script(agent = "tantan", status = FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn tantan_special_hi_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick = Vector2f::new(
        fighter.stick_x(),
        fighter.stick_y()        
    );
    let angle = ((stick.x)*-10.0*PostureModule::lr(fighter.module_accessor))-5.0;
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_SHIFT_ANGLE_L);
    if (fighter.motion_frame()>=10.0)
    {
        fighter.set_joint_rotate("claviclel", Vector3f::new(0.0, angle, 0.0));
    }
    return 0.into();
}
#[status_script(agent = "tantan", status = FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR_REACH, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn tantan_special_hi_air_reach_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let angle = WorkModule::get_float(fighter.module_accessor,*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_SHIFT_ANGLE_L);
    fighter.set_joint_rotate("claviclel", Vector3f::new(0.0, angle, 0.0));
    return 0.into();
}