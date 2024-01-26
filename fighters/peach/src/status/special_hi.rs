use super::*;
use globals::*;

#[status_script(agent = "peach", status = FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_hi_air_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(special_hi_substatus as *const () as _));
    original!(fighter)
}

unsafe extern "C" fn special_hi_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && param_1.get_bool() {
        fighter.sub_air_check_dive();
    }
    0.into()
}

pub fn install() {
    smashline::install_status_scripts!(
        special_hi_air_end_main
    );
}