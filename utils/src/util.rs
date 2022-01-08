use smash::app::BattleObject;
use crate::offsets;

#[macro_export]
macro_rules! dump_trace {
    () => {{
        let text = ::skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        ::utils::dump_trace!(text)
    }};
    ($base:expr) => {{
        const MAXIMUM_BT_LEN: usize = 0x20;
        let text = $base;
        println!("Current text address: {:#x}", text);

        let mut lr: *const u64;
        unsafe {
            asm!("mov {}, x30", out(reg) lr);
        }

        let mut fp: *const u64;
        unsafe {
            asm!("mov {}, x29", out(reg) fp);
        }

        println!("Current LR:\t\t{:#x} ({:#x})", (lr as u64) - text, (lr as u64));

        let mut counter = 0usize;
        while !fp.is_null() && counter < MAXIMUM_BT_LEN {
            lr = *fp.offset(1) as *const u64;
            if !lr.is_null() {
                println!("[{}]: {:#x} ({:#x})", counter, (lr as u64), (lr as u64) - text);
                counter += 1;
            }
            fp = *fp as *const u64;
        }
    }}
}

#[macro_export]
macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr()
    }
}

pub fn byte_search<T: Eq>(needle: &[T]) -> Option<usize> {   
    let text = unsafe {
        let start = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const T;
        let end = skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as *const T;
        let length = end.offset_from(start) as usize;
        std::slice::from_raw_parts(start, length)
    };

    text.windows(needle.len()).position(|window| window == needle)
}

pub fn byte_search_rodata<T: Eq>(needle: &[T]) -> Option<usize> {
    const RODATA_LEN: usize = 0xCC8C9B;
    let (rodata, text_len) = unsafe {
        let start = skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as *const T;
        let end = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Rodata) as usize + RODATA_LEN) as *const T;
        let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const T;
        let length = end.offset_from(start) as usize;
        (std::slice::from_raw_parts(start, length), start.offset_from(text) as usize)
    };

    rodata.windows(needle.len()).position(|window| window == needle).map(|x| x + text_len)
}

pub fn offset_to_addr<T>(offset: usize) -> *const T {
    unsafe {
        (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8).add(offset) as _
    }
}

#[skyline::from_offset(offsets::get_battle_object_from_id())]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;