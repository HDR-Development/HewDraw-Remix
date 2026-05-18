use super::*;

pub unsafe extern "C" fn special_lw2_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.check_hold_input(0, 6, Buttons::SpecialAll) {
        VarModule::on_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW2_HOLD);
    }
    
    return 0.into();
}