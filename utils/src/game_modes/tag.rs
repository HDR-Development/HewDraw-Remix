use parking_lot::RwLock;
use smash::app::{self, BattleObject, BattleObjectModuleAccessor, lua_bind::*};
use smash::lib::lua_const::*;
use smash::phx::{Hash40, Vector4f};
use smash::hash40;
use crate::modules::*;
use utils_dyn::consts::*;
use utils_dyn::ext::*;
use smash::phx::Vector3f;
use crate::util;

const TAG_DURATION: usize = 60 * 30;
#[repr(C)]
struct TagInfo {
    pub target_die_frame: usize,
    pub target_id: u32,
    pub last_damage_dealt: f32,
    pub last_damage_receiver_id: u32,
    pub aura_effect_handle: u32,
    pub is_init: bool
}

lazy_static! {
    static ref TAG_STATE: RwLock<TagInfo> = RwLock::new(TagInfo {
        target_die_frame: 0,
        target_id: 0,
        last_damage_dealt: 0.0,
        last_damage_receiver_id: 0,
        aura_effect_handle: 0,
        is_init: false
    });
}

impl TagInfo {
    fn is_id_active(object: u32) -> bool {
        let object = unsafe { util::get_battle_object_from_id(object) };
        if object.is_null() { return false; }
        let status = unsafe {
            StatusModule::status_kind((*object).module_accessor)
        };
        if status != *FIGHTER_STATUS_KIND_NONE && status != *FIGHTER_STATUS_KIND_STANDBY {
            true
        } else {
            unsafe {
                let entry_id = WorkModule::get_int((*object).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
                let info = app::lua_bind::FighterManager::get_fighter_information(crate::singletons::FighterManager(), app::FighterEntryID(entry_id));
                if info.is_null() {
                    false
                } else {
                    app::lua_bind::FighterInformation::stock_count(info) > 0
                }
            }
        }
    }

    pub fn get_target(&self) -> Option<*mut BattleObject> {
        unsafe {
            match self.is_init {
                true => Some(util::get_battle_object_from_id(self.target_id)),
                false => None
            }
        }
    }

    pub fn find_random_active_fighter() -> u32 {
        unsafe {
            loop {
                let random_id = app::sv_math::rand(hash40("stage"), 8) as u32;
                if let Some(obj_id) = util::get_active_battle_object_id_from_entry_id(random_id) {
                    if Self::is_id_active(obj_id) {
                        break obj_id;
                    }
                }
            }
        }
    }

    pub fn handle_active_id_change(&mut self) {
        if let Some(target) = self.get_target() {
            let entry_id = unsafe {
                WorkModule::get_int((*target).module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32
            };
            let active_id = util::get_active_battle_object_id_from_entry_id(entry_id).unwrap_or(*BATTLE_OBJECT_ID_INVALID as u32);
            if active_id != *BATTLE_OBJECT_ID_INVALID as u32 && active_id != self.target_id {
                self.switch_to_id(active_id);
            }
        }
    }

    pub fn switch_to_id(&mut self, id: u32) {
        if let Some(current_target) = self.get_target() {
            unsafe {
                if EffectModule::is_exist_effect((*current_target).module_accessor, self.aura_effect_handle) {
                    EffectModule::kill((*current_target).module_accessor, self.aura_effect_handle, true, true);
                }
                ColorBlendModule::cancel_main_color((*current_target).module_accessor, 0);
            }
        }
        self.target_id = id;
        unsafe {
            let new_target = util::get_battle_object_from_id(self.target_id);
            self.last_damage_dealt = VarModule::get_float(new_target, vars::common::instance::LAST_ATTACK_DAMAGE_DEALT);
            self.last_damage_receiver_id = VarModule::get_int(new_target, vars::common::instance::LAST_ATTACK_RECEIVER_ENTRY_ID) as u32;
            self.aura_effect_handle = EffectModule::req_follow(
                (*new_target).module_accessor,
                Hash40::new("sys_final_aura"),
                Hash40::new("head"),
                &Vector3f::zero(),
                &Vector3f::zero(),
                3.0,
                true,
                0,
                0,
                0,
                0,
                0,
                true,
                true
            ) as u32;
        }
    }

    pub fn handle_hit(&mut self) {
        if let Some(target) = self.get_target() {
            let damage = VarModule::get_float(target, vars::common::instance::LAST_ATTACK_DAMAGE_DEALT);
            let receiver = VarModule::get_int(target, vars::common::instance::LAST_ATTACK_RECEIVER_ENTRY_ID) as u32;
            if damage != self.last_damage_dealt || receiver != self.last_damage_receiver_id {
                self.switch_to_id(receiver);
            }
        }
    }

    /// remaining_frames: how many frames are left before a player is killed
    pub fn update_eff(&mut self) {
        if let Some(target) = self.get_target() {
            unsafe {
                let remaining_frames = self.target_die_frame - util::get_global_frame_count();
                let boma = &mut *(*target).module_accessor;
                if !EffectModule::is_exist_effect(boma, self.aura_effect_handle) {
                    self.aura_effect_handle = EffectModule::req_follow(
                        boma,
                        Hash40::new("sys_final_aura"),
                        Hash40::new("head"),
                        &Vector3f::zero(),
                        &Vector3f::zero(),
                        3.0,
                        true,
                        0,
                        0,
                        0,
                        0,
                        0,
                        true,
                        true
                    ) as u32;
                }

                // check if we need to change color due to impending timeout
                if remaining_frames < 60 {
                    ColorBlendModule::set_main_color(boma, &Vector4f{ x: 1.0, y: 0.2, z: 0.2, w: 0.7 }, &Vector4f{ x: 1.0, y: 0.2, z: 0.2, w: 0.7 }, 1.0, 0.5, 10, true);
                } else if remaining_frames < 120 {
                    ColorBlendModule::set_main_color(boma, &Vector4f{ x: 0.7, y: 0.2, z: 0.2, w: 0.7 }, &Vector4f{ x: 0.7, y: 0.2, z: 0.2, w: 0.7 }, 1.0, 0.5, 10, true);
                } else if remaining_frames < 180 {
                    ColorBlendModule::set_main_color(boma, &Vector4f{ x: 0.4, y: 0.2, z: 0.2, w: 0.7 }, &Vector4f{ x: 0.4, y: 0.2, z: 0.2, w: 0.7 }, 1.0, 0.5, 10, true);
                } else {
                    ColorBlendModule::cancel_main_color(boma, 0);
                }

            }
        }
    }

    pub fn handle_die(&mut self) {
        if let Some(target) = self.get_target() {
            unsafe {
                StatusModule::change_status_request((*target).module_accessor, *FIGHTER_STATUS_KIND_DEAD, false);
            }
        }
    }

    pub fn find_new_target(&mut self) {
        loop {
            let id = Self::find_random_active_fighter();
            if id != self.target_id {
                self.switch_to_id(id);
                break;
            }
        }
    }

    pub fn update(&mut self) {
        unsafe {

            // skip this frame because the match hasnt started
            if !app::sv_information::is_ready_go() {
                if self.is_init {
                    self.target_die_frame += 1;
                }
                return;
            }

            if !self.is_init {
                let id = Self::find_random_active_fighter();
                self.switch_to_id(id);
                self.target_die_frame += TAG_DURATION;
                self.is_init = true;
            }

            self.handle_hit();
            self.handle_active_id_change();

            if self.target_die_frame > util::get_global_frame_count() {
                self.update_eff();
                return;
            }

            self.handle_die();
            self.find_new_target();
            self.target_die_frame += TAG_DURATION;
        }
    }
}

pub fn clear() {
    *TAG_STATE.write() = TagInfo {
        target_die_frame: 0,
        target_id: 0,
        last_damage_dealt: 0.0,
        last_damage_receiver_id: 0,
        aura_effect_handle: 0,
        is_init: false
    };
}

pub fn update() {
    TAG_STATE.write().update();
}