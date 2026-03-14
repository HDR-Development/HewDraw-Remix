use std::ops::{Deref, DerefMut};

use crate::*;

pub type StatusFn = extern "C" fn(*mut L2CAgentBase) -> lib::L2CValueHack;

extern "C" {
    #[link_name = "_ZN7lua2cpp12L2CAgentBase14call_coroutineEiN3phx6Hash40E"]
    fn call_coroutine(this: *mut L2CAgentBase, idx: i32, function_hash: phx::Hash40) -> phx::Fiber;

    #[link_name = "_ZN7lua2cpp12L2CAgentBase14change_contextEPN3app12BattleObjectEPNS1_26BattleObjectModuleAccessorE"]
    fn change_context(this: *mut L2CAgentBase, object: *mut app::BattleObject, module_accessor: *mut app::BattleObjectModuleAccessor);

    #[link_name = "_ZN7lua2cpp12L2CAgentBase15clean_coroutineEi"]
    fn clean_coroutine(this: *mut L2CAgentBase) -> bool;

    #[link_name = "_ZN7lua2cpp12L2CAgentBase15coroutine_yieldEv"]
    fn coroutine_yield(this: *mut L2CAgentBase) -> bool;

    #[link_name = "_ZNK7lua2cpp12L2CAgentBase16get_parent_fiberEv"]
    fn get_parent_fiber(this: *mut L2CAgentBase) -> phx::Fiber;

    #[link_name = "_ZNK7lua2cpp12L2CAgentBase26get_unused_coroutine_indexEi"]
    fn get_unused_coroutine_index(this: *mut L2CAgentBase, max: i32) -> i32;

    #[link_name = "_ZNK7lua2cpp12L2CAgentBase28is_coroutine_release_controlEv"]
    fn is_coroutine_release_control(this: *mut L2CAgentBase) -> bool;

    #[link_name = "_ZN7lua2cpp12L2CAgentBase25reserve_status_data_arrayEj"]
    fn reserve_status_data_array(this: *mut L2CAgentBase, length: u32);
    
    #[link_name = "_ZN7lua2cpp12L2CAgentBase16resume_coroutineEiRi"]
    fn resume_coroutine(this: *mut L2CAgentBase, index: i32, success: *mut i32) -> i32;

    #[link_name = "_ZN7lua2cpp12L2CAgentBase13run_coroutineEiRiN5boost8optionalIPN3phx5FiberEEE"]
    fn run_coroutine(this: *mut L2CAgentBase, index: i32, success: *mut i32, parent: cpp::Optional<*mut phx::Fiber>) -> i32;

    #[link_name = "_ZN7lua2cpp12L2CAgentBase29set_coroutine_release_controlEb"]
    fn set_coroutine_release_control(this: *mut L2CAgentBase, coroutine_release_control: bool);

    #[link_name = "_ZN7lua2cpp12L2CAgentBase15start_coroutineEiN3phx6Hash40ERi"]
    fn start_coroutine(this: *mut L2CAgentBase, index: i32, function_name: phx::Hash40, success: *mut i32) -> i32;

    #[link_name = "_ZN7lua2cpp12L2CAgentBase19sv_copy_status_funcERKN3lib8L2CValueES4_S4_"]
    fn sv_copy_status_func(this: *mut L2CAgentBase, dst_status_kind: *const lib::L2CValue, src_status_kind: *const lib::L2CValue, condition: *const lib::L2CValue);

    #[link_name = "_ZN7lua2cpp12L2CAgentBase21sv_delete_status_funcERKN3lib8L2CValueES4_"]
    fn sv_delete_status_func(this: *mut L2CAgentBase, status_kind: *const lib::L2CValue, condition: *const lib::L2CValue);

    #[link_name = "_ZN7lua2cpp12L2CAgentBase18sv_get_status_funcERKN3lib8L2CValueES4_"]
    fn sv_get_status_func(this: *mut L2CAgentBase, status_kind: *const lib::L2CValue, condition: *const lib::L2CValue) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp12L2CAgentBase18sv_set_status_funcERKN3lib8L2CValueES4_Pv"]
    fn sv_set_status_func(this: *mut L2CAgentBase, status_kind: *const lib::L2CValue, condition: *const lib::L2CValue, func: StatusFn);
}

#[repr(C)]
struct StatusData {
    pub pre: StatusFn,
    pub main: StatusFn,
    pub end: StatusFn,
    pub init: StatusFn,
    pub exec: StatusFn,
    pub exec_stop: StatusFn,
    pub exit: StatusFn,
    pub map_correction: StatusFn,
    pub fix_camera: StatusFn,
    pub fix_pos_slow: StatusFn,
    pub check_damage: StatusFn,
    pub check_attack: StatusFn,
    pub on_change_lr: StatusFn,
    pub leave_stop: StatusFn,
    pub notify_event_gimmick: StatusFn,
    pub calc_param: StatusFn,
    pub reserve1: StatusFn,
    pub reserve2: StatusFn,
    pub reserve3: StatusFn,
    pub reserve4: StatusFn,
    pub reserve5: StatusFn
}

#[repr(C)]
struct Coroutine {
    fiber: *mut phx::Fiber,
    parent_fiber: *mut phx::Fiber,
    status: u32,
    is_running: bool,
    padding: [u8; 3]
}

#[repr(C)]
pub struct L2CAgentBase {
    agent: lib::L2CAgent,
    statuses: cpp::Vector<StatusData>,
    coroutines: [Coroutine; 4],
    yield_via_exception: bool,
    coroutine_release_control: bool,
    padding: [u8; 6]
}

impl Deref for L2CAgentBase {
    type Target = lib::L2CAgent;

    fn deref(&self) -> &Self::Target {
        &self.agent
    }
}

impl DerefMut for L2CAgentBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.agent
    }
}

impl L2CAgentBase {
    /// Starts a coroutine on this agent
    /// # Arguments
    /// * `coroutine_idx`: The index (0-3) of which coroutine to call this function on.
    /// * `function_hash`: The hash of the function to call for this coroutine
    /// 
    /// Returns the parent of the fiber that begins running the coroutine
    pub fn call_coroutine(&mut self, coroutine_idx: i32, function_hash: phx::Hash40) -> phx::Fiber {
        unsafe {
            call_coroutine(self, coroutine_idx, function_hash)
        }
    }

    pub fn change_context(&mut self, object: *mut app::BattleObject, module_accessor: *mut app::BattleObjectModuleAccessor) {
        unsafe {
            change_context(self, object, module_accessor)
        }
    }

    /// Checks all 4 coroutines to see if they are finished and if they have finished and finalizes them if so
    pub fn clean_coroutine(&mut self) -> bool {
        unsafe {
            clean_coroutine(self)
        }
    }

    /// Yields the current coroutine back to the parent
    /// WARNING: If this function is called from Rust code *without* either an EH frame or any other way
    /// to avoid an invalid EH frame, then there this will crash
    pub fn coroutine_yield(&mut self) -> bool {
        unsafe {
            coroutine_yield(self)
        }
    }

    /// Gets the parent of the current operating fiber, returns `null` if the current fiber is not one of this agent's
    pub fn get_parent_fiber(&mut self) -> phx::Fiber {
        unsafe {
            get_parent_fiber(self)
        }
    }

    /// Gets the index of the first coroutine that is not currently in use
    pub fn get_unused_coroutine_index(&mut self, max: i32) -> i32 {
        unsafe {
            get_unused_coroutine_index(self, max)
        }
    }

    pub fn is_coroutine_release_control(&mut self) -> bool {
        unsafe {
            is_coroutine_release_control(self)
        }
    }

    /// Sets the capacity of the status data array
    pub fn reserve_status_data_array(&mut self, length: u32) {
        unsafe {
            reserve_status_data_array(self, length)
        }
    }

    /// Resumes the coroutine on the given index
    pub fn resume_coroutine(&mut self, index: i32, success: &mut i32) -> i32 {
        unsafe {
            resume_coroutine(self, index, success)
        }
    }

    /// Runs a coroutine on the given index, with an optional parent argument
    pub fn run_coroutine(&mut self, index: i32, success: &mut i32, parent: Option<*mut phx::Fiber>) -> i32 {
        unsafe {
            run_coroutine(self, index, success, parent.into())
        }
    }

    /// Sets the coroutine release control
    pub fn set_coroutine_release_control(&mut self, coroutine_release_control: bool) {
        unsafe {
            set_coroutine_release_control(self, coroutine_release_control)
        }
    }

    /// Starts the coroutine on the given index with the given function name
    pub fn start_coroutine(&mut self, index: i32, func_name: phx::Hash40, success: &mut i32) -> i32 {
        unsafe {
            start_coroutine(self, index, func_name, success)
        }
    }

    /// Copies the status function specified to the other status kind
    pub fn sv_copy_status_func(&mut self, dst_status_kind: &lib::L2CValue, src_status_kind: &lib::L2CValue, condition: &lib::L2CValue) {
        unsafe {
            sv_copy_status_func(self, dst_status_kind, src_status_kind, condition)
        }
    }

    /// Deletes the status function specified
    pub fn sv_delete_status_func(&mut self, status_kind: &lib::L2CValue, condition: &lib::L2CValue) {
        unsafe {
            sv_delete_status_func(self, status_kind, condition)
        }
    }

    /// Get the status function specified
    pub fn sv_get_status_func(&mut self, status_kind: &lib::L2CValue, condition: &lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sv_get_status_func(self, status_kind, condition).into()
        }
    }

    /// Set the status function specified
    pub fn sv_set_status_func(&mut self, status_kind: &lib::L2CValue, condition: &lib::L2CValue, func: StatusFn) {
        unsafe {
            sv_set_status_func(self, status_kind, condition, func)
        }
    }
}

#[cfg(feature = "type_assert")]
impl L2CAgentBase {
    pub fn assert() {
        assert_eq!(size_of!(L2CAgentBase), 0xC8);
        assert_eq!(offset_of!(L2CAgentBase, agent), 0x0);
        assert_eq!(offset_of!(L2CAgentBase, statuses), 0x48);
        assert_eq!(offset_of!(L2CAgentBase, coroutines), 0x60);
        assert_eq!(offset_of!(L2CAgentBase, yield_via_exception), 0xc0);
        assert_eq!(offset_of!(L2CAgentBase, coroutine_release_control), 0xc1);
    }
}