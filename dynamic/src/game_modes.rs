use std::str::FromStr;
use std::collections::HashSet;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum CustomMode {
    SmashballTag = 0,
    TurboMode = 1,
    HitfallMode = 2,
    AirdashMode = 3,
}

impl fmt::Display for CustomMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomMode::SmashballTag => write!(f, "Tag"),
            CustomMode::TurboMode => write!(f, "Turbo"),
            CustomMode::HitfallMode => write!(f, "Hitfall"),
            CustomMode::AirdashMode => write!(f, "Airdash"),
        }
    }
}

impl FromStr for CustomMode {

    type Err = ();

    fn from_str(input: &str) -> Result<CustomMode, Self::Err> {
        match input {
            "tag"  => Ok(CustomMode::SmashballTag),
            "turbo"  => Ok(CustomMode::TurboMode),
            "hitfall"  => Ok(CustomMode::HitfallMode),
            "airdash" => Ok(CustomMode::AirdashMode),
            _      => Err(()),
        }
    }
}

extern "Rust" {
    #[link_name = "hdr__game_modes__is_custom_mode"]
    fn _is_custom_mode() -> bool;

    #[link_name = "hdr__game_modes__get_custom_mode"]
    fn _get_custom_mode() -> Option<HashSet<CustomMode>>;

    #[link_name = "hdr__game_modes__signal_new_game"]
    fn _signal_new_game();
}

pub fn is_custom_mode() -> bool {
    unsafe {
        _is_custom_mode()
    }
}

pub fn get_custom_mode() -> Option<HashSet<CustomMode>> {
    unsafe {
        _get_custom_mode()
    }
}

pub fn signal_new_game() {
    unsafe {
        _signal_new_game()
    }
}