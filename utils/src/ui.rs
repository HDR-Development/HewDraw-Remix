use once_cell::sync::Lazy;
use parking_lot::RwLock;

use self::ex_meter::ExMeter;
use self::ff_meter::FfMeter;
use self::power_board::PowerBoard;
use self::pichu_meter::PichuMeter;
use self::aura_meter::AuraMeter;

mod ex_meter;
mod ff_meter;
mod power_board;
mod pichu_meter;
mod aura_meter;

trait UiObject {
    fn update(&mut self);
    fn is_valid(&self) -> bool;
    fn set_enable(&mut self, enable: bool);
    fn is_enabled(&self) -> bool;
}

static UI_MANAGER: Lazy<RwLock<UiManager>> = Lazy::new(|| RwLock::new(UiManager { 
    ex_meter: [ExMeter::default(); 8],
    ff_meter: [FfMeter::default(); 8],
    power_board: [PowerBoard::default(); 8],
    pichu_meter: [PichuMeter::default(); 8],
    aura_meter: [AuraMeter::default(); 8],
}));

#[repr(C)]
pub struct UiManager {
    ex_meter: [ExMeter; 8],
    ff_meter: [FfMeter; 8],
    power_board: [PowerBoard; 8],
    pichu_meter: [PichuMeter; 8],
    aura_meter: [AuraMeter; 8]
}

impl UiManager {
    /// Gets the relevant UI entry based on the entry_id of the fighter.
    /// This is nececessary because the UI is indexed from zero every match,
    /// but the entry_id is based on player entries, I.E. the portraits in
    /// the character select screen. So if player 2 and player 4 are playing
    /// a 1v1 match, player 2 will have the first UI slot, and player 4 will
    /// have the second UI slot.
    /// # Arguments
    /// - entry_id: the entry id of this fighter
    /// # Returns:
    /// - the ui index
    fn get_ui_index_from_entry_id(entry_id: u32) -> u32 {
        // start at index 0
        let mut index = 0;

        // check all of the possible entry IDs less than or equal to this one,
        // and see how many "slots" should be filled, counting up from 0.
        for n in 0..entry_id {
            if crate::util::get_battle_object_from_entry_id(n).is_some() {
                // this is a valid fighter in this match, which means they will
                // be occupying a UI slot. Thus, we cannot take that slot.
                index += 1;
            }
        }

        return index;
    }

    #[export_name = "UiManager__set_dk_barrel_enable"]
    pub extern "C" fn set_dk_barrel_enable(entry_id: u32, enable: bool) {
        // let manager = UI_MANAGER.read();
        // unsafe {
        //     set_pane_visible(manager.dk_handles[entry_id as usize], enable);
        // }
    }

    #[export_name = "UiManager__set_shoto_meter_enable"]
    pub extern "C" fn set_shoto_meter_enable(entry_id: u32, enable: bool) {
        // let manager = UI_MANAGER.read();
        // unsafe {
        //     set_pane_visible(manager.shoto_meter_handles[entry_id as usize], enable);
        //     set_pane_visible(manager.shoto_bar_handles[entry_id as usize], enable);
        //     set_pane_visible(manager.shoto_number_handles[entry_id as usize], enable);
        // }
    }

    #[export_name = "UiManager__set_shoto_bar_percentage"]
    pub extern "C" fn set_shoto_bar_percentage(entry_id: u32, percentage: f32) {
        // let mut manager = UI_MANAGER.write();
        // unsafe {
        //     if manager.shoto_bar_widths[entry_id as usize] < 0.0 {
        //         manager.shoto_bar_widths[entry_id as usize] = get_width_height(manager.shoto_bar_handles[entry_id as usize]).0;
        //         manager.shoto_bar_heights[entry_id as usize] = get_width_height(manager.shoto_bar_handles[entry_id as usize]).1;
        //     }
        //     set_tex_coords(
        //         manager.shoto_bar_handles[entry_id as usize],
        //         [
        //             0.0, 0.0,
        //             percentage / 100.0, 0.0,
        //             0.0, 1.0,
        //             percentage / 100.0, 1.0
        //         ]
        //     );
        //     set_width_height(
        //         manager.shoto_bar_handles[entry_id as usize],
        //         manager.shoto_bar_widths[entry_id as usize] * (percentage / 100.0),
        //         manager.shoto_bar_heights[entry_id as usize]
        //     );
        // }
    }

    #[export_name = "UiManager__set_shoto_number"]
    pub extern "C" fn set_shoto_number(entry_id: u32, number: i32) {
        // let number = number.clamp(0, 5);
        // let manager = UI_MANAGER.read();
        // unsafe {
        //     let left_x = number as f32 / 6.0;
        //     let right_x = (number + 1) as f32 / 6.0;

        //     set_tex_coords(
        //         manager.shoto_number_handles[entry_id as usize],
        //         [
        //             left_x, 0.0,
        //             right_x, 0.0,
        //             left_x, 1.0,
        //             right_x, 1.0
        //         ]
        //     );
        // }
    }

    #[export_name = "UiManager__set_ex_meter_enable"]
    pub extern "C" fn set_ex_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.ex_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_enable(enable);
        }
    }

    #[export_name = "UiManager__set_ex_meter_info"]
    pub extern "C" fn set_ex_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.ex_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_meter_info(current, max, per_level);
        }
    }

    #[export_name = "UiManager__set_ff_meter_enable"]
    pub extern "C" fn set_ff_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.ff_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_enable(enable);
        }
    }

    #[export_name = "UiManager__set_ff_meter_info"]
    pub extern "C" fn set_ff_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.ff_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_meter_info(current, max, per_level);
        }
    }

    #[export_name = "UiManager__change_ff_meter_cap"]
    pub extern "C" fn change_ff_meter_cap(entry_id: u32, cap: f32) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.ff_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].change_cap(cap);
        }
    }

    #[export_name = "UiManager__set_power_board_enable"]
    pub extern "C" fn set_power_board_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.power_board[Self::get_ui_index_from_entry_id(entry_id) as usize].set_enable(enable);
        }
    }

    #[export_name = "UiManager__set_power_board_info"]
    pub extern "C" fn set_power_board_info(entry_id: u32, current: f32, max: f32, per_level: f32, color_1: i32, color_2: i32) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.power_board[Self::get_ui_index_from_entry_id(entry_id) as usize].set_meter_info(current, max, per_level, color_1, color_2);
        }
    }

    #[export_name = "UiManager__change_power_board_color"]
    pub extern "C" fn change_power_board_color(entry_id: u32, color_1: i32, color_2: i32) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.power_board[Self::get_ui_index_from_entry_id(entry_id) as usize].change_color(color_1, color_2);
        }
    }
    
    #[export_name = "UiManager__set_pichu_meter_enable"]
    pub extern "C" fn set_pichu_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.pichu_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_enable(enable);
        }
    }

    #[export_name = "UiManager__set_pichu_meter_info"]
    pub extern "C" fn set_pichu_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32, charged: bool) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.pichu_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_meter_info(current, max, per_level, charged);
        }
    }

    #[export_name = "UiManager__set_aura_meter_enable"]
    pub extern "C" fn set_aura_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.aura_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_enable(enable);
        }
    }

    #[export_name = "UiManager__set_aura_meter_info"]
    pub extern "C" fn set_aura_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32, burnout: bool) {
        let mut manager = UI_MANAGER.write();
        unsafe {
            manager.aura_meter[Self::get_ui_index_from_entry_id(entry_id) as usize].set_meter_info(current, max, per_level, burnout);
        }
    }
}

fn set_pane_visible(pane: u64, visible: bool) {
    unsafe {
        let internal = *(pane as *const u64);
        *(internal as *mut u8).add(0x58) &= 0xFE;
        *(internal as *mut u8).add(0x58) |= visible as u8;
    }
}

fn set_pane_colors(
    pane: u64,
    white: [f32; 4],
    black: [f32; 4]
) {
    set_vertex_colors(pane, black, black, white, white);
}

fn set_vertex_colors(
    pane: u64,
    tl: [f32; 4],
    tr: [f32; 4],
    bl: [f32; 4],
    br: [f32; 4]
) {
    unsafe {
        let internal = *(pane as *const u64);
        let colors = [tl, tr, bl, br];
        for (index, color) in colors.iter().enumerate() {
            *(internal as *mut u8).add(0xe0 + index * 4) = (color[0] * 255.0) as u8;
            *(internal as *mut u8).add(0xe1 + index * 4) = (color[1] * 255.0) as u8;
            *(internal as *mut u8).add(0xe2 + index * 4) = (color[2] * 255.0) as u8;
            *(internal as *mut u8).add(0xe3 + index * 4) = (color[3] * 255.0) as u8;
        }
    }
}

unsafe fn get_pane_by_name(layout_view: u64, name: &str) -> [u64; 4] {
    let func: extern "C" fn(u64, *const u8, ...) -> [u64; 4] = std::mem::transmute((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x37752e0));
    func(layout_view, name.as_ptr())
}

fn set_tex_coords(pane: u64, coords: [f32; 8]) {
    unsafe {
        let internal = *(pane as *const u64);
        let coordinates = std::slice::from_raw_parts_mut(
            *((internal + 0xf8) as *mut *mut f32),
            8
        );
        coordinates[0] = coords[0];
        coordinates[1] = coords[1];
        coordinates[2] = coords[2];
        coordinates[3] = coords[3];
        coordinates[4] = coords[4];
        coordinates[5] = coords[5];
        coordinates[6] = coords[6];
        coordinates[7] = coords[7];
    }
}

fn is_pane_valid(pane: u64) -> bool {
    unsafe {
        *(pane as *const u64) != 0
    }
}

fn set_width_height(pane: u64, width: f32, height: f32) {
    unsafe {
        let internal = *(pane as *const u64);
        *(internal as *mut f32).add(0x50 / 4) = width;
        *(internal as *mut f32).add(0x54 / 4) = height;
    }
}

fn get_width_height(pane: u64) -> (f32, f32) {
    unsafe {
        let internal = *(pane as *const u64);
        (
            *(internal as *mut f32).add(0x50 / 4),
            *(internal as *mut f32).add(0x54 / 4)
        )
    }
}

fn get_pane_from_layout(layout_data: u64, name: &str) -> Option<u64> {
    unsafe {
        let pane_udata = get_pane_by_name(layout_data, name);
        if pane_udata[1] != 0 {
            Some(pane_udata[1])
        } else {
            None
        }
    }
}

#[skyline::hook(offset = 0x1b6c108, inline)]
unsafe fn get_set_info_alpha(ctx: &skyline::hooks::InlineCtx) {
    let layout_udata = *ctx.registers[0].x.as_ref();
    let layout_view = *(layout_udata as *const u64).add(1);
    let layout_pane = *(layout_view as *const u64).add(3);
    let ui2d_pane = *(layout_pane as *const u64);

    let name_ptr = (ui2d_pane as *const u8).add(0xb0);
    let mut len = skyline::libc::strlen(name_ptr);

    let name = std::str::from_utf8_unchecked(std::slice::from_raw_parts(name_ptr, len));
    let index = match name {
        "p1" => 0,
        "p2" => 1,
        "p3" => 2,
        "p4" => 3,
        "p5" => 4,
        "p6" => 5,
        "p7" => 6,
        "p8" => 7,
        _ => return
    };

    let mut manager = UI_MANAGER.write();

    manager.ex_meter[index] = ExMeter::new(layout_udata);
    manager.ff_meter[index] = FfMeter::new(layout_udata);
    manager.power_board[index] = PowerBoard::new(layout_udata);
    manager.pichu_meter[index] = PichuMeter::new(layout_udata);
    manager.aura_meter[index] = AuraMeter::new(layout_udata);
}

#[skyline::hook(offset = 0x138a6f0, inline)]
fn hud_update(_: &skyline::hooks::InlineCtx) {
    unsafe {
        // check the global static menu-based mode field
        let mode = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64 + 0x53030f0) as *const u64;
        // if we are in the following modes, there is no ui overlay, so dont update the hud
        if [
            0x6020000, // Controls Menu
            0x4050000, // Mii Maker
        ].contains(&*mode) {
            return;
        }
    }
    let mut mgr = UI_MANAGER.write();
    for ex_meter in mgr.ex_meter.iter_mut() {
        if ex_meter.is_valid() && ex_meter.is_enabled() {
            ex_meter.update();
        }
    }
    for ff_meter in mgr.ff_meter.iter_mut() {
        if ff_meter.is_valid() && ff_meter.is_enabled() {
            ff_meter.update();
        }
    }
    for power_board in mgr.power_board.iter_mut() {
        if power_board.is_valid() && power_board.is_enabled() {
            power_board.update();
        }
    }
    for pichu_meter in mgr.pichu_meter.iter_mut() {
        if pichu_meter.is_valid() && pichu_meter.is_enabled() {
            pichu_meter.update();
        }
    }
    for aura_meter in mgr.aura_meter.iter_mut() {
        if aura_meter.is_valid() && aura_meter.is_enabled() {
            aura_meter.update();
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        get_set_info_alpha,
        hud_update,
    );
}