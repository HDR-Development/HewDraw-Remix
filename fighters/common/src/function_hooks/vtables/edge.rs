use super::*;
use utils::ext::*;

extern "C" {
    #[link_name = "edge_flash_on_search_inner"]
    fn edge_flash_on_search_inner(vtable: u64, weapon: &mut app::Weapon, log: *mut CollisionLog);
}

unsafe fn edge_flash_on_search(vtable: u64, weapon: &mut app::Weapon, log: *mut CollisionLog) {
    edge_flash_on_search_inner(vtable, weapon, log);
}

pub fn install() {
    unsafe {
        let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u64;
        let _ = skyline::patching::Patch::in_text(0x5189860 + (*WEAPON_KIND_EDGE_FLASH as usize * 0x1d * 0x8)).data(text.add(0x33b8a80 / 0x8));
    }

    let _ = skyline::patching::Patch::in_text(0x51c0ff0).data(edge_flash_on_search as *const () as u64);
}
