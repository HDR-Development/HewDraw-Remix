use super::*;
use std::arch::asm;

#[skyline::hook(offset = 0x993ec0)]
pub unsafe extern "C" fn donkey_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash_rs::app::LinkEvent) -> u64 {
    // param_3 + 0x10
    if event.link_event_kind.0 == hash40("capture") {
        println!("hi");
        let capture_event : &mut smash_rs::app::LinkEventCapture = std::mem::transmute(event);
        let module_accessor = fighter.battle_object.module_accessor;
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            // param_3[0x28]
            capture_event.result = true;
            // capture_event.constraint = false;
            // param_3 + 0x30
            capture_event.node = smash_rs::phx::Hash40::new("throw");
            StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_CATCH_PULL, false);
            return 0;
        }
        return 1;
    }
    original!()(vtable, fighter, event)
}

pub fn install() {
    skyline::install_hooks!(
        donkey_link_event
    );
}