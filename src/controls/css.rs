use std::collections::HashMap;

use super::submenu::{TagSubMenu, TopLevel};
use dynamic::ext::ControllerMapping;
use locks::Mutex;
use once_cell::sync::Lazy;
use skyline::hooks::InlineCtx;

use super::submenu;
use super::swkbd;

pub struct ControllerInfo(*mut u8);

impl ControllerInfo {
    unsafe fn tag_len(&self) -> u32 {
        *(self.0.add(0x8) as *const u32)
    }

    unsafe fn tag_slice(&self) -> &[u16] {
        let len = self.tag_len();
        std::slice::from_raw_parts(self.0.add(0xc) as *const u16, len as usize)
    }

    unsafe fn set_tag(&mut self, new_tag: &[u16]) {
        let len = new_tag.len().min(10);
        std::slice::from_raw_parts_mut(self.0.add(0xc) as *mut u16, len)
            .copy_from_slice(&new_tag[..len]);

        if len < 10 {
            *self.0.cast::<u16>().add(0xC / 2 + len) = 0x0;
        }
        *self.0.add(0x8).cast::<u32>() = len as u32;
    }

    pub fn controls_mut(&mut self) -> &mut ControllerMapping {
        unsafe { &mut *self.0.add(0x24).cast::<ControllerMapping>() }
    }

    pub fn tag(&self) -> String {
        String::from_utf16_lossy(unsafe { self.tag_slice() })
    }
}

pub unsafe fn get_ptr_to_controls(entry: usize) -> ControllerInfo {
    let one = *((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8)
        .add(0x5314510) as *const u64);
    let two = *(one as *const u64);
    let three = ***((two + 0x58) as *const *const *const u64);

    ControllerInfo((three as *mut u8).add(entry * 0xf7d8))
}

#[derive(Debug, Copy, Clone)]
pub struct ExtraInputDetection {
    pub pressing_down: bool,
    pub pressing_up: bool,
    pub pressing_y: bool,
    pub pressing_x: bool,
}

#[derive(Default)]
struct WithCancel {
    pub submenu: Option<Box<dyn TagSubMenu>>,
    pub was_cancel_pressed_last: bool,
}

static VIRTUAL_INPUT_MAPS: Lazy<Mutex<HashMap<u64, ExtraInputDetection>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static SUBMENU_STATES: Lazy<Mutex<HashMap<u64, WithCancel>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

#[skyline::from_offset(0x19fa710)]
fn init_layout(ptr: u64);

#[skyline::from_offset(0x3777950)]
fn play_animation(layout_ptr: u64, name: *const u8, speed: f32);

#[skyline::from_offset(0x37a18c0)]
fn create_text_pane(out_pane: u64, text: *const u16, color: u32, value: i32);

pub unsafe fn get_pane_by_name(arg: u64, arg2: *const u8) -> [u64; 4] {
    let func_addr =
        (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3775F60);
    let callable: extern "C" fn(u64, *const u8, ...) -> [u64; 4] = std::mem::transmute(func_addr);
    callable(arg, arg2)
}

#[skyline::hook(offset = 0x19fba98, inline)]
unsafe fn set_layout_position(ctx: &mut InlineCtx) {
    let ptr = *ctx.registers[19].x.as_ref();

    // If we don't have a submenu state then we are just going to use the default position,
    // which should be the position for the name which the user currently has picked
    let states = SUBMENU_STATES.lock();
    let Some(state) = states.get(&ptr).and_then(|s| s.submenu.as_ref()) else {
        return;
    };

    *ctx.registers[21].x.as_mut() = state
        .get_start_index()
        .map(|index| index + 1)
        .unwrap_or_default() as u64;
}

#[skyline::hook(offset = 0x19fba0c, inline)]
unsafe fn create_layout(ctx: &mut InlineCtx) {
    let ptr = *ctx.registers[19].x.as_ref();

    // If we don't have a submenu state then we are just going to use the default button count
    let states = SUBMENU_STATES.lock();
    let Some(state) = states.get(&ptr).and_then(|s| s.submenu.as_ref()) else {
        return;
    };

    // Add one button for the title button
    *ctx.registers[1].x.as_mut() = state.get_button_count() as u64 + 1;
}

#[skyline::from_offset(0x37a1ef0)]
pub unsafe fn set_text_string(pane: u64, string: *const u8);

#[skyline::hook(offset = 0x19f6790)]
unsafe fn create_layout_button(root: u64, index: i32, button: u64) {
    // In the degenerate case that we haven't set a state yet, let's just call original
    // This will likely happen the very first time we call this function
    let states = SUBMENU_STATES.lock();
    let Some(submenu) = states.get(&root).and_then(|s| s.submenu.as_ref()) else {
        return call_original!(root, index, button);
    };

    let (is_normal, mut text) = if index == 0 {
        (false, submenu.get_title().to_vec())
    } else if let Some(text) = submenu.get_button_text(index as usize - 1) {
        (true, text)
    } else {
        (false, b"?????".to_vec())
    };

    // We don't expect the submenu to provide a null terminator, so we handle that ourselves
    text.push(b'\0');

    // TODO: Call destructor of pane handle after performing set_txt operation
    let pane = get_pane_by_name(button, b"set_txt\0".as_ptr());
    let text_pane = *((pane[1] + 0x10) as *const u64);

    // Play a different background animation depending on if the button is normal (aka a submenu
    // button) or the title
    if is_normal {
        play_animation(
            *(button as *const u64).add(1),
            b"color_bg_normal\0".as_ptr(),
            1.0,
        );
    } else {
        play_animation(
            *(button as *const u64).add(1),
            b"color_bg_normal\0".as_ptr(),
            1.0,
        );
    }

    // Finally, set the button pane text
    set_text_string(text_pane, text.as_ptr());
}

unsafe fn get_controls_id_from_button_id(root_layout: u64, button_id: i32) -> Option<usize> {
    if button_id == 0 {
        return None;
    }

    // This is an index into the buffer, each buffer is 0x400 long so this really
    // should just never be a problem to index. Imagine having more than like 60
    // tags, couldn't be smash ultimate
    let on_css_tag_index = *((root_layout + 0x28) as *const u64) + (button_id - 1) as u64;
    let buffer_idx = on_css_tag_index / 0x400;
    let el_idx = on_css_tag_index % 0x400;
    let buffer_list_start = *((root_layout + 0x10) as *const u64);
    let buffer = *(buffer_list_start as *const u64).add(buffer_idx as usize);
    let index = *(buffer as *const i32).add(el_idx as usize);

    Some(index as usize)
}

#[skyline::hook(offset = 0x19f9b98, inline)]
unsafe fn check_virtual_inputs(ctx: &mut InlineCtx) {
    let ptr = *ctx.registers[8].x.as_ref();
    let virt = VIRTUAL_INPUT_MAPS.lock().get(&ptr).copied();

    let Some(virt) = virt else {
        return;
    };

    let mut states = SUBMENU_STATES.lock();

    let root_layout = *ctx.registers[19].x.as_ref();

    let state = states.entry(root_layout).or_default();
    let currently_pressing = *(ptr as *const i32).add(0x2a0 / 4);

    if state.was_cancel_pressed_last {
        state.was_cancel_pressed_last = *ctx.registers[9].x.as_ref() != 0;
        *ctx.registers[9].x.as_mut() = 0;
    }

    let reinitialize = match &mut state.submenu {
        submenu @ None => {
            if let Some(controls_id) =
                get_controls_id_from_button_id(root_layout, currently_pressing)
            {
                if virt.pressing_y {
                    *submenu = Some(Box::new(TopLevel { controls_id }));
                    true
                } else if virt.pressing_x {
                    let mut controls = get_ptr_to_controls(controls_id as usize);
                    let new_name = swkbd::prompt_change_text(controls.tag_slice());
                    controls.set_tag(&new_name);
                    *(ptr as *mut i32).add(0xd0 / 4) = controls_id as i32;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
        submenu @ Some(_) => {
            let unwrapped = submenu.take().unwrap_unchecked();
            let picked = *((ptr + 0x2b4) as *const i32);
            *((ptr + 0x2b4) as *mut i32) = -1;
            if *ctx.registers[9].x.as_ref() != 0 {
                *submenu = unwrapped.cancel();
                *ctx.registers[9].x.as_mut() = 0x0;
                state.was_cancel_pressed_last = true;
                true
            } else if picked < 0 {
                *submenu = Some(unwrapped);
                false
            } else if picked > 0 {
                *submenu = unwrapped.decide(picked as usize - 1);
                true
            } else {
                *submenu = Some(unwrapped);
                false
            }
        }
    };

    drop(states);

    if reinitialize {
        init_layout(root_layout);
    }
}

unsafe fn check_for_input(mask: u32, ptr: u64) -> bool {
    for index in 0..51 {
        let value = *(ptr as *const u32).add(index);
        if value == mask {
            if *(ptr as *const u32).add(index + 0x330 / 4) != 0 {
                return true;
            }
        }
    }

    false
}

#[skyline::hook(offset = 0x377ce90, inline)]
unsafe fn handle_virtual_inputs(ctx: &InlineCtx) {
    let ptr = *ctx.registers[0].x.as_ref();
    let virtual_input = *(*ctx.registers[1].x.as_ref() as *const u64).add(1);
    let extra = ExtraInputDetection {
        pressing_down: check_for_input(0x00400004, virtual_input),
        pressing_up: check_for_input(0x00100001, virtual_input),
        pressing_y: check_for_input(0x00000080, virtual_input),
        pressing_x: check_for_input(0x00000010, virtual_input),
    };

    VIRTUAL_INPUT_MAPS.lock().insert(ptr, extra);
}

/* 0x19f5cb0 -> Called where x1 is the index of the pane to render
 *
 */

/*
 * 0x19f5cb0 -> Sets up the UI colors
 * 0x19faf40 -> required to control the UI
 * 0x19f94b4 -> sets the index to the newly selected one
 * 0x377d204 -> proc'd when pressing A
 * 0x3762eac -> proc'd when activating virtual input
 */

/*
 * To check if virtual input is being pressed, do this:
 * - Get virtual input
 * - Iterate over `ButtonBitfields` from 0x00 - 0xCC (0x51 of them)
 * - If any of them include the button you are checking for, add 0x330 to your offset and check if
 * that is 0x01
 */

pub fn install() {
    skyline::install_hooks!(
        check_virtual_inputs,
        handle_virtual_inputs,
        create_layout,
        create_layout_button,
        set_layout_position,
    );
}
