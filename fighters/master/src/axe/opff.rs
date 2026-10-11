// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub unsafe extern "C" fn axe_frame(weapon: &mut L2CFighterBase) {
    // disable DSpecial cancel when parried
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_PARRY) {
        let owner_boma = weapon.get_owner_boma();
        VarModule::off_flag(owner_boma.object(), vars::master::status::SPECIAL_LW_ENABLE_CANCEL);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, axe_frame);
}