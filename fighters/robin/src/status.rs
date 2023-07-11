use super::*;
use globals::*;

#[status_script(agent = "reflet", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_attack_air(fighter: &mut L2CFighterCommon) -> L2CValue {
    VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
    fighter.sub_attack_air_uniq_process_init();
    0.into()
}

#[status_script(agent = "reflet", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn init_special_n(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int(fighter.battle_object, vars::robin::instance::THUNDER_CHARGE, fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT));
    original!(fighter)
}

pub fn install() {
    install_status_scripts!(
        init_attack_air,
        init_special_n
    );
}