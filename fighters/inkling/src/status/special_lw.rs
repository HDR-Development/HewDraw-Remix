use super::*;
use globals::*;

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

pub fn install() {
    smashline::Agent::new("inkling")
        .status(
            Main,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            special_lw_main,
        )
        .status(
            Exec,
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            special_lw_exec,
        )
        .status(
            Init,
            *FIGHTER_INKLING_STATUS_KIND_SPECIAL_LW_THROW,
            special_lw_throw_init,
        )
        .install();
}
