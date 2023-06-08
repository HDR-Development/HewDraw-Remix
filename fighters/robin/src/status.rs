use super::*;
use globals::*;

#[status_script(agent = "reflet", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_button_off(Buttons::Smash) {
        VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
        0.into()
    } else {
        0.into()
    }
}

pub fn install() {
    install_status_scripts!(
        init_attack_air
    );
}