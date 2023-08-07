use super::*;
use globals::*;

 
pub fn install() {
    install_status_scripts!(
        attack_air_pre,
    );
}

// FIGHTER_STATUS_KIND_ATTACK_AIR //

#[status_script(agent = "bayonetta", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    original!(fighter)
}