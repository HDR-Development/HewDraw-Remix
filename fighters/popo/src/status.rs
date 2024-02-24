use super::*;
use globals::*;
// status script import
 

#[utils::export(popo)]
pub unsafe fn ics_dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Dash_Sub();
    fighter.sub_shift_status_main(L2CValue::Ptr(ics_dash_main as *const () as _))
}

unsafe extern "C" fn ics_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[OBJECT_ID] == FIGHTER_KIND_NANA {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_POPO_INSTANCE_WORK_ID_FLAG_ENABLE_TRANSITION_FORBID) {
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
        }
    }
    fighter.status_Dash_Main();
    0.into()
}

// FIGHTER_STATUS_KIND_DASH //

pub unsafe extern "C" fn dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    ics_dash(fighter)
}

// FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP //

pub unsafe extern "C" fn special_hi_jump_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}
pub fn install() {
    smashline::Agent::new("popo")
        .status(Main, *FIGHTER_STATUS_KIND_DASH, dash)
        .status(
            Exit,
            *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP,
            special_hi_jump_exit,
        )
        .install();
}
