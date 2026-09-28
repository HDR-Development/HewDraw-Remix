use once_cell::sync::Lazy;
use parking_lot::RwLock;

use self::aura_meter::AuraMeter;
use self::cyan_meter::CyanMeter;
use self::vtrigger_meter::VTriggerMeter;
use self::ff_meter::FfMeter;
use self::pichu_meter::PichuMeter;
use self::power_board::PowerBoard;
use self::robot_meter::RobotMeter;
use self::garlic_meter::GarlicMeter;
use self::plant_meter::PlantMeter;
use self::ptrainer_meter::PledgeMeter;

mod aura_meter;
mod cyan_meter;
mod vtrigger_meter;
mod ff_meter;
mod pichu_meter;
mod power_board;
mod robot_meter;
mod garlic_meter;
mod plant_meter;
mod ptrainer_meter;

trait UiObject {
    fn update(&mut self);
    fn is_valid(&self) -> bool;
    fn set_enable(&mut self, enable: bool);
    fn is_enabled(&self) -> bool;
}

static UI_MANAGER: Lazy<RwLock<UiManager>> = Lazy::new(|| RwLock::new(UiManager::default()));

/// Address (relative to the text region) of the game's global pointer to the
/// melee UI info object. Its second field points at the HUD info data block that
/// the game fills in at match start and reads every frame to draw the HUD.
const MELEE_UI_INFO_OBJECT: usize = 0x52c3420;
/// Offset of the first HUD slot record inside the HUD info data block.
const HUD_SLOT_TABLE: usize = 0xd60;
/// Size of one HUD slot record.
const HUD_SLOT_STRIDE: usize = 0x1d8;
/// Offset of the fighter entry id inside a HUD slot record. The slot's own index
/// is stored at offset 0 and is -1 while the slot is unused.
const HUD_SLOT_ENTRY_ID: usize = 0x8;

/// Reads the game's HUD slot -> fighter entry id table.
///
/// Returns `None` when the HUD info data is not available (for example outside
/// of a match). Otherwise each element is the entry id shown in that HUD slot,
/// or -1 for an unused slot.
fn hud_slot_entry_ids() -> Option<[i32; 8]> {
    unsafe {
        let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8;
        let object = *(text.add(MELEE_UI_INFO_OBJECT) as *const *const u64);
        if object.is_null() {
            return None;
        }
        let data_ptr = *object.add(1) as *const *const u8;
        if data_ptr.is_null() {
            return None;
        }
        let data = *data_ptr;
        if data.is_null() {
            return None;
        }

        let mut slots = [-1i32; 8];
        for (index, slot) in slots.iter_mut().enumerate() {
            let record = data.add(HUD_SLOT_TABLE + index * HUD_SLOT_STRIDE);
            let slot_index = *(record as *const i32);
            let entry_id = *(record.add(HUD_SLOT_ENTRY_ID) as *const i32);
            if slot_index != -1 && (0..8).contains(&entry_id) {
                *slot = entry_id;
            }
        }
        Some(slots)
    }
}

/// Resolves the UI index for an entry id, returning early from the enclosing
/// function when this fighter has no HUD slot.
macro_rules! ui_index {
    ($entry_id:expr) => {
        match UiManager::get_ui_index_from_entry_id($entry_id) {
            Some(index) => index,
            None => return,
        }
    };
}

/// Resolves the meter in this fighter's HUD slot.
/// Early return fixes crash in local wireless.
macro_rules! ui_meter {
    ($meters:expr, $entry_id:expr) => {{
        let meter = &mut $meters[ui_index!($entry_id)];
        if !meter.is_valid() {
            return;
        }
        meter
    }};
}

#[repr(C)]
#[derive(Default)]
pub struct UiManager {
    vtrigger_meter: [VTriggerMeter; 8],
    ff_meter: [FfMeter; 8],
    power_board: [PowerBoard; 8],
    cyan_meter: [CyanMeter; 8],
    pichu_meter: [PichuMeter; 8],
    aura_meter: [AuraMeter; 8],
    robot_meter: [RobotMeter; 8],
    garlic_meter: [GarlicMeter; 8],
    plant_meter: [PlantMeter; 8],
    ptrainer_meter: [PledgeMeter; 8],
}

impl UiManager {
    /// Gets the HUD slot (0-7, matching the `p1`..`p8` layouts) that the game
    /// is using to display the fighter with this entry id.
    ///
    /// The game keeps its own table of which fighter entry each HUD slot shows
    /// (see [`hud_slot_entry_ids`]). Slots are assigned per player at match start
    /// and, in online modes, the slot order does not follow entry id order, so
    /// counting occupied entries below this one (what `FighterManager::get_entry_no`
    /// does) picks the wrong slot there. Reading the game's table gives the true
    /// slot in every mode.
    ///
    /// # Arguments
    /// - entry_id: the entry id of this fighter
    /// # Returns:
    /// - `Some(index)` with the UI index to use
    /// - `None` if the game's table is populated but does not list this entry id,
    ///   in which case no UI element should be touched for this fighter
    fn get_ui_index_from_entry_id(entry_id: u32) -> Option<usize> {
        if let Some(slots) = hud_slot_entry_ids() {
            if slots.iter().any(|id| *id != -1) {
                return slots.iter().position(|id| *id == entry_id as i32);
            }
        }

        // The game's table is not available (no HUD info data yet), so fall back
        // to the old behaviour: count how many entries below this one are occupied.
        // This matches FighterManager::get_entry_no and is correct for local play.
        let mut index = 0;
        for n in 0..entry_id {
            if crate::util::get_battle_object_from_entry_id(n).is_some() {
                index += 1;
            }
        }

        Some(index)
    }

    #[export_name = "UiManager__set_dk_barrel_enable"]
    pub extern "C" fn set_dk_barrel_enable(_entry_id: u32, _enable: bool) {
        // let manager = UI_MANAGER.read();
        // unsafe {
        //     set_pane_visible(manager.dk_handles[entry_id as usize], enable);
        // }
    }

    #[export_name = "UiManager__set_shoto_meter_enable"]
    pub extern "C" fn set_shoto_meter_enable(_entry_id: u32, _enable: bool) {
        // let manager = UI_MANAGER.read();
        // unsafe {
        //     set_pane_visible(manager.shoto_meter_handles[entry_id as usize], enable);
        //     set_pane_visible(manager.shoto_bar_handles[entry_id as usize], enable);
        //     set_pane_visible(manager.shoto_number_handles[entry_id as usize], enable);
        // }
    }

    #[export_name = "UiManager__set_shoto_bar_percentage"]
    pub extern "C" fn set_shoto_bar_percentage(_entry_id: u32, _percentage: f32) {
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
    pub extern "C" fn set_shoto_number(_entry_id: u32, _number: i32) {
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

    #[export_name = "UiManager__set_vtrigger_meter_enable"]
    pub extern "C" fn set_vtrigger_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.vtrigger_meter, entry_id).set_enable(enable);
    }

    #[export_name = "UiManager__set_vtrigger_meter_info"]
    pub extern "C" fn set_vtrigger_meter_info(entry_id: u32, current: f32, level_max: i32, per_level: f32, is_vtrigger: bool) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.vtrigger_meter, entry_id)
            .set_meter_info(current, level_max, per_level, is_vtrigger);
    }

    #[export_name = "UiManager__set_ff_meter_enable"]
    pub extern "C" fn set_ff_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.ff_meter, entry_id).set_enable(enable);
    }

    #[export_name = "UiManager__set_ff_meter_info"]
    pub extern "C" fn set_ff_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.ff_meter, entry_id)
            .set_meter_info(current, max, per_level);
    }

    #[export_name = "UiManager__change_ff_meter_cap"]
    pub extern "C" fn change_ff_meter_cap(entry_id: u32, cap: f32) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.ff_meter, entry_id).change_cap(cap);
    }

    #[export_name = "UiManager__set_power_board_enable"]
    pub extern "C" fn set_power_board_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.power_board, entry_id).set_enable(enable);
    }

    #[export_name = "UiManager__set_power_board_info"]
    pub extern "C" fn set_power_board_info(
        entry_id: u32,
        color_1: i32,
        color_2: i32,
    ) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.power_board, entry_id)
            .set_meter_info(color_1, color_2);
    }

    #[export_name = "UiManager__change_power_board_color"]
    pub extern "C" fn change_power_board_color(entry_id: u32, color_1: i32, color_2: i32) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.power_board, entry_id)
            .set_meter_info(color_1, color_2);
    }

    #[export_name = "UiManager__set_cyan_meter_enable"]
    pub extern "C" fn set_cyan_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.cyan_meter, entry_id).set_enable(enable);
    }

    #[export_name = "UiManager__set_cyan_meter_info"]
    pub extern "C" fn set_cyan_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.cyan_meter, entry_id)
            .set_meter_info(current, max, per_level);
    }

    #[export_name = "UiManager__set_pichu_meter_enable"]
    pub extern "C" fn set_pichu_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.pichu_meter, entry_id).set_enable(enable);
    }

    #[export_name = "UiManager__set_pichu_meter_info"]
    pub extern "C" fn set_pichu_meter_info(
        entry_id: u32,
        current: f32,
        max: f32,
        per_level: f32,
        charged: bool,
    ) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.pichu_meter, entry_id)
            .set_meter_info(current, max, per_level, charged);
    }

    #[export_name = "UiManager__set_aura_meter_enable"]
    pub extern "C" fn set_aura_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.aura_meter, entry_id).set_enable(enable);
    }

    #[export_name = "UiManager__set_aura_meter_info"]
    pub extern "C" fn set_aura_meter_info(
        entry_id: u32,
        current: f32,
        max: f32,
        per_level: f32,
        burnout: bool,
    ) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.aura_meter, entry_id)
            .set_meter_info(current, max, per_level, burnout);
    }

    #[export_name = "UiManager__set_robot_meter_enable"]
    pub extern "C" fn set_robot_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.robot_meter, entry_id).set_enable(enable);
    }

    #[export_name = "UiManager__set_robot_meter_info"]
    pub extern "C" fn set_robot_meter_info(entry_id: u32, current: f32, max: f32, per_level: f32) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.robot_meter, entry_id)
            .set_meter_info(current, max, per_level);
    }

    #[export_name = "UiManager__set_garlic_meter_enable"]
    pub extern "C" fn set_garlic_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.garlic_meter, entry_id).set_enable(enable);
    }

    #[export_name = "UiManager__set_garlic_meter_info"]
    pub extern "C" fn set_garlic_meter_info(entry_id: u32, current: f32, level1: f32, level2: f32, level3: f32) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.garlic_meter, entry_id)
            .set_meter_info(current, level1, level2, level3);
    }

    #[export_name = "UiManager__set_plant_meter_enable"]
    pub extern "C" fn set_plant_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.plant_meter, entry_id).set_enable(enable);
    }

    #[export_name = "UiManager__set_plant_meter_info"]
    pub extern "C" fn set_plant_meter_info(entry_id: u32, element: i32) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.plant_meter, entry_id)
            .set_meter_info(element);
    }

    #[export_name = "UiManager__set_ptrainer_meter_enable"]
    pub extern "C" fn set_ptrainer_meter_enable(entry_id: u32, enable: bool) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.ptrainer_meter, entry_id).set_enable(enable);
    }

    #[export_name = "UiManager__set_ptrainer_meter_info"]
    pub extern "C" fn set_ptrainer_meter_info(
        entry_id: u32,
        current_pledge: f32,
        max_pledge: f32,
        current_swap: f32,
        max_swap: f32,
        pledge_state: i32,
        disabled: bool
    ) {
        let mut manager = UI_MANAGER.write();
        ui_meter!(manager.ptrainer_meter, entry_id)
            .set_meter_info(current_pledge, max_pledge, current_swap, max_swap, pledge_state, disabled);
    }
}

fn set_pane_visible(pane: u64, visible: bool) {
    unsafe {
        let internal = *(pane as *const u64);
        *(internal as *mut u8).add(0x58) &= 0xFE;
        *(internal as *mut u8).add(0x58) |= visible as u8;
    }
}

fn set_pane_colors(pane: u64, white: [f32; 4], black: [f32; 4]) {
    set_vertex_colors(pane, black, black, white, white);
}

fn set_vertex_colors(pane: u64, tl: [f32; 4], tr: [f32; 4], bl: [f32; 4], br: [f32; 4]) {
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
    let func: extern "C" fn(u64, *const u8, ...) -> [u64; 4] = std::mem::transmute(
        (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3776910),
    );
    func(layout_view, name.as_ptr())
}

fn set_tex_coords(pane: u64, coords: [f32; 8]) {
    unsafe {
        let internal = *(pane as *const u64);
        let coordinates = std::slice::from_raw_parts_mut(*((internal + 0xf8) as *mut *mut f32), 8);
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
    unsafe { pane != 0 && *(pane as *const u64) != 0 }
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
            *(internal as *mut f32).add(0x54 / 4),
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

#[skyline::hook(offset = 0x1b6cc08, inline)]
unsafe fn get_set_info_alpha(ctx: &skyline::hooks::InlineCtx) {
    let layout_udata = ctx.registers[0].x();
    let layout_view = *(layout_udata as *const u64).add(1);
    let layout_pane = *(layout_view as *const u64).add(3);
    let ui2d_pane = *(layout_pane as *const u64);

    let name_ptr = (ui2d_pane as *const u8).add(0xb0);
    let len = skyline::libc::strlen(name_ptr);

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
        _ => return,
    };

    let mut manager = UI_MANAGER.write();

    manager.vtrigger_meter[index] = VTriggerMeter::new(layout_udata);
    manager.ff_meter[index] = FfMeter::new(layout_udata);
    manager.power_board[index] = PowerBoard::new(layout_udata);
    manager.cyan_meter[index] = CyanMeter::new(layout_udata);
    manager.pichu_meter[index] = PichuMeter::new(layout_udata);
    manager.aura_meter[index] = AuraMeter::new(layout_udata);
    manager.robot_meter[index] = RobotMeter::new(layout_udata);
    manager.garlic_meter[index] = GarlicMeter::new(layout_udata);
    manager.plant_meter[index] = PlantMeter::new(layout_udata);
    manager.ptrainer_meter[index] = PledgeMeter::new(layout_udata);
}

/// Reset at match teardown
#[skyline::hook(offset = 0x134cb38, inline)]
fn melee_ui_teardown(_: &skyline::hooks::InlineCtx) {
    *UI_MANAGER.write() = UiManager::default();
}

#[skyline::hook(offset = 0x138a710, inline)]
fn hud_update(_: &skyline::hooks::InlineCtx) {
    unsafe {
        // check the global static menu-based mode field
        let mode = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64
            + 0x53050f0) as *const u64;
        // if we are in the following modes, there is no ui overlay, so dont update the hud
        if [
            0x6020000, // Controls Menu
            0x4050000, // Mii Maker
        ]
        .contains(&*mode)
        {
            return;
        }
    }
    let mut mgr = UI_MANAGER.write();
    for vtrigger_meter in mgr.vtrigger_meter.iter_mut() {
        if vtrigger_meter.is_valid() && vtrigger_meter.is_enabled() {
            vtrigger_meter.update();
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
    for cyan_meter in mgr.cyan_meter.iter_mut() {
        if cyan_meter.is_valid() && cyan_meter.is_enabled() {
            cyan_meter.update();
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
    for robot_meter in mgr.robot_meter.iter_mut() {
        if robot_meter.is_valid() && robot_meter.is_enabled() {
            robot_meter.update();
        }
    }
    for garlic_meter in mgr.garlic_meter.iter_mut() {
        if garlic_meter.is_valid() && garlic_meter.is_enabled() {
            garlic_meter.update();
        }
    }
    for plant_meter in mgr.plant_meter.iter_mut() {
        if plant_meter.is_valid() && plant_meter.is_enabled() {
            plant_meter.update();
        }
    }
    for ptrainer_meter in mgr.ptrainer_meter.iter_mut() {
        if ptrainer_meter.is_valid() && ptrainer_meter.is_enabled() {
            ptrainer_meter.update();
        }
    }
}

pub fn install() {
    skyline::install_hooks!(get_set_info_alpha, melee_ui_teardown, hud_update,);
}
