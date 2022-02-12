#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CustomMode {
    SmashballTag
}

extern "Rust" {
    #[link_name = "hdr__game_modes__is_custom_mode"]
    fn _is_custom_mode() -> bool;
}

pub fn is_custom_mode() -> bool {
    unsafe {
        _is_custom_mode()
    }
}