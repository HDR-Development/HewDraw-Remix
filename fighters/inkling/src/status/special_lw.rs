use super::*;
use globals::*;

unsafe extern "C" fn special_lw_empty_main(agent: &mut L2CFighterCommon) -> L2CValue {
    let use_cancel_anim = WorkModule::is_flag(agent.module_accessor,*FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_LW_BOMB);
    let motion_g = if use_cancel_anim {hash40("special_lw_cancel")} else {hash40("special_lw_empty")};
    println!("{}", use_cancel_anim);
    let motion_a = if use_cancel_anim {hash40("special_air_lw_cancel")} else {hash40("special_air_lw_empty")};
    agent.sub_change_motion_by_situation(motion_g.into(), motion_a.into(), false.into());
    agent.sub_set_ground_correct_by_situation(true.into());
    agent.main_shift(special_lw_empty_main_loop)
}

unsafe extern "C" fn special_lw_empty_main_loop(agent: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(agent.module_accessor) {
        if agent.sub_wait_ground_check_common(false.into()).get_bool()
        || agent.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(agent.module_accessor) {
        let new_status = if agent.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {FIGHTER_STATUS_KIND_FALL} else {FIGHTER_STATUS_KIND_WAIT};
        agent.change_status(new_status.into(), false.into());
        return 1.into();
    }
    if !StatusModule::is_changing(agent.module_accessor)
    && StatusModule::is_situation_changed(agent.module_accessor) {
        agent.sub_set_ground_correct_by_situation(false.into());
        agent.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
        let use_cancel_anim =WorkModule::is_flag(agent.module_accessor,*FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_CAN_SPECIAL_LW_BOMB);
        let motion_g = if use_cancel_anim {hash40("special_lw_cancel")} else {hash40("special_lw_empty")};
        let motion_a = if use_cancel_anim {hash40("special_air_lw_cancel")} else {hash40("special_air_lw_empty")};
        agent.sub_change_motion_by_situation(motion_g.into(), motion_a.into(), false.into());
    }

    0.into()
} 


pub unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let toreturn = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);

    let stored_charge = VarModule::get_float(fighter.battle_object,vars::inkling::instance::SPECIAL_LW_CHARGE);
    if stored_charge > 0.0 {
        MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, stored_charge, true, true, false);
        WorkModule::set_float(fighter.module_accessor,stored_charge,*FIGHTER_INKLING_STATUS_SPECIAL_LW_WORK_FLOAT_CHARGE_FRAME);
        if stored_charge >= MotionModule::end_frame(fighter.module_accessor) {
            EffectModule::remove_common(fighter.module_accessor, Hash40::new("charge_max"));
            fighter.change_status(FIGHTER_INKLING_STATUS_KIND_SPECIAL_LW_THROW.into(),false.into());
        }
    }

    toreturn
}

pub unsafe extern "C" fn special_lw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let end_frame = MotionModule::end_frame(fighter.module_accessor);
    if ControlModule::check_button_on(fighter.module_accessor,*CONTROL_PAD_BUTTON_GUARD) {
        VarModule::set_float(fighter.battle_object,vars::inkling::instance::SPECIAL_LW_CHARGE,frame);
        fighter.change_status(FIGHTER_INKLING_STATUS_KIND_SPECIAL_LW_EMPTY.into(),false.into());
    }
    if frame >= end_frame-1.0 {
        VarModule::set_float(fighter.battle_object,vars::inkling::instance::SPECIAL_LW_CHARGE,end_frame);
        EffectModule::req_common(fighter.module_accessor, Hash40::new("charge_max"), 0.0);
        EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_flash"), Hash40::new("top"), &Vector3f::new(6.0, 10.0, 0.0), &Vector3f::zero(), 0.4, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        //Could change to uniq end status for a charge succeeded anim
        let next_status = if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {*FIGHTER_STATUS_KIND_WAIT} else {*FIGHTER_STATUS_KIND_FALL};
        fighter.change_status(next_status.into(),false.into());
    }
    return 0.into();
}
pub unsafe extern "C" fn special_lw_throw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_float(fighter.battle_object,vars::inkling::instance::SPECIAL_LW_CHARGE,0.0);
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_exec);
    agent.status(Init, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_LW_THROW, special_lw_throw_init);
    agent.status(Exec, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_LW_EMPTY, special_lw_empty_main);
}