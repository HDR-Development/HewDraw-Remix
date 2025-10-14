use super::*;

use std::time::{SystemTime, UNIX_EPOCH};
use rand::{
    prelude::SliceRandom,
    Rng, 
    rngs::StdRng,
    SeedableRng
};

const RANDOM_CFG_TOML: &str = "ui/param/menu/chara_random_config.toml";

#[derive(Debug, Deserialize)]
#[serde(default)]
struct RandomConfig {
    #[serde(flatten)]
    tags: HashMap<String, TagData>
}
impl Default for RandomConfig {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("global_settings".into(), TagData::default());

        RandomConfig { tags: map }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
struct TagData {
    kind: ListKind,
    list: Vec<String>
}
impl Default for TagData {
    fn default() -> Self {
        TagData {
            kind: ListKind::Blacklist,
            list: Vec::new()
        }
    }
}
#[derive(Debug, Deserialize, Clone)]
enum ListKind {
    #[serde(rename = "whitelist")]
    Whitelist,
    #[serde(rename = "blacklist")]
    Blacklist
}

// this hook occurs during the main loop of the css, where it calls a function to select a chosen fighter from random
#[skyline::hook(offset = 0x1a14280, inline)]
unsafe fn decide_random(ctx: &mut skyline::hooks::InlineCtx) {
    let src = ctx.registers[23].x() as *mut u64;
    let obj_ptr = *(src as *mut *mut u64).add(1);

    let main_chara = *obj_ptr.add(0x200 / 0x8);
    let sub_chara = *obj_ptr.add(0x208 / 0x8);
    // println!("Main character: {:#x}", main_chara as u32);
    // println!("Sub character: {:#x}", sub_chara as u32);

    // casting the u64 to u32 will give us the hash40 equivalent, so no need for any bit masking 
    let is_random = [main_chara as u32, sub_chara as u32].contains(&(hash40("ui_chara_random").0 as u32));
    if !is_random { return };
    
    let is_melee = ninput::any::is_down_any(ninput::Buttons::ZL | ninput::Buttons::ZR);
    if is_melee {
        let player_id = (*(*(ctx.registers[21].x() as *const u64) as *const u64) + 0x150) as *const u8;
        generate_random(*player_id as usize, main_chara, sub_chara);
        ctx.registers[24].set_x(CHARA_DATA.read().unwrap().main_id);
    }

    CHARA_DATA.write().unwrap().melee_random = is_melee;
}

unsafe fn generate_random(player_id: usize, main_data: u64, sub_data: u64) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut rng_seed = StdRng::seed_from_u64((now.as_millis() as u64 + 1) * rand::thread_rng().gen::<u64>());
    let mut chara_string = decide_fighter_from_id(player_id, &mut rng_seed);
    let mut chara_hash = hash40(&format!("ui_chara_{}", chara_string)).0;

    let mut chara_data = CHARA_DATA.write().unwrap();
    chara_data.main_id = chara_hash | (main_data & KEY_MASK);

    if chara_string == "ptrainer" {
        chara_hash = [
            hash40("ui_chara_pzenigame").0,
            hash40("ui_chara_plizardon").0,
            hash40("ui_chara_pfushigisou").0,
        ]
        .choose(&mut rng_seed).copied()
        .unwrap_or(hash40("ui_chara_random").0);
    }
    chara_data.sub_id = chara_hash | (sub_data & KEY_MASK);
   
    // handle costume rng
    let mut rng = chara_data.costume_rng.clone();
    let costume = {
        rng.choose(&mut rng_seed).copied()
        .unwrap_or((rng_seed.gen::<u32>() % 8) as i32)
    };
    rng.retain(|&x| x != costume);

    if rng.is_empty() { rng = CharaData::default().costume_rng };

    chara_data.costume = costume as u8;
    chara_data.costume_rng = rng;
    println!("Randomly selected costume slot to be {costume}");
}

unsafe fn decide_fighter_from_id(id: usize, seed: &mut StdRng) -> String {
    let chara_data = { CHARA_DATA.read().unwrap().clone() };
    let mut whitelist = chara_data.whitelist;

    // make sure miis cannot be selected
    for mii in [
        "miifighter", "miiswordsman", "miigunner"
    ] {
        whitelist.retain(|x| *x != mii.to_owned());
    }

    // choose a fighter from base whitelist
    let mut chara_string =  match whitelist.choose(seed) {
        Some(string) => string.as_str(),
        None => return dbg!("mario").to_owned()
    };
    let default = chara_string.to_owned();
    println!("Default character decision: {chara_string}");

    if is_tourney_mode() {
        println!("Tourney mode enabled! Bypassing random config.");
        return default;
    }

    // Collect all relevant data from config TOML
    let path = Path::new("mods:/").join(RANDOM_CFG_TOML);
    let data = match std::fs::read_to_string(&path) {
        Ok(result) => result,
        Err(e) => return dbg!(default)
    };
    let config: RandomConfig = match toml::from_str(&data) {
        Ok(result) => result,
        Err(e) => return dbg!(default)
    };
    
    let tag_id = id.clamp(0, 7);
    let tag_index = PLAYER_TAG_INDEX[tag_id];
    let tag = &get_tag_from_save(tag_index);
    println!("Tag data for slot {}: {}", id, &tag); 
    let mut tag_data: TagData = match config.tags.get(tag) {
        Some(data) => data.clone(),
        None => {
            println!("No settings defined for tag [{tag}]! Using global settings.");
            match config.tags.get("global_settings") {
                Some(global) => global.clone(),
                None => {
                    println!("...No global settings defined! Using default.");
                    TagData::default()
                }
            }
        }
    };

    if tag_data.list.contains(&"element".to_string()) { // convert aegis
        let idx = tag_data.list.iter().position(|x| x == "element").unwrap();
        tag_data.list[idx] = "flame_first".to_string();
        tag_data.list.insert(idx + 1, "light_first".to_string());
    }

    match tag_data.kind {
        ListKind::Whitelist => {
            if tag_data.list.contains(&default) {
                println!("{chara_string} is allowed!");
                return default;
            }

            whitelist.retain(|x| {
                let restrict_prev = *x == chara_data.last_selection && tag_data.list.len() > 1;
                tag_data.list.contains(&x) && !restrict_prev
            });

            // if none of the whitelisted characters were allowed originally, return the default
            if whitelist.len() <= 0 {
                return dbg!(default);
            }
        }
        ListKind::Blacklist => {
            if !tag_data.list.contains(&default) {
                println!("{chara_string} is allowed!");
                return default;
            }

            for chara in tag_data.list {
                whitelist.retain(|x| *x != chara);
            }
            if whitelist.len() > 1 {
                // prevent the same character from being picked twice in a row (if possible)
                whitelist.retain(|x| *x != chara_data.last_selection);
            }
        }
    }
    // re-query with adjusted list
    println!("{} is not allowed for tag {}! Adjusting...", default, &tag);
    chara_string = match whitelist.choose(seed) {
        Some(str) => str.as_str(),
        None => return dbg!(default)
    };
    println!("Randomly decided on {chara_string}");
    CHARA_DATA.write().unwrap().last_selection = chara_string.to_owned();

    chara_string.to_owned()
}

#[skyline::hook(offset = 0x1a0d540)]
unsafe fn set_random_fighter_data(base_ptr: *mut u64, arg2: u64, arg3: u64, arg4: u64) -> u64 {
    let chara_data = { CHARA_DATA.read().unwrap().clone() };
    
    let main_chara = base_ptr.add(2);
    let sub_chara = base_ptr.add(3);
    // println!("Fighter: {:#x}, Sub-fighter: {:#x}", *main_chara, *sub_chara);

    if chara_data.melee_random { // melee random should force these values right away
        *main_chara = chara_data.main_id;
        *sub_chara = chara_data.sub_id;
    } else {
        // ensure random is re-rolled between games
        let player_id = (*base_ptr as u8 - 1) as usize;
        generate_random(player_id, *main_chara, *sub_chara);
    }

    let ret = call_original!(base_ptr, arg2, arg3, arg4);

    // at this point, for the normal random, it's safe to modify the data without affecting any UI
    // note that we will only change the SUB fighter here. the main fighter will always just be ui_chara_random
    let chara_data = CHARA_DATA.read().unwrap();
    if !chara_data.melee_random {
        *sub_chara = chara_data.sub_id;
    }

    // set costume
    let costume_ptr = (base_ptr as *mut u64).add(4) as *mut u8;
    *costume_ptr = chara_data.costume;

    ret
}

#[skyline::hook(offset = 0x1798ac8, inline)]
unsafe fn fix_chara_replace(ctx: &skyline::hooks::InlineCtx) {
    let ptr1 = ctx.registers[0].x() as *mut u64;
    let ptr2 = ctx.registers[1].x() as *mut u64;

    *ptr2.add(0x2) = *ptr1.add(0x2);
    *ptr2.add(0x3) = *ptr1.add(0x3);
    *ptr2.add(0x4) = *ptr1.add(0x4);
}

pub fn install() {
    skyline::install_hooks!(
        decide_random,
        set_random_fighter_data,
        fix_chara_replace
    );
}