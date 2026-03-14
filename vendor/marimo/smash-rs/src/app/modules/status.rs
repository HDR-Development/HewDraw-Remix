use crate::*;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct StatusChange(u64);

impl StatusChange {
    /// Returns the status kind that this status change is transitioning to
    pub fn status_kind(self) -> i32 {
        (self.0 & 0xFFFF_FFFF) as i32
    }

    /// Checks if the status change is set to clear the buffers when the status change occurs
    pub fn clear_command(self) -> bool {
        (self.0 & 0xFF_0000_0000) != 0x0
    }
}

#[repr(C)]
#[vtable_impl(StatusModule)]
#[derive(TypeAssert)]
#[size = 0x1F8]
pub(crate) struct StatusModuleVTable {
    #[offset = 0x0]
    destructor: extern "C" fn(this: &mut StatusModule),

    #[offset = 0x8]
    deleter: extern "C" fn(this: &mut StatusModule),

    /// Checks if this module is implemented
    /// 
    /// ### Returns
    /// * `true` - The module is **not** implemented
    /// * `false` - The module **is** implemented
    /// 
    /// ### Notes
    /// If the module is not implemented, there should be no attempt to
    /// either of the following:
    /// * Get the [owner module accessor](app::BattleObjectModuleAccessor) from it
    /// * Cast it to any object's implementation and read fields
    #[offset = 0x10]
    pub is_virtual: extern "C" fn(this: &StatusModule) -> bool,

    #[offset = 0x18]
    handle_int_msc_command: extern "C" fn(this: &mut StatusModule, command: &lib::MscCommand) -> lib::TValue,

    #[offset = 0x20]
    handle_float_msc_command: extern "C" fn(this: &mut StatusModule, command: &lib::MscCommand) -> lib::TValue,

    /// Initializes the module
    /// ### Arguments
    /// * `prev_status_kinds` - A pointer to a [fixed size vector/circular array](cpp::FixedVec) of status kinds
    /// * `status_scripts` - A set of status scripts for this agent (usually found in the main executable, not to be confused with lua statuses)
    /// * `transition_info` -  A set of booleans that specify whether user can transition out of hitstop into a specific status (true means they cannot)
    /// * `added_line_mask` - A bitmask of which lines the module is allowed to call
    /// 
    /// ### Notes
    /// * The parameter `added_line_mask` only appears to have the lowest 8-bits checked by the fighter, weapon, and item implementations, however there is
    /// enough space for a 32-bit integer.
    /// * Even though the const generic field for the size on the `prev_status_kinds` parameter is `1`, the size of the vector depends on the caller (the
    /// generic field is used to specify how much space is taken by the array)
    #[offset = 0x28]
    initialize: extern "C" fn(
        this: &mut StatusModule,
        prev_status_kinds: *mut cpp::FixedVec<i32, 1>,
        status_scripts: *mut cpp::Array2<*const skyline::libc::c_void>,
        transition_info: *const cpp::Array<bool>,
        added_line_mask: u32
    ),

    /// Finalizes the module
    /// 
    /// ### Behavior (Fighter, Weapon, and Item)
    /// This zeros out all pointers in the status script array
    #[offset = 0x30]
    finalize: extern "C" fn(this: &mut StatusModule),

    /// Starts the module
    /// 
    /// ### Behavior (Fighter, Weapon, and Item)
    /// This method will:
    /// 1. Enable status execution and start by transitioning the fighter to [`app::FighterStatusKind::None`]
    /// without clearing the control buffer.
    /// 2. Initializes all flags and status request values to non-impactful values
    /// 3. [Clears all transition terms](app::WorkModule::clear_transition_term_forbid)
    #[offset = 0x38]
    start_module: extern "C" fn(this: &mut StatusModule),

    /// Ends the module
    /// 
    /// ### Behavior (Fighter, Weapon, and Items)
    /// Clears the previous status kind queue, sets current status kind to [`app::FighterStatusKind::None`],
    /// and sets the current situation kind to [`app::SituationKind::Outfield`]
    #[offset = 0x40]
    end_module: extern "C" fn(this: &mut StatusModule),

    /// Attempts to change the current status
    /// 
    /// ### Arguments
    /// * `status_kind` - The status kind to transition to
    /// * `clear_command` - Whether to clear the `ControlModule` buffer
    /// 
    /// ### Returns
    /// * `true` - The status change was successful and happened during this call
    /// * `false` - The status change was either not successful or was queued to happen later
    /// 
    /// ### Behavior (Fighter, Weapon, and Item)
    /// If there is currently a status running, then this function will internally call [`StatusModule::change_status_request_from_script`]
    /// and return false.
    /// 
    /// Otherwise, it checks if the current state of execution is doing battle object and ground collision detection. If it is,
    /// it will place the status change into a queue to happen the next time it is able to outside of the collision detection processing.
    /// 
    /// If it is neither in a status or being called from collision detection, it will then attempt to change the status script. Changing
    /// the status script will fail if the object is currently in hitstop and the status does not permit changing out of hitstop.
    #[offset = 0x48]
    pub change_status_request: extern "C" fn(this: &mut StatusModule, status_kind: i32, clear_command: bool) -> bool,

    /// Queues a status change for after the current script is not executing
    /// 
    /// ### Arguments
    /// * `status_kind` - The status kind to transition to
    /// * `clear_command` - Whether to clear the `ControlModule` buffer
    #[offset = 0x50]
    pub change_status_request_from_script: extern "C" fn(this: &mut StatusModule, status_kind: i32, clear_command: bool),

    /// Deletes the queued status change, set by either [`StatusModule::change_status_request`] or [`StatusModule::change_status_request_from_script`]
    /// 
    /// ### Notes
    /// This does not delete the `clear_command` flag which is also set by those functions, however that flag will get reset again by
    /// a new status queue regardless.
    #[offset = 0x58]
    pub delete_status_request_from_script: extern "C" fn(this: &mut StatusModule),

    /// Gets the currently queued status change, set by either [`StatusModule::change_status_request`] or [`StatusModule::change_status_request_from_script`]
    /// 
    /// ### Returns
    /// Will return the currently queued status kind, returning -1 if there is nothing queued
    #[offset = 0x60]
    pub status_kind_que_from_script: extern "C" fn(this: &StatusModule) -> i32,

    /// Runs the main status script from the lua agents
    /// 
    /// ### Behavior (Fighter, Weapon, & Item)
    /// If status execution is not disabled this function will first check if there were any statuses queued to be changed
    /// from the ground collision detection state, and if there are none it will call
    /// [the lua status sytem's main functions](app::LuaModule::call_line_status_system).
    /// 
    /// If there is a requested status change via [`StatusModule::change_status_request`] or [`StatusModule::change_status_request_from_script`]
    /// it will be transitioned to after the lua status functions have terminated
    #[offset = 0x68]
    pub run_lua_status: extern "C" fn(this: &mut StatusModule),

    /// Enables running the lua status script system
    /// 
    /// ### Notes
    /// Enabling the lua status system does not matter if the overall status system is disabled
    #[offset = 0x70]
    pub enable_lua_status: extern "C" fn(this: &mut StatusModule),

    /// Runs the line system script from the C++ status script set.
    /// 
    /// ### Notes
    /// This is similar to the `exec` status script functions.
    #[offset = 0x78]
    pub call_cpp_line_system: extern "C" fn(this: &mut StatusModule),

    /// Runs the hitstop line system script from the C++ status script set.
    /// 
    /// ### Notes
    /// This is similar to the `exec_stop` status script functions.
    #[offset = 0x80]
    pub call_cpp_line_system_stop: extern "C" fn(this: &mut StatusModule),

    /// Runs the line system status scripts from the lua status system
    /// 
    /// ### Notes
    /// This will call the `exec` status script if the object is not currently
    /// in hitstop, else it will call the `exec_stop` status script.
    #[offset = 0x88]
    pub call_lua_line_system: extern "C" fn(this: &mut StatusModule),

    /// Calls the 8th added line script
    /// 
    /// ### Notes
    /// While this function is generic across all agents, it's behavior varies per
    /// object/category
    #[offset = 0x90]
    call_added_line_7: extern "C" fn(this: &mut StatusModule),

    /// Calls the 1st added line script
    /// 
    /// ### Notes
    /// While this function is generic across all agents, it's behavior varies per
    /// object/category
    #[offset = 0x98]
    call_added_line_0: extern "C" fn(this: &mut StatusModule),
    
    #[offset = 0xA0]
    pub call_cpp_line_fix_pos2: extern "C" fn(this: &mut StatusModule),
    
    #[offset = 0xA8]
    pub call_cpp_line_fix_pos: extern "C" fn(this: &mut StatusModule),

    /// Calls the 1st added line script
    /// 
    /// ### Notes
    /// While this function is generic across all agents, it's behavior varies per
    /// object/category
    #[offset = 0xB0]
    call_added_line_6: extern "C" fn(this: &mut StatusModule),

    /// Calls the 2nd added line script
    /// 
    /// ### Notes
    /// While this function is generic across all agents, it's behavior varies per
    /// object/category
    #[offset = 0xB8]
    call_added_line_1: extern "C" fn(this: &mut StatusModule),

    /// Calls the 3rd added line script
    /// 
    /// ### Notes
    /// While this function is generic across all agents, it's behavior varies per
    /// object/category
    #[offset = 0xC0]
    call_added_line_2: extern "C" fn(this: &mut StatusModule),

    /// Calls the 4th added line script
    /// 
    /// ### Notes
    /// While this function is generic across all agents, it's behavior varies per
    /// object/category
    #[offset = 0xC8]
    call_added_line_3: extern "C" fn(this: &mut StatusModule),

    /// Calls the 5th added line script
    /// 
    /// ### Notes
    /// While this function is generic across all agents, it's behavior varies per
    /// object/category
    #[offset = 0xD0]
    call_added_line_4: extern "C" fn(this: &mut StatusModule),

    /// Calls the 6th added line script
    /// 
    /// ### Notes
    /// While this function is generic across all agents, it's behavior varies per
    /// object/category
    #[offset = 0xD8]
    call_added_line_5: extern "C" fn(this: &mut StatusModule),

    /// Calls the 1st reserved script from the C++ status script set
    /// 
    /// ### Notes
    /// While this function is generic across all agents, it's behavior varies per
    /// object/category
    #[offset = 0xE0]
    call_reserved_0: extern "C" fn(this: &mut StatusModule),

    /// Calls the 1st reserved script from the C++ status script set
    /// 
    /// ### Notes
    /// While this function is generic across all agents, it's behavior varies per
    /// object/category
    #[offset = 0xE8]
    call_reserved_1: extern "C" fn(this: &mut StatusModule),

    /// Calls the 2nd reserved script from the C++ status script set
    /// 
    /// ### Notes
    /// While this function is generic across all agents, it's behavior varies per
    /// object/category
    #[offset = 0xF0]
    call_reserved_2: extern "C" fn(this: &mut StatusModule),

    /// Calls the 3rd reserved script from the C++ status script set
    /// 
    /// ### Notes
    /// While this function is generic across all agents, it's behavior varies per
    /// object/category
    #[offset = 0xF8]
    call_reserved_3: extern "C" fn(this: &mut StatusModule),

    /// Calls the 4th reserved script from the C++ status script set
    /// 
    /// ### Notes
    /// While this function is generic across all agents, it's behavior varies per
    /// object/category
    #[offset = 0x100]
    call_reserved_4: extern "C" fn(this: &mut StatusModule),

    /// Calls the 5th reserved script from the C++ status script set
    /// 
    /// ### Notes
    /// While this function is generic across all agents, it's behavior varies per
    /// object/category
    #[offset = 0x108]
    call_reserved_5: extern "C" fn(this: &mut StatusModule),

    /// Gets the current status kind
    #[offset = 0x110]
    pub status_kind: extern "C" fn(this: &StatusModule) -> i32,

    /// Gets the next status kind
    #[offset = 0x118]
    pub status_kind_next: extern "C" fn(this: &StatusModule) -> i32,

    /// Sets the interrupting status kind
    /// 
    /// ### Notes
    /// When executing inside of the lua status script system's `pre` status, calling
    /// this function and returning `1` is the standard way to designate a different
    /// target status than the one that is currently being transitioned to. For example,
    /// autoturn characters will use this to change their shift from turn to backwalk
    #[offset = 0x120]
    pub set_status_kind_interrupt: extern "C" fn(this: &mut StatusModule, status_kind: i32),

    /// Gets the interrupting status kind
    #[offset = 0x128]
    pub status_kind_interrupt: extern "C" fn(this: &StatusModule) -> i32,

    /// Clears the flag designating that this object has updated
    #[offset = 0x130]
    pub clear_update_flag: extern "C" fn(this: &mut StatusModule),

    /// Acquires the flag designating that this object has updated.
    /// 
    /// ### Returns
    /// * `true` - The object has not yet updated and the caller is now responsible for updating it
    /// * `false` - The obejct has already updated
    /// 
    /// ### Notes
    /// This function is mostly assumed in what it's purpose is, however when it is called by the owning
    /// object, it usually checks if the flag is non-zero and then performs some updates on other modules.
    #[offset = 0x138]
    pub update: extern "C" fn(this: &mut StatusModule) -> bool,

    /// Checks if the object is currently in the process of changing statuses
    /// 
    /// ### Notes
    /// This function will return true on the first execution of the main status/exec status scripts
    #[offset = 0x140]
    pub is_changing: extern "C" fn(this: &StatusModule) -> bool,

    /// Gets the `index`-th previous status kind
    /// 
    /// ### Arguments
    /// * `index` - Which previous status you want to get
    /// 
    /// ### Notes
    /// * There is a [fixed-length vector](cpp::FixedVec) that maintains the list of previous status kinds. Any request
    /// for a status kind that is greater than the current length will return an invalid status kind
    /// * All previous status kinds are cleared by [`StatusModule::change_status_force_and_clear`]
    #[offset = 0x148]
    pub prev_status_kind: extern "C" fn(this: &StatusModule, index: u32) -> i32,

    /// Gets the status C++ status script set from the specified status kind
    /// 
    /// ### Arguments
    /// * `status_kind` - The status kind to get the status scripts for
    #[offset = 0x150]
    pub get_status_script: extern "C" fn(this: &StatusModule, status_kind: i32) -> *const skyline::libc::c_void,

    /// Attempts to immediately change the current status, ignoring whether or not
    /// the status is currently changing/ending, if there is one running, or if
    /// the status system(s) are disabled.
    /// 
    /// ### Arguments
    /// * `status_kind` - The status to change to
    /// * `clear_command` - Whether or not to clear the `ControlModule` buffer
    /// 
    /// ### Returns
    /// * `true` - The status was successfully changed
    /// * `false` - The status was unable to be changed (likely because the status transition info prevents
    /// it from changing during hitstop)
    #[offset = 0x158]
    pub change_status_force: extern "C" fn(this: &mut StatusModule, status_kind: i32, clear_command: bool) -> bool,

    /// Clears all previous status kinds and calls [`StatusModule::change_status_force`]
    /// 
    /// ### Arguments
    /// * `status_kind` - The status to change to
    /// 
    /// ### Returns
    /// * `true` - The status was successfully changed
    /// * `false` - The status was unable to be changed (likely because the status transition info prevents
    /// it from changing during hitstop)
    /// 
    /// ### Notes
    /// * Unlike the alternative status changing methods, this one does not provide an option to 
    /// clear the `ControlModule` buffer. It does not clear the buffer
    /// * The previous status kinds are cleared even if the status change is not performed successfully
    #[offset = 0x160]
    pub change_status_force_and_clear: extern "C" fn(this: &mut StatusModule, status_kind: i32) -> bool,

    /// Gets the current situation kind
    #[offset = 0x168]
    pub situation_kind: extern "C" fn(this: &StatusModule) -> app::SituationKind,

    /// Gets the previous situation kind
    #[offset = 0x170]
    pub prev_situation_kind: extern "C" fn(this: &StatusModule) -> app::SituationKind,

    /// Sets the current situation kind
    /// 
    /// ### Arguments
    /// * `situation_kind` - The situation kind to change to
    /// * `keep` - Whether or not this situation kind should be updated while in this status
    #[offset = 0x178]
    pub set_situation_kind: extern "C" fn(this: &mut StatusModule, situation_kind: app::SituationKind, keep: bool),

    /// Sets the ground check kinds for the current status (can be changed later)
    /// 
    /// ### Arguments
    /// * `correct` - The ground correction kind
    /// * `cliff_check` - The ground-cliff check kind
    /// 
    /// ### Notes
    /// These are set by [`StatusModule::init_settings`] and are likely checked as part of the status change
    /// event which is sent to other modules
    #[offset = 0x180]
    pub set_ground_check_kinds: extern "C" fn(this: &mut StatusModule, correct: i32, cliff_check: i32),

    /// Sets whether or not to keep the situation kind if it is [`app::SituationKind::Air`]
    /// 
    /// ### Arguments
    /// * `keep` - Whether or not to keep [`app::SituationKind::Air`]
    /// 
    /// ### Notes
    /// This function has the same functionality as the `keep` flag in [`StatusModule::set_situation_kind`]
    /// but only when the situation kind is [`app::SituationKind::Air`]. This means that 
    /// the status can transition into an aerial situation kind but not out of it
    #[offset = 0x188]
    pub set_keep_situation_air: extern "C" fn(this: &mut StatusModule, keep: bool),

    /// Sets the agent status user data
    /// 
    /// ### Arguments
    /// * `status_data` - The status data pointer to set
    /// 
    /// ### Notes
    /// This is caled internally by [`FighterStatusModuleImpl::set_fighter_status_data`]
    #[offset = 0x190]
    set_agent_status_data: extern "C" fn(this: &mut StatusModule, status_data: *mut skyline::libc::c_void),

    /// Sets the previous agent status user data
    /// 
    /// ### Arguments
    /// * `status_data` - The status data pointer to set
    /// 
    /// ### Notes
    /// This is caled internally by [`FighterStatusModuleImpl::set_fighter_status_data`]
    #[offset = 0x198]
    set_prev_agent_status_data: extern "C" fn(this: &mut StatusModule, status_data: *mut skyline::libc::c_void),

    /// Sets the current status's kinetic type
    /// 
    /// ### Arguments
    /// * `kinetic_type` - The kinetic type
    /// 
    /// ### Notes
    /// This is called internally by [`StatusModule::init_settings`]
    #[offset = 0x1A0]
    pub set_kinetic_type: extern "C" fn(this: &mut StatusModule, kinetic_type: i32),

    /// Sets the current status's succeeds bits
    /// 
    /// ### Arguments
    /// * `succeeds_bits` - The succeeds bits
    /// 
    /// ### Notes
    /// This is called internally by [`StatusModule::init_settings`]
    #[offset = 0x1A8]
    pub set_succeeds_bit: extern "C" fn(this: &mut StatusModule, succeeds_bit: i32),

    /// Sets the current object's permanent succeeds bits, which are typically OR'd together with the succeeds bits
    /// when checking for flags
    /// 
    /// ### Arguments
    /// * `succeeds_bits` - The succeeds bits
    #[offset = 0x1B0]
    pub set_permanent_succeeds_bits: extern "C" fn(this: &mut StatusModule, succeeds_bits: u32),

    /// Updates the current status's situation kind based on their current state/location
    /// 
    /// ### Notes
    /// * If the `keep` flag was set to true during [when setting the situation kind](StatusModule::set_situation_kind),
    /// then this function does nothing. If the current situation kind is [`app::SituationKind::Air`] and the (keep aerial
    /// situation flag was set to true)[StatusModule::set_keep_situation_air] and the object is not in hitstop,
    /// this function does nothing.
    #[offset = 0x1B8]
    pub update_situation_kind: extern "C" fn(this: &mut StatusModule),

    /// Sets the C++ status script set for the specified status kind
    /// 
    /// ### Arguments
    /// * `status_kind` - The status to set the script for
    /// * `status_script` - The status script set
    /// 
    /// ### Notes
    /// Unlike the [lua alternative](lua2cpp::L2CAgentBase::sv_set_status_func), the caller must replace the entire
    /// status script set, not just a singular function.
    #[offset = 0x1C0]
    set_status_script: extern "C" fn(this: &mut StatusModule, status_kind: i32, status_script: *const skyline::libc::c_void),

    /// Initializes the settings for the current status
    /// 
    /// ### Arguments
    /// * `situation_kind` - The situation kind
    /// * `kinetic_type` - The kinetic type to use during the status
    /// * `ground_correct_kind` - The ground correction kind
    /// * `ground_cliff_check_kind` - The ground cliff check kind
    /// * `enable_jostle` - Whether to enable jostle while in the status
    /// * `keep_flag` - Bitmask of flags to keep, passed to [work module](app::WorkModule::clear_status)
    /// * `keep_int` - Bitmask of ints to keep, passed to [work module](app::WorkModule::clear_status)
    /// * `keep_float` - Bitmask of floats to keep, passed to [work module](app::WorkModule::clear_status)
    /// * `succeeds_bits` - Bitmask of things to keep if the status transition is successful
    #[offset = 0x1C8]
    pub init_settings: extern "C" fn(
        this: &mut StatusModule,
        situation_kind: app::SituationKind,
        kinetic_type: i32,
        ground_correct_kind: u32,
        ground_cliff_check_kind: i32,
        enable_jostle: bool,
        keep_flag: i32,
        keep_int: i32,
        keep_float: i32,
        succeeds_bits: i32
    ),

    /// Sets the [work module](app::WorkModule) clear set for this current status
    /// 
    /// ### Arguments
    /// * `keep_float` - Bitmask of status floats to keep
    /// * `keep_int` - Bitmask of status ints to keep
    /// * `keep_flag` - Bitmask of status flags to keep
    #[offset = 0x1D0]
    pub set_work_keep_info: extern "C" fn(this: &mut StatusModule, keep_float: i32, keep_int: i32, keep_flag: i32),

    /// Sets whether or not the status module should execute status scripts
    /// 
    /// ### Arguments
    /// * `disable` - Whether or not to disable the module
    #[offset = 0x1D8]
    pub set_disable: extern "C" fn(this: &mut StatusModule, disable: bool),

    /// Checks whether or not status execution is disabled
    #[offset = 0x1E0]
    pub is_disabled: extern "C" fn(this: &StatusModule) -> bool,

    /// Checks whether or not the status is currently ending
    #[offset = 0x1E8]
    pub is_status_ending: extern "C" fn(this: &StatusModule) -> bool,

    /// Ends the current status and starts the next status
    /// 
    /// ### Arguments
    /// * `status_kind` - The status kind to change to
    /// 
    /// ### Behavior (Fighter, Weapon, & Item)
    /// This function starts by setting `status_kind_next` and `status_kind_interrupt` to the `status_kind` argument,
    /// after which it will attempt to call the `end` and `exit` status scripts from the lua system as well as the
    /// `system_post` status script from the C++ system
    /// 
    /// After ending those statuses, it will check if there is a queued status from a call to [`StatusModule::change_status_request_from_script`]
    /// and will then attempt to transition to it (this function can be called recursively).
    /// 
    /// If the status kind is not invalid, it will [clear all transition terms](app::WorkModule::clear_transition_term) and
    /// if it is invalid it will [clear all of work module](app::WorkModule::clear_all)
    /// 
    /// At this point the function will call the (pre and init statuses from lua)[app::LuaModule::call_line_status_shift] (which in turn
    /// are expected to [initialize the status settings](StatusModule::init_settings)). If an interrupting status was set, it will
    /// recall the pre/init statuses one more time, after which it calls the `main`, `exec`/`exec_stop`, and `status` scripts.
    #[offset = 0x1F0]
    pub change_status: extern "C" fn(this: &mut StatusModule, status_kind: i32)
}

#[repr(C)]
pub struct StatusModule {
    vtable: &'static StatusModuleVTable,
    owner: *mut app::BattleObjectModuleAccessor,
}

#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x40]
pub struct FighterStatusData {
    #[offset = 0x0]  status_kind: i32,
    #[offset = 0x4]  unk: bool,
    #[offset = 0x8]  treaded_kind: i32,
    #[offset = 0xC]  unk2: bool,
    #[offset = 0xD]  unk3: bool,
    #[offset = 0xE]  unk4: bool,
    #[offset = 0xF]  keep_secondary: bool,
    #[offset = 0x10] disable_turn_damage: bool, 
    #[offset = 0x11] damage: bool,
    #[offset = 0x12] no_drop_item: bool,
    #[offset = 0x13] into_door: bool,
    #[offset = 0x14] disable_interrupt_scaling: bool,
    #[offset = 0x15] disable_interrupt_spring: bool,
    #[offset = 0x16] disbale_interrupt_superstar: bool,
    #[offset = 0x17] disable_interrupt_slow: bool,
    #[offset = 0x18] disable_swim: bool,
    #[offset = 0x19] disable_curry_shot: bool,
    #[offset = 0x1A] disable_curry_face: bool,
    #[offset = 0x1B] start_turn: bool,
    #[offset = 0x1C] disable_shield_recovery: bool,
    #[offset = 0x1D] clear_motion_energy: bool,
    #[offset = 0x1E] inherit_cursor: bool,
    #[offset = 0x1F] slope_top_unlimit: bool,
    #[offset = 0x20] inherit_camera_range: bool,
    #[offset = 0x21] hide_name_cursor: bool,
    #[offset = 0x22] disable_ground_friction: bool,
    #[offset = 0x23] final_: bool,
    #[offset = 0x24] final_damage: bool,
    #[offset = 0x25] scale_kinetic_energy: bool,
    #[offset = 0x26] detach_pikmin: bool,
    #[offset = 0x27] keep_motion_rate: bool,
    #[offset = 0x28] unused: bool,
    #[offset = 0x29] enable_rocketbelt_eject: bool,
    #[offset = 0x2A] ignore_speed_limit: bool,
    #[offset = 0x2B] disable_interrupt_stop: bool,
    #[offset = 0x2C] disable_interrupt_warp: bool,
    #[offset = 0x30] powerup_bits: u32,
    #[offset = 0x34] disable_dissolve_cursor: bool,
    #[offset = 0x35] disable_jump_board_effect: bool,
    #[offset = 0x38] log_flags: u64
}

#[repr(C)]
#[virtual_implementor(StatusModule)]
#[derive(TypeAssert)]
#[size = 0x188]
pub struct FighterStatusModuleImpl {
    #[offset = 0x000] parent: StatusModule,
    #[offset = 0x010] status_change_queue: cpp::FixedVec<StatusChange, 8>,
    #[offset = 0x080] previous_status_kinds: *mut cpp::FixedVec<i32, 1>, // the 1 is to determine statically allocated space, but since this is a we don't really care
    #[offset = 0x088] cpp_status_script_count: usize,
    #[offset = 0x090] cpp_status_scripts: *const *const skyline::libc::c_void,
    #[offset = 0x098] status_kind: i32,
    #[offset = 0x09C] status_kind_next: i32,
    #[offset = 0x0A0] status_kind_interrupt: i32,
    #[offset = 0x0A4] status_request_from_script: i32,
    #[offset = 0x0A8] agent_status_data: *mut skyline::libc::c_void,
    #[offset = 0x0B0] prev_agent_status_data: *mut skyline::libc::c_void,
    #[offset = 0x0B8] kinetic_type: i32,
    #[offset = 0x0BC] succeeds_bits: i32,
    #[offset = 0x0C0] permanent_succeeds_bits: i32,
    #[offset = 0x0C4] work_keep_flag: i32,
    #[offset = 0x0C8] work_keep_int: i32,
    #[offset = 0x0CC] work_keep_float: i32,
    #[offset = 0x0D0] ground_correct_kind: i32,
    #[offset = 0x0D4] ground_cliff_check_kind: i32,
    #[offset = 0x0D8] added_line_mask: i32,
    #[offset = 0x0DC] situation_kind: app::SituationKind,
    #[offset = 0x0E0] prev_situation_kind: app::SituationKind,
    #[offset = 0x0E8] current_status_script: *const skyline::libc::c_void,
    #[offset = 0x0F0] lua_script_running: bool,
    #[offset = 0x0F1] lua_script_ending: bool,
    #[offset = 0x0F2] disable_lua_script: bool,
    #[offset = 0x0F3] has_updated: bool,
    #[offset = 0x0F4] is_changing: bool,
    #[offset = 0x0F5] disable_status: bool,
    #[offset = 0x0F6] clear_command_from_script: bool,
    #[offset = 0x0F7] keep_situation_kind: bool,
    #[offset = 0x0F8] keep_situation_air: bool,
    #[offset = 0x0F9] cpp_script_ending: bool,
    #[offset = 0x100] status_data: FighterStatusData,
    #[offset = 0x140] prev_status_data: FighterStatusData,
    #[offset = 0x180] static_status_transition_info: *const *const skyline::libc::c_void
}
