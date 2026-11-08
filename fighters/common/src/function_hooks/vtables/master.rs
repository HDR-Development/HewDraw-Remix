use super::*;
use utils::ext::*;

extern "C" {
    #[link_name = "master_link_event_inner"]
    fn master_link_event_inner(
        vtable: u64,
        fighter: &mut Fighter,
        event: &mut smash_rs::app::LinkEvent,
        original: extern "C" fn(u64, &mut Fighter, &mut smash_rs::app::LinkEvent) -> bool
    ) -> bool;
}

#[skyline::hook(offset = 0xceb020)]
pub unsafe extern "C" fn master_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash_rs::app::LinkEvent) -> bool {
    master_link_event_inner(vtable, fighter, event, original!())
}

pub fn install() {
    skyline::install_hooks!(
        master_link_event
    );
}