use std::convert::TryInto;
use std::io::Seek;
use std::sync::Arc;
use std::{borrow::BorrowMut, collections::HashMap};

use arcropolis_api::{arc_callback, load_original_file};
use parking_lot::RwLock;
use prc::hash40::hash40 as to_hash40;
use prc::{hash40::Hash40, ParamKind};
use smash::phx::Hash40 as Hash40_2;
use smash::{app::BattleObject, hash40};

use super::PARAM_MODULE_OFFSET;

macro_rules! get_param_module {
    ($object:ident) => {{
        unsafe {
            let vtable = *($object as *mut *mut *mut u64);
            &mut *super::get_entry::<ParamModule>(vtable, PARAM_MODULE_OFFSET)
                .expect("Did not find VarModule!")
        }
    }};
}

macro_rules! has_param_module {
    ($object:ident) => {{
        unsafe {
            if $object.is_null() {
                false
            } else {
                let vtable = *($object as *mut *mut *mut u64);
                super::is_hdr_object(vtable as _)
                    && !super::get_entry::<ParamModule>(vtable, PARAM_MODULE_OFFSET).is_none()
            }
        }
    }};
}

macro_rules! require_param_module {
    ($object:ident) => {{
        if !has_param_module!($object) {
            panic!("BattleObject does not contain reference to ParamModule!");
        }
        get_param_module!($object)
    }};
}

#[derive(Clone)]
enum ParamListing {
    Int(i32),
    Hash(Hash40),
    Float(f32),
    Flag(bool),
    String(String),
    List(Vec<ParamListing>),
    Struct(HashMap<Hash40, ParamListing>),
}

impl ParamListing {
    pub fn get_int(&self) -> Option<i32> {
        match self {
            Self::Int(int) => Some(*int),
            Self::Float(float) => Some(*float as i32),
            Self::Flag(flag) => Some(*flag as i32),
            _ => None,
        }
    }

    pub fn get_hash(&self) -> Option<Hash40> {
        match self {
            Self::Hash(hash) => Some(*hash),
            Self::String(string) => Some(to_hash40(string.as_str())),
            _ => None,
        }
    }

    pub fn get_float(&self) -> Option<f32> {
        match self {
            Self::Int(int) => Some(*int as f32),
            Self::Float(float) => Some(*float),
            _ => None,
        }
    }

    pub fn get_flag(&self) -> Option<bool> {
        match self {
            Self::Int(int) => Some(*int != 0),
            Self::Hash(hash) => Some(hash.0 != 0),
            Self::Float(float) => Some(*float != 0.0),
            Self::Flag(flag) => Some(*flag),
            Self::String(string) => Some(!string.is_empty()),
            Self::List(list) => Some(!list.is_empty()),
            Self::Struct(struc) => Some(!struc.is_empty()),
        }
    }

    pub fn get_string(&self) -> Option<&String> {
        match self {
            Self::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn get_item_in_list(&self, index: usize) -> Option<&ParamListing> {
        match self {
            Self::List(list) => list.get(index),
            _ => None,
        }
    }
}

impl From<ParamKind> for ParamListing {
    fn from(prc: ParamKind) -> Self {
        match prc {
            ParamKind::Bool(flag) => Self::Flag(flag),
            ParamKind::I8(int) => Self::Int(int as i32),
            ParamKind::U8(int) => Self::Int(int as i32),
            ParamKind::I16(int) => Self::Int(int as i32),
            ParamKind::U16(int) => Self::Int(int as i32),
            ParamKind::I32(int) => Self::Int(int),
            ParamKind::U32(int) => Self::Int(int as i32),
            ParamKind::Hash(hash) => Self::Hash(hash),
            ParamKind::Float(float) => Self::Float(float),
            ParamKind::Str(string) => Self::String(string),
            ParamKind::List(list) => {
                Self::List(list.0.into_iter().map(|x| ParamListing::from(x)).collect())
            }
            ParamKind::Struct(struc) => Self::Struct(
                struc
                    .0
                    .into_iter()
                    .map(|(hash, param)| (hash, ParamListing::from(param)))
                    .collect(),
            ),
        }
    }
}

impl ParamListing {
    fn try_get_key(index: &str) -> (&str, &str) {
        let index = if index.starts_with(".") {
            index.split_at(1).1
        } else {
            index
        };
        let slash_index = index.find(".").unwrap_or(0x300);
        let bracket_index = index.find("[").unwrap_or(0x300);
        if slash_index == bracket_index {
            (index, "")
        } else {
            let split = slash_index.min(bracket_index);
            index.split_at(split)
        }
    }

    fn try_get_index(index: &str) -> (Option<usize>, &str) {
        if !index.starts_with("[") {
            (None, index)
        } else {
            let end = match index.find("]") {
                Some(idx) => idx,
                None => return (None, index),
            };
            let (index_str, remaining) = index.split_at(end + 1);
            let index_str = index_str.trim_start_matches("[").trim_end_matches("]");
            let index = if index_str.starts_with("0x") {
                match usize::from_str_radix(index_str.trim_start_matches("0x"), 16) {
                    Ok(idx) => idx,
                    Err(_) => return (None, remaining),
                }
            } else {
                match index_str.parse::<usize>() {
                    Ok(idx) => idx,
                    Err(_) => return (None, remaining),
                }
            };
            (Some(index), remaining)
        }
    }

    pub fn index(&self, key: &str) -> Option<&ParamListing> {
        if key.is_empty() {
            return Some(self);
        }

        match self {
            ParamListing::List(list) => {
                let (idx, remaining) = Self::try_get_index(key);
                if let Some(idx) = idx {
                    if let Some(listing) = list.get(idx) {
                        listing.index(remaining)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            ParamListing::Struct(struc) => {
                let (key, remaining) = Self::try_get_key(key);
                if let Some(listing) = struc.get(&to_hash40(key)) {
                    listing.index(remaining)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[repr(C)]
struct FighterParam {
    pub fighter_list: ParamListing,
}

impl FighterParam {
    pub fn get_int(&self, kind: i32, index: &str) -> i32 {
        self.fighter_list
            .get_item_in_list(kind as usize)
            .map(|x| x.index(index))
            .flatten()
            .map(|x| x.get_int())
            .flatten()
            .unwrap_or(0)
    }

    pub fn get_hash(&self, kind: i32, index: &str) -> Hash40_2 {
        self.fighter_list
            .get_item_in_list(kind as usize)
            .map(|x| x.index(index))
            .flatten()
            .map(|x| x.get_hash())
            .flatten()
            .map_or_else(|| Hash40_2::new("invalid"), |x| Hash40_2::new_raw(x.0))
    }

    pub fn get_float(&self, kind: i32, index: &str) -> f32 {
        self.fighter_list
            .get_item_in_list(kind as usize)
            .map(|x| x.index(index))
            .flatten()
            .map(|x| x.get_float())
            .flatten()
            .unwrap_or(0.0)
    }

    pub fn is_flag(&self, kind: i32, index: &str) -> bool {
        self.fighter_list
            .get_item_in_list(kind as usize)
            .map(|x| x.index(index))
            .flatten()
            .map(|x| x.get_flag())
            .flatten()
            .unwrap_or(false)
    }

    pub fn get_string<'a>(&'a self, kind: i32, index: &str) -> &'a str {
        self.fighter_list
            .get_item_in_list(kind as usize)
            .map(|x| x.index(index))
            .flatten()
            .map(|x| x.get_string())
            .flatten()
            .map_or_else(|| "", |x| x.as_str())
    }
}

static AGENT_PARAMS: &'static str = hdr_macros::agent_params!("romfs/agent_params.txt");

lazy_static! {
    static ref GLOBAL_FIGHTER_PARAM: RwLock<Option<FighterParam>> = RwLock::new(None);
    static ref GLOBAL_COMMON_PARAM: RwLock<Option<ParamListing>> = RwLock::new(None);
    // We don't have a good way to manage dropping the memory here, so we will just replace when it gets reloaded.
    static ref GLOBAL_AGENT_PARAMS: RwLock<HashMap<Hash40_2, Arc<ParamListing>>> = RwLock::new(HashMap::new());

    static ref AGENT_PARAM_REVERSE: HashMap<Hash40_2, (Hash40_2, usize)> = {
        let mut hashes = HashMap::new();
        for line in AGENT_PARAMS.lines() {
            if !line.starts_with("#") && !line.is_empty() {
                let mut split = line.split(":");
                let agent = split.next().unwrap();
                let file = split.next().unwrap();
                let size = split.next().unwrap().parse::<usize>().unwrap();
                hashes.insert(Hash40_2::new(file), (Hash40_2::new(agent), size));
            }
        }
        hashes
    };
}

const STAGE_DB_PRC: &str = "ui/param/database/ui_stage_db.prc";
const STAGE_SELECT_LAYOUT: &str = "ui/layout/menu/stage_select/stage_select/layout.arc";
const STAGE_SELECT_PATCH_LAYOUT: &str = "ui/layout/patch/stage_select2/stage_select2/layout.arc";
const STAGE_SELECT_TOURNEY_LAYOUT: &str =
    "ui/layout/menu/stage_select/stage_select/layout.tourney.arc";
const DEFAULT_ROW_LENGTH: usize = 7;

const STAGE_SELECT_ACTOR_LUA: &str = "ui/script_patch/common/stage_select_actor3.lc";
const STAGE_SELECT_ACTOR_LUA_TOURNEY: &str =
    "mods:/ui/script_patch/common/stage_select_actor3.tourney.lc";

use prc::{ParamList, ParamStruct};
use serde::{Deserialize, Serialize};
use serde_json::Result;

/// stores the tournament mode configuration
#[derive(Serialize, Deserialize, Debug)]
#[repr(C)]
struct TourneyConfig {
    /// whether the tourney mode is enabled
    enabled: bool,
    /// the ordered list of starters stages which should be enabled,
    /// or `None` if there are no starters
    starters: Option<Vec<String>>,
    /// the ordered list of counterpick stages which should be enabled,
    /// or `None` if there are no counterpicks
    counterpicks: Option<Vec<String>>,
}

impl TourneyConfig {
    fn is_valid(&self) -> bool {
        // if tourney mode is not enabled, we are not configured
        if !self.enabled {
            println!("tourney mode stages are not enabled");
            return false;
        }

        // if the stages are not defined, we are not configured
        if self.starters.is_none() && self.starters.is_none() {
            println!("tourney mode stages are enabled, but the stages are missing!");
            return false;
        }

        // if there are too many stages for the allowed number per row, this is invalid.
        if self.starters.as_ref().unwrap().len() > DEFAULT_ROW_LENGTH
            || self.counterpicks.as_ref().unwrap().len() > DEFAULT_ROW_LENGTH
        {
            println!("Too many starters or counterpicks were enabled! Invalid stage config!")
        }

        // otherwise, we are configured
        return true;
    }
    /// loads the tourney config from the sdcard
    fn load() -> Option<TourneyConfig> {
        // load the tourney config
        let config: Option<TourneyConfig> =
            match std::fs::read_to_string("sd:/ultimate/hdr-config/tourney_mode.json") {
                Ok(json) => serde_json::from_str(&json)
                    .expect("A tourney_mode.json was found, but its contents were invalid!"),
                Err(e) => {
                    println!(
                        "No tourney mode config was found. Assuming tourney mode is disabled."
                    );
                    None
                }
            };
        return config;
    }
}

/// This is used for tournament mode. Modifies on-the-fly whether individual stages will
/// be shown or not on the stage select screen.
///
/// This logic works by modifying the display order (`disp_order`) of the stages, based
/// on what has been configured in the tournament configuration json file, and then
/// recreating the prc file contents to *only* include those stages.
///
/// More specifically, if both counterpicks and starters are specified, then
/// the `Training` stage is used as a buffer between the two sets of stages.
/// - Starter stages are given 1
/// - Training is given 2
/// - Counterpick stages are given 3
///
/// The result is that the stages are given to the UI in a list,
/// ordered as `[(starters), Training...Training, (counterpicks)]`. The UI then hides
/// all Training stages, and makes them non-interactable, so that the UI looks as expected.
#[arc_callback]
fn ui_stage_db_prc_callback(hash: u64, mut data: &mut [u8]) -> Option<usize> {
    // ensure this is the file data it should be
    assert_eq!(hash, hash40(STAGE_DB_PRC));

    let config = TourneyConfig::load();
    if config.is_none() {
        return None;
    }
    let config = config.unwrap();
    if !config.is_valid() {
        println!("not loading tourney config for stage PRC - config is not valid.");
        return None;
    }

    // load the original file data first
    let size = load_original_file(hash, &mut data).expect("Unable to load ui_stage_db.prc!");

    // get the slice of the allocated buffer which was *actually filled* via loading the original file
    let loaded_data = &data[..size];
    let mut base_struct = match prc::read_stream(&mut std::io::Cursor::new(loaded_data)) {
        Ok(struc) => struc,
        Err(e) => {
            panic!("Unable to parse '{}'. Reason: {:?}", STAGE_DB_PRC, e)
        }
    };

    // get the db_root list
    let db_root = &mut (&mut (&mut base_struct).0)[0];
    assert_eq!(*db_root.0, hash40("db_root"));
    let list_kind = (&mut db_root.1);
    let param_list = list_kind
        .try_into_mut::<ParamList>()
        .expect("Failed to load db_root list from ui_stage_db.prc!");
    let list = &mut param_list.0;
    let mut out_list: Vec<ParamKind> = vec![];

    let starters = config.starters.unwrap();
    let counterpicks = config.counterpicks.unwrap();

    // disable and enable the appropriate stages in the original structure
    for entry in list.iter_mut() {
        let stage_entry = &mut (entry
            .try_into_mut::<ParamStruct>()
            .expect("failed to get struct from ui_stage_db.prc entry!")
            .0);

        // the name_id of the stage: BattleField, Yoshi_Story, Kirby_Pupupu64, etc
        let name_id = stage_entry
            .iter()
            .find(|param| param.0 == prc::hash40::Hash40(hash40("name_id")))
            .unwrap()
            .1
            .try_into_ref::<String>()
            .expect("Could not get name_id as String for a stage entry in tourney mode!")
            .clone();

        // the display order for the stage
        let disp_order = stage_entry
            .iter_mut()
            .find(|param| param.0 == prc::hash40::Hash40(hash40("disp_order")))
            .unwrap()
            .1
            .try_into_mut::<i8>()
            .expect("Could not get disp_order as an i8 for a stage entry in tourney mode");

        *disp_order = -1;

        // set the disp_order if this is a starter to 1 (see fn docs)
        match starters.contains(&name_id) {
            true => {
                *disp_order = 1;
                out_list.push(entry.clone());
                continue;
            }
            false => {}
        };

        // set Random to 2 (see fn docs)
        if name_id == "Training" {
            *disp_order = 2;
            // pad with a bunch of Training stages, which will be hidden
            let len = starters.len();
            for _ in 0..(DEFAULT_ROW_LENGTH - len) {
                out_list.push(entry.clone());
            }
            continue;
        }

        // set the disp_order if this is a counterpick to 3 (see fn docs)
        match counterpicks.contains(&name_id) {
            true => {
                *disp_order = 3;
                out_list.push(entry.clone());
                continue;
            }
            false => {}
        };

        // otherwise, push this as hidden
        *disp_order = -1;
        out_list.push(entry.clone());
        continue;
    }

    *list = out_list;

    // write the data back into the buffer
    let buf = &mut std::io::Cursor::new(data);
    prc::write_stream(buf, &base_struct);
    let size = buf.stream_len().unwrap() as usize;

    Some(size)
}

#[arc_callback]
fn stage_select_layout_callback(hash: u64, mut data: &mut [u8]) -> Option<usize> {
    // ensure that the tourney config is loaded, and if not, return
    match TourneyConfig::load() {
        Some(config) => match config.is_valid() {
            true => {}
            false => return None,
        },
        None => return None,
    }

    println!("loading tourney mode layout");

    // load the tourney layout
    let size = load_original_file(hash40(STAGE_SELECT_TOURNEY_LAYOUT), &mut data)
        .expect("Unable to load '/ui/layout/menu/stage_select/stage_select/layout.tourney.arc'");
    Some(size)
}

/// this callback is used to allocate more space than normal for the stage
#[arc_callback]
fn stage_select_actor_callback(hash: u64, mut data: &mut [u8]) -> Option<usize> {
    // ensure that the tourney config is loaded, and if not, return
    match TourneyConfig::load() {
        Some(config) => match config.is_valid() {
            true => {}
            false => return None,
        },
        None => return None,
    }

    println!("loading tourney mode sss lua");

    // load the actor lua in hdr-dev (REMOVE THIS)
    let mut bytes = match std::fs::read(STAGE_SELECT_ACTOR_LUA_TOURNEY) {
        Ok(b) => b,
        Err(e) => {
            println!("\n\n\n\n\nFAILED TO LOAD DEV STAGE SELECT ACTOR!\n\n\n\n\n");
            return None;
        }
    };

    // pad the file bytes
    let last = *bytes.last().unwrap();
    while bytes.len() < data.len() {
        bytes.push(last as u8)
    }

    let bytes_slice = bytes.as_slice();
    data.copy_from_slice(&bytes_slice);

    let size = data.len();
    Some(size)
}

#[arc_callback]
fn fighter_param_callback(hash: u64, mut data: &mut [u8]) -> Option<usize> {
    assert_eq!(hash, hash40("fighter/common/hdr/param/fighter_param.prc"));
    let size = load_original_file(hash, &mut data)
        .expect("Unable to load file for 'fighter/common/hdr/param/fighter_param.prc'");
    let exact_data = &data[..size];
    let listing = match prc::read_stream(&mut std::io::Cursor::new(exact_data)) {
        Ok(struc) => ParamListing::from(ParamKind::Struct(struc)),
        Err(e) => {
            panic!(
                "Unable to parse 'fighter/common/hdr/param/fighter_param.prc'. Reason: {:?}",
                e
            )
        }
    };

    let fighter_list = match listing.index("fighter_param_table") {
        Some(listing) if matches!(listing, ParamListing::List(_)) => listing,
        _ => panic!("Invalid data found in 'fighter/common/hdr/param/fighter_param.prc'"),
    };

    let mut param = GLOBAL_FIGHTER_PARAM.write();
    *param = Some(FighterParam {
        fighter_list: fighter_list.clone(),
    });
    Some(size)
}

#[arc_callback]
fn common_param_callback(hash: u64, mut data: &mut [u8]) -> Option<usize> {
    assert_eq!(hash, hash40("fighter/common/hdr/param/common.prc"));
    let size = load_original_file(hash, &mut data)
        .expect("Unable to load file for 'fighter/common/hdr/param/common.prc'");
    let exact_data = &data[..size];
    let listing = match prc::read_stream(&mut std::io::Cursor::new(exact_data)) {
        Ok(struc) => ParamListing::from(ParamKind::Struct(struc)),
        Err(e) => {
            panic!(
                "Unable to parse 'fighter/common/hdr/param/common.prc'. Reason: {:?}",
                e
            )
        }
    };

    *GLOBAL_COMMON_PARAM.write() = Some(listing);
    Some(size)
}

#[arc_callback]
fn agent_param_callback(hash: u64, mut data: &mut [u8]) -> Option<usize> {
    let size =
        load_original_file(hash, &mut data).expect("Unable to load file for agent param file!");
    let exact_data = &data[..size];
    let param_listing = match prc::read_stream(&mut std::io::Cursor::new(exact_data)) {
        Ok(struc) => ParamListing::from(ParamKind::Struct(struc)),
        Err(e) => {
            panic!("Unable to parse param file {:#x}. Reason: {:?}", hash, e);
        }
    };
    let hash = Hash40_2::new_raw(hash);
    let agent = match AGENT_PARAM_REVERSE.get(&hash) {
        Some((agent, _)) => *agent,
        None => panic!("Failed to find agent kind hash in param file reverse lookup!"),
    };
    let mut params = GLOBAL_AGENT_PARAMS.write();
    if let Some(loaded_data) = params.get_mut(&agent) {
        *loaded_data = Arc::new(param_listing);
    } else {
        params.insert(agent, Arc::new(param_listing));
    }
    Some(size)
}

/// Specified which kind of param to retrieve from ParamModule
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParamType {
    /// References common params that exist across the entire game instance
    Common,
    /// References "shared" params that exist for all fighters but are different depending on fighter kind
    Shared,
    /// References agent specific params
    Agent,
}

// An additional module to be used with Smash's `BattleObject` class. This uses an ARCropolis API to load (new) files and store them as global param data
// that objects can access at runtime. This allows for storing constants in data modifiable for easy balancing.
#[repr(C)]
pub struct ParamModule {
    owner: *mut BattleObject,
    agent_params: Option<Arc<ParamListing>>,
}

impl ParamModule {
    /// Constructs a new `ParamModule` instance
    /// # Arguments
    /// * `owner` - The owning `BattleObject` instance
    /// # Returns
    /// A new `ParamModule` instance. This assumes that when this is constructed that the file has already been loaded.
    pub(crate) fn new(owner: *mut BattleObject) -> Self {
        let kind = unsafe { (*owner).agent_kind_hash };
        let agent_params = GLOBAL_AGENT_PARAMS.read().get(&kind).map(|x| x.clone());
        Self {
            owner: owner,
            agent_params,
        }
    }

    /// Attempts to pull the agent param listing from the global agent params
    /// # Arguments
    /// * `owner` - The owning `BattleObject` instance
    fn try_get_agent_params(owner: *mut BattleObject) {
        let module = require_param_module!(owner);
        if module.agent_params.is_some() {
            return;
        }

        let kind = unsafe { (*module.owner).agent_kind_hash };
        module.agent_params = GLOBAL_AGENT_PARAMS.read().get(&kind).map(|x| x.clone());
    }

    /// Retrieves an integer from the specified ParamModule instance
    /// # Arguments
    /// * `owner` - The owning `BattleObject` instance
    /// * `ty` - Where to retreive the param from
    /// * `key` - The key/path of the param
    /// # Returns
    /// The integer specified
    /// # Example
    /// ```
    /// let dacus_start_frame = ParamModule::get_int(fighter.battle_object, ParamType::Shared, "dacus_frame_min");
    /// if fighter.global_table[STATUS_FRAME] >= dacus_start_frame && should_dacus {
    ///     // perform dacus here
    /// }
    /// ```
    #[export_name = "ParamModule__get_int"]
    pub fn get_int(object: *mut BattleObject, ty: ParamType, key: &str) -> i32 {
        let module = require_param_module!(object);
        Self::try_get_agent_params(module.owner);
        match ty {
            ParamType::Common => {
                let read = GLOBAL_COMMON_PARAM.read();
                //println!("{}", read.is_none());
                read.as_ref()
                    .map(|x| x.index(key))
                    .flatten()
                    .map(|x| x.get_int())
                    .flatten()
                    .unwrap_or_else(|| panic!("Could not retrieve Common ParamModule int: {}", key))
            }
            ParamType::Shared => {
                let index = unsafe { (*module.owner).kind };

                GLOBAL_FIGHTER_PARAM
                    .read()
                    .as_ref()
                    .map(|x| x.get_int(index as i32, key))
                    .unwrap_or_else(|| panic!("Could not retrieve Shared ParamModule int: {}", key))
            }
            ParamType::Agent => module
                .agent_params
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_int())
                .flatten()
                .unwrap_or_else(|| panic!("Could not retrieve Agent ParamModule int: {}", key)),
        }
    }

    /// Retrieves a hash from the specified ParamModule instance
    /// # Arguments
    /// * `owner` - The owning `BattleObject` instance
    /// * `ty` - Where to retreive the param from
    /// * `key` - The key/path of the param
    /// # Returns
    /// The hash specified
    /// # Example
    /// ```
    /// let frame_window_start = ParamModule::get_float(fighter.object(), ParamType::Shared, "gentleman_combo_start_frame");
    /// let frame_window_end = ParamModule::get_float(fighter.object(), ParamType::Shared, "gentleman_combo_end_frame");
    /// let target_motion = ParamModule::get_hash(fighter.module_accessor, ParamType::Shared, "gentleman_combo_target_motion");
    /// let current_frame = MotionModule::frame(fighter.module_accessor);
    /// if frame_window_start <= current_frame && current_frame < frame_window_end && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
    ///     MotionModule::change_motion(fighter.module_accessor, target_motion, 0.0, 1.0, false, false, 0.0, 0.0);
    /// }
    /// ```
    #[export_name = "ParamModule__get_hash"]
    pub fn get_hash(object: *mut BattleObject, ty: ParamType, key: &str) -> Hash40_2 {
        let module = require_param_module!(object);
        Self::try_get_agent_params(module.owner);
        match ty {
            ParamType::Common => GLOBAL_COMMON_PARAM
                .read()
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_hash())
                .flatten()
                .map_or_else(|| Hash40_2::new("invalid"), |x| Hash40_2::new_raw(x.0)),
            ParamType::Shared => {
                let index = unsafe { (*module.owner).kind };

                GLOBAL_FIGHTER_PARAM
                    .read()
                    .as_ref()
                    .map(|x| x.get_hash(index as i32, key))
                    .unwrap_or(Hash40_2::new("invalid"))
            }
            ParamType::Agent => module
                .agent_params
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_hash())
                .flatten()
                .map_or_else(|| Hash40_2::new("invalid"), |x| Hash40_2::new_raw(x.0)),
        }
    }

    /// Retrieves a float from the specified ParamModule instance
    /// # Arguments
    /// * `owner` - The owning `BattleObject` instance
    /// * `ty` - Where to retreive the param from
    /// * `key` - The key/path of the param
    /// # Returns
    /// The float specified
    /// # Example
    /// ```
    /// let grab_angle_hi = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "angleable_grab.angle_hi");
    /// let grab_angle_lw = ParamModule::get_float(fighter.battle_object, ParamType::Agent, "angleable_grab.angle_lw");
    /// let stick_y = (ControlModule::get_stick_y(fighter.module_accessor).clamp(-1.0, 1.0) + 1.0) / 2.0;
    /// let angle = stick_y.lerp(grab_angle_lw, grab_angle_hi);
    /// fighter.rotate_waist_for_grab(angle);
    /// ```
    #[export_name = "ParamModule__get_float"]
    pub fn get_float(object: *mut BattleObject, ty: ParamType, key: &str) -> f32 {
        let module = require_param_module!(object);
        Self::try_get_agent_params(module.owner);
        match ty {
            ParamType::Common => GLOBAL_COMMON_PARAM
                .read()
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_float())
                .flatten()
                .unwrap_or_else(|| panic!("Could not retrieve Common ParamModule float: {}", key)),
            ParamType::Shared => {
                let index = unsafe { (*module.owner).kind };

                GLOBAL_FIGHTER_PARAM
                    .read()
                    .as_ref()
                    .map(|x| x.get_float(index as i32, key))
                    .unwrap_or_else(|| {
                        panic!("Could not retrieve Shared ParamModule float: {}", key)
                    })
            }
            ParamType::Agent => module
                .agent_params
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_float())
                .flatten()
                .unwrap_or_else(|| panic!("Could not retrieve Agent ParamModule float: {}", key)),
        }
    }

    /// Retrieves a flag from the specified ParamModule instance
    /// # Arguments
    /// * `owner` - The owning `BattleObject` instance
    /// * `ty` - Where to retreive the param from
    /// * `key` - The key/path of the param
    /// # Returns
    /// The flag specified
    /// # Example
    /// ```
    /// if !ParamModule::is_flag(fighter.battle_object, ParamType::Common, "disable_shorthop_macro") {
    ///     if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
    ///         WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_RESERVE_JUMP_MINI_ATTACK);
    ///     }
    /// }
    /// ```
    #[export_name = "ParamModule__is_flag"]
    pub fn is_flag(object: *mut BattleObject, ty: ParamType, key: &str) -> bool {
        let module = require_param_module!(object);
        Self::try_get_agent_params(module.owner);
        match ty {
            ParamType::Common => GLOBAL_COMMON_PARAM
                .read()
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_flag())
                .flatten()
                .unwrap_or_else(|| panic!("Could not retrieve Common ParamModule flag: {}", key)),
            ParamType::Shared => {
                let index = unsafe { (*module.owner).kind };

                GLOBAL_FIGHTER_PARAM
                    .read()
                    .as_ref()
                    .map(|x| x.is_flag(index as i32, key))
                    .unwrap_or_else(|| {
                        panic!("Could not retrieve Shared ParamModule flag: {}", key)
                    })
            }
            ParamType::Agent => module
                .agent_params
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_flag())
                .flatten()
                .unwrap_or_else(|| panic!("Could not retrieve Agent ParamModule flag: {}", key)),
        }
    }

    /// Retrieves a string from the specified ParamModule instance
    /// # Arguments
    /// * `owner` - The owning `BattleObject` instance
    /// * `ty` - Where to retreive the param from
    /// * `key` - The key/path of the param
    /// # Returns
    /// The string specified
    /// # Example
    /// ```
    /// println!("HDR Version String: {}", ParamModule::get_string(fighter.battle_object, ParamType::Common, "version_string"));
    /// ```
    #[export_name = "ParamModule__get_string"]
    pub fn get_string(object: *mut BattleObject, ty: ParamType, key: &str) -> String {
        let module = require_param_module!(object);
        Self::try_get_agent_params(module.owner);
        match ty {
            ParamType::Common => GLOBAL_COMMON_PARAM
                .read()
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_string())
                .flatten()
                .unwrap_or_else(|| panic!("Could not retrieve Common ParamModule string: {}", key))
                .clone(),
            ParamType::Shared => {
                let index = unsafe { (*module.owner).kind };

                GLOBAL_FIGHTER_PARAM
                    .read()
                    .as_ref()
                    .map(|x| x.get_string(index as i32, key))
                    .unwrap_or_else(|| {
                        panic!("Could not retrieve Shared ParamModule string: {}", key)
                    })
                    .to_string()
            }
            ParamType::Agent => module
                .agent_params
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_string())
                .flatten()
                .unwrap_or_else(|| panic!("Could not retrieve Agent ParamModule string: {}", key))
                .clone(),
        }
    }
}

pub(crate) fn init() {
    fighter_param_callback::install(
        "fighter/common/hdr/param/fighter_param.prc",
        hdr_macros::size_of_rom_file!("fighter/common/hdr/param/fighter_param.prc"),
    );
    common_param_callback::install(
        "fighter/common/hdr/param/common.prc",
        hdr_macros::size_of_rom_file!("fighter/common/hdr/param/common.prc"),
    );
    for (file, (_, size)) in AGENT_PARAM_REVERSE.iter() {
        agent_param_callback::install(arcropolis_api::Hash40(file.hash), *size);
    }

    // if TourneyConfig isn't valid, then don't bother installing callbacks
    match TourneyConfig::load() {
        Some(config) => match config.is_valid() {
            true => {
                // install the callback for selectively displaying stages
                // max_size is technically unknown, but since we aren't adding new data to the prc, and the
                // hdr file is presently 16kb, this should be sufficient.
                ui_stage_db_prc_callback::install(STAGE_DB_PRC, /* 20kb */ 20480);
                stage_select_layout_callback::install(
                    STAGE_SELECT_LAYOUT,
                    /* 10mb */ 10485760,
                );
                stage_select_layout_callback::install(
                    STAGE_SELECT_PATCH_LAYOUT,
                    /* 10mb */ 10485760,
                );
                stage_select_actor_callback::install(
                    STAGE_SELECT_ACTOR_LUA,
                    /* 10mb */
                    hdr_macros::size_of_rom_file!("ui/script_patch/common/stage_select_actor3.lc"),
                );
            }
            false => {}
        },
        None => {}
    }
}
