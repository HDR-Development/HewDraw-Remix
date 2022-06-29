use smash::app::BattleObject;

extern "Rust" {
    #[link_name = "BufferModule__persist_command_one"]
    fn BufferModule__persist_command_one(object: *mut BattleObject, category: i32, flag: i32);

    #[link_name = "BufferModule__persist_command_one_with_lifetime"]
    fn BufferModule__persist_command_one_with_lifetime(object: *mut BattleObject, category: i32, flag: i32, lifetime: i32);

    #[link_name = "BufferModule__set_persist_lifetime"]
    fn BufferModule__set_persist_lifetime(object: *mut BattleObject, lifetime: i32);

    #[link_name = "BufferModule__enable_persist"]
    fn BufferModule__enable_persist(object: *mut BattleObject);

    #[link_name = "BufferModule__disable_persist"]
    fn BufferModule__disable_persist(object: *mut BattleObject);

    #[link_name = "BufferModule__clear_persist"]
    fn BufferModule__clear_persist(object: *mut BattleObject);

    #[link_name = "BufferModule__clear_persist_one"]
    fn BufferModule__clear_persist_one(object: *mut BattleObject, category: i32, flag: i32);

    #[link_name = "BufferModule__exec"]
    fn BufferModule__exec(object: *mut BattleObject, cats: &mut [&mut [u8]; 4]);

    #[link_name = "BufferModule__is_persist"]
    fn BufferModule__is_persist(object: *mut BattleObject) -> bool;

    #[link_name = "BufferModule__is_persist_one"]
    fn BufferModule__is_persist_one(object: *mut BattleObject, category: i32, flag: i32) -> bool;

    #[link_name = "BufferModule__persist_lifetime"]
    fn BufferModule__persist_lifetime(object: *mut BattleObject) -> i32;

    #[link_name = "BufferModule__persist_lifetime_one"]
    fn BufferModule__persist_lifetime_one(object: *mut BattleObject, category: i32, flag: i32) -> i32;

    #[link_name = "BufferModule__persist_lifetime_max_one"]
    fn BufferModule__persist_lifetime_max_one(object: *mut BattleObject, category: i32, flag: i32) -> i32;

}

/// An additional module to be used with Smash's `BattleObject` class. This handles manipulating and adjusting hold buffer
/// depending on the situation to encourage more precise inputs with some exceptions to allow for overall better game health and feel.
/// You can reference all of these calls from just passing the `BattleObject` into function. If a function is called on a `BattleObject` that does not have
/// `BufferModule` set up, it will panic.
#[allow(non_snake_case)]
pub mod BufferModule {
    use super::*;

    /// Enables the hold buffer on one specific input.
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are enabling hold buffer for
    pub fn persist_command_one(object: *mut BattleObject, category: i32, flag: i32) {
        unsafe {
            BufferModule__persist_command_one(object, category, flag)
        }
    }

    /// Enables the hold buffer on one specific input with a specified lifetime
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are enabling hold buffer for
    /// * `lifetime` - The maximum number of frames hold buffer is enabled for (-1 is infinite). This lifetime includes tap buffer frames.
    pub fn persist_command_one_with_lifetime(object: *mut BattleObject, category: i32, flag: i32, lifetime: i32) {
        unsafe {
            BufferModule__persist_command_one_with_lifetime(object, category, flag, lifetime)
        }
    }

    /// Sets the global hold buffer lifetime
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `lifetime` - The maximum number of frames hold buffer is enabled for (-1 is infinite). This lifetime includes tap buffer frames.
    pub fn set_persist_lifetime(object: *mut BattleObject, lifetime: i32) {
        unsafe {
            BufferModule__set_persist_lifetime(object, lifetime)
        }
    }

    /// Enables global hold buffer on all inputs for this `BattleObject`
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    pub fn enable_persist(object: *mut BattleObject) {
        unsafe {
            BufferModule__enable_persist(object)
        }
    }

    /// Disables global hold buffer for this `BattleObject`
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// # Note
    /// If specific inputs have hold buffer enabled, calling `disable_persist` will not disable those,
    /// only the global flag which enabled hold buffer on all inputs will be disabled
    pub fn disable_persist(object: *mut BattleObject) {
        unsafe {
            BufferModule__disable_persist(object)
        }
    }

    /// Clears all of the hold buffer information for every input
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// # Note
    /// This function is similar to `ControlModule::clear_command_flag_cat` in that it resets all information regarding holding those inputs.
    /// This does not impact anything in the `ControlModule` command information, only the `BufferModule` implementation
    pub fn clear_persist(object: *mut BattleObject) {
        unsafe {
            BufferModule__clear_persist(object)
        }
    }

    /// Clears the hold buffer information for one input
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are clearing hold buffer for
    pub fn clear_persist_one(object: *mut BattleObject, category: i32, flag: i32) {
        unsafe {
            BufferModule__clear_persist_one(object, category, flag)
        }
    }

    /// Updates the hold buffer information
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `cats` - `ControlModule` command flag information to update `BufferModule` with.
    /// # Note
    /// This method is not intended to be used by users of `BufferModule`. It is instead used internally with a hook to update every frame.
    pub fn exec(object: *mut BattleObject, cats: &mut [&mut [u8]; 4]) {
        unsafe {
            BufferModule__exec(object, cats)
        }
    }

    /// Checks whether or not global hold buffer is enabled
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// # Returns
    /// A boolean representing whether or not global hold buffer is enabled.
    pub fn is_persist(object: *mut BattleObject) -> bool {
        unsafe {
            BufferModule__is_persist(object)
        }
    }

    /// Checks whether or not hold buffer is enabled for a specific input
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are checking hold buffer for
    /// # Returns
    /// A boolean representing whether or not hold buffer is enabled for a specific input.
    pub fn is_persist_one(object: *mut BattleObject, category: i32, flag: i32) -> bool {
        unsafe {
            BufferModule__is_persist_one(object, category, flag)
        }
    }

    /// Gets the max amount of global hold buffer frames (can vary depending on input)
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// # Returns
    /// The number of frames hold buffer is enabled for globally.
    /// #Note
    /// This returns whatever value was last last set with `set_persist_lifetime` and
    /// is a valid value even when `is_persist` is false.
    pub fn persist_lifetime(object: *mut BattleObject) -> i32 {
        unsafe {
            BufferModule__persist_lifetime(object)
        }
    }

    /// Gets the current amount of frames an object has been holding an input for
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are checking hold buffer for
    /// # Returns
    /// The number of frames the input has been held
    pub fn persist_lifetime_one(object: *mut BattleObject, category: i32, flag: i32) -> i32 {
        unsafe {
            BufferModule__persist_lifetime_one(object, category, flag)
        }
    }

    /// Gets the max amount of hold buffer frames for a specified input
    /// # Arguments
    /// * `object` - Owning `BattleObject` instance
    /// * `category` - Which command flag category the input is under (valid values are 0-3)
    /// * `flag` - Which flag in the category you are checking hold buffer for
    /// # Returns
    /// The max amount of frames a specific input can have hold buffer for.
    pub fn persist_lifetime_max_one(object: *mut BattleObject, category: i32, flag: i32) -> i32 {
        unsafe {
            BufferModule__persist_lifetime_max_one(object, category, flag)
        }
    }
}