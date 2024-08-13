use skyline::hooks::InlineCtx;
use std::fmt::Display;

#[skyline::from_offset(0x37a1ef0)]
pub unsafe fn set_text_string(pane: u64, string: *const u8);

pub unsafe fn get_pane_by_name(arg: u64, arg2: *const u8) -> [u64; 4] {
    let func_addr =
        (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3775F60);
    let callable: extern "C" fn(u64, *const u8, ...) -> [u64; 4] = std::mem::transmute(func_addr);
    callable(arg, arg2)
}

unsafe fn set_room_text(arg: u64, string: String) {
    let func_addr =
        (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3776910);
    let callable: extern "C" fn(u64, *const u8, usize, *const u16, ...) =
        std::mem::transmute(func_addr);
    callable(
        arg,
        b"mnu_online_room_inside_room_id\0".as_ptr(),
        1,
        string.encode_utf16().collect::<Vec<u16>>().as_ptr(),
    )
}

static mut CURRENT_PANE_HANDLE: usize = 0;
static mut CURRENT_ARENA_ID: String = String::new();
static mut CURRENT_INPUT_BUFFER: isize = -1;
static mut MOST_RECENT_AUTO: isize = -1;

const MAX_INPUT_BUFFER: isize = 25;
const MIN_INPUT_BUFFER: isize = -1;

unsafe fn set_info<S>(latency: S)
where
    S: Display,
{
    let modes = utils::get_custom_mode();
    // for every mode in the list, we need to add leading newlines.
    let mut modes_newlines = "".to_string();
    let modes_string = match modes {
        Some(mode_set) => mode_set
            .into_iter()
            .map(|entry| {
                modes_newlines += "\n";
                format!("{} is ON", entry)
            })
            .collect::<Vec<String>>()
            .join("\n"),
        None => "No Custom Modes".to_string(),
    };

    set_text_string(
        CURRENT_PANE_HANDLE as u64,
        format!(
            "\n\n\n\n\n\n{}
            Arena ID: {}
            Input Delay: {}
            {}
            DPAD ◄► Set Delay
            DPAD ▼ Edit Modes

            HDR Version:
            {}
            Assets {}\0",
            modes_newlines,
            CURRENT_ARENA_ID,
            latency,
            modes_string,
            crate::get_plugin_version(),
            crate::get_romfs_version()
        )
        .as_ptr(),
    );
}

#[skyline::hook(offset = 0x18881d0, inline)]
unsafe fn update_room_hook(_: &skyline::hooks::InlineCtx) {
    static mut CURRENT_COUNTER: usize = 0;
    if ninput::any::is_press(ninput::Buttons::RIGHT) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER += 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 15;
    } else if ninput::any::is_press(ninput::Buttons::LEFT) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER -= 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 15;
    } else if ninput::any::is_press(ninput::Buttons::DOWN) {
        if CURRENT_COUNTER == 0 {
            // open session
            utils::open_modes_session();
            skyline_web::dialog_ok::DialogOk::ok(
                "Please ensure that all players have the same custom modes enabled!",
            );
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 60;
    } else {
        CURRENT_COUNTER = 0;
    }

    CURRENT_INPUT_BUFFER = CURRENT_INPUT_BUFFER.clamp(MIN_INPUT_BUFFER, MAX_INPUT_BUFFER);
    if CURRENT_INPUT_BUFFER == -1 {
        if MOST_RECENT_AUTO == -1 {
            set_info("Auto".to_string());
        } else {
            set_info(format!("Auto ({})", MOST_RECENT_AUTO));
        }
    } else {
        set_info(format!("{}", CURRENT_INPUT_BUFFER));
    }
}

#[skyline::hook(offset = 0x1887afc, inline)]
unsafe fn set_room_id(ctx: &skyline::hooks::InlineCtx) {
    let panel = *((*((*ctx.registers[0].x.as_ref() + 8) as *const u64) + 0x10) as *const u64);
    CURRENT_PANE_HANDLE = panel as usize;
    CURRENT_ARENA_ID = dbg!(String::from_utf16(std::slice::from_raw_parts(
        *ctx.registers[3].x.as_ref() as *const u16,
        5
    ))
    .unwrap());
}

static mut PANE: u64 = 0;

#[skyline::hook(offset = 0x1a12f40)]
unsafe fn update_css2(arg: u64) {
    static mut CURRENT_COUNTER: usize = 0;
    if ninput::any::is_press(ninput::Buttons::X) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER += 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
    } else if ninput::any::is_press(ninput::Buttons::Y) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER -= 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
    } else {
        CURRENT_COUNTER = 0;
    }

    CURRENT_INPUT_BUFFER = CURRENT_INPUT_BUFFER.clamp(MIN_INPUT_BUFFER, MAX_INPUT_BUFFER);
    set_text_string(
        *((*((arg + 0xe58) as *const u64) + 0x10) as *const u64),
        format!("Input Latency: {}\0", CURRENT_INPUT_BUFFER).as_ptr(),
    );
    set_text_string(
        *((*((arg + 0xe68) as *const u64) + 0x10) as *const u64),
        format!("Input Latency: {}\0", CURRENT_INPUT_BUFFER).as_ptr(),
    );
    call_original!(arg)
}

#[skyline::hook(offset = 0x16ccc58, inline)]
unsafe fn set_online_latency(ctx: &InlineCtx) {
    let auto = *(*ctx.registers[19].x.as_ref() as *mut u8);
    MOST_RECENT_AUTO = auto as isize;
    if CURRENT_INPUT_BUFFER != -1 {
        *(*ctx.registers[19].x.as_ref() as *mut u8) = CURRENT_INPUT_BUFFER as u8;
    }
}

pub fn install() {
    skyline::install_hooks!(set_room_id, update_room_hook, set_online_latency);
}
