// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

//Disable grab on fishingrod when pullingback
unsafe extern "C" fn fishingrod_callback(weapon : &mut L2CFighterBase) {
    let object_id = (*weapon.battle_object).battle_object_id;
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let activate_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID);
    let serial_id = WorkModule::get_int(weapon.module_accessor, 0x10000002);
    let kind = utility::get_kind(&mut (*weapon.module_accessor));
    if kind == *WEAPON_KIND_SHIZUE_FISHINGROD {
        let status = StatusModule::status_kind(weapon.module_accessor);
        if status == *WEAPON_SHIZUE_FISHINGROD_STATUS_KIND_REEL {
            WeaponSpecializer_ShizueFishingrod::enable_search(weapon.module_accessor, false);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, fishingrod_callback);
}
