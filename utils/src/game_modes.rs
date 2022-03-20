use skyline_web::Webpage;
use utils_dyn::game_modes::CustomMode;
use crate::offsets;

pub mod tag;

lazy_static! {
    static ref GAME_MODE_HTML: Vec<u8> = std::fs::read("mods:/ui/docs/gamemodes.html").unwrap();
    static ref GAME_MODE_JS: Vec<u8> = std::fs::read("mods:/ui/docs/gamemodes.js").unwrap();
    static ref TAG_WEBP: Vec<u8> = std::fs::read("mods:/ui/docs/tag.webp").unwrap();
}

static mut CURRENT_CUSTOM_MODE: Option<CustomMode> = None;
static mut IS_PENDING_NEW_GAME: bool = false;

#[export_name = "hdr__game_modes__is_custom_mode"]
pub extern "Rust" fn is_custom_mode() -> bool {
    unsafe {
        CURRENT_CUSTOM_MODE.is_some()
    }
}

#[export_name = "hdr__game_modes__get_custom_mode"]
pub extern "Rust" fn get_custom_mode() -> Option<CustomMode> {
    unsafe {
        CURRENT_CUSTOM_MODE
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
                CURRENT_CUSTOM_MODE = Some(CustomMode::SmashballTag);
            } else {
                CURRENT_CUSTOM_MODE = None;
            }
            return;
        }
    }

    if !ninput::any::is_down(ninput::Buttons::R) {
        CURRENT_CUSTOM_MODE = None;
        return;
    }

    let response = Webpage::new()
        .htdocs_dir("contents")
        .file("help/html/USen/gamemodes.html", GAME_MODE_HTML.as_slice())
        .file("hdr/gamemodes.js", GAME_MODE_JS.as_slice())
        .file("hdr/tag.webp", TAG_WEBP.as_slice())
        .start_page("help/html/USen/gamemodes.html")
        .open()
        .unwrap();

    match response.get_last_url() {
        Ok("http://localhost/btn-tag") => CURRENT_CUSTOM_MODE = Some(CustomMode::SmashballTag),
        _ => {}
    }
}

#[skyline::hook(offset = offsets::once_per_game_frame())]
unsafe fn once_per_game_frame(game_state_ptr: u64) {

    // check if current match mode is not regular smash, if so sub out the custom mode
    if utils_dyn::util::get_match_mode().0 != 1 {
        CURRENT_CUSTOM_MODE = None;
    }

    if detect_new_game(game_state_ptr) {
        match get_custom_mode() {
            Some(CustomMode::SmashballTag) => tag::clear(),
            _ => {}
        }
    }

    match get_custom_mode() {
        Some(CustomMode::SmashballTag) => tag::update(),
        _ => {}
    }

    call_original!(game_state_ptr)
}

pub fn install() {
    skyline::install_hooks!(on_rule_select_hook, once_per_game_frame);
}