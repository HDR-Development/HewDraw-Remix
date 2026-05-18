// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub unsafe extern "C" fn tornadoshot_frame(weapon: &mut L2CFighterBase) {
    if weapon.is_status(*WEAPON_MIISWORDSMAN_TORNADOSHOT_STATUS_KIND_FLY) {
        ModelModule::set_joint_scale(weapon.module_accessor, Hash40::new("top"), &Vector3f::new(0.6, 0.6, 0.6));
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, tornadoshot_frame);
}