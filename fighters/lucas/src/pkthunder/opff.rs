// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub extern "C" fn pkthunder_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_NO_DEAD);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pkthunder_callback);
}