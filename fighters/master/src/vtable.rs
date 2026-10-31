use super::*;

#[skyline::from_offset(0xceb020)]
unsafe extern "C" fn master_on_link_event_original(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> bool;

// FighterVtableSlot::OnLinkEvent
unsafe extern "C" fn master_on_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> bool {
    let kind = event.link_event_kind.0;
    let (enable_flag, early_return) = match kind {
        0xcf409680b => (true, false),
        0x1e4c0767e5 => (false, true),
        _ => (false, false)
    };
    if enable_flag {
        let battle_object = &mut fighter.battle_object;
        let module_accessor = battle_object.module_accessor;
        let status = StatusModule::status_kind(module_accessor);
        if [
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_MASTER_STATUS_KIND_SPECIAL_LW_HIT,
        ].contains(&status) {
            VarModule::on_flag(battle_object, vars::master::status::SPECIAL_LW_ENABLE_CANCEL);
        }
    }
    if early_return {
        return false;
    }

    master_on_link_event_original(vtable, fighter, event)
}

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_MASTER,
        FighterVtableSlot::OnLinkEvent,
        master_on_link_event as *const () as _
    );
}
