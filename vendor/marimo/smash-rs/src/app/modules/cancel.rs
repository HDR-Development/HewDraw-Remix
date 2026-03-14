use crate::*;

#[repr(C)]
pub struct SituationTransitionSet {
    situation_kind: app::SituationKind,
    transition_group_ids: *const app::WorkId,
    transition_group_count: u32,
}

impl SituationTransitionSet {
    /// Gets the list of transition group ids as a slice
    pub fn group_ids<'a>(&'a self) -> &'a [app::WorkId] {
        unsafe {
            std::slice::from_raw_parts(self.transition_group_ids, self.transition_group_count as usize)
        }
    }

    /// Gets the situation kind of the set
    pub fn situation(&self) -> app::SituationKind {
        self.situation_kind
    }
}

#[repr(C)]
pub struct SituationTransitionSetInfo {
    situation_transition_sets: *const SituationTransitionSet,
    set_count: u32
}

impl SituationTransitionSetInfo {
    /// Gets the number of transition sets in this group
    pub fn count(&self) -> usize {
        self.set_count as usize
    }

    /// Gets the transition sets as a slice
    pub fn as_slice<'a>(&'a self) -> &'a [SituationTransitionSet] {
        unsafe {
            std::slice::from_raw_parts(self.situation_transition_sets, self.set_count as usize)
        }
    }

    /// Attempts to get the transition set for the provided situation kind, if it exists
    /// in this group
    /// 
    /// ### Arguments
    /// * `situation` - The situation kind to get the transition set for
    /// 
    /// ### Returns
    /// * `Some(&SituationTransitionSet)` - If there was a transition set for the provided situation
    /// * `None` - If there was not a transition set for the provided situation
    pub fn get_set_for_situation<'a>(&'a self, situation: app::SituationKind) -> Option<&'a SituationTransitionSet> {
        self.as_slice()
            .iter()
            .find(|set| set.situation_kind == situation)
    }
}

#[repr(C)]
#[vtable_impl(CancelModule)]
#[derive(TypeAssert)]
#[size = 0x70]
pub(crate) struct CancelModuleVTable {
    #[offset = 0x0]
    destructor: extern "C" fn(this: &mut CancelModule),

    #[offset = 0x8]
    deleter: extern "C" fn(this: &mut CancelModule),
    
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
    pub is_virtual: extern "C" fn(this: &CancelModule) -> bool,
    
    #[offset = 0x18]
    handle_int_msc_command: extern "C" fn(this: &mut CancelModule, command: &lib::MscCommand) -> lib::TValue,

    #[offset = 0x20]
    handle_float_msc_command: extern "C" fn(this: &mut CancelModule, command: &lib::MscCommand) -> lib::TValue,

    /// This method is blank and does not do anything
    #[offset = 0x28]
    initialize: extern "C" fn(this: &mut CancelModule),

    /// Despite the initialization not doing anything, this method will
    /// attempt to remove some event listeners
    #[offset = 0x30]
    finalize: extern "C" fn(this: &mut CancelModule),

    /// This method is blank and does not do anything
    #[offset = 0x38]
    start_module: extern "C" fn(this: &mut CancelModule),

    /// This method is blank and does not do anything
    #[offset = 0x40]
    end_module: extern "C" fn(this: &mut CancelModule),

    /// Checks if the fighter's cancel groups have been enabled
    /// 
    /// ### Behavior
    /// At least in the fighter's implementation of [`CancelModule`], canceling is determined based on 
    /// whether or not there is a list of transition groups for the fighter's situation kind. If there
    /// is a list of them, then when either [`CancelModule::enable_cancel`] or [`CancelModule::re_renable_cancel`]
    /// is called it will enable those transitions groups via [`app::WorkModule::enable_transition_term_group`].
    /// 
    /// ### Returns
    /// * `true` - The fighter has transition groups for their current situation kind and they have been
    /// enabled by [`CancelModule`]
    /// * `false` - Either there are no transition groups fo the fighter's current situation kind or 
    /// they haven't been enabled yet
    /// 
    /// ### Notes
    /// This function can be `true` while also not having canceling be enabled if something else disables
    /// the transition term groups manually
    #[offset = 0x48]
    pub is_enable_cancel: extern "C" fn(this: &CancelModule) -> bool,

    /// Conditionally enables the fighter to cancel their current action
    /// 
    /// ### Behavior (Fighter Implementation)
    /// If the fighter is in any of the following states (checked via [`app::WorkModule`] flags), this function exits without enabling
    /// the cancel:
    /// * Using a hammer
    /// * Holding up the special flag (the one that gives you an extra life) or the bomber (the one that explodes)
    /// * Using the daybreak
    /// * When the flag for [`app::work_ids::fighter::instance::KNOCKOUT`] is set
    /// 
    /// It also checks to see if the flag for `unable_cancel_status` has been set, and if it is it will not enable the transitions.
    /// 
    /// ### Notes
    /// * If you are performing an aerial, then [`app::transition_terms::LANDING`] and [`app::transition_terms::LANDING_LIGHT`]
    /// are disabled.
    /// * If you are performing down tilt, then [`app::transition_terms::CONT_SQUAT`] is disabled.
    #[offset = 0x50]
    pub enable_cancel: extern "C" fn(this: &mut CancelModule),

    /// Re-enables cancelling if your situation kind has changed.
    /// 
    /// ### Behavior (Fighter Implementation)
    /// After checking to make sure that you were able to cancel before ([`CancelModule::is_enable_cancel`] is true)
    /// and your situation kind has changed, it will disable the previous situation's transition term groups
    /// and enable the new situation's transition term groups.
    /// 
    /// It also checks to see if the flag for `unable_cancel_status` has been set, and if it is it will not enable the transitions.
    /// 
    /// ### Notes
    /// Unlike [`CancelModule::enable_cancel`], this method will not disable [`app::transition_terms::LANDING`], [`app::transition_terms::LANDING_LIGHT`]
    /// or [`app::transition_terms::CONT_SQUAT] after switching the transition term groups, likely because the status kind has not changed yet.
    /// 
    /// ### Notes
    /// This method is not exported by the main executable, its name is assumed
    #[offset = 0x58]
    pub re_enable_cancel: extern "C" fn(this: &mut CancelModule),

    /// Disables all transition term groups for all situation kinds and blocks cancelling for the rest of the status
    /// 
    /// ### Behavior (Fighter Implementation)
    /// Disables the transition term groups for **all** situation kinds (does not check the current one) and sets a flag which
    /// will only be unset upon a status change.
    /// 
    /// ### Notes
    /// This method is not exported by the main executable, its name is taken from an MSC command constant
    #[offset = 0x60]
    pub unable_cancel_status: extern "C" fn(this: &mut CancelModule),

    /// Disables all transition term groups for all situation kinds 
    /// 
    /// ### Behavior (Fighter Implementation)
    /// The behavior for this method is identical to [`CancelModule::unable_cancel_status`], except it does not set the flag
    /// which blocks cancelling until the next status
    /// 
    /// ### Notes
    /// This method is not exported by the main executable, its name is taken from an MSC command constant
    #[offset = 0x68]
    pub unable_cancel: extern "C" fn(this: &mut CancelModule)
}

/// A utility for prematurely exiting states/animations
/// 
/// `CancelModule` is a very simple module that supports only a handful of operations. Its main goal
/// is to manage whether or not a fighter is able to cancel their current animation/status by enabling
/// and disabling various [transition groups](app::TransitionGroup) depending on the fighter's
/// [situation kind](app::SituationKind).
#[repr(C)]
pub struct CancelModule {
    vtable: &'static CancelModuleVTable,
    owner: *mut app::BattleObjectModuleAccessor,
}

/// The fighter implementation of [`CancelModule`](app::CancelModule)
#[repr(C)]
#[virtual_implementor(CancelModule)]
#[derive(TypeAssert)]
#[size = 0x20]
pub struct FighterCancelModuleImpl {
    #[offset = 0x00] parent: CancelModule,
    #[offset = 0x10] transition_groups: *const SituationTransitionSetInfo,
    #[offset = 0x18] current_transition_set: app::SituationKind,
    #[offset = 0x1C] has_canceled: bool,
    #[offset = 0x1D] is_cancel_blocked: bool
}

impl FighterCancelModuleImpl {
    /// Gets this module's transition sets
    pub fn transition_groups<'a>(&'a self) -> &'a SituationTransitionSetInfo {
        unsafe {
            &*self.transition_groups
        }
    }

    /// Checks if canceling is blocked until the end of the status
    pub fn is_cancel_blocked(&self) -> bool {
        self.is_cancel_blocked
    }

    /// Checks if canceling is currently enabled
    pub fn has_already_canceled(&self) -> bool {
        self.has_canceled
    }

    /// Tries to get the situation kind of the current enabled cancel set,
    /// returning [`None`] if it has not canceled.
    pub fn current_enabled_transition_set(&self) -> Option<app::SituationKind> {
        if self.has_canceled && !self.is_cancel_blocked {
            Some(self.current_transition_set)
        } else {
            None
        }
    }
}