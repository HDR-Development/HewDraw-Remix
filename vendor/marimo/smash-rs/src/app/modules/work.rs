use std::{
    convert::TryFrom,
    ops::{Index, IndexMut},
};

use crate::*;

use thiserror::Error;

// macro_rules! const_partial_eq {
//     ($id:ident) => {
//         impl const std::cmp::PartialEq for $id {
//             fn eq(&self, other: &Self) -> bool {
//                 (*self as u8) == (*other as u8)
//             }
//         }
//     };
// }

/// Enum representing the category fo data that a [`WorkId`] refers to. This field
/// determines where data is stored. [`WorkKind::Transition`] and [`WorkKind::TransitionGroup`]
/// are not checked by any calls to the [`WorkModule`] accessors, however [`WorkKind::Instance`]
/// and [`WorkKind::Status`] are used.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WorkKind {
    Instance = 0x0,
    Status = 0x1,
    Transition = 0xE,
    TransitionGroup = 0xF,

    None = 0xFF,
}

impl From<u8> for WorkKind {
    fn from(arg: u8) -> Self {
        match arg {
            0x0 => WorkKind::Instance,
            0x1 => WorkKind::Status,
            0xE => WorkKind::Transition,
            0xF => WorkKind::TransitionGroup,
            _ => WorkKind::None,
        }
    }
}

/// Enum representing the type of data that a [`WorkId`] refers to. This field
/// has no impact on where the data is stored, and is not check by any calls to the
/// [`WorkModule`] accessors.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WorkType {
    Float = 0x0,
    Int = 0x1,
    Flag = 0x2,

    None = 0xFF,
}

impl From<u8> for WorkType {
    fn from(arg: u8) -> Self {
        match arg {
            0x0 => WorkType::Float,
            0x1 => WorkType::Int,
            0x2 => WorkType::Flag,
            _ => WorkType::None,
        }
    }
}

/// The symbolic constant used to read and write data to the [`WorkModule`]'s internal storage.
///
/// They are made up of 3 components:
/// * The [`WorkType`], which designates what kind of data the constant is referring to
/// * The [`WorkKind`], which designates what category the data belongs to
/// * The index, which is used to find the location of the data in the internal storage.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct WorkId(u32);

impl WorkId {
    pub(crate) const fn from_parts(ty: WorkType, kind: WorkKind, index: u32) -> Self {
        let ty = (ty as u8 as u32) << 0x1C;
        let kind = (kind as u8 as u32) << 0x18;
        let index = index & 0x00FF_FFFF;
        Self(ty | kind | index)
    }

    pub fn get_type(self) -> WorkType {
        WorkType::from((self.0 >> 0x1C) as u8)
    }

    pub fn get_kind(self) -> WorkKind {
        WorkKind::from(((self.0 >> 0x18) & 0xF) as u8)
    }

    pub fn get_index(self) -> u32 {
        self.0 & 0x00FF_FFFF
    }

    pub unsafe fn from_u32_unchecked(value: u32) -> Self {
        Self(value)
    }

    pub fn as_u32(self) -> u32 {
        self.0
    }
}

#[derive(Error, Debug)]
pub enum WorkIdTryFromError {
    #[error("The type field of the ID was invalid! {0} is not a valid WorkType")]
    InvalidType(u8),

    #[error("The kind field of the ID was invalid! {0} is not a valid WorkKind")]
    InvalidKind(u8),
}

impl TryFrom<u32> for WorkId {
    type Error = WorkIdTryFromError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let ty = ((value >> 0x1C) & 0xF) as u8;
        let kind = ((value >> 0x18) & 0xF) as u8;
        if WorkType::from(ty) == WorkType::None {
            Err(WorkIdTryFromError::InvalidType(ty))
        } else if WorkKind::from(kind) == WorkKind::None {
            Err(WorkIdTryFromError::InvalidKind(kind))
        } else {
            Ok(Self(value))
        }
    }
}

bitflags! {
    pub struct TransitionTerm: u8 {
        /// Controls whether or not the transition term is enabled when it is not a part of a group
        const ENABLED                  = 0b0000_0001;

        /// Declares whether or not the transition term is enabled as part of a group requires the `ENABLED_EX` flag to really be enabled
        const ENABLED_GROUP            = 0b0000_0010;

        /// Declares whether or not the transition term is enabled when it is part of a group
        const ENABLED_EX               = 0b0000_0100;

        /// Forbids the transition from occurring
        const ENABLE_FORBID            = 0b0000_1000;

        /// Another flag to forbid the transition from occurring (potentially the equivalent to `ENABLED_GROUP`)
        const ENABLE_FORBID_OTHER      = 0b0001_0000; // not exported by any symbols but checked when seeing if transition is forbidden

        /// Another flag to forbid the transition from occurring
        const ENABLE_FORBID_INDIVIDUAL = 0b0010_0000;

        /// A helper mask for all forbid flags
        const FORBIDDEN_MASK = 0b0011_1000;
    }
}

impl TransitionTerm {
    /// Checks to see if the transition term is enabled.
    ///
    /// A transition term is enabled under the following criteria:
    /// 1. None of the `forbid` flags are set
    /// 2. If it is enabled as part of a group, the `ENABLED_EX` flag must be set
    /// 3. If it is not enabled as part of a group, the `ENABLED` flag must be set
    pub fn is_enabled(self) -> bool {
        if self.intersects(TransitionTerm::FORBIDDEN_MASK) {
            return false;
        }

        if self.contains(TransitionTerm::ENABLED_GROUP) {
            self.contains(TransitionTerm::ENABLED_EX)
        } else {
            self.contains(TransitionTerm::ENABLED)
        }
    }

    /// Checks to see if the transition term is forbidden
    pub fn is_forbidden(self) -> bool {
        self.intersects(TransitionTerm::FORBIDDEN_MASK)
    }
}

#[repr(C)]
struct WorkArrayVTable {
    pub destructor: extern "C" fn(&mut WorkArray),
    pub deleter: extern "C" fn(*mut WorkArray),
    pub get_int: extern "C" fn(&WorkArray, index: u32) -> i32,
}

/// A structure to hold on to the three required arrays of internal storage for [`WorkModule`].
///
/// The structure holds sizes and raw pointers to data. It should be noted that a `WorkArray` *does*
/// inherit from a base, abstract class because the implementation for `GetInt` can vary per implementation.
/// For fighters, the implementation is to allocate space for 64-bit integers and just set the lower 32-bits if need be
///
/// This implementation also appears to be the same for weapons and items.
#[repr(C)]
pub struct WorkArray {
    vtable: &'static WorkArrayVTable,
    float_count: usize,
    floats: *mut f32,
    int_count: usize,
    ints: *mut i64,
    flag_count: usize,
    flags: *mut u32,
}

impl WorkArray {
    /// Gets the underlying array of float variables
    pub fn get_floats<'a>(&'a self) -> &'a [f32] {
        unsafe { std::slice::from_raw_parts(self.floats, self.float_count) }
    }

    /// Gets the underlying mutable array of float variables
    pub fn get_floats_mut<'a>(&'a mut self) -> &'a mut [f32] {
        unsafe { std::slice::from_raw_parts_mut(self.floats, self.float_count) }
    }

    /// Gets the underlying array of integer variables
    pub fn get_ints<'a>(&'a self) -> &'a [i64] {
        unsafe { std::slice::from_raw_parts(self.ints, self.int_count) }
    }

    /// Gets the underlying mutable array of integer variables
    pub fn get_ints_mut<'a>(&'a mut self) -> &'a mut [i64] {
        unsafe { std::slice::from_raw_parts_mut(self.ints, self.int_count) }
    }

    /// Gets the underyling array of flag variables
    pub fn get_flags<'a>(&'a self) -> &'a [u32] {
        unsafe { std::slice::from_raw_parts(self.flags, self.flag_count) }
    }

    /// Gets the underlying mutable array of flag variables
    pub fn get_flags_mut<'a>(&'a mut self) -> &'a mut [u32] {
        unsafe { std::slice::from_raw_parts_mut(self.flags, self.flag_count) }
    }

    /// Gets the float variable located at `index`
    pub fn get_float(&self, index: usize) -> f32 {
        self.get_floats()[index]
    }

    /// Gets the 32-bit integer variable located at `index`
    pub fn get_int(&self, index: usize) -> i32 {
        (self.get_ints()[index] & 0xFFFF_FFFF) as i32
    }

    /// Gets the 64-bit integer variable located at `index`
    pub fn get_int64(&self, index: usize) -> i64 {
        self.get_ints()[index]
    }

    /// Gets the flag variable located at `index`
    pub fn get_flag(&self, index: usize) -> bool {
        (self.get_flags()[index / 0x20] >> (index % 0x20)) & 0x1 != 0x0
    }

    /// Sets the float variable located at `index`
    pub fn set_float(&mut self, value: f32, index: usize) {
        self.get_floats_mut()[index] = value;
    }

    /// Sets the 32-bit integer variable located at `index`
    pub fn set_int(&mut self, value: i32, index: usize) {
        self.get_ints_mut()[index] = value as i64;
    }

    /// Sets the 64-bit integer variable located at `index`
    pub fn set_int64(&mut self, value: i64, index: usize) {
        self.get_ints_mut()[index] = value;
    }

    /// Sets the flag variable located at `index`
    pub fn set_flag(&mut self, value: bool, index: usize) {
        if value {
            self.get_flags_mut()[index / 0x20] |= 1 << (index % 0x20);
        } else {
            self.get_flags_mut()[index / 0x20] &= !(1 << (index % 0x20));
        }
    }
}

/// A structure to represent the amount of underlying [`WorkArray`] structures that a [`WorkModule`] uses
/// for it's variable storage.
///
/// For fighters, weapons, and likely items, this is always 2 [`WorkArray`]s, one for instance variables
/// and one for status variables
#[repr(C)]
pub struct WorkArrayInfo {
    array_count: usize,
    arrays: *mut *mut WorkArray,
}

impl WorkArrayInfo {
    /// Gets the underlying [`WorkArray`]s as a slice
    pub fn as_slice<'a>(&'a self) -> &'a [&'a WorkArray] {
        unsafe { std::slice::from_raw_parts(self.arrays as *const &'a WorkArray, self.array_count) }
    }

    /// Gets the underlying [`WorkArray`]s as a mutable slice
    pub fn as_slice_mut<'a>(&'a mut self) -> &'a mut [&'a mut WorkArray] {
        unsafe {
            std::slice::from_raw_parts_mut(self.arrays as *mut &'a mut WorkArray, self.array_count)
        }
    }

    /// Gets the [`WorkArray`] for the instance variables
    pub fn instance<'a>(&'a self) -> &'a WorkArray {
        &self.as_slice()[0]
    }

    /// Gets the [`WorkArray`] for the status variables
    pub fn status<'a>(&'a self) -> &'a WorkArray {
        &self.as_slice()[1]
    }
}

/// A structure to represent the transition terms that a [`WorkModule`] supports and manages.
///
/// According to the lua consts, there don't appear to be any transition terms for objects other
/// than fighters, so it's unlikely that this would be useful for weapons or items
#[repr(C)]
pub struct TransitionTermInfo {
    count: usize,
    transitions: *mut TransitionTerm,
}

impl TransitionTermInfo {
    /// Gets the underlying [`TransitionTerm`]s as a slice
    pub fn as_slice<'a>(&'a self) -> &'a [TransitionTerm] {
        unsafe { std::slice::from_raw_parts(self.transitions, self.count) }
    }

    /// Gets the underlying [`TransitionTerm`]s as a mutable slice
    pub fn as_slice_mut<'a>(&'a mut self) -> &'a mut [TransitionTerm] {
        unsafe { std::slice::from_raw_parts_mut(self.transitions, self.count) }
    }

    /// Gets the [`TransitionTerm`] at the location `index`
    pub fn get_transition(&self, index: usize) -> TransitionTerm {
        self.as_slice()[index]
    }

    /// Sets the [`TransitionTerm`] at the location `index`
    pub fn set_transition(&mut self, transition: TransitionTerm, index: usize) {
        self.as_slice_mut()[index] = transition;
    }
}

impl Index<usize> for TransitionTermInfo {
    type Output = TransitionTerm;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for TransitionTermInfo {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_slice_mut()[index]
    }
}

impl Index<WorkId> for TransitionTermInfo {
    type Output = TransitionTerm;

    fn index(&self, index: WorkId) -> &Self::Output {
        if index.get_kind() != WorkKind::Transition {
            panic!(
                "WorkKind {:?} is an invalid index for TransitionTermInfo",
                index.get_kind()
            );
        }

        self.index(index.get_index() as usize)
    }
}

impl IndexMut<WorkId> for TransitionTermInfo {
    fn index_mut(&mut self, index: WorkId) -> &mut Self::Output {
        if index.get_kind() != WorkKind::Transition {
            panic!(
                "WorkKind {:?} is an invalid index for TransitionTermInfo",
                index.get_kind()
            );
        }

        self.index_mut(index.get_index() as usize)
    }
}

/// A structure to represent a group of [`TransitionTerm`]s.
///
/// The underlying representation is an array of [`WorkId`]s which have a type of [`WorkType::Int`] and
/// a kind of [`WorkKind::Transition`].
#[repr(C)]
pub struct TransitionGroup {
    transition_term_ids: *const WorkId,
    count: usize,
}

impl TransitionGroup {
    /// Gets the underlying [`WorkId`] array
    pub fn as_slice<'a>(&'a self) -> &'a [WorkId] {
        unsafe { std::slice::from_raw_parts(self.transition_term_ids, self.count) }
    }

    /// Gets the [`WorkId`] at `index`
    pub fn get_transition_term(&self, index: usize) -> WorkId {
        self.as_slice()[index]
    }
}

impl Index<usize> for TransitionGroup {
    type Output = WorkId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

#[repr(C)]
#[vtable_impl(WorkModule)]
#[derive(TypeAssert)]
#[size = 0x288]
pub(crate) struct WorkModuleVTable {
    #[offset = 0x0]
    destructor: extern "C" fn(this: &mut WorkModule),

    #[offset = 0x8]
    deleter: extern "C" fn(this: &mut WorkModule),

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
    pub is_virtual: extern "C" fn(this: &WorkModule) -> bool,

    #[offset = 0x18]
    handle_int_msc_command:
        extern "C" fn(this: &mut WorkModule, command: &lib::MscCommand) -> lib::TValue,

    #[offset = 0x20]
    handle_float_msc_command:
        extern "C" fn(this: &mut WorkModule, command: &lib::MscCommand) -> lib::TValue,

    /// Initializes the module by copying the provided arrays/transition information and initializing the memory
    /// ### Arguments
    /// * `array_info` - The array information the module should use. This must be valid for the lifetime of the module.
    /// * `transition_term_info` - The transition term information the module should use. This must be valid for the lifetime of the module.
    /// * `transition_groups` - The transition groups the module should use. This must be valid for the lifetime of the module.
    /// * `param_object` - The structure holding on to the parameter information this object should reference. This field can be `null`
    /// * `agent_kind` - What kind of agent owns this module
    #[offset = 0x28]
    initialize: extern "C" fn(
        this: &mut WorkModule,
        array_info: &WorkArrayInfo,
        transition_term_info: &TransitionTermInfo,
        transition_groups: *const TransitionGroup,
        param_object: u64,
        agent_kind: u32,
    ),

    /// Finalizes the module by removing references to everything passed in during initialization
    #[offset = 0x30]
    finalize: extern "C" fn(this: &mut WorkModule),

    /// Starts the module by clearing all of the work storage and attaching required event listeners
    #[offset = 0x38]
    start_module: extern "C" fn(this: &mut WorkModule),

    /// Ends the module by clearing all of the work storage and removing its event listeners
    #[offset = 0x40]
    end_module: extern "C" fn(this: &mut WorkModule),

    /// Calculates all params set to be calculated at runtime, including, but not limited to, params like `jump_initial_speed_y`, `jump_initial_accel_y`, etc.
    ///
    /// After this call, these params are accessible through the param accessor methods and are cached in the module
    #[offset = 0x48]
    pub calc_params: extern "C" fn(this: &mut WorkModule),

    /// Sets the transition term information for this module to use
    ///
    /// This enables the user to swap out the transition term info on the fly if need be, and is called during initialization
    #[offset = 0x50]
    pub set_transition_term_info: extern "C" fn(this: &mut WorkModule, info: &TransitionTermInfo),

    /// Gets the specified float from the module's internal storage
    /// ### Arguments
    /// * `what` - The work ID specifiying what to get
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for a float, it only checks the [`WorkKind`] and the index
    #[offset = 0x58]
    pub get_float: extern "C" fn(this: &WorkModule, what: WorkId) -> f32,

    /// Sets the specified float in the module's internal storage
    /// ### Arguments
    /// * `value` - The new value of the variable
    /// * `what` - The work ID specify what to set
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for a float, it only checks the [`WorkKind`] and the index
    #[offset = 0x60]
    pub set_float: extern "C" fn(this: &mut WorkModule, value: f32, what: WorkId),

    /// Sets the specified floats in the module's internal storage
    /// ### Arguments
    /// * `value` - The new value of the variables
    /// * `ids` - The [`WorkId`]s to set the value of
    /// * `count` - The number of provided IDs
    ///
    /// ### Notes
    /// The implementation does not check that these IDs are for floats, it only checks the [`WorkKind`]s and the indices
    #[offset = 0x68]
    set_floats_impl:
        extern "C" fn(this: &mut WorkModule, value: f32, ids: *const WorkId, count: u32),

    /// Assigns a random value in the specified range to the specified float in the module's internal storage
    /// ### Arguments
    /// * `minimum` - The minimum value of the random range
    /// * `maximum` - The maximum value of the random range
    /// * `what` - The [`WorkId`] to set the value of
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a float, it only checks the [`WorkKind`] and the index
    /// * If `maximum` is less than `minimum`, then the value is not set at all.
    /// * If `maximum` is equal to `minimum`, then the value is set to `maximum`
    #[offset = 0x70]
    pub random_float:
        extern "C" fn(this: &mut WorkModule, minimum: f32, maximum: f32, what: WorkId),

    /// Adds the specified amount to the specified float in the module's internal storage
    /// ### Arguments
    /// * `amount` - The amount to add
    /// * `what` - The [`WorkId`] to change the value of
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for a float, it only checks the [`WorkKind`] and the index
    #[offset = 0x78]
    pub add_float: extern "C" fn(this: &mut WorkModule, amount: f32, what: WorkId),

    /// Subtracts the specified amount from the specified float in the module's internal storage
    /// ### Arguments
    /// * `amount` - The amount to subtract
    /// * `what` - The [`WorkId`] to change the value of
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for a float, it only checks the [`WorkKind`] and the index
    #[offset = 0x80]
    pub sub_float: extern "C" fn(this: &mut WorkModule, amount: f32, what: WorkId),

    /// Multiplies the specified amount by the specified float in the module's internal storage and sets it
    /// ### Arguments
    /// * `amount` - The amount to multiply by
    /// * `what` - The [`WorkId`] to change the value of
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for a float, it only checks the [`WorkKind`] and the index
    #[offset = 0x88]
    pub mul_float: extern "C" fn(this: &mut WorkModule, amount: f32, what: WorkId),

    /// Divides the specified float in the module's internal storage by the specified amount
    /// ### Arguments
    /// * `amount` - The amount to divide by
    /// * `what` - The [`WorkId`] to change the value of
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for a float, it only checks the [`WorkKind`] and the index
    #[offset = 0x90]
    pub div_float: extern "C" fn(this: &mut WorkModule, amount: f32, what: WorkId),

    /// Gets the specified 32-bit integer from the module's internal storage
    /// ### Arguments
    /// * `what` - The work ID specifiying what to get
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    #[offset = 0x98]
    pub get_int: extern "C" fn(this: &mut WorkModule, what: WorkId) -> i32,

    /// Sets the specified 32-bit integer in the module's internal storage
    /// ### Arguments
    /// * `value` - The new value of the variable
    /// * `what` - The work ID specify what to set
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    #[offset = 0xA0]
    pub set_int: extern "C" fn(this: &mut WorkModule, value: i32, what: WorkId),

    /// Sets the specified 32-bit integers in the module's internal storage
    /// ### Arguments
    /// * `value` - The new value of the variables
    /// * `ids` - The [`WorkId`]s to set the value of
    /// * `count` - The number of provided IDs
    ///
    /// ### Notes
    /// The implementation does not check that these IDs are for integers, it only checks the [`WorkKind`]s and the indices
    #[offset = 0xA8]
    set_ints_impl: extern "C" fn(this: &mut WorkModule, value: i32, ids: *const WorkId, count: u32),

    /// Gets the specified 64-bit integer from the module's internal storage
    /// ### Arguments
    /// * `what` - The work ID specifiying what to get
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    #[offset = 0xB0]
    pub get_int64: extern "C" fn(this: &mut WorkModule, what: WorkId) -> i64,

    /// Sets the specified 64-bit integer in the module's internal storage
    /// ### Arguments
    /// * `value` - The new value of the variable
    /// * `what` - The work ID specify what to set
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    #[offset = 0xB8]
    pub set_int64: extern "C" fn(this: &mut WorkModule, value: i64, what: WorkId),

    /// Assigns a random value in the specified range to the specified 32-bit integer in the module's internal storage
    /// ### Arguments
    /// * `minimum` - The minimum value of the random range
    /// * `maximum` - The maximum value of the random range
    /// * `what` - The [`WorkId`] to set the value of
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    /// * If `maximum` is less than `minimum`, then the value is not set at all.
    /// * If `maximum` is equal to `minimum`, then the value is set to `maximum`
    #[offset = 0xC0]
    pub random_int: extern "C" fn(this: &mut WorkModule, minimum: i32, maximum: i32, what: WorkId),

    /// Increments the specified 32-bit integer in the module's internal storage
    /// ### Arguments
    /// * `what` - The work ID specifying what to increment
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    #[offset = 0xC8]
    pub inc_int: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Decrements the specified 32-bit integer in the module's internal storage
    /// ### Arguments
    /// * `what` - The work ID specifying what to increment
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    #[offset = 0xD0]
    pub dec_int: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Adds the specified amount to the specified 32-bit integer in the module's internal storage
    /// ### Arguments
    /// * `amount` - The amount to add
    /// * `what` - The [`WorkId`] to change the value of
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    #[offset = 0xD8]
    pub add_int: extern "C" fn(this: &mut WorkModule, amount: i32, what: WorkId),

    /// Subtracts the specified amount from the specified 32-bit integer in the module's internal storage
    /// ### Arguments
    /// * `amount` - The amount to subtract
    /// * `what` - The [`WorkId`] to change the value of
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    #[offset = 0xE0]
    pub sub_int: extern "C" fn(this: &mut WorkModule, amount: i32, what: WorkId),

    /// Multiplies the specified amount by the specified 32-bit integer in the module's internal storage and sets it
    /// ### Arguments
    /// * `amount` - The amount to multiply by
    /// * `what` - The [`WorkId`] to change the value of
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    #[offset = 0xE8]
    pub mul_int: extern "C" fn(this: &mut WorkModule, amount: i32, what: WorkId),

    /// Divides the specified 32-bit integer in the module's internal storage by the specified amount
    /// ### Arguments
    /// * `amount` - The amount to divide by
    /// * `what` - The [`WorkId`] to change the value of
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    #[offset = 0xF0]
    pub div_int: extern "C" fn(this: &mut WorkModule, amount: i32, what: WorkId),

    /// Counts down the specified 32-bit integer until it is equal to the specified minimum
    /// ### Arguments
    /// * `what` - The [`WorkId`] to count down
    /// * `floor` - The value to count down to
    ///
    /// ### Behavior
    /// The module will fetch the specified value and then check if it is greater than `floor`. If it is,
    /// then it decrements the value.
    ///
    /// This call does nothing if the value was already less than or equal to floor, and returns `false`.
    ///
    /// ### Returns
    /// * `true` - The value was greater than `floor` before the call and is equal to `floor` after the call
    /// * `false` - The value was either less than or equal to `floor` before the call **or** it is still greater than
    /// `floor` after the call
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    #[offset = 0xF8]
    pub count_down_int: extern "C" fn(this: &mut WorkModule, what: WorkId, floor: i32) -> bool,

    /// Counts down the specified 32-bit integer until it is equal to the specified minimum
    /// ### Arguments
    /// * `ids` - The [`WorkId`]s to count down
    /// * `count` - The number of provided IDs
    /// * `floor` - The value to count down to
    ///
    /// ### Behavior
    /// For each value, the module will fetch the specified value and then check if it is greater than `floor`. If it is,
    /// then it decrements the value, doing nothing if the value was already less than or equal to `floor`.
    ///
    /// Unlike `count_down_int`, this call does not return anything.
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    #[offset = 0x100]
    count_down_ints_impl:
        extern "C" fn(this: &mut WorkModule, ids: *const WorkId, count: u32, floor: i32),

    /// Gets the specified flag from the module's internal storage
    /// ### Arguments
    /// * `what` - The work ID specifiying what to get
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for a flag, it only checks the [`WorkKind`] and the index
    #[offset = 0x108]
    pub is_flag: extern "C" fn(this: &WorkModule, what: WorkId) -> bool,

    /// Sets the specified flag to true
    /// ### Arguments
    /// * `what` - The work ID specifiying which flag to enable
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for a flag, it only checks the [`WorkKind`] and the index
    #[offset = 0x110]
    pub on_flag: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Sets the specified flags to true
    /// ### Arguments
    /// * `ids` - The [`WorkId`]s to turn on
    /// * `count` - The number of provided IDs
    ///
    /// ### Notes
    /// The implementation does not check that these IDs are for flags, it only checks the [`WorkKind`] and the indices
    #[offset = 0x118]
    on_flags_impl: extern "C" fn(this: &mut WorkModule, ids: *const WorkId, count: u32),

    /// Sets the specified flag to false
    /// ### Arguments
    /// * `what` - The work ID specifiying which flag to disable
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for a flag, it only checks the [`WorkKind`] and the index
    #[offset = 0x120]
    pub off_flag: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Sets the specified flags to false
    /// ### Arguments
    /// * `ids` - The [`WorkId`]s to turn off
    /// * `count` - The number of provided IDs
    ///
    /// ### Notes
    /// The implementation does not check that these IDs are for flags, it only checks the [`WorkKind`] and the indices
    #[offset = 0x128]
    off_flags_impl: extern "C" fn(this: &mut WorkModule, ids: *const WorkId, count: u32),

    /// Sets the specified flag in the module's internal storage
    /// ### Arguments
    /// * `value` - The value to set the flag to
    /// * `what` - The work ID specifiying which flag to set
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for a flag, it only checks the [`WorkKind`] and the index
    #[offset = 0x130]
    pub set_flag: extern "C" fn(this: &mut WorkModule, value: bool, what: WorkId),

    /// Sets the specified flag to false and returns true if it was previously on
    /// ### Arguments
    /// * `what` - The work ID specifying which flag to disable
    ///
    /// ### Returns
    /// * `true` - If the flag was on before the call and was turned off by the call
    /// * `false` - If the flag was off before the call
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for a flag, it only checks the [`WorkKind`] and the index
    #[offset = 0x138]
    pub turn_off_flag: extern "C" fn(this: &mut WorkModule, what: WorkId) -> bool,

    /// Enables the specified transition term group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to enable
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * Enabling the transition term **group** does not enable all contained transition terms, you still must call
    /// `enable_transition_term_group_ex` to enable a transition term that is within the group.
    /// * Enabling a transition term as part of a group will override being enabled by `enable_transition_term`
    /// ## Example
    /// ```rs
    /// // this code will enable the landing group and specifically enable the landing transition
    /// //
    /// // this ensures that the landing group (which includes regular and light landings)
    /// // is enabled, so that some other check can turn on specifically
    /// // light landing with `enable_transition_term_group_ex`
    /// //
    /// // if we didn't enable the landing group, then that `ex` enable wouldn't do anything when
    /// // the transition is checked
    /// fn enable_air_landing_transitions(module_accessor: &mut BattleObjectModuleAccessor) {
    ///     let wm = module_accessor.work_module();
    ///     wm.enable_transition_term_group(app::transition_groups::CHECK_AIR_LANDING);
    ///     wm.enable_transition_term_group_ex(app::transition_terms::LANDING);
    /// }
    /// ```
    #[offset = 0x140]
    pub enable_transition_term_group: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disabled the specified transition term group
    /// ### Arguments
    /// * `what` - The work ID specifiying which transition term **group** to disable
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * Disabling the transition term **group** does not disable all contained transition terms inherently. If any of them
    /// have been enabled by a call to `enable_transition_term`, they will still be enabled after this call.
    #[offset = 0x148]
    pub unable_transition_term_group: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Clears both group flags for all flags in the transition term group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to clear
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * This call has the same behavior as calling both `unable_transition_term_group` and `unable_transition_term_group_ex_all` with the argument `what`
    #[offset = 0x150]
    pub clear_transition_term_group: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Checks if the transition term group is enabled
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to check
    ///
    /// ### Behavior
    /// This call will check the first transition term in the group exclusively to see if it's enabled. This means that if any
    /// transition term groups share the same first term in their list, the result of this function can be incorrect. Fortunately,
    /// none of the groups overlap that way
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for a transition term group, it only checks the index
    #[offset = 0x158]
    pub is_enable_transition_term_group: extern "C" fn(this: &WorkModule, what: WorkId) -> bool,

    /// Enables the EX flag on the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to enable the EX flag on
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * Calling this function (and by extension setting the flag) will do nothing until the group flag is also set
    /// for this transition term (can be set by calling `enable_transition_term_group` on a group that contains the flag)
    #[offset = 0x160]
    pub enable_transition_term_group_ex: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Enables the EX flag on all of the transition terms of the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to enable the EX flag on
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * Pairing this function to a call of `enable_transition_term_group` with the same group ID effectively enables
    /// every transition term in the group
    #[offset = 0x168]
    pub enable_transition_term_group_ex_all: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the EX flag on the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to disable the EX flag on
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * Calling this function (and by extension disabling the flag) will do nothing if the group flag is not also set
    /// for this transition term (can be set by calling `enable_transition_term_group` on a group that contains the flag)
    #[offset = 0x170]
    pub unable_transition_term_group_ex: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the EX flag on all of the transition terms of the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to disable the EX flag on
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for a transition term group, it only checks the index
    #[offset = 0x178]
    pub unable_transition_term_group_ex_all: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Checks if the transition term is enabled.
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to check
    ///
    /// ### Behavior
    /// This call does all of the checks on the transition term, including checking if it's in a group or if it's forbidden.
    ///
    /// It's functionality is:
    /// ```rs
    /// let term = get_term(what);
    /// if term.is_forbidden() {
    ///     return false;
    /// }
    ///
    /// if term.is_enabled_group() {
    ///     return term.is_enabled_group_ex();   
    /// } else {
    ///     return term.is_enabled();
    /// }
    /// ```
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for a transition term, it only checks the index
    #[offset = 0x180]
    pub is_enable_transition_term: extern "C" fn(this: &WorkModule, what: WorkId) -> bool,

    /// Enables the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to enable
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This call will set the flag regardless of whether or not this is enabled as part of a group, but if
    /// it is enabled as part of a group the flag means nothing.
    #[offset = 0x188]
    pub enable_transition_term: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to disable
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This call will disable the flag regardless of whether or not this is enabled as part of a group, but if
    /// it is enabled as part of a group the flag means nothing.
    #[offset = 0x190]
    pub unable_transition_term: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables all transition terms in the module
    #[offset = 0x198]
    pub clear_transition_term: extern "C" fn(this: &mut WorkModule),

    /// Checks if any of the `forbid` flags are set for the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to check
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for a transition term, it only checks the index
    #[offset = 0x1A0]
    pub is_enable_transition_term_forbid: extern "C" fn(this: &WorkModule) -> bool,

    /// Sets the [`TransitionTerm::ENABLE_FORBID`] flag for the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to forbid
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This only sets one of the three flags which can forbid a transition term
    #[offset = 0x1A8]
    pub enable_transition_term_forbid: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID`] flag for the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to no longer forbid
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This only disables one of the three flags which can forbid a transition term
    #[offset = 0x1B0]
    pub unable_transition_term_forbid: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Sets the [`TransitionTerm::ENABLE_FORBID`] flag for all transition terms in the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to forbid
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * This only sets one of the three flags which can forbid a transition term
    #[offset = 0x1B8]
    pub enable_transition_term_forbid_group: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID`] flag for all transition terms in the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to no longer forbid
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * This only disables one of the three flags which can forbid a transition term
    #[offset = 0x1C0]
    pub unable_transition_term_forbid_group: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID`] flag for all transition terms in the module
    ///
    /// ### Notes
    /// This only disables one of the three flags which can forbid a transition term
    #[offset = 0x1C8]
    pub clear_transition_term_forbid: extern "C" fn(this: &mut WorkModule),

    /// Sets the [`TransitionTerm::ENABLE_FORBID_OTHER`] flag for the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to forbid
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This only sets one of the three flags which can forbid a transition term
    /// * This function is not exported by the main executable, the name is assumed
    #[offset = 0x1D0]
    pub enable_transition_term_forbid_other: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID_OTHER`] flag for the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to no longer forbid
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This only disables one of the three flags which can forbid a transition term
    /// * This function is not exported by the main executable, the name is assumed
    #[offset = 0x1D8]
    pub unable_transition_term_forbid_other: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Sets the [`TransitionTerm::ENABLE_FORBID_OTHER`] flag for all transition terms in the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to forbid
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * This only sets one of the three flags which can forbid a transition term
    /// * This function is not exported by the main executable, the name is assumed
    #[offset = 0x1E0]
    pub enable_transition_term_forbid_group_other:
        extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID_OTHER`] flag for all transition terms in the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to no longer forbid
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * This only disables one of the three flags which can forbid a transition term
    /// * This function is not exported by the main executable, the name is assumed
    #[offset = 0x1E8]
    pub unable_transition_term_forbid_group_other:
        extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID_OTHER`] flag for all transition terms in the module
    ///
    /// ### Notes
    /// * This only disables one of the three flags which can forbid a transition term
    /// * This function is not exported by the main executable, the name is assumed
    #[offset = 0x1F0]
    pub clear_transition_term_forbid_other: extern "C" fn(this: &mut WorkModule),

    /// Sets the [`TransitionTerm::ENABLE_FORBID_INDIVIDUAL`] flag for the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to forbid
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This only sets one of the three flags which can forbid a transition term
    #[offset = 0x1F8]
    pub enable_transition_term_forbid_indivi: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID_INDIVIDUAL`] flag for the specified transition term
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term to no longer forbid
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term, it only checks the index
    /// * This only disables one of the three flags which can forbid a transition term
    #[offset = 0x200]
    pub unable_transition_term_forbid_indivi: extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Sets the [`TransitionTerm::ENABLE_FORBID_INDIVIDUAL`] flag for all transition terms in the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to forbid
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * This only sets one of the three flags which can forbid a transition term
    #[offset = 0x208]
    pub enable_transition_term_forbid_group_indivi:
        extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID_INDIVIDUAL`] flag for all transition terms in the specified group
    /// ### Arguments
    /// * `what` - The work ID specifying which transition term **group** to no longer forbid
    ///
    /// ### Notes
    /// * The implementation does not check that this ID is for a transition term group, it only checks the index
    /// * This only disables one of the three flags which can forbid a transition term
    #[offset = 0x210]
    pub unable_transition_term_forbid_group_indivi:
        extern "C" fn(this: &mut WorkModule, what: WorkId),

    /// Disables the [`TransitionTerm::ENABLE_FORBID_INDIVIDUAL`] flag for all transition terms in the module
    ///
    /// ### Notes
    /// * This only disables one of the three flags which can forbid a transition term
    /// * This function is not exported by the main executable, the name is assumed
    #[offset = 0x218]
    pub clear_transition_term_forbid_indivi: extern "C" fn(this: &mut WorkModule),

    /// Re-initializes all of the internal storage to 0 (floats are `0.0`, ints are `0`, and flags are `false`)
    #[offset = 0x220]
    pub clear_all: extern "C" fn(this: &mut WorkModule),

    /// Clears the status arrays of the module's internal storage
    /// ### Arguments
    /// * `keep_float` - Bitmask of the first 32 status float variables to keep (-1 keeps all)
    /// * `keep_int` - Bitmask of the first 32 status ints to keep (-1 keeps all)
    /// * `keep_flag` - Bitmask of the first 32 status flags to keep (-1 keeps all)
    ///
    /// ### Notes
    /// * Passing `0` for all three arguments causes the implementation to turn into a `memset` call
    /// * If there are more than 32 variables for the status, you either clear everything (pass in 0 for all three arguments)
    /// or you anything with index 32 and greater must be cleared/kept manually
    /// * This function is called by `StatusModule::init_settings`
    /// * This function is not exported by the main executable, the name is assumed
    #[offset = 0x228]
    pub clear_status:
        extern "C" fn(this: &mut WorkModule, keep_float: i32, keep_int: i32, keep_flag: i32),

    /// Gets the internal work array info structure
    ///
    /// ### Notes
    /// This function is not exported by the main executable, the name is assumed
    #[offset = 0x230]
    get_work_array_info: extern "C" fn(this: &mut WorkModule) -> *mut WorkArrayInfo,

    /// Gets the specified internal work array
    ///
    /// ### Notes
    /// This function is not exported by the main executable, the name is assumed
    #[offset = 0x238]
    get_work_array: extern "C" fn(this: &mut WorkModule) -> *mut WorkArray,

    #[offset = 0x240]
    unk_3: extern "C" fn(this: &mut WorkModule),

    /// Gets a 32-bit integer from this module's param object
    /// ### Arguments
    /// * `param_object` - The structure which holds the sought param (for fighter params, this field should be the name of the param)
    /// * `param_name` - The name of the param to be retrieved (for fighter params, this field should be `0`)
    ///
    /// ### Notes
    /// * This function is called internally by `get_param_int`
    /// * This function is not exported by the main executable, the name is assumed
    #[offset = 0x248]
    get_param_int_impl:
        extern "C" fn(this: &WorkModule, param_object: phx::Hash40, param_name: phx::Hash40) -> i32,

    /// Gets a 32-bit integer from this module's param object
    /// ### Arguments
    /// * `param_object` - The structure which holds the sought param (for fighter params, this field should be the name of the param)
    /// * `param_name` - The name of the param to be retrieved (for fighter params, this field should be `0`)
    #[offset = 0x250]
    pub get_param_int:
        extern "C" fn(this: &WorkModule, param_object: phx::Hash40, param_name: phx::Hash40) -> i32,

    /// Gets a 64-bit integer from this module's param object
    /// ### Arguments
    /// * `param_object` - The structure which holds the sought param (for fighter params, this field should be the name of the param)
    /// * `param_name` - The name of the param to be retrieved (for fighter params, this field should be `0`)
    ///
    /// ### Notes
    /// * This function is called internally by `get_param_int64`
    /// * This function is not exported by the main executable, the name is assumed
    #[offset = 0x258]
    get_param_int64_impl:
        extern "C" fn(this: &WorkModule, param_object: phx::Hash40, param_name: phx::Hash40) -> i64,

    /// Gets a 64-bit integer from this module's param object
    /// ### Arguments
    /// * `param_object` - The structure which holds the sought param (for fighter params, this field should be the name of the param)
    /// * `param_name` - The name of the param to be retrieved (for fighter params, this field should be `0`)
    #[offset = 0x260]
    pub get_param_int64:
        extern "C" fn(this: &WorkModule, param_object: phx::Hash40, param_name: phx::Hash40) -> i64,

    /// Gets a float from this module's param object
    /// ### Arguments
    /// * `param_object` - The structure which holds the sought param (for fighter params, this field should be the name of the param)
    /// * `param_name` - The name of the param to be retrieved (for fighter params, this field should be `0`)
    ///
    /// ### Notes
    /// * This function is called internally by `get_param_float`
    /// * This function is not exported by the main executable, the name is assumed
    #[offset = 0x268]
    get_param_float_impl:
        extern "C" fn(this: &WorkModule, param_object: phx::Hash40, param_name: phx::Hash40) -> f32,

    /// Gets a float from this module's param object
    /// ### Arguments
    /// * `param_object` - The structure which holds the sought param (for fighter params, this field should be the name of the param)
    /// * `param_name` - The name of the param to be retrieved (for fighter params, this field should be `0`)
    #[offset = 0x270]
    pub get_param_float:
        extern "C" fn(this: &WorkModule, param_object: phx::Hash40, param_name: phx::Hash40) -> f32,

    /// Sets the move customization fields of the param object
    /// ### Arguments
    /// * `variant` - The variant number of the move
    /// * `move_no` - The number of the move to customize
    ///
    /// ### Notes
    /// * This is only really used by mii fighters and joker that we are aware of (joker for arsene, mii fighters for their specials)
    /// * This is associated with the `FIGHTER_WAZA_CUSTOMIZE` constants and work IDs
    #[offset = 0x278]
    pub set_customize_no: extern "C" fn(this: &mut WorkModule, variant: i32, move_no: i32),

    /// Recalculates runtime-cached params based off of a stat change.
    /// ### Arguments
    /// * `_unused` - Doesn't appear to be used, is usually `0`
    /// * `is_abnormal` - Set if the stat change is not your normal stats (i.e. picking up a bunny hood)
    ///
    /// ### Notes
    /// * This can be triggered when you pick up an item that changes some stats for the fighter, such as a bunny hood, mushroom, or
    /// getting struck by lightning
    /// * This function sends an event with ID `0x31`
    /// * This function is not exported by the main executable, the name is assumed
    #[offset = 0x280]
    pub recalculate_params: extern "C" fn(this: &mut WorkModule, _unused: u64, is_abnormal: bool),
}

/// A method to store agent/game state across multiple users
///
/// `WorkModule` is one of Ultimate's modules, and it's primary purpose is store and retrieve
/// variables/information that are related to an object's state. It does this in two main ways:
/// 1. Getting and setting mutable variables using [`WorkModule::get_int`], [`WorkModule::get_float`],
/// [`WorkModule::get_int64`], [`WorkModule::is_flag`], and their `set` alternatives.
/// 2. Getting (mostly) read-only data via param accessing functions [`WorkModule::get_param_int`] along
/// with `float` and `int64` (usually a hash) alternatives.
///
/// `WorkModule` is the most effective way to share this information, as the amount of storage required
/// will vary depending on the user. It can be accessed between multiple different users as it is tied
/// to an owning [`app::BattleObjectModuleAccessor`]
#[repr(C)]
pub struct WorkModule {
    vtable: &'static WorkModuleVTable,
    owner: *mut app::BattleObjectModuleAccessor,
}

impl WorkModule {
    /// Sets the specified floats in the module's internal storage
    /// ### Arguments
    /// * `value` - The new value of the variables
    /// * `ids` - The [`WorkId`]s to set the value of
    ///
    /// ### Notes
    /// The implementation does not check that these IDs are for floats, it only checks the [`WorkKind`]s and the indices
    pub fn set_floats(&mut self, value: f32, ids: impl AsRef<[WorkId]>) {
        let ids = ids.as_ref();
        self.set_floats_impl(value, ids.as_ptr(), ids.len() as u32)
    }

    /// Sets the specified 32-bit integers in the module's internal storage
    /// ### Arguments
    /// * `value` - The new value of the variables
    /// * `ids` - The [`WorkId`]s to set the value of
    ///
    /// ### Notes
    /// The implementation does not check that these IDs are for integers, it only checks the [`WorkKind`]s and the indices
    pub fn set_ints(&mut self, value: i32, ids: impl AsRef<[WorkId]>) {
        let ids = ids.as_ref();
        self.set_ints_impl(value, ids.as_ptr(), ids.len() as u32)
    }

    /// Counts down the specified 32-bit integer until it is equal to the specified minimum
    /// ### Arguments
    /// * `ids` - The [`WorkId`]s to count down
    /// * `floor` - The value to count down to
    ///
    /// ### Behavior
    /// For each value, the module will fetch the specified value and then check if it is greater than `floor`. If it is,
    /// then it decrements the value, doing nothing if the value was already less than or equal to `floor`.
    ///
    /// Unlike `count_down_int`, this call does not return anything.
    ///
    /// ### Notes
    /// The implementation does not check that this ID is for an integer, it only checks the [`WorkKind`] and the index
    pub fn count_down_ints(&mut self, ids: impl AsRef<[WorkId]>, floor: i32) {
        let ids = ids.as_ref();
        self.count_down_ints_impl(ids.as_ptr(), ids.len() as u32, floor)
    }

    /// Sets the specified flags to true
    /// ### Arguments
    /// * `ids` - The [`WorkId`]s to turn on
    ///
    /// ### Notes
    /// The implementation does not check that these IDs are for flags, it only checks the [`WorkKind`] and the indices
    pub fn on_flags(&mut self, ids: impl AsRef<[WorkId]>) {
        let ids = ids.as_ref();
        self.on_flags_impl(ids.as_ptr(), ids.len() as u32)
    }

    /// Sets the specified flags to false
    /// ### Arguments
    /// * `ids` - The [`WorkId`]s to turn off
    /// * `count` - The number of provided IDs
    ///
    /// ### Notes
    /// The implementation does not check that these IDs are for flags, it only checks the [`WorkKind`] and the indices
    pub fn off_flags(&mut self, ids: impl AsRef<[WorkId]>) {
        let ids = ids.as_ref();
        self.off_flags_impl(ids.as_ptr(), ids.len() as u32)
    }
}

/// The fighter implementation of [`WorkModule`](app::WorkModule)
#[repr(C)]
#[virtual_implementor(WorkModule)]
#[derive(TypeAssert)]
#[size = 0x98]
pub struct FighterWorkModuleImpl {
    #[offset = 0x00]
    parent: WorkModule,
    #[offset = 0x10]
    pub work_array_info: WorkArrayInfo,
    #[offset = 0x20]
    pub transition_term_info: TransitionTermInfo,
    #[offset = 0x30]
    transition_groups: *const cpp::Array<TransitionGroup>,
    #[offset = 0x38]
    param_object: u64,
    #[offset = 0x40]
    fighter_kind: app::FighterKind,
    #[offset = 0x44]
    entry_id: app::FighterEntryID,
    #[offset = 0x48]
    _x48: u64,
    #[offset = 0x50]
    fighter_info: *mut app::FighterInformation,
    #[offset = 0x58]
    _x58: [u64; 8],
}
