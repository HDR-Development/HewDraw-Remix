use crate::*;

pub type LuaAnimcmdContinueYield = extern "C" fn(*mut lua_State, f32) -> bool;

#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x40]
pub struct LuaAnimcmdSubState {
    #[offset = 0x00]
    agent: *mut AnimcmdAgent,
    #[offset = 0x08]
    coroutine_name: phx::Hash40,
    #[offset = 0x10]
    _x10: u64,
    #[offset = 0x18]
    wait_timer_sub: f32,
    #[offset = 0x1C]
    wait_timer_mul_sub: f32,
    #[offset = 0x20]
    current_frame: f32,
    #[offset = 0x24]
    enable_frame_advance: f32,
    #[offset = 0x28]
    coroutine_index: i32,
    #[offset = 0x2C]
    enabled: bool,
    #[offset = 0x2D]
    _x2D: bool,
    #[offset = 0x2E]
    is_available: bool,
    #[offset = 0x2F]
    is_yielding: bool,
    #[offset = 0x30]
    should_continue_yielding: LuaAnimcmdContinueYield,
    #[offset = 0x38]
    target_frame: f32,
}

#[repr(C)]
#[derive(TypeAssert)]
#[size = 0xC]
pub struct LuaAnimcmdExecutionState {
    #[offset = 0x0]
    rate: f32,
    #[offset = 0x4]
    elapsed_frames: f32,
    #[offset = 0x8]
    is_excute: bool,
    #[offset = 0x9]
    _x9: bool,
}

#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x98]
pub struct LuaAnimcmdState {
    #[offset = 0x00]
    current_state: LuaAnimcmdSubState,
    #[offset = 0x40]
    backup_state: LuaAnimcmdSubState,
    #[offset = 0x80]
    execution_state: LuaAnimcmdExecutionState,
    #[offset = 0x8C]
    backup_execution_state: LuaAnimcmdExecutionState,
}

#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x2A0]
pub struct AnimcmdAgent {
    #[offset = 0x000]
    pub thread_info: *mut lib::LuaThreadInfo,
    #[offset = 0x008]
    pub accessor: lib::LuaStateAccessor,
    #[offset = 0x1C8]
    pub animcmd_state: LuaAnimcmdState,
    #[offset = 0x260]
    pub function_map: cpp::HashMap<phx::Hash40, *const skyline::libc::c_void>,
    #[offset = 0x288]
    pub _x268: i32,
    #[offset = 0x290]
    pub lua2cpp_agent: *mut lib::L2CAgent,
    #[offset = 0x298]
    pub is_using_lua2cpp: bool,
}
