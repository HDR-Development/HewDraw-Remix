use super::*;

#[skyline::from_offset(0x993ee0)]
unsafe extern "C" fn donkey_on_link_event_original(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64;

// FighterVtableSlot::OnLinkEvent
unsafe extern "C" fn donkey_on_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    // param_3 + 0x10
    if event.link_event_kind.0 == hash40("capture") {
        let capture_event: &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        let module_accessor = fighter.battle_object.module_accessor;
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            // param_3[0x28]
            capture_event.result = true;
            // capture_event.constraint = false;
            // param_3 + 0x30
            capture_event.node = smash2::phx::Hash40::new("throw");
            StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_CATCH_PULL, false);
            return 0;
        }
        return 1;
    }

    donkey_on_link_event_original(vtable, fighter, event)
}

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_DONKEY,
        FighterVtableSlot::OnLinkEvent,
        donkey_on_link_event as *const () as _
    );
}
