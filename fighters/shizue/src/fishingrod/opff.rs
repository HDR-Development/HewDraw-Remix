// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

//Disable grab on fishingrod when pullingback
unsafe extern "C" fn fishingrod_frame(weapon : &mut L2CFighterBase) {
    if weapon.is_status(*WEAPON_SHIZUE_FISHINGROD_STATUS_KIND_REEL) {
        WeaponSpecializer_ShizueFishingrod::enable_search(weapon.module_accessor, false);
    }
    
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, fishingrod_frame);
}
