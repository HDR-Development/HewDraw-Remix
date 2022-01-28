use super::*;
use utils::ext::*;

#[repr(C)]
struct SomeControllerStruct {
    padding: [u8; 0x10],
    controller: &'static Controller
}

#[skyline::hook(offset = offsets::map_controls())]
unsafe fn map_controls_hook(
    mappings: *mut ControllerMapping,
    player_idx: i32,
    out: *mut MappedInputs,
    controller_struct: &SomeControllerStruct,
    arg: bool
) {
    let controller = controller_struct.controller;
    let ret = original!()(mappings, player_idx, out, controller_struct, arg);

    // Check if the button combos are being pressed and then force Stock Share + AttackRaw/SpecialRaw depending on input

    if controller.current_buttons.l()
    && controller.current_buttons.r()
    && controller.current_buttons.a()
    && (controller.current_buttons.minus() || controller.current_buttons.plus())
    {
        if controller.current_buttons.x() {
            (*out).buttons = Buttons::StockShare | Buttons::AttackRaw;
        } else if controller.current_buttons.y() {
            (*out).buttons = Buttons::StockShare | Buttons::SpecialRaw;
        }
    }
}

pub fn install() {
    skyline::install_hook!(map_controls_hook);
}