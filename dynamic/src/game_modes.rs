#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CustomMode {
    SmashballTag,
    TurboMode
}

extern "Rust" {
    #[link_name = "hdr__game_modes__is_custom_mode"]
    fn _is_custom_mode() -> bool;

    #[link_name = "hdr__game_modes__get_custom_mode"]
    fn _get_custom_mode() -> Option<CustomMode>;

    #[link_name = "hdr__game_modes__signal_new_game"]
    fn _signal_new_game();
}

pub fn is_custom_mode() -> bool {
    unsafe {
        _is_custom_mode()
    }
}

pub fn get_custom_mode() -> Option<CustomMode> {
    unsafe {
        _get_custom_mode()
    }
}

pub fn signal_new_game() {
    unsafe {
        _signal_new_game()
    }
}