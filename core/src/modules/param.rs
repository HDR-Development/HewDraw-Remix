use std::collections::HashMap;
use std::sync::Arc;

use arcropolis_api::{arc_callback, load_original_file};
use prc::{hash40::{Hash40, to_hash40}, ParamKind};
use smash::{hash40, app::BattleObject};
use smash::phx::Hash40 as Hash40_2;
use parking_lot::RwLock;

use crate::utils;

use super::PARAM_MODULE_OFFSET;

macro_rules! get_param_module {
    ($object:ident) => {{
        unsafe {
            let vtable = *($object as *mut *mut *mut u64);
            &mut *super::get_entry::<ParamModule>(vtable, PARAM_MODULE_OFFSET).expect("Did not find VarModule!")
        }
    }}
}

macro_rules! has_param_module {
    ($object:ident) => {{
        unsafe {
            if $object.is_null() {
                false
            } else {
                let vtable = *($object as *mut *mut *mut u64);
                super::is_hdr_object(vtable as _) && !super::get_entry::<ParamModule>(vtable, PARAM_MODULE_OFFSET).is_none()
            }
        }
    }}
}

macro_rules! require_param_module {
    ($object:ident) => {{
        if !has_param_module!($object) {
            panic!("BattleObject does not contain reference to ParamModule!");
        }
        get_param_module!($object)
    }}
}

#[derive(Clone)]
enum ParamListing {
    Int(i32),
    Hash(Hash40),
    Float(f32),
    Flag(bool),
    String(String),
    List(Vec<ParamListing>),
    Struct(HashMap<Hash40, ParamListing>)
}

impl ParamListing {
    pub fn get_int(&self) -> Option<i32> {
        match self {
            Self::Int(int) => Some(*int),
            Self::Float(float) => Some(*float as i32),
            Self::Flag(flag) => Some(*flag as i32),
            _ => None
        }
    }

    pub fn get_hash(&self) -> Option<Hash40> {
        match self {
            Self::Hash(hash) => Some(*hash),
            Self::String(string) => Some(to_hash40(string.as_str())),
            _ => None
        }
    }

    pub fn get_float(&self) -> Option<f32> {
        match self {
            Self::Int(int) => Some(*int as f32),
            Self::Float(float) => Some(*float),
            _ => None
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
            _ => None
        }
    }

    pub fn get_item_in_list(&self, index: usize) -> Option<&ParamListing> {
        match self {
            Self::List(list) => list.get(index),
            _ => None
        }
    }

    pub fn get_item_in_struct(&self, key: Hash40) -> Option<&ParamListing> {
        match self {
            Self::Struct(struc) => struc.get(&key),
            _ => None
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
            ParamKind::List(list) => Self::List(list.0.into_iter().map(|x| ParamListing::from(x)).collect()),
            ParamKind::Struct(struc) => Self::Struct(struc.0.into_iter().map(|(hash, param)| (hash, ParamListing::from(param))).collect())
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
                None => return (None, index)
            };
            let (index_str, remaining) = index.split_at(end + 1);
            let index_str = index_str.trim_start_matches("[").trim_end_matches("]");
            let index = if index_str.starts_with("0x") {
                match usize::from_str_radix(index_str.trim_start_matches("0x"), 16) {
                    Ok(idx) => idx,
                    Err(_) => return (None, remaining)
                }
            } else {
                match index_str.parse::<usize>() {
                    Ok(idx) => idx,
                    Err(_) => return (None, remaining)
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
            },
            ParamListing::Struct(struc) => {
                let (key, remaining) = Self::try_get_key(key);
                if let Some(listing) = struc.get(&to_hash40(key)) {
                    listing.index(remaining)
                } else {
                    None
                }
            },
            _ => None
        }
    }
}

struct FighterParam {
    pub fighter_list: ParamListing
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

static AGENT_PARAMS: &'static str = hdr_macros::agent_params!("core/agent_params.txt");

lazy_static! {
    static ref GLOBAL_FIGHTER_PARAM: RwLock<Option<FighterParam>> = RwLock::new(None);
    static ref GLOBAL_COMMON_PARAM: RwLock<Option<ParamListing>> = RwLock::new(None);
    // We don't have a good way to manage dropping the memory here, so we will just replace when it gets reloaded.
    static ref GLOBAL_AGENT_PARAMS: RwLock<HashMap<Hash40_2, Arc<ParamListing>>> = RwLock::new(HashMap::new());

    static ref AGENT_PARAM_REVERSE: HashMap<Hash40_2, (Hash40_2, usize)> = {
        let mut hashes = HashMap::new();
        for line in AGENT_PARAMS.lines() {
            let mut split = line.split(":");
            let agent = split.next().unwrap();
            let file = split.next().unwrap();
            let size = split.next().unwrap().parse::<usize>().unwrap();
            hashes.insert(Hash40_2::new(file), (Hash40_2::new(agent), size));
        }
        hashes
    };
}

#[arc_callback]
fn fighter_param_callback(hash: u64, mut data: &mut [u8]) -> Option<usize> {
    assert_eq!(hash, hash40("fighter/common/hdr/param/fighter_param.prc"));
    let size = load_original_file(hash, &mut data).expect("Unable to load file for 'fighter/common/hdr/param/fighter_param.prc'");
    let exact_data = &data[..size];
    let listing = match prc::read_stream(&mut std::io::Cursor::new(exact_data)) {
        Ok(struc) => ParamListing::from(ParamKind::Struct(struc)),
        Err(e) => {
            panic!("Unable to parse 'fighter/common/hdr/param/fighter_param.prc'. Reason: {:?}", e)
        }
    };

    let fighter_list = match listing.index("fighter_param_table") {
        Some(listing) if matches!(listing, ParamListing::List(_)) => {
            listing
        },
        _ => panic!("Invalid data found in 'fighter/common/hdr/param/fighter_param.prc'")
    };

    let mut param = GLOBAL_FIGHTER_PARAM.write();
    *param = Some(FighterParam {
        fighter_list: fighter_list.clone()
    });
    Some(size)
}

#[arc_callback]
fn common_param_callback(hash: u64, data: &mut [u8]) -> Option<usize> {
    assert_eq!(hash, hash40("fighter/common/hdr/param/common.prc"));
    load_original_file(hash, data)
}

#[arc_callback]
fn agent_param_callback(hash: u64, mut data: &mut [u8]) -> Option<usize> {
    let size = load_original_file(hash, &mut data).expect("Unable to load file for agent param file!");
    let exact_data = &data[..size];
    let param_listing = match prc::read_stream(&mut std::io::Cursor::new(exact_data)) {
        Ok(struc) => ParamListing::from(ParamKind::Struct(struc)),
        Err(e) => {
            panic!("Unable to parse param file {:#x}. Reason: {:?}", hash, e);
        }
    };
    if let Some(loaded_data) = GLOBAL_AGENT_PARAMS.write().get_mut(&Hash40_2::new_raw(hash)) {
        *loaded_data = Arc::new(param_listing);
    }
    Some(size)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParamType {
    Common,
    Shared,
    Agent,
}

pub struct ParamModule {
    owner: *mut BattleObject,
    agent_params: Option<Arc<ParamListing>>
}

impl ParamModule {
    pub(crate) fn new(owner: *mut BattleObject) -> Self {
        let kind = unsafe {
            (*owner).agent_kind_hash
        };
        let agent_params = GLOBAL_AGENT_PARAMS.read().get(&kind).map(|x| x.clone());
        Self {
            owner: owner,
            agent_params
        }
    }

    #[export_name = "ParamModule__get_int"]
    pub extern "Rust" fn get_int(object: *mut BattleObject, ty: ParamType, key: &str) -> i32 {
        let module = require_param_module!(object);

        match ty {
            ParamType::Common => GLOBAL_COMMON_PARAM
                .read()
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_int())
                .flatten()
                .unwrap_or(0),
            ParamType::Shared => {
                let index = unsafe {
                    (*object).kind
                };

                GLOBAL_FIGHTER_PARAM
                    .read()
                    .as_ref()
                    .map(|x| x.get_int(index as i32, key))
                    .unwrap_or(0)
            },
            ParamType::Agent => module.agent_params
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_int())
                .flatten()
                .unwrap_or(0)
        }
    }

    #[export_name = "ParamModule__get_hash"]
    pub extern "Rust" fn get_hash(object: *mut BattleObject, ty: ParamType, key: &str) -> Hash40_2 {
        let module = require_param_module!(object);

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
                let index = unsafe {
                    (*object).kind
                };

                GLOBAL_FIGHTER_PARAM
                    .read()
                    .as_ref()
                    .map(|x| x.get_hash(index as i32, key))
                    .unwrap_or(Hash40_2::new("invalid"))
            },
            ParamType::Agent => module.agent_params
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_hash())
                .flatten()
                .map_or_else(|| Hash40_2::new("invalid"), |x| Hash40_2::new_raw(x.0)),
        }
    }

    #[export_name = "ParamModule__get_float"]
    pub extern "Rust" fn get_float(object: *mut BattleObject, ty: ParamType, key: &str) -> f32 {
        let module = require_param_module!(object);

        match ty {
            ParamType::Common => GLOBAL_COMMON_PARAM
                .read()
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_float())
                .flatten()
                .unwrap_or(0.0),
            ParamType::Shared => {
                let index = unsafe {
                    (*object).kind
                };

                GLOBAL_FIGHTER_PARAM
                    .read()
                    .as_ref()
                    .map(|x| x.get_float(index as i32, key))
                    .unwrap_or(0.0)
            },
            ParamType::Agent => module.agent_params
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_float())
                .flatten()
                .unwrap_or(0.0)
        }
    }

    #[export_name = "ParamModule__is_flag"]
    pub extern "Rust" fn is_flag(object: *mut BattleObject, ty: ParamType, key: &str) -> bool {
        let module = require_param_module!(object);

        match ty {
            ParamType::Common => GLOBAL_COMMON_PARAM
                .read()
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_flag())
                .flatten()
                .unwrap_or(false),
            ParamType::Shared => {
                let index = unsafe {
                    (*object).kind
                };

                GLOBAL_FIGHTER_PARAM
                    .read()
                    .as_ref()
                    .map(|x| x.is_flag(index as i32, key))
                    .unwrap_or(false)
            },
            ParamType::Agent => module.agent_params
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_flag())
                .flatten()
                .unwrap_or(false)
        }
    }

    #[export_name = "ParamModule__get_string"]
    pub extern "Rust" fn get_string(object: *mut BattleObject, ty: ParamType, key: &str) -> String {
        let module = require_param_module!(object);

        match ty {
            ParamType::Common => GLOBAL_COMMON_PARAM
                .read()
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_string())
                .flatten()
                .map_or_else(|| String::new(), |x| x.clone()),
            ParamType::Shared => {
                let index = unsafe {
                    (*object).kind
                };

                GLOBAL_FIGHTER_PARAM
                    .read()
                    .as_ref()
                    .map(|x| x.get_string(index as i32, key))
                    .map_or_else(|| String::new(), |x| x.to_string())
            },
            ParamType::Agent => module.agent_params
                .as_ref()
                .map(|x| x.index(key))
                .flatten()
                .map(|x| x.get_string())
                .flatten()
                .map_or_else(|| String::new(), |x| x.clone())
        }
    }
}

pub(crate) fn init() {
    fighter_param_callback::install("fighter/common/hdr/param/fighter_param.prc", hdr_macros::size_of_rom_file!("fighter/common/hdr/param/fighter_param.prc"));
    common_param_callback::install("fighter/common/hdr/param/common.prc", hdr_macros::size_of_rom_file!("fighter/common/hdr/param/common.prc"));
    for (file, (_, size)) in AGENT_PARAM_REVERSE.iter() {
        agent_param_callback::install(arcropolis_api::Hash40(file.hash), *size);
    }
}