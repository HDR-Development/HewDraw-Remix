use super::*;
use globals::*;

pub unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let toreturn = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_LW)(fighter);

    let stored_charge = VarModule::get_float(fighter.battle_object,vars::inkling::instance::SPECIAL_LW_CHARGE);
    if stored_charge > 0.0 {
        let fighter_ptr = fighter.global_table[0x4].get_ptr() as *mut Fighter;
        let ink_cost = FighterSpecializer_Inkling::get_sub_ink_special_lw(fighter_ptr);
        super::spend_ink(fighter,-ink_cost);
    }

    toreturn
}

pub unsafe extern "C" fn special_lw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stored_charge = VarModule::get_float(fighter.battle_object,vars::inkling::instance::SPECIAL_LW_CHARGE);
    let current_charge = WorkModule::get_float(fighter.module_accessor,*FIGHTER_INKLING_STATUS_SPECIAL_LW_WORK_FLOAT_CHARGE_FRAME);
    if current_charge < stored_charge {
        MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, stored_charge, true, true, false);
        WorkModule::set_float(fighter.module_accessor,stored_charge,*FIGHTER_INKLING_STATUS_SPECIAL_LW_WORK_FLOAT_CHARGE_FRAME);
    }
    if ControlModule::check_button_on(fighter.module_accessor,*CONTROL_PAD_BUTTON_GUARD) {
        VarModule::set_float(fighter.battle_object,vars::inkling::instance::SPECIAL_LW_CHARGE,MotionModule::frame(fighter.module_accessor));
        fighter.change_status(FIGHTER_INKLING_STATUS_KIND_SPECIAL_LW_EMPTY.into(),false.into());
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
