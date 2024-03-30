// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub extern "C" fn koopajr_weapon_remainclown_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        let boma = weapon.boma();
        if StatusModule::status_kind(boma) == *WEAPON_KOOPAJR_REMAINCLOWN_STATUS_KIND_FALL
        && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            StatusModule::change_status_request_from_script(boma, *WEAPON_KOOPAJR_REMAINCLOWN_STATUS_KIND_BURST, true);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, koopajr_weapon_remainclown_frame);
}
