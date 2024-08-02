use super::*;

// WEAPON_PICKEL_FISHINGROD_STATUS_KIND_SHOOT

unsafe extern "C" fn shoot_exit(weapon: &mut L2CFighterCommon) -> L2CValue {
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Exit, *WEAPON_PICKEL_FISHINGROD_STATUS_KIND_SHOOT, shoot_exit);
}