use super::*;
use globals::*;


// FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP

#[status_script(agent = "daisy", status = FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn special_s_jump_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, vars::daisy::instance::DISABLE_SPECIAL_S);
    original!(fighter)
}

pub fn install() {
    install_status_scripts!(
        special_s_jump_main,
    );
}