use super::*;

pub unsafe extern "C" fn update_littlemac_ui(entry_id: i32, total_gauge: f32) {
    let manager = singletons::FighterManager() as *mut u64;
    let offset = (*manager + (entry_id as u64 * 8) + 0x20) as *mut u64;
    update_littlemac_ui_internal((*offset + 0x41e4) as *mut u32, total_gauge as i32);
}

#[skyline::from_offset(0x68cda0)]
fn update_littlemac_ui_internal(manager_offset: *mut u32, total_gauge: i32);
