use super::*;
use globals::*;

#[status_script(agent = "pit", status = FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    ret
}

pub fn install() {
    install_status_scripts!(
        special_s_end_main
    );
}