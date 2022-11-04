use skyline::hooks::InlineCtx;

#[skyline::from_offset(0x37a1270)]
unsafe fn set_text_string(pane: u64, string: *const u8);

unsafe fn get_pane_by_name(arg: u64, arg2: *const u8) -> [u64; 4] {
    let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x37752e0);
    let callable: extern "C" fn(u64, *const u8, ...) -> [u64; 4] = std::mem::transmute(func_addr);
    callable(arg, arg2)
}

unsafe fn set_room_text(arg: u64, string: String) {
    let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3778c50);
    let callable: extern "C" fn(u64, *const u8, usize, *const u16, ...) = std::mem::transmute(func_addr);
    callable(arg, b"mnu_online_room_inside_room_id\0".as_ptr(), 1, string.encode_utf16().collect::<Vec<u16>>().as_ptr())
}

static mut CURRENT_PANE_HANDLE: usize = 0;
static mut CURRENT_ARENA_ID: String = String::new();
static mut CURRENT_INPUT_BUFFER: isize = 4;
static mut MOST_RECENT_AUTO: isize = -1;

const MAX_INPUT_BUFFER: isize = 25;
const MIN_INPUT_BUFFER: isize = -1;

#[skyline::hook(offset = 0x1887700, inline)]
unsafe fn update_room_hook(_: &skyline::hooks::InlineCtx) {
    static mut CURRENT_COUNTER: usize = 0;
    if ninput::any::is_press(ninput::Buttons::RIGHT) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER += 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
    } else if ninput::any::is_press(ninput::Buttons::LEFT) {
        if CURRENT_COUNTER == 0 {
            CURRENT_INPUT_BUFFER -= 1;
        }
        CURRENT_COUNTER = (CURRENT_COUNTER + 1) % 10;
    } else {
        CURRENT_COUNTER = 0;
    }

    CURRENT_INPUT_BUFFER = CURRENT_INPUT_BUFFER.clamp(MIN_INPUT_BUFFER, MAX_INPUT_BUFFER);
    if CURRENT_INPUT_BUFFER == -1 {
        if MOST_RECENT_AUTO == -1 {
            set_text_string(
                CURRENT_PANE_HANDLE as u64,
                format!("ROOM ID: {}\nInput Latency: Auto", CURRENT_ARENA_ID).as_ptr(),
            );
        } else {
            set_text_string(
                CURRENT_PANE_HANDLE as u64,
                format!("ROOM ID: {}\nInput Latency: Auto ({})", CURRENT_ARENA_ID, MOST_RECENT_AUTO).as_ptr()
            )
        }
    } else {
        set_text_string(CURRENT_PANE_HANDLE as u64, format!("{}\nInput Latency: {}\0", CURRENT_ARENA_ID, CURRENT_INPUT_BUFFER).as_ptr());
    }
}

#[skyline::hook(offset = 0x188702c, inline)]
unsafe fn set_room_id(ctx: &skyline::hooks::InlineCtx) {
    let panel = *((*((*ctx.registers[0].x.as_ref() + 8) as *const u64) + 0x10) as *const u64);
    CURRENT_PANE_HANDLE = panel as usize;
    CURRENT_ARENA_ID = dbg!(String::from_utf16(std::slice::from_raw_parts(*ctx.registers[3].x.as_ref() as *const u16, 5)).unwrap());
}

static mut PANE: u64 = 0;

#[skyline::hook(offset = 0x1a12460)]
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
    set_text_string(*((*((arg + 0xe58) as *const u64) + 0x10) as *const u64), format!("Input Latency: {}\0", CURRENT_INPUT_BUFFER).as_ptr());
    set_text_string(*((*((arg + 0xe68) as *const u64) + 0x10) as *const u64), format!("Input Latency: {}\0", CURRENT_INPUT_BUFFER).as_ptr());
    call_original!(arg)
}

#[skyline::hook(offset = 0x16cdb08, inline)]
unsafe fn set_online_latency(ctx: &InlineCtx) {
    let auto = *(*ctx.registers[19].x.as_ref() as *mut u8);
    MOST_RECENT_AUTO = auto as isize;
    if CURRENT_INPUT_BUFFER == -1 {
        *(*ctx.registers[19].x.as_ref() as *mut u8) = CURRENT_INPUT_BUFFER as u8;
    }
}

pub fn install() {
    skyline::install_hooks!(set_room_id, update_room_hook, set_online_latency);
}