use std::{
    collections::HashMap,
    ffi::c_char,
    fs::*,
    path::{ Path, PathBuf },
    sync::{ LazyLock, RwLock, atomic::Ordering }
};
use skyline::hooks::InlineCtx;
use smash2::{
    cpp::Vector,
    phx::hash40
};
use serde::Deserialize;
use utils::modules::TourneyConfig;
use utils::consts::{melee_mode, smash_mode};
use crate::CSS_FIRST;

mod layout;
mod random;
mod player_port;

pub const KEY_MASK: u64 = 0xFFFFFF_0000000000;

pub static mut CHARA_DATA: LazyLock<RwLock<CharaData>> = LazyLock::new(|| 
    RwLock::new(CharaData::default())
);

#[derive(Debug, Clone)]
pub struct CharaData {
    main_id: u64,
    sub_id: u64,
    last_selection: String,
    melee_random: bool,
    costume: u8,
    costume_rng: Vec<i32>,

    whitelist: Vec<String>,
    blacklist: Vec<String>
}
impl Default for CharaData {
    fn default() -> Self {
        CharaData {
            main_id: 0x0,
            sub_id: 0x0,
            last_selection: String::new(),
            melee_random: false,
            costume: 0,
            costume_rng: (0..8).collect::<Vec<i32>>(),

            whitelist: Vec::new(),
            blacklist: Vec::new()
        }
    }
}

// this structure is gross and largely undefined but it holds some useful information about the CSS instance
// not all field labels are accurate
#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct CharaSelect {
    addr: *const u64,
    some_node: *const u64,
    unk1: [u64; 3],
    unk_structs: [[u64; 2]; 12],
    _0xe8: u32,
    _0xec: u32,
    _0xf0: u32,
    _0xf4: u32,
    _0xf8: u64,
    _ptr_100: *const u64, // layout related
    _pad1: [u64; 6],
    _0x138: u32,
    frames_elapsed: i32,
    loading_state: u32,
    _0x144: u32,
    _ptr_148: *const u64,
    _ptr_150: *const u64,
    unk_bytes: [u8; 8],
    current_player_count: u32, //union
    css_mode: u32, //union
    _0x168: u8,
    is_team_battle: bool,
    _0x16a: u8,
    _0x16b: u8,
    game_mode: u32,
    local_wireless: u32, // 1 in local wireless, otherwise may be unrelated
    ready_state: u32,
    _0x178: u32,
    min_players_allowed: u32, // aka min # of ui panes
    max_players_allowed: u32,
    _0x184: u32,
    _0x188: u64,
    player_buffer: *const u64,
    player_root: *const u64,
    _ptr_1a0: *const u64,
    players: [[u64;2]; 8], // not researched enough
    _pad2: [u64; 2],
    first_player: *const PlayerInfo,
    max_allowed_player: *const PlayerInfo,
    _ptr_248: *const u64,
    player_base: *const PlayerInfo,
    player_max: *const PlayerInfo,
    _ptr_260: *const u64,
    card_array_start: *const u64,
    card_array_end: *const u64,
    // theres way more here :)
}

#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct PlayerInfo {
    root: *const u64,
    card: *const PlayerCard,
    next: *const PlayerInfo
}

// much like the other struct, largely undefined and potentially inaccurate
#[repr(C, align(8))]
#[derive(Debug, Copy, Clone)]
pub struct PlayerCard {
    _0x0: u64,
    parts: *const u64,
    layout: *const u64,
    _0x18: u64,
    css_instance: *const CharaSelect,
    active_slot_id: u32,
    current_state: u8,
    target_state: u8,
    bool_2e: bool,
    _0x2f: u8,
    index: u32,
    _0x34: u8,
    is_visible: bool,
    is_active: bool,
    root_pane: *const u64,
    _unk_range_40: [u64; 34],
    max_card_count: u32,
    _0x154: u32,
    _unk_range_158: [u64; 13],
    bool_1c0: bool,
    bool_1c1: bool,
    bool_1c2: bool,
    bool_1c3: bool,
    bool_1c4: bool,
    bool_1c5: bool,
    bool_1c6: bool,
    bool_1c7: bool,
    _0x1c8: u64,
    current_id: u16,
    id_1: u16,
    id_2: u16,
    id_3: u16,
    id_4: u16,
    id_5: u16,
    id_6: u16,
    id_7: u16,
    id_8: u16,
    id_9: u16,
    id_10: u16,
    _0x1e8: u64,
    player_num: u32,
    _0x1f4: u32,
    player_kind: i32, // 0 = player, 1 = cpu, 2 = amiibo, 3 = none
    _0x1fc: u32,
    _0x200: u64,
    _0x208: u64,
    _0x210: u8,
    bool_211: bool,
    _0x212: u8,
    _0x213: u8,
    _0x214: u16,
    _0x216: u16,
    _0x218: u32,
    card_type: u32,
    team_id: u32,
    _0x224: u32,
    _0x228: u64,
    _0x230: u64,
    _0x238: u64,
    max_card_count2: u32,
    layout_variant: u32,
    _0x248: u64,
    _unk_range_250: [u64; 40],
    controller_id: u32,
    some_state: u32,
    // theres more here
}

pub fn is_tourney_mode() -> bool {
    match TourneyConfig::load() {
        Some(config) => config.enabled,
        None => false
    }
}

pub static mut PLAYER_TAG_INDEX: &'static mut [u8] = &mut [0; 8];

// big thanks to azel-s for this tag parsing code ^^
pub unsafe fn get_tag_from_save(idx: u8) -> String {
    let tag_address =
        (***(((*((*((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8)
            .add(0x5313510) as *const u64)) as *const u64))
            + 0x58) as *const *const *const u64)
            + ((idx as u64) * 0xF7D8)
            + 0xC) as *const u16;

    let mut tag_length = 0;
    while *tag_address.add(tag_length) != 0 {
        tag_length += 1;
    }

    String::from_utf16_lossy(std::slice::from_raw_parts(tag_address, tag_length))
}

#[skyline::hook(offset = 0x19fd0b0)]
unsafe fn update_player_tag(arg1: u64, tag_index: *const u8) {
    let player_id = *((arg1 + 0x1d4) as *const u8) as usize;
    if (0..8).contains(&player_id) {
        PLAYER_TAG_INDEX[player_id] = *tag_index;
    }
    call_original!(arg1, tag_index);
}

#[skyline::hook(offset = 0x1a2d440, inline)]
unsafe fn css_advance_sfx_hook(ctx: &mut skyline::hooks::InlineCtx) {
    // 0x18d72a665a = hash40("se_system_amiibo_write_2") // original sound
    // 0x13d3b19adc = hash40("se_system_r2f_fixed") // original sound
    let param_1 = ctx.registers[0].x() as *mut u32;
    let sfx = if CSS_FIRST { 0x18d72a665a as u64 } else { 0x13d3b19adc as u64 };
    play_se(param_1, sfx);
}

#[skyline::hook(offset = 0x1a2d594, inline)]
unsafe fn css_advance_sfx2_hook(ctx: &mut skyline::hooks::InlineCtx) {
    if !CSS_FIRST {
        // 0x17a3061361 = hash40("se_audience_suddendeath")
        let sfx = 0x17a3061361 as u64;
        let param_1 = ctx.registers[0].x() as *mut u32;
        play_se(param_1, sfx);
    }
}

#[skyline::from_offset(0x2407280)]
unsafe fn play_se(
    param_1: *mut u32,
    sfx_hash_id: u64);

// Tells any callers to this function that no echos are available
#[skyline::hook(offset = 0x1a1fa30)]
unsafe fn echo_swap_hook(
    _param_1: i32, _param_2: u64, _param_3: u64, _param_4: u64,
    _param_5: u64, _param_6: u64, _param_7: u64, _param_8: u64
) -> u64 {
    1
}

// Kills the "Rules" button on the CSS when the CSS is first because it
// actually goes all the way to the main menu
#[skyline::hook(offset = 0x3771220)]
unsafe fn register_panel_button(
    panel: *mut u64,
    event_code: i32,
    name: *const c_char,
    arg4: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
) {
    if CSS_FIRST && !name.is_null() && skyline::from_c_str(name as *const u8) == "set_btn_03_rule" {
        return;
    }
    call_original!(panel, event_code, name, arg4, arg5, arg6, arg7, arg8);
}

#[skyline::hook(offset = 0x1a2fecc, inline)]
fn override_min_players(ctx: &mut InlineCtx) {
    let param_1 = ctx.registers[24].x() as *const u8;
    let game_mode = unsafe { *(param_1.add(0x16c) as *const u32) } as i32;
    let ruleset = unsafe { *(param_1.add(0x158) as *const u8) } as i32;

    // Set rule type for 1P mode
    utils::one_player::IS_RULE_TIME.store(ruleset == smash_mode::TIME, Ordering::Relaxed);

    // set minimum required "ready" players to 1
    if game_mode == melee_mode::SMASH {
        ctx.registers[11].set_w(1);
    }
}

pub fn install() {
    skyline::install_hooks!(
        update_player_tag,
        css_advance_sfx_hook,
        css_advance_sfx2_hook,
        echo_swap_hook,
        register_panel_button,
        override_min_players,
    );

    // Prevent the game from playing any CSS advance sound effects by default
    skyline::patching::Patch::in_text(0x1a2d43c).nop();
    skyline::patching::Patch::in_text(0x1a2d590).nop();


    layout::install();
    random::install();
    player_port::install();

    // These patches are required to "undo" a stacked CSS

    // 1. Force the CSS to always use the "separate" fighter list instead of "stacked".
    // This fixes the echo portraits on the character cards.
    skyline::patching::Patch::in_text(0x1a20260).data(0x52800028u32);

    // 2. Force the singleton character vector builder (0x1a0a3e0) to always store separate=1
    // into inner_data+0x258. This fixes the miis.
    skyline::patching::Patch::in_text(0x1a0a410).data(0x52800028u32);
}