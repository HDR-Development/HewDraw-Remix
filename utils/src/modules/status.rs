use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;
use skyline::nro::NroInfo;
use smash::{
    phx::Hash40,
    app::BattleObject,
    lua2cpp::*
};
use utils_dyn::ext::{StatusInfo, StatusFunc};

use super::CUSTOM_STATUS_MODULE_OFFSET;

macro_rules! get_status_module {
    ($object:ident) => {{
        unsafe {
            let vtable = *($object as *mut *mut *mut u64);
            &mut *super::get_entry::<CustomStatusModule>(vtable, CUSTOM_STATUS_MODULE_OFFSET).expect("Did not find CustomStatusModule!")
        }
    }}
}

macro_rules! has_status_module {
    ($object:ident) => {{
        unsafe {
            if $object.is_null() {
                false
            } else {
                let vtable = *($object as *mut *mut *mut u64);
                super::is_hdr_object(vtable as _) && !super::get_entry::<CustomStatusModule>(vtable, CUSTOM_STATUS_MODULE_OFFSET).is_none()
            }
        }
    }}
}

macro_rules! require_status_module {
    ($object:ident) => {{
        if !has_status_module!($object) {
            panic!("BattleObject does not contain reference to CustomStatusModule!");
        }
        get_status_module!($object)
    }}
}

lazy_static! {
    static ref CUSTOM_STATUS_MANAGER: RwLock<CustomStatusManager> = RwLock::new(CustomStatusManager::new());
}

#[repr(C)]
pub struct CustomStatusManager {
    pub agent_statuses: HashMap<Hash40, Arc<RwLock<HashMap<i32, StatusInfo>>>>,
    pub common_statuses: Arc<RwLock<HashMap<i32, StatusInfo>>>,
}

impl CustomStatusManager {
    pub(crate) fn new() -> Self {
        Self {
            agent_statuses: HashMap::new(),
            common_statuses: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    #[export_name = "CustomStatusManager__add_new_common_status_script"]
    pub extern "Rust" fn add_new_common_status_script(id: i32, info: StatusInfo) -> bool {
        let mut manager = CUSTOM_STATUS_MANAGER.write();
        let x = if let Some(mut common_statuses) = manager.common_statuses.try_write() {
            let _ = common_statuses.insert(id, info);
            true
        } else {
            false
        };

        x
    }

    #[export_name = "CustomStatusManager__add_new_agent_status_script"]
    pub extern "Rust" fn add_new_agent_status_script(agent: Hash40, id: i32, info: StatusInfo) -> bool {
        let mut manager = CUSTOM_STATUS_MANAGER.write();
        if let Some(agent_statuses) = manager.agent_statuses.get_mut(&agent) {
            if let Some(mut agent_statuses) = agent_statuses.try_write() {
                let _ = agent_statuses.insert(id, info);
                true
            } else {
                false
            }
        } else {
            let mut map = HashMap::new();
            let _ = map.insert(id, info);
            let _ = manager.agent_statuses.insert(agent, Arc::new(RwLock::new(map)));
            true
        }
    }

    #[export_name = "CustomStatusManager__get_common_status_scripts"]
    pub extern "Rust" fn get_common_status_scripts() -> Arc<RwLock<HashMap<i32, StatusInfo>>> {
        CUSTOM_STATUS_MANAGER.read().common_statuses.clone()
    }

    #[export_name = "CustomStatusManager__get_agent_status_scripts"]
    pub extern "Rust" fn get_agent_status_scripts(agent: Hash40) -> Option<Arc<RwLock<HashMap<i32, StatusInfo>>>> {
        CUSTOM_STATUS_MANAGER.read().agent_statuses.get(&agent).map(|x| x.clone())
    }
}

#[repr(C)]
pub struct CustomStatusModule {
    pub common_statuses: Arc<RwLock<HashMap<i32, StatusInfo>>>,
    pub agent_statuses: Option<Arc<RwLock<HashMap<i32, StatusInfo>>>>,
    pub vanilla_status_max: i32,
    pub common_status_end: i32,
}

impl CustomStatusModule {
    pub fn new() -> Self {
        Self {
            common_statuses: CustomStatusManager::get_common_status_scripts(),
            agent_statuses: None,
            vanilla_status_max: 0,
            common_status_end: 0
        }
    }

    pub(crate) fn init_from_hash(object: *mut BattleObject, max: i32) -> i32 {
        let module = require_status_module!(object);

        unsafe {
            module.agent_statuses = CustomStatusManager::get_agent_status_scripts((*object).agent_kind_hash);
        }

        module.vanilla_status_max = max;

        let mut common_max = -1;
        let mut agents_max = -1;

        let common = module.common_statuses.read();
        for (id, _) in common.iter() {
            common_max = common_max.max(*id);
        }
        
        if let Some(agents) = module.agent_statuses.as_ref().map(|x| x.read()) {
            for (id, _) in agents.iter() {
                agents_max = agents_max.max(*id);
            }
        }

        module.common_status_end = module.vanilla_status_max + common_max + 1;
        module.common_status_end + agents_max + 1
    }

    pub(crate) fn install_statuses(object: *mut BattleObject, agent: &mut L2CFighterCommon) {
        let module = require_status_module!(object);

        let common = module.common_statuses.read();

        for (id, info) in common.iter() {
            let kind = module.vanilla_status_max + *id;
            if let Some(func) = info.pre.clone() {
                set_status_func(agent, kind, 0, func);
            }
            if let Some(func) = info.main.clone() {
                set_status_func(agent, kind, 1, func);
            }
            if let Some(func) = info.end.clone() {
                set_status_func(agent, kind, 2, func);
            }
            if let Some(func) = info.init.clone() {
                set_status_func(agent, kind, 3, func);
            }
            if let Some(func) = info.exec.clone() {
                set_status_func(agent, kind, 4, func);
            }
            if let Some(func) = info.exec_stop.clone() {
                set_status_func(agent, kind, 5, func);
            }
            if let Some(func) = info.exec_post.clone() {
                set_status_func(agent, kind, 6, func);
            }
            if let Some(func) = info.exit.clone() {
                set_status_func(agent, kind, 7, func);
            }
            if let Some(func) = info.map_correction.clone() {
                set_status_func(agent, kind, 8, func);
            }
            if let Some(func) = info.fix_camera.clone() {
                set_status_func(agent, kind, 9, func);
            }
            if let Some(func) = info.fix_pos_slow.clone() {
                set_status_func(agent, kind, 10, func);
            }
            if let Some(func) = info.check_damage.clone() {
                set_status_func(agent, kind, 11, func);
            }
            if let Some(func) = info.check_attack.clone() {
                set_status_func(agent, kind, 12, func);
            }
            if let Some(func) = info.on_change_lr.clone() {
                set_status_func(agent, kind, 13, func);
            }
            if let Some(func) = info.leave_stop.clone() {
                set_status_func(agent, kind, 14, func);
            }
            if let Some(func) = info.notify_event_gimmick.clone() {
                set_status_func(agent, kind, 15, func);
            }
            if let Some(func) = info.calc_param.clone() {
                set_status_func(agent, kind, 16, func);
            }
        }

        if let Some(agents) = module.agent_statuses.as_ref().map(|x| x.read()) {
            
            for (id, info) in agents.iter() {
                let kind = module.common_status_end + *id;
                println!("installing {} | {}", id, kind);
                if let Some(func) = info.pre.clone() {
                    set_status_func(agent, kind, 0, func);
                }
                if let Some(func) = info.main.clone() {
                    set_status_func(agent, kind, 1, func);
                }
                if let Some(func) = info.end.clone() {
                    set_status_func(agent, kind, 2, func);
                }
                if let Some(func) = info.init.clone() {
                    set_status_func(agent, kind, 3, func);
                }
                if let Some(func) = info.exec.clone() {
                    set_status_func(agent, kind, 4, func);
                }
                if let Some(func) = info.exec_stop.clone() {
                    set_status_func(agent, kind, 5, func);
                }
                if let Some(func) = info.exec_post.clone() {
                    set_status_func(agent, kind, 6, func);
                }
                if let Some(func) = info.exit.clone() {
                    set_status_func(agent, kind, 7, func);
                }
                if let Some(func) = info.map_correction.clone() {
                    set_status_func(agent, kind, 8, func);
                }
                if let Some(func) = info.fix_camera.clone() {
                    set_status_func(agent, kind, 9, func);
                }
                if let Some(func) = info.fix_pos_slow.clone() {
                    set_status_func(agent, kind, 10, func);
                }
                if let Some(func) = info.check_damage.clone() {
                    set_status_func(agent, kind, 11, func);
                }
                if let Some(func) = info.check_attack.clone() {
                    set_status_func(agent, kind, 12, func);
                }
                if let Some(func) = info.on_change_lr.clone() {
                    set_status_func(agent, kind, 13, func);
                }
                if let Some(func) = info.leave_stop.clone() {
                    set_status_func(agent, kind, 14, func);
                }
                if let Some(func) = info.notify_event_gimmick.clone() {
                    set_status_func(agent, kind, 15, func);
                }
                if let Some(func) = info.calc_param.clone() {
                    set_status_func(agent, kind, 16, func);
                }
            }
        } 
    }

    #[export_name = "CustomStatusModule__get_agent_status_kind"]
    pub extern "Rust" fn get_agent_status_kind(object: *mut BattleObject, id: i32) -> i32 {
        let module = require_status_module!(object);
        module.common_status_end + id
    }

    #[export_name = "CustomStatusModule__get_common_status_kind"]
    pub extern "Rust" fn get_common_status_kind(object: *mut BattleObject, id: i32) -> i32 {
        let module = require_status_module!(object);
        module.vanilla_status_max + id
    }
}

fn set_status_func(agent: &mut L2CFighterBase, kind: i32, condition: i32, func: StatusFunc) {
    unsafe {
        agent.sv_set_status_func(
            kind.into(),
            condition.into(),
            std::mem::transmute(func)
        )
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CAgentBase_reserve_status_data_array)]
unsafe fn reserve_status_data_array_hook(agent: &mut L2CFighterCommon, count: i32) {
    let obj = agent.battle_object;
    if !has_status_module!(obj) {
        call_original!(agent, count);
        return;
    }

    let new_count = CustomStatusModule::init_from_hash(obj, count);
    agent.global_table[crate::consts::globals::STATUS_COUNT].assign(&new_count.into());
    call_original!(agent, new_count);


    CustomStatusModule::install_statuses(agent.battle_object, agent);
}

pub(crate) fn init() {
    fn nro_hook(info: &NroInfo) {
        if info.name == "common" {
            skyline::install_hook!(reserve_status_data_array_hook);
        }
    }

    skyline::nro::add_hook(nro_hook);
}