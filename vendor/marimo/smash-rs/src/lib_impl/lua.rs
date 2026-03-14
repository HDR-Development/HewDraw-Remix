use crate::*;

use app::*;

#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x1C0]
pub struct LuaStateAccessor {
    #[offset = 0x000]
    pub _x0: [u64; 3],
    #[offset = 0x018]
    pub posture_module: *mut Module,
    #[offset = 0x020]
    pub _x20: u64,
    #[offset = 0x028]
    pub status_module: *mut StatusModule,
    #[offset = 0x030]
    pub control_module: *mut Module,
    #[offset = 0x038]
    pub work_module: *mut WorkModule,
    #[offset = 0x040]
    pub ground_module: *mut Module,
    #[offset = 0x048]
    pub camera_module: *mut Module,
    #[offset = 0x050]
    pub kinetic_module: *mut Module,
    #[offset = 0x058]
    pub color_blend_module: *mut Module,
    #[offset = 0x060]
    pub model_module: *mut Module,
    #[offset = 0x068]
    pub physics_module: *mut Module,
    #[offset = 0x070]
    pub motion_module: *mut Module,
    #[offset = 0x078]
    pub stop_module: *mut Module,
    #[offset = 0x080]
    pub article_module: *mut Module,
    #[offset = 0x088]
    pub attack_module: *mut Module,
    #[offset = 0x090]
    pub damage_module: *mut Module,
    #[offset = 0x098]
    pub hit_module: *mut Module,
    #[offset = 0x0A0]
    pub combo_module: *mut Module,
    #[offset = 0x0A8]
    pub area_module: *mut Module,
    #[offset = 0x0B0]
    pub item_module: *mut Module,
    #[offset = 0x0B8]
    pub link_module: *mut Module,
    #[offset = 0x0C0]
    pub team_module: *mut Module,
    #[offset = 0x0C8]
    pub search_module: *mut Module,
    #[offset = 0x0D0]
    pub unk_module_1: *mut Module,
    #[offset = 0x0D8]
    pub turn_module: *mut Module,
    #[offset = 0x0E0]
    pub reflect_module: *mut Module,
    #[offset = 0x0E8]
    pub shield_module: *mut Module,
    #[offset = 0x0F0]
    pub absorber_module: *mut Module,
    #[offset = 0x0F8]
    pub reflector_module: *mut Module,
    #[offset = 0x100]
    pub jostle_module: *mut Module,
    #[offset = 0x108]
    pub catch_module: *mut Module,
    #[offset = 0x110]
    pub cancel_module: *mut CancelModule,
    #[offset = 0x118]
    pub unk_module_2: *mut Module,
    #[offset = 0x120]
    pub capture_module: *mut Module,
    #[offset = 0x128]
    pub effect_module: *mut Module,
    #[offset = 0x130]
    pub sound_module: *mut Module,
    #[offset = 0x138]
    pub visibility_module: *mut Module,
    #[offset = 0x140]
    pub grab_module: *mut Module,
    #[offset = 0x148]
    pub slope_module: *mut Module,
    #[offset = 0x150]
    pub shake_module: *mut Module,
    #[offset = 0x158]
    pub slow_module: *mut Module,
    #[offset = 0x160]
    pub unk_module_3: *mut Module,
    #[offset = 0x168]
    pub shadow_module: *mut Module,
    #[offset = 0x170]
    pub motion_animcmd_module: *mut MotionAnimcmdModule,
    #[offset = 0x178]
    pub lua_module: *mut LuaModule,
    #[offset = 0x180]
    pub ink_paint_module: *mut Module,
    #[offset = 0x188]
    pub _x188: u64,
    #[offset = 0x190]
    pub battle_object_id: u32,
    #[offset = 0x194]
    pub _x194: u32,
    #[offset = 0x198]
    pub agent_kind: i32,
    #[offset = 0x19C]
    pub entry_id: app::FighterEntryID,
    #[offset = 0x1A0]
    pub module_accessor: *mut BattleObjectModuleAccessor,
    #[offset = 0x1A8]
    pub battle_object: *mut BattleObject,
    #[offset = 0x1B0]
    pub animcmd_state: *mut lib::LuaAnimcmdState,
    #[offset = 0x1B8]
    pub animcmd_exec_state: *mut lib::LuaAnimcmdExecutionState,
}

#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x30]
pub struct LuaThread {
    #[offset = 0x00]
    vtable: *const *const skyline::libc::c_void,
    #[offset = 0x08]
    state: *mut lua_State,
    #[offset = 0x10]
    _xC: u32,
    #[offset = 0x18]
    thread_name: phx::Hash40,
    #[offset = 0x20]
    parent_thread_name: phx::Hash40,
    #[offset = 0x28]
    _x28: u32,
    #[offset = 0x2C]
    _x2C: u32,
}

#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x50]
pub struct LuaThreadInfo {
    #[offset = 0x00]
    thread: *mut LuaThread,
    #[offset = 0x08]
    coroutines: [lib::TValue; 4],
    #[offset = 0x48]
    is_release_control: bool,
}
