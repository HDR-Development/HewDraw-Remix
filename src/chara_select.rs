use std::{
    fmt::format,
    fs::*,
    path::{ Path, PathBuf },
    sync::Mutex
};
use skyline::hooks::InlineCtx;
use smash2::{
    cpp::Vector,
    phx::hash40
};
use rand::{
    prelude::SliceRandom,
    Rng
};
use toml::Value;

use utils::modules::TourneyConfig;

lazy_static! {
    static ref CHARA_WHITELIST: Mutex<Vec<String>> = {
        let mut m = Vec::new();
        Mutex::new(m)
    };
    static ref CHARA_BLACKLIST: Mutex<Vec<String>> = {
        let mut m = Vec::new();
        Mutex::new(m)
    };
    static ref COSTUME_RNG: Mutex<Vec<i32>> = {
        let mut m = Vec::new();
        Mutex::new(m)
    };
}

static mut PLAYER_TAG_INDEX: &'static mut [u8] = &mut [0; 8];

static mut MAIN_FIGHTER_ID: u64 = 0x0;
static mut SUB_FIGHTER_ID: u64 = 0x0;
static mut LAST_FIGHTER_PICKED: String = String::new();

static mut IS_MELEE_RANDOM: bool = false;

const ORDER_TOML: &str = "ui/param/menu/chara_icon_order.toml";
const RANDOM_IDX_TOML: &str = "ui/param/menu/chara_random_idx.toml";
const RANDOM_CFG_TOML: &str = "ui/param/menu/chara_random_config.toml";

// transmutes the supplied chara string into data for the CSS icon vector
fn ui_chara(i: usize, chara: &str) -> u64 {
    (0xc1u64 << 56)
    | (((i as u64) & 0xFFFF) << 40)
    | hash40(&format!("ui_chara_{}", chara)).0
}

fn is_tourney_mode() -> bool {
    match TourneyConfig::load() {
        Some(config) => config.enabled,
        None => false
    }
}

// hardcoded order of fighters in the CSS for when tourney mode is enabled
const DEFAULT_ORDER: [&str; 86] = [
    "mario", "mariod", "luigi", "peach", "daisy", "rosetta", "koopa", "koopajr", "packun", "yoshi", "wario", "donkey", "diddy", "krool", "buddy", "ice_climber", "gamewatch",
    "link", "younglink", "toonlink", "zelda", "sheik", "ganon", "samus", "szerosuit", "ridley", "samusd", "kirby", "metaknight", "dedede", "fox", "falco", "wolf", "robot", "duckhunt",
    "pikachu", "pichu", "ptrainer", "purin", "mewtwo", "lucario", "gekkouga", "gaogaen", "marth", "roy", "ike", "reflet", "chrom", "lucina", "kamui", "master",
    "ness", "lucas", "captain", "pit", "pitb", "palutena", "pikmin", "murabito", "shizue", "wiifit", "littlemac", "shulk", "element", "inkling", "tantan", "miifighter", "miiswordsman", "miigunner",
    "snake", "simon", "richter", "sonic", "bayonetta", "jack", "rockman", "ryu", "ken", "dolly", "demon", "pacman", "cloud", "edge", "trail", "brave", "pickel"
];

#[skyline::hook(offset = 0x19eb840, inline)]
pub unsafe fn display_css_hook(ctx: &InlineCtx) {
    // populate costume slot rng
    let mut costume_rng = COSTUME_RNG.lock().unwrap();
    costume_rng.clear();
    for i in 0..8 {
        costume_rng.push(i);
    }

    // obtains the original vector of fighter entries to be loaded
    let chara_vec = &mut *(*ctx.registers[4].x.as_ref() as *mut smash2::cpp::Vector<u64>);

    // collect data from TOML
    let path = Path::new("mods:/").join(ORDER_TOML);
    if !path.exists() || !path.is_file() { return };
    let data = match std::fs::read_to_string(&path).unwrap().parse::<Value>() {
        Ok(result) => result,
        Err(_) => { 
            println!("[src::chara_select] Invalid TOML data!");
            return; 
        }
    };
    let config = match data.as_table().unwrap().get("config") {
        Some(table) => table.as_table().unwrap(),
        None => {
            println!("[src::chara_select] File does not contain header [config]!");
            return;
        }
    };
    let enabled = match config.get("enabled") {
        Some(value) => value.as_bool().unwrap(),
        None => {
            println!("[src::chara_select] Config does not contain key [enabled]!");
            return;
        }
    };
    if !enabled { return }; // aborts operation if the config set this to false
    let order = match config.get("order") {
        Some(value) => value.as_str().unwrap(),
        None => {
            println!("[src::chara_select] Config does not contain key [order]!");
            return;
        }
    };
    let schema = match data.as_table().unwrap().get(order) {
        Some(table) => table.as_table().unwrap(),
        None => {
            println!("[src::chara_select] TOML does not contain header [schema]!");
            return;
        }
    };
    if !schema.contains_key("order") || !schema.contains_key("centered_random") {
        println!("[src::chara_select] Invalid schema format!");
        return;
    }
    let chara_order = 
        if is_tourney_mode() { &DEFAULT_ORDER.map(|x| toml::Value::String(x.to_string())).to_vec() }
        else { schema.get("order").unwrap().as_array().unwrap() };
    let center_random = schema.get("centered_random").unwrap().as_bool().unwrap_or(true);

    let mut chara_whitelist = CHARA_WHITELIST.lock().unwrap();
    let mut chara_blacklist = CHARA_BLACKLIST.lock().unwrap();
    chara_whitelist.clear();
    chara_blacklist.clear();

    for idx in 0..chara_order.len() {
        let mut chara = chara_order[idx].as_str().unwrap_or("goku");
        if chara == "element" { chara = "flame_first" };
        // check if this fighter is supposed to be loaded
        let mut should_load = false;
        for i in 0..chara_vec.len() {
            let raw = format!("{:#x}", chara_vec.get(i).unwrap());
            let hash = u64::from_str_radix(&raw[8..], 16).unwrap_or(0); // ui_chara index hash in u64
            if hash == hash40(&format!("ui_chara_{}", chara)).0 {
                should_load = true;
                break;
            }
        }
        if should_load {
            // println!("Fighter {chara} will be loaded");
            if chara == "flame_first" { chara = "element" };
            chara_whitelist.push(chara.to_string());
        } else {
            // println!("Fighter {chara} will not be loaded");
            chara_blacklist.push(chara.to_string());
        }
    }

    let r_offset = center_random as i32 as usize;
    let mut icon_count = chara_order.len() + r_offset; // +1 to the order if random is to be inserted
    icon_count -= chara_blacklist.len(); // subtract blacklisted fighters from the total

    // determine where the random icon should be placed in the order
    let mut random_idx = (icon_count / 2) as usize; // default placement
    let r_path = Path::new("mods:/").join(RANDOM_IDX_TOML);
    if r_path.exists() && r_path.is_file() {
        let r_data = match std::fs::read_to_string(&r_path).unwrap().parse::<Value>() {
            Ok(result) => result,
            Err(_) => { return; }
        };
        let placement = match r_data.as_table().unwrap().get("placement") {
            Some(table) => table.as_table().unwrap(),
            None => { return; }
        };
        match placement.get(&icon_count.to_string()) {
            Some(value) => { random_idx = value.as_integer().unwrap() as usize },
            None => {}
        };
    }
    // println!("{icon_count} icons to load ({} out of {} blacklisted). Random will be placed in slot {random_idx}", chara_blacklist.len(), chara_order.len() + center_random as i32 as usize);

    // construct the new order
    let new_order: &mut Vec<u64> = &mut Vec::new();
    let mut push = false; // whether to shift to compensate the inserted random entry
    let use_general_all = match chara_vec.get(chara_vec.len() - 1) {
        Some(entry) => (*entry & !KEY_MASK) == hash40("ui_chara_general_all").0,
        None => false
    };
    let mut idx: usize = 0; // handler for the custom order. this is needed to crosscheck skipped entries for things like smashdown
    for i in 0..(icon_count - r_offset) {
        if i == random_idx && center_random == true {
            let entry = if use_general_all { "general_all" } else { "random" };
            new_order.push(ui_chara(i, entry));
            // println!("{} / {} = random", i + 1, icon_count);
            push = true;
        }

        let mut fighter = ""; // character we will try inserting
        for _ in 0..chara_order.len() {
            fighter = chara_order[idx].as_str().unwrap_or("goku");
            let allowed = chara_whitelist.contains(&fighter.to_string());
            if allowed { break };
            // Repeat until we find an allowed entry
            // println!("Fighter {fighter} is not part of the original load, and will not be loaded.");
            idx += 1;
        }
        
        let num = i - if push { 1 } else { 0 };
        if fighter == "element" { // aegis is a special case and is loaded with two entries
            new_order.push(ui_chara(num, "flame_first"));
            new_order.push(ui_chara(num, "light_first"));
        } 
        else if fighter == "random" && use_general_all {
            new_order.push(ui_chara(num, "general_all"));
        } 
        else {
            new_order.push(ui_chara(num, fighter));
        }
        // println!("{} / {} = {}", n + 1, icon_count, fighter);
        
        idx += 1;
    };

    // replace the original vec data with the new order
    chara_vec.clear();
    for i in 0..new_order.len() {
        chara_vec.push(*new_order.get(i).unwrap_or(&1));
    };
}

// this hook occurs during the main loop of the css, where it calls a function to select a chosen fighter from random
#[skyline::hook(offset = 0x1a14280, inline)]
unsafe fn decide_random(ctx: &mut skyline::hooks::InlineCtx) {
    let src = *ctx.registers[23].x.as_ref() as *mut u64;
    let obj_ptr = *(src as *mut *mut u64).add(1);

    let main_chara = *obj_ptr.add(0x200 / 0x8);
    let sub_chara = *obj_ptr.add(0x208 / 0x8);
    // println!("Main character: {:#x}", main_chara as u32);
    // println!("Sub character: {:#x}", sub_chara as u32);

    // casting the u64 to u32 will give us the hash40 equivalent, so no need for any bit masking 
    let is_random = [main_chara as u32, sub_chara as u32].contains(&(hash40("ui_chara_random").0 as u32));
    if !is_random { return };
    
    IS_MELEE_RANDOM = ninput::any::is_down_any(ninput::Buttons::ZL | ninput::Buttons::ZR);

    // by altering this register we can immediately change the selected fighter (melee style)
    if IS_MELEE_RANDOM {
        let player_id = (*(*(ctx.registers[21].x.as_ref() as *const u64) as *const u64) + 0x150) as *const u8;
        generate_random(*player_id as usize, main_chara, sub_chara);
        *ctx.registers[24].x.as_mut() = MAIN_FIGHTER_ID;
    }
}

static PT_CHARA_HASHES: &[u64] = &[
    hash40("ui_chara_pzenigame").0,
    hash40("ui_chara_plizardon").0,
    hash40("ui_chara_pfushigisou").0,
];

const KEY_MASK: u64 = 0xFFFFFF_0000000000;
unsafe fn generate_random(player_id: usize, main_data: u64, sub_data: u64) {
    let mut chara_string = decide_fighter_from_id(player_id);
    if chara_string == "element" {
        let aegis = [
            "flame_first", 
            "light_first"
        ]
        .choose(&mut rand::thread_rng()).copied()
        .unwrap_or("flame_first");
        
        chara_string = String::from(aegis);
    }

    let mut chara_hash = hash40(&format!("ui_chara_{}", chara_string)).0;

    MAIN_FIGHTER_ID = chara_hash | (main_data & KEY_MASK);
    
    if chara_string == "ptrainer" {
        chara_hash = 
        PT_CHARA_HASHES.choose(&mut rand::thread_rng()).copied()
        .unwrap_or(hash40("ui_chara_random").0);
    }
    SUB_FIGHTER_ID = chara_hash |  (sub_data & KEY_MASK);
}

unsafe fn get_tag_from_save(idx: u8) -> String {
    let tag_address =
        (***(((*((*((skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8)
            .add(0x5314510) as *const u64)) as *const u64))
            + 0x58) as *const *const *const u64)
            + ((idx as u64) * 0xF7D8)
            + 0xC) as *const u16;

    let mut tag_length = 0;
    while *tag_address.add(tag_length) != 0 {
        tag_length += 1;
    }

    String::from_utf16_lossy(std::slice::from_raw_parts(tag_address, tag_length))
}

unsafe fn decide_fighter_from_id(id: usize) -> String {
    // get player tag
    let tag_index = PLAYER_TAG_INDEX[id];
    let tag = &get_tag_from_save(tag_index);
    // println!("Tag data for slot {}: {}", id, &tag); 

    // get the base whitelist from the initial CSS load
    let mut whitelist = CHARA_WHITELIST.lock().unwrap().clone();

    // the following entries are present for CSS loading, but should never be chosen by random
    let forbidden: [&str; 4] = [
        "miifighter",
        "miiswordsman",
        "miigunner",
        "light_first",
    ];
    for chara in forbidden {
        whitelist.retain(|x| *x != chara.to_owned());
    }

    // choose a fighter from base whitelist
    let mut chara_string =  match whitelist.choose(&mut rand::thread_rng()) {
        Some(string) => string.as_str(),
        None => {
            println!("Whitelist is empty!");
            return "mario".to_string();
        }
    };
    let default = chara_string.to_owned();
    println!("Random character decision: {chara_string}");

    if is_tourney_mode() {
        return default;
    }

    // Collect all relevant data from config TOML
    let path = Path::new("mods:/").join(RANDOM_CFG_TOML);
    if !path.exists() || !path.is_file() { return default };
    let data = match std::fs::read_to_string(&path).unwrap().parse::<Value>() {
        Ok(result) => result,
        Err(_) => { 
            println!("[src::chara_select] Invalid random_config TOML data!");
            return default; 
        }
    };
    let tag_data = match data.as_table().unwrap().get(tag) {
        Some(table) => table.as_table().unwrap(),
        None => {
            if tag == "" {
                println!("[src::chara_select] No player tag detected on player ID {id}! Using global settings.");
            } else {
                println!("[src::chara_select] No settings defined for tag [{tag}]! Using global settings.");
            }
            match data.as_table().unwrap().get("global_settings") {
                Some(table) => table.as_table().unwrap(),
                None => {
                    println!("Just kidding... no global settings are present!");
                    return default;
                }
            }
        }
    };
    let kind = match tag_data.get("kind") {
        Some(value) => value.as_str().unwrap(),
        None => {
            println!("[src::chara_select] Tag data does not contain key [kind]!");
            return default;
        }
    };
    if !["whitelist", "blacklist"].contains(&kind) { return default };
    let list = match tag_data.get("list") {
        Some(value) => value.as_array().unwrap(),
        None => {
            println!("[src::chara_select] Tag data does not contain key [list]!");
            return default;
        }
    };

    // return the original choice if it aligns with the data
    if (kind == "whitelist" && list.contains(&toml::Value::String(default.clone())))
    || (kind == "blacklist" && !list.contains(&toml::Value::String(default.clone()))) {
        println!("{chara_string} is allowed!");
        return default;
    }

    // getting to this point means the original choice isn't permitted, so we will adjust the selection
    println!("{chara_string} is not allowed for tag {}! Adjusting...", &tag);
    if kind == "whitelist" {
        // for whitelists, clear and rebuild with the allowed characters
        let reference = whitelist.clone();
        whitelist.clear();
        for chara in list {
            let fighter = chara.as_str().unwrap_or("mario");
            // prevent the same character from being picked twice in a row (if possible)
            let restrict_prev = &fighter.to_owned() == &LAST_FIGHTER_PICKED && list.len() > 1;
            if reference.contains(&fighter.to_owned()) && !restrict_prev {
                whitelist.push(fighter.to_owned());
            }
        }
        // if none of the whitelisted characters were allowed originally, return the default
        if whitelist.len() <= 0 {
            println!("None of the whitelisted characters are allowed! Continuing with the default.");
            return default;
        }
    } else {
        // for blacklists, remove listed fighters from the selection
        for chara in list {
            let fighter = chara.as_str().unwrap_or("");
            whitelist.retain(|x| *x != fighter.to_owned());
        }
        if whitelist.len() > 1 {
            // prevent the same character from being picked twice in a row (if possible)
            whitelist.retain(|x| *x != LAST_FIGHTER_PICKED);
        }
    }

    // re-query with adjusted list
    chara_string = match whitelist.choose(&mut rand::thread_rng()) {
        Some(str) => str.as_str(),
        None => {
            println!("Whitelist is empty! Returning to default.");
            &default
        }
    };
    LAST_FIGHTER_PICKED = chara_string.to_owned();
    println!("Randomly decided on {chara_string}");

    return chara_string.to_owned();
}

#[skyline::hook(offset = 0x1a0d540)]
unsafe fn set_random_fighter_data(base_ptr: *mut u64, arg2: u64, arg3: u64, arg4: u64) -> u64 {
    // println!("Entering decide_fighter");

    let main_chara = base_ptr.add(2);
    let sub_chara = base_ptr.add(3);
    // println!("Fighter: {:#x}, Sub-fighter: {:#x}", *main_chara, *sub_chara);
    
    // for melee random we will set these values right away, before the original logic
    // this will change the entire selection (including ui)
    if IS_MELEE_RANDOM {
        *main_chara = MAIN_FIGHTER_ID;
        *sub_chara = SUB_FIGHTER_ID;
    } else {
        // this function also runs between games, so make sure the fighter is re-rolled
        // otherwise the same fighter will remain loaded for the next match
        let player_id = (*base_ptr as u8 - 1) as usize;
        generate_random(player_id, *main_chara, *sub_chara);
    }

    // now we'll call the original function into a var, which will do its thing and process data along the way
    let ret = call_original!(base_ptr, arg2, arg3, arg4);

    // at this point, for the normal random, it's safe to modify the data without affecting any UI
    // note that we will only change the SUB fighter here. the main fighter will always just be ui_chara_random
    if !IS_MELEE_RANDOM {
        *sub_chara = SUB_FIGHTER_ID;
    }

    // handle costume rng
    let costume_ptr = (base_ptr as *mut u64).add(4) as *mut u8;
    let mut rng = COSTUME_RNG.lock().unwrap();
    let costume = 
        rng.choose(&mut rand::thread_rng()).copied()
        .unwrap_or((rand::thread_rng().gen::<u32>() % 8) as i32);
    // remove the selection from the rng pool, and refill if empty
    rng.retain(|&x| x != costume);
    if rng.is_empty() {
        for i in 0..8 { rng.push(i) };
    }

    *costume_ptr = costume as u8;
    println!("Randomly selected costume slot to be {costume} (data: {:#x})", *costume_ptr);

    IS_MELEE_RANDOM = false;

    ret
}

#[skyline::hook(offset = 0x19fd0b0)]
unsafe fn update_player_tag(arg1: u64, tag_index: *const u8) {
    let player_id = *((arg1 + 0x1d4) as *const u8) as usize;
    PLAYER_TAG_INDEX[player_id] = *tag_index;
    call_original!(arg1, tag_index);
}

#[skyline::hook(offset = 0x1798ac8, inline)]
unsafe fn fix_chara_replace(ctx: &skyline::hooks::InlineCtx) {
    let ptr1 = *ctx.registers[0].x.as_ref() as *mut u64;
    let ptr2 = *ctx.registers[1].x.as_ref() as *mut u64;

    *ptr2.add(0x2) = *ptr1.add(0x2);
    *ptr2.add(0x3) = *ptr1.add(0x3);
    *ptr2.add(0x4) = *ptr1.add(0x4);
}

pub fn install() {
    skyline::install_hooks!(
        display_css_hook,
        decide_random,
        set_random_fighter_data,
        update_player_tag,
        fix_chara_replace
    );
}