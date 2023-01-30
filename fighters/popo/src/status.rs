use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        dash
    );
}

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

#[status_script(agent = "popo", status = FIGHTER_STATUS_KIND_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn dash(fighter: &mut L2CFighterCommon) -> L2CValue {
    ics_dash(fighter)
}