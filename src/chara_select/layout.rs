use super::*;

const ORDER_TOML: &str = "ui/param/menu/chara_icon_order.toml";
const RANDOM_IDX_TOML: &str = "ui/param/menu/chara_random_idx.toml";

#[derive(Debug, Deserialize)]
#[serde(default)]
struct CharaConfig {
    enabled: bool,
    order: String,
    #[serde(flatten)]
    schemas: HashMap<String, CharaSchema>
}
impl Default for CharaConfig {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("series".into(), CharaSchema::default());
        CharaConfig {
            enabled: true,
            order: "series".into(),
            schemas: map
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
struct CharaSchema {
    centered_random: bool,
    order: Vec<String>
}
impl Default for CharaSchema {
    fn default() -> Self {
        CharaSchema {
            centered_random: true,
            order: [
                // default order of fighters in the CSS for when tourney mode is enabled, or config is invalid
                "mario", "luigi", "mariod", "peach", "daisy", "rosetta", "koopa", "koopajr", "packun", "yoshi", "wario", "donkey", "diddy", "krool", "buddy", "murabito", "shizue",
                "link", "younglink", "toonlink", "zelda", "sheik", "ganon", "samus", "szerosuit", "ridley", "samusd", "kirby", "metaknight", "dedede", "fox", "falco", "wolf", "ness", "lucas",
                "pikachu", "pichu", "purin", "mewtwo", "ptrainer", "lucario", "gekkouga", "gaogaen", "marth", "roy", "ike", "reflet", "chrom", "lucina", "kamui", "master",
                "captain", "ice_climber", "gamewatch", "pit", "pitb", "palutena", "pikmin", "robot", "wiifit", "littlemac", "shulk", "element", "duckhunt", "inkling", "tantan", "miifighter", "miiswordsman", "miigunner",
                "snake", "simon", "richter", "sonic", "bayonetta", "jack", "rockman", "ryu", "ken", "dolly", "demon", "pacman", "cloud", "edge", "trail", "brave", "pickel"
            ]
            .map(|x| x.to_string()).to_vec()
        }
    }
}

#[skyline::hook(offset = 0x19eb840, inline)]
pub unsafe fn init_css_hook(ctx: &InlineCtx) {
    // Change "stacked" CSS flag to "separate"
    // This unstacks echo fighters
    let param_4 = ctx.registers[3].x() as *mut u8;
    *param_4.add(1) = 1;

    // reset all stored data to default
    let mut chara_data = CHARA_DATA.write().unwrap();
    *chara_data = CharaData::default();

    let path = Path::new("mods:/").join(ORDER_TOML);
    let data = match std::fs::read_to_string(&path) {
        Ok(result) => result,
        Err(e) => {
            println!("[src::chara_select] Could not read TOML: {}", e);
            return;
        }
    };
    let config: CharaConfig = match toml::from_str(&data) {
        Ok(result) => result,
        Err(e) => {
            println!("[src::chara_select] Error parsing TOML: {}", e);
            return;
        }
    };
    
    // get the original vector of fighter entries to be loaded
    let chara_vec = &mut *(ctx.registers[4].x() as *mut smash2::cpp::Vector<u64>);

    if !config.enabled {
        for char in CharaSchema::default().order.iter() {
            let hash = hash40(&format!("ui_chara_{}", char)).0;
            let allowed = chara_vec.iter().any(|x| (*x & !KEY_MASK) == hash);
            if allowed {
                chara_data.whitelist.push(char.to_string());
            }
        }

        return;
    }

    let (mut whitelist, mut blacklist) = (Vec::new(), Vec::new());

    let schema: CharaSchema = config.schemas.get(&config.order).cloned().unwrap_or_default();
    let mut chara_order = if is_tourney_mode() { CharaSchema::default().order } else { schema.order.clone() };
    // aegis is a special case and is loaded with two entries
    if chara_order.contains(&"element".to_string()) {
        let idx = chara_order.iter().position(|x| x == "element").unwrap();
        chara_order[idx] = "flame_first".to_string();
        chara_order.insert(idx + 1, "light_first".to_string());
    }

    for idx in 0..chara_order.len() {
        let chara = match chara_order.get(idx) {
            Some(string) => string.as_str(),
            None => "goku"
        };
        let should_load = chara_vec.iter().any(|x|
            (*x & !KEY_MASK) == hash40(&format!("ui_chara_{}", chara)).0
        );

        let dest = if should_load { &mut whitelist } else { &mut blacklist };
        dest.push(chara.to_string());
    }

    let mut icon_count = chara_order.len();
    if schema.centered_random { icon_count += 1 }; // +1 to the order if random is to be inserted
    icon_count -= blacklist.len(); // subtract blacklisted fighters from the total

    let kill_mythra = if whitelist.contains(&"light_first".to_string()) { 1 } else { 0 }; // die
    let random_idx = get_random_idx(dbg!(icon_count - kill_mythra));
    // println!(
    //     "{icon_count} icons to load ({} out of {} blacklisted).\nRandom will be placed in slot {random_idx}",
    //     blacklist.len(), chara_order.len() + if schema.centered_random { 1 } else { 0 }
    // );

    let mut fighters = chara_order.clone();
    fighters.reverse(); // convert into a stack

    let mut new_order = Vec::new();
    let mut push = false;
    let use_general_all = chara_vec.iter().any(|x| (*x & !KEY_MASK) == hash40("ui_chara_general_all").0);
    for i in 0..icon_count {
        if i == random_idx && schema.centered_random && !push {
            let entry = if use_general_all { "general_all" } else { "random" };
            new_order.push(ui_chara(entry, i));
            push = true;
            continue;
        }

        let mut fighter = String::new();
        while fighter.is_empty() {
            let next = match fighters.pop() {
                Some(fighter) => fighter,
                None => break
            };
            if whitelist.contains(&next) { fighter = next };
        }

        let num = i - if push { 1 } else { 0 };
        if fighter == "random" && use_general_all {
            new_order.push(ui_chara("general_all", num));
        } else {
            new_order.push(ui_chara(fighter.as_str(), num));
        }
    };

    // replace the original vec data with the new order
    chara_vec.clear();
    for entry in new_order {
        chara_vec.push(entry);
    }

    chara_data.whitelist = whitelist;
    chara_data.blacklist = blacklist;
}


#[derive(Debug, Deserialize)]
struct RandomIndex{ placement: Vec<usize> }

fn get_random_idx(icon_count: usize) -> usize {
    let mut idx = icon_count / 2;
    let path = Path::new("mods:/").join(RANDOM_IDX_TOML);
    let data = match std::fs::read_to_string(&path) {
        Ok(result) => result,
        Err(e) => {
            println!("[src::chara_select] Could not read TOML: {}", e);
            return idx;
        }
    };
    let config: RandomIndex = match toml::from_str(&data) {
        Ok(result) => result,
        Err(e) => {
            println!("[src::chara_select] Error parsing TOML: {}", e);
            return idx;
        }
    };

    match config.placement.get(icon_count - 1) {
        Some(&value) => value,
        None => idx
    }
}

// formats the supplied index + chara string into valid u64 for the CSS icon vector
fn ui_chara(chara: &str, idx: usize) -> u64 {
    (0xc1u64 << 56)
    | (((idx as u64) & 0xFFFF) << 40)
    | hash40(&format!("ui_chara_{}", chara)).0
}

pub fn install() {
    skyline::install_hooks!(
        init_css_hook
    );
}