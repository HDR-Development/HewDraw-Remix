use crate::*;

#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x18]
pub struct MotionScriptState {
    #[offset = 0x00]
    motion_kind: phx::Hash40,
    #[offset = 0x08]
    start_frame: f32,
    #[offset = 0x0C]
    motion_rate: f32,
    #[offset = 0x10]
    alternate_fighter_kind: i32,
    #[offset = 0x14]
    disable_excute: bool,
    #[offset = 0x15]
    disable_acmd: bool,
    #[offset = 0x16]
    use_alternate_fighter_kind: bool,
}
#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x118]
pub struct MotionAnimcmdState {
    #[offset = 0x000]
    pub owner: *mut app::BattleObjectModuleAccessor,
    #[offset = 0x008]
    pub agent_kind: i32,
    #[offset = 0x00C]
    pub _xC: i32,
    #[offset = 0x010]
    pub entry_id: app::FighterEntryID,
    #[offset = 0x018]
    pub main_script: MotionScriptState,
    #[offset = 0x030]
    pub partial_script: MotionScriptState,
    #[offset = 0x048]
    pub main_update_rate: f32,
    #[offset = 0x04C]
    pub partial_update_rate: f32,
    #[offset = 0x050]
    pub sleep_game: bool,
    #[offset = 0x051]
    pub sleep_expression: bool,
    #[offset = 0x052]
    pub sleep_sound: bool,
    #[offset = 0x053]
    pub sleep_effect: bool,
    #[offset = 0x058]
    pub self_main_agents: [*mut lib::AnimcmdAgent; 4],
    #[offset = 0x078]
    pub self_partial_agents: [*mut lib::AnimcmdAgent; 4],
    #[offset = 0x098]
    pub self_agent_kinds: [i32; 4],
    #[offset = 0x0A8]
    pub shared_agent_kinds: [i32; 4],
    #[offset = 0x0B8]
    pub shared_agents: [*mut lib::AnimcmdAgent; 4],
    #[offset = 0x0D8]
    pub _padding: u64,
    #[offset = 0x0E0]
    pub function_object: [u64; 6],
    #[offset = 0x110]
    pub skip_delay_update: bool,
    #[offset = 0x111]
    pub is_sleep: bool,
    #[offset = 0x112]
    pub change_main_immediate: bool,
    #[offset = 0x113]
    pub change_partial_immediate: bool,
}

#[repr(C)]
#[vtable_impl(MotionAnimcmdModule)]
#[derive(TypeAssert)]
#[size = 0xE8]
pub(crate) struct MotionAnimcmdModuleVTable {
    #[offset = 0x0]
    destructor: extern "C" fn(this: &mut MotionAnimcmdModule),

    #[offset = 0x8]
    deleter: extern "C" fn(this: &mut MotionAnimcmdModule),

    /// Checks if this module is implemented
    ///
    /// ### Returns
    /// * `true` - The module is **not** implemented
    /// * `false` - The module **is** implemented
    ///
    /// ### Notes
    /// If the module is not implemented, there should be no attempt to
    /// either of the following:
    /// * Get the [animcmd state](MotionAnimcmdState) from it
    /// * Cast it to any object's implementation and read fields
    #[offset = 0x10]
    pub is_virtual: extern "C" fn(this: &MotionAnimcmdModule),

    #[offset = 0x18]
    handle_int_msc_command:
        extern "C" fn(this: &mut MotionAnimcmdModule, command: &lib::MscCommand) -> lib::TValue,

    #[offset = 0x20]
    handle_float_msc_command:
        extern "C" fn(this: &mut MotionAnimcmdModule, command: &lib::MscCommand) -> lib::TValue,

    #[offset = 0x28]
    initialize: extern "C" fn(this: &mut MotionAnimcmdModule, module_init_args: *const u64),

    #[offset = 0x30]
    finalize: extern "C" fn(this: &mut MotionAnimcmdModule),

    #[offset = 0x38]
    start_module: extern "C" fn(this: &mut MotionAnimcmdModule),

    #[offset = 0x40]
    end_module: extern "C" fn(this: &mut MotionAnimcmdModule),

    #[offset = 0x48]
    unk_1: extern "C" fn(this: &mut MotionAnimcmdModule),

    #[offset = 0x50]
    cleanup_agents: extern "C" fn(this: &mut MotionAnimcmdModule),

    #[offset = 0x58]
    pub exec_motion_lines: extern "C" fn(this: &mut MotionAnimcmdModule, update_rate: f32) -> i32,

    #[offset = 0x60]
    pub exec_partial_motion_lines:
        extern "C" fn(this: &mut MotionAnimcmdModule, update_rate: f32) -> i32,

    #[offset = 0x68]
    pub change_script_motion_lines: extern "C" fn(
        this: &mut MotionAnimcmdModule,
        motion_kind: phx::Hash40,
        start_frame: f32,
        disable_excute: bool,
        disable_acmd: bool,
        rate: f32,
        alternate_agent: bool,
    ),

    #[offset = 0x70]
    pub change_script_motion_partial_lines: extern "C" fn(
        this: &mut MotionAnimcmdModule,
        motion_kind: phx::Hash40,
        start_frame: f32,
        disable_excute: bool,
        disable_acmd: bool,
        rate: f32,
        alternate_agent: bool,
    ),

    #[offset = 0x78]
    pub call_script_single: extern "C" fn(
        this: &mut MotionAnimcmdModule,
        acmd_category: i32,
        script_name: phx::Hash40,
        agent_kind: i32,
    ),

    #[offset = 0x80]
    pub change_script_motion_line_single: extern "C" fn(
        this: &mut MotionAnimcmdModule,
        acmd_category: i32,
        motion_kind: phx::Hash40,
        agent_kind: i32,
    ),

    #[offset = 0x88]
    pub exec_motion_lines_initialize: extern "C" fn(
        this: &mut MotionAnimcmdModule,
        start_frame: f32,
        disable_excute: bool,
    ) -> i32,

    #[offset = 0x90]
    pub flush_current_motion: extern "C" fn(this: &mut MotionAnimcmdModule),

    #[offset = 0x98]
    pub flush: extern "C" fn(this: &mut MotionAnimcmdModule),

    #[offset = 0xA0]
    pub set_sleep: extern "C" fn(this: &mut MotionAnimcmdModule, sleep: bool),

    #[offset = 0xA8]
    pub set_sleep_game: extern "C" fn(this: &mut MotionAnimcmdModule, sleep: bool),

    #[offset = 0xB0]
    pub set_sleep_effect: extern "C" fn(this: &mut MotionAnimcmdModule, sleep: bool),

    #[offset = 0xB8]
    pub set_sleep_sound: extern "C" fn(this: &mut MotionAnimcmdModule, sleep: bool),

    #[offset = 0xC0]
    pub is_sleep: extern "C" fn(this: &MotionAnimcmdModule) -> bool,

    #[offset = 0xC8]
    pub enable_skip_delay_update: extern "C" fn(this: &mut MotionAnimcmdModule),

    #[offset = 0xD0]
    pub set_change_partial_immediate:
        extern "C" fn(this: &mut MotionAnimcmdModule, change_immediate: bool),

    #[offset = 0xD8]
    set_function_object: extern "C" fn(this: &mut MotionAnimcmdModule, object: *const u64),

    #[offset = 0xE0]
    unk_2: extern "C" fn(this: &mut MotionAnimcmdModule),
}

#[repr(C)]
pub struct MotionAnimcmdModule {
    vtable: &'static MotionAnimcmdModuleVTable,
    state: *mut MotionAnimcmdState,
}
