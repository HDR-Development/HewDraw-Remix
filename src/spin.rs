use skyline::hooks::InlineCtx;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

const DISPLAY_PATH: &str = "sd:/ultimate/mods/hdr/hdr_limiter.txt";

unsafe fn calc_offset() -> u64 {
    let mut symbol = 0usize;
    skyline::nn::ro::LookupSymbol(&mut symbol, b"_ZN7android7IBinderD1Ev\0".as_ptr());
    (symbol - 0x240) as u64
}

static mut DUMMY_BLOCK: [u8; 0x100] = [0; 0x100];

#[skyline::hook(offset = 0x3747b7c, inline)]
unsafe fn hook_1(_: &skyline::hooks::InlineCtx) {
    while !RUN.swap(false, Ordering::SeqCst) {
        skyline::nn::hid::GetNpadFullKeyState(DUMMY_BLOCK.as_mut_ptr() as _, &0);
    }
}

#[skyline::hook(replace = OFFSET1)]
unsafe fn hook_2(window: u64, _: i32) {
    call_original!(window, 0);
}

#[skyline::hook(replace = OFFSET2, inline)]
unsafe fn hook_3(ctx: &mut InlineCtx) {
    ctx.registers[8].set_x(0);
}

static RUN: AtomicBool = AtomicBool::new(false);

#[skyline::hook(offset = 0x3810a64, inline)]
unsafe fn hook_4(_: &skyline::hooks::InlineCtx) {
    RUN.store(true, Ordering::SeqCst);
}

static mut OFFSET1: u64 = 0;
static mut OFFSET2: u64 = 0;

fn display_bytes() -> Vec<u8> {
    fs::read(DISPLAY_PATH).unwrap_or_default()
}

#[repr(C)]
pub struct RootData64 {
    pub len: u8,
    pub data: [u8; 64],
}

fn first_64_chars(input: &[u8]) -> RootData64 {
    let mut out = [0u8; 64];
    let text = String::from_utf8_lossy(input);
    let mut used = 0usize;

    for ch in text.chars().take(64) {
        let mut buf = [0u8; 4];
        let encoded = ch.encode_utf8(&mut buf).as_bytes();
        if used + encoded.len() > out.len() {
            break;
        }
        out[used..used + encoded.len()].copy_from_slice(encoded);
        used += encoded.len();
    }

    RootData64 {
        len: used as u8,
        data: out,
    }
}

#[no_mangle]
pub unsafe extern "C" fn check_root() -> RootData64 {
    let bytes = display_bytes();
    let display_exists = Path::new(DISPLAY_PATH).is_file();
    if !display_exists && !super::is_on_ryujinx() {
        OFFSET1 = calc_offset() + 0x429d60;
        OFFSET2 = calc_offset() + 0x26e94;
        skyline::install_hooks!(hook_1, hook_2, hook_3, hook_4,);
    }

    first_64_chars(&bytes)
}
