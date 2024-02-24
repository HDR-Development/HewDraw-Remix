use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        special_hi_damage_end_main
    );
}

// FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END

#[status_script(agent = "koopajr", status = FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_hi_damage_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    original!(fighter);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_DAMAGE_FLY) {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_DAMAGE_FALL, false);
        }
        else {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
    }
    1.into()
}