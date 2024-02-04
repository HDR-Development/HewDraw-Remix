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


extern "C" fn tantan_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_TANTAN {                
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}



// FIGHTER_STATUS_KIND_JUMP //

pub unsafe extern "C" fn pre_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
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

pub unsafe extern "C" fn pre_jump_squat(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_button_on(Buttons::Catch) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_CATCH, false);
        return 1.into();
    }
    return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_JUMP_SQUAT)(fighter);
}


unsafe extern "C" fn tantan_attack_jump_aerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("tantan_jump_line_s"), false, true);
    macros::EFFECT_OFF_KIND(fighter, Hash40::new("tantan_jump_line_l"), false, true);
    macros::EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    
    return smashline::original_status(Main, fighter, *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_AERIAL)(fighter);
}

//ARMS land, prevents ARMDashing

unsafe extern "C" fn tantan_attack_landing_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.motion_frame() < 2.0 {
        sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.5);
    }

    return 0.into();
}

// Normal Jab //

unsafe extern "C" fn tantan_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_Attack();
}

unsafe extern "C" fn tantan_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.check_attack_mtrans();
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_check_attack_mtrans as *const () as _));
    fighter.sub_status_AttackComboCommon();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Attack_Main as *const () as _))
}

//Ftilt//

unsafe extern "C" fn tantan_attack_s3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_AttackS3();
}

unsafe extern "C" fn tantan_attack_s3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackS3_Main();
}

unsafe extern "C" fn tantan_attack_s3_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
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

unsafe extern "C" fn tantan_attack_s4_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_AttackS4Start();
}

unsafe extern "C" fn tantan_attack_s4_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackS4Start();
}


unsafe extern "C" fn tantan_attack_s4_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    macros::PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
    return fighter.status_pre_AttackS4Hold();
}

unsafe extern "C" fn tantan_attack_s4_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackS4Hold as *const () as _))
}

pub unsafe extern "C" fn tantan_attack_s4_hold_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let bigFrame =  WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME);
    if 0 < bigFrame && bigFrame < 2 {
        WorkModule::set_int(fighter.module_accessor, 2,*FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME);
    }
    return 0.into()
}

pub unsafe extern "C" fn tantan_attack_s4_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let bigFrame =  WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME);
    if 0 < bigFrame && bigFrame < 2 {
        WorkModule::set_int(fighter.module_accessor, 2,*FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME);
    }
    return 0.into()
}

unsafe extern "C" fn tantan_attack_s4_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let bigFrame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME);
    if fighter.motion_frame() > 16.0 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R) == 0 {
        if bigFrame > 0 {
            let maxBigFrame = WorkModule::get_param_int(fighter.module_accessor,hash40("param_private"),hash40("arm_l_big_frame"));
            let newBigFrame = (bigFrame-(maxBigFrame/2)).max(1);
            WorkModule::set_int(fighter.module_accessor, newBigFrame, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ARM_L_BIG_FRAME);
        }
    }
    return fighter.status_end_AttackS4();
}

// Aerial Attacks

unsafe extern "C" fn tantan_attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_AttackAir();
}

unsafe extern "C" fn tantan_attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_end_AttackAir();
}


//Grab//

unsafe extern "C" fn tantan_catch_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_Catch();
}

unsafe extern "C" fn tantan_catch_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_Catch();
}

unsafe extern "C" fn tantan_catch_pull_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_CATCH {
        fighter.status_CatchPull_common(hash40("catch_wait").into());
        ControlModule::reset_trigger(fighter.module_accessor);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_pull2"), 9.0, 1.0, false, 0.0, false, false);
        return fighter.main_shift(catch_pull_main_loop)
    }
    else {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_CATCH_PULL)(fighter);
    }
}

unsafe extern "C" fn catch_pull_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchPull_Main()
}
// Neutral Special (Air)

unsafe extern "C" fn tantan_special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR
    {
        return fighter.status_pre_AttackAir();
    }
    else{
        return smashline::original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
}

unsafe extern "C" fn tantan_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR
    {
        let fighter_log_attack_kind = *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F;
        smash_script::notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_KEEP, fighter_log_attack_kind);
        let motion_kind = Hash40::new("attack_air_f").hash;
        WorkModule::set_int64(fighter.module_accessor, motion_kind as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);

        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackAir_Main as *const () as _))
    }
    else {
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
    }
}

unsafe extern "C" fn tantan_special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        return 0.into();
    }

    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_SHORT_R);
    return 0.into();
}


unsafe extern "C" fn tantan_landing_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_SPECIAL_N
    {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_end"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_LandingAttackAir_Main as *const () as _))
    }
    else{
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR)(fighter);
    }
}


unsafe extern "C" fn tantan_special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (fighter.motion_frame() <= 30.0)
    && !fighter.is_button_on(Buttons::Special)
    {
        fighter.change_status_req(*FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR, false);
        return 0.into();
    }
    return smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter);
}


unsafe extern "C" fn tantan_special_hi_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        return smashline::original_status(Pre, fighter, *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR)(fighter);
    }
}

unsafe extern "C" fn tantan_special_hi_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
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

unsafe extern "C" fn tantan_special_hi_air_reach_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let angle = WorkModule::get_float(fighter.module_accessor,*FIGHTER_TANTAN_INSTANCE_WORK_ID_FLOAT_ATTACK_SHIFT_ANGLE_L);
    fighter.set_joint_rotate("claviclel", Vector3f::new(0.0, angle, 0.0));
    return 0.into();
}
pub fn install() {
    smashline::Agent::new("tantan")
        .on_init(tantan_init)
        .status(Pre, *FIGHTER_STATUS_KIND_JUMP, pre_jump)
        .status(Pre, *FIGHTER_STATUS_KIND_JUMP_SQUAT, pre_jump_squat)
        .status(
            Main,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_AERIAL,
            tantan_attack_jump_aerial_main,
        )
        .status(
            Exec,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING_LIGHT,
            tantan_attack_landing_exec,
        )
        .status(Pre, *FIGHTER_STATUS_KIND_ATTACK, tantan_attack_pre)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK, tantan_attack_main)
        .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_S3, tantan_attack_s3_pre)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_S3, tantan_attack_s3_main)
        .status(Exec, *FIGHTER_STATUS_KIND_ATTACK_S3, tantan_attack_s3_exec)
        .status(
            Pre,
            *FIGHTER_STATUS_KIND_ATTACK_S4_START,
            tantan_attack_s4_start_pre,
        )
        .status(
            Main,
            *FIGHTER_STATUS_KIND_ATTACK_S4_START,
            tantan_attack_s4_start_main,
        )
        .status(
            Pre,
            *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
            tantan_attack_s4_hold_pre,
        )
        .status(
            Main,
            *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
            tantan_attack_s4_hold_main,
        )
        .status(
            Exec,
            *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
            tantan_attack_s4_hold_exec,
        )
        .status(Exec, *FIGHTER_STATUS_KIND_ATTACK_S4, tantan_attack_s4_exec)
        .status(End, *FIGHTER_STATUS_KIND_ATTACK_S4, tantan_attack_s4_end)
        .status(Pre, *FIGHTER_STATUS_KIND_ATTACK_AIR, tantan_attack_air_pre)
        .status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, tantan_attack_air_end)
        .status(Pre, *FIGHTER_STATUS_KIND_CATCH, tantan_catch_pre)
        .status(Main, *FIGHTER_STATUS_KIND_CATCH, tantan_catch_main)
        .status(
            Main,
            *FIGHTER_STATUS_KIND_CATCH_PULL,
            tantan_catch_pull_main,
        )
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, tantan_special_n_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, tantan_special_n_main)
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, tantan_special_n_exec)
        .status(
            Main,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            tantan_landing_air_main,
        )
        .status(
            Exec,
            *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_GROUND,
            tantan_special_hi_exec,
        )
        .status(
            Pre,
            *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR,
            tantan_special_hi_air_pre,
        )
        .status(
            Exec,
            *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR,
            tantan_special_hi_air_exec,
        )
        .status(
            Exec,
            *FIGHTER_TANTAN_STATUS_KIND_SPECIAL_HI_AIR_REACH,
            tantan_special_hi_air_reach_exec,
        )
        .install();
}
