// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub extern "C" fn flowerpot_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        if weapon.is_status( *WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_THROWED ) && AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
            weapon.change_status(WEAPON_MURABITO_FLOWERPOT_STATUS_KIND_BURST.into(), false.into());
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, flowerpot_callback);
}