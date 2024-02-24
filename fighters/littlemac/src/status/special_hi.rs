use super::*;
use globals::*;


// FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP

#[status_script(agent = "littlemac", status = FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn special_hi_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    original!(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_super_jump_punch_main as *const () as _))
}

pub fn install() {
    install_status_scripts!(
        special_hi_jump_main
    );
}