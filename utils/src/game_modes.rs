use skyline_web::Webpage;
use utils_dyn::game_modes::CustomMode;
use crate::offsets;
use std::collections::HashSet;
use std::str::FromStr;

pub mod tag;
pub mod turbo;
pub mod hitfall;
pub mod airdash;

lazy_static! {
    static ref GAME_MODE_HTML: Vec<u8> = std::fs::read("mods:/ui/docs/gamemodes.html").unwrap();
    static ref GAME_MODE_JS: Vec<u8> = std::fs::read("mods:/ui/docs/gamemodes.js").unwrap();
    static ref TAG_WEBP: Vec<u8> = std::fs::read("mods:/ui/docs/tag.webp").unwrap();
    static ref TURBO_WEBP: Vec<u8> = std::fs::read("mods:/ui/docs/turbo.webp").unwrap();
    static ref HITFALL_WEBP: Vec<u8> = std::fs::read("mods:/ui/docs/hitfall.webp").unwrap();
    static ref AIRDASH_WEBP: Vec<u8> = std::fs::read("mods:/ui/docs/airdash.webp").unwrap();
    static ref SMASH64_WEBP: Vec<u8> = std::fs::read("mods:/ui/docs/smash64.webp").unwrap();
}

static mut CURRENT_CUSTOM_MODES: Option<HashSet<CustomMode>> = None;
static mut IS_PENDING_NEW_GAME: bool = false;

#[export_name = "hdr__game_modes__is_custom_mode"]
pub extern "Rust" fn is_custom_mode() -> bool {
    unsafe {
        CURRENT_CUSTOM_MODES.is_some()
    }
}

#[export_name = "hdr__game_modes__get_custom_mode"]
pub extern "Rust" fn get_custom_mode() -> Option<HashSet<CustomMode>> {
    unsafe {
        CURRENT_CUSTOM_MODES.clone()
    }
}

#[export_name = "hdr__game_modes__signal_new_game"]
pub extern "Rust" fn signal_new_game() {
    unsafe {
        IS_PENDING_NEW_GAME = true;
    }
}

fn detect_new_game(game_state_ptr: u64) -> bool {
    static mut PREVIOUS_GAME_STATE_PTR: u64 = 0;

    unsafe {
        let prev = PREVIOUS_GAME_STATE_PTR;
        PREVIOUS_GAME_STATE_PTR = game_state_ptr;

        let pending = IS_PENDING_NEW_GAME;
        IS_PENDING_NEW_GAME = false;

        prev != game_state_ptr || pending
    }

}

#[skyline::hook(offset = offsets::on_rule_select(), inline)]
unsafe fn on_rule_select_hook(_: &skyline::hooks::InlineCtx) {
    unsafe { // Ryujinx handle separately
        let text_addr = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
        if text_addr == 0x8004000 {
            if ninput::any::is_down(ninput::Buttons::R) && ninput::any::is_down(ninput::Buttons::L) {
                let mut modes = HashSet::new();
                modes.insert(CustomMode::SmashballTag);
                CURRENT_CUSTOM_MODES = Some(modes);
            } else {
                CURRENT_CUSTOM_MODES = None;
            }
            return;
        }
    }

    if !ninput::any::is_down(ninput::Buttons::R) {
        CURRENT_CUSTOM_MODES = None;
        return;
    }

    open_modes_session();
}

pub unsafe fn open_modes_session() {
    let response = Webpage::new()
        .htdocs_dir("contents")
        .file("help/html/USen/gamemodes.html", GAME_MODE_HTML.as_slice())
        .file("hdr/gamemodes.js", GAME_MODE_JS.as_slice())
        .file("hdr/tag.webp", TAG_WEBP.as_slice())
        .file("hdr/turbo.webp", TURBO_WEBP.as_slice())
        .file("hdr/hitfall.webp", HITFALL_WEBP.as_slice())
        .file("hdr/airdash.webp", AIRDASH_WEBP.as_slice())
        .file("hdr/smash64.webp", SMASH64_WEBP.as_slice())
        .start_page("help/html/USen/gamemodes.html")
        .open()
        .unwrap();

    match response.get_last_url() {
        Ok(url) => {
            let modes_str = url.trim_start_matches("http://localhost/");
            // if no modes were selected, then set None
            if modes_str.is_empty() || modes_str.contains("none") {
                CURRENT_CUSTOM_MODES = None;
                return;
            }

            let mut modes_enabled = HashSet::new();
            let modes_list = modes_str.split("-");
            modes_list.for_each(|mode| {
                modes_enabled.insert(CustomMode::from_str(mode).unwrap());
            });
            CURRENT_CUSTOM_MODES = Some(modes_enabled);
        },
        Err(e) => {
            println!("Error getting modes: {:?}", e);
            CURRENT_CUSTOM_MODES = None;
        }
    }
}

#[skyline::hook(offset = offsets::once_per_game_frame())]
unsafe fn once_per_game_frame(game_state_ptr: u64) {

    // check the current match mode
    // 1 is regular smash, 45 is online arena match
    if utils_dyn::util::get_match_mode().0 != 1 && utils_dyn::util::get_match_mode().0 != 45 {
        //println!("mode is {}, so not running custom game modes.", utils_dyn::util::get_match_mode().0);
        CURRENT_CUSTOM_MODES = None;
    }

    if detect_new_game(game_state_ptr) {
        match get_custom_mode() {
            Some(modes) => {
                if modes.contains(&CustomMode::SmashballTag) {
                    tag::clear();
                }
            },
            _ => {}
        }
    }

    match get_custom_mode() {
        Some(modes) => {
            if modes.contains(&CustomMode::SmashballTag) {
                tag::update();
            }
            if modes.contains(&CustomMode::TurboMode) {
                turbo::update();
            }
            if modes.contains(&CustomMode::HitfallMode) {
                hitfall::update();
            }
            if modes.contains(&CustomMode::AirdashMode) {
                airdash::update();
            }
        },
        _ => {}
    }

    call_original!(game_state_ptr)
}

pub fn install() {
    skyline::install_hooks!(on_rule_select_hook, once_per_game_frame);
}