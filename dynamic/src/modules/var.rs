use smash::phx::*;
use smash::app::BattleObject;
extern "Rust" {
    #[link_name = "VarModule__has_var_module"]
    fn VarModule__has_var_module(object: *mut BattleObject) -> bool;

    #[link_name = "VarModule__reset"]
    fn VarModule__reset(object: *mut BattleObject, mask: u8);

    #[link_name = "VarModule__get_int"]
    fn VarModule__get_int(object: *mut BattleObject, what: i32) -> i32;

    #[link_name = "VarModule__get_float"]
    fn VarModule__get_float(object: *mut BattleObject, what: i32) -> f32;

    #[link_name = "VarModule__get_int64"]
    fn VarModule__get_int64(object: *mut BattleObject, what: i32) -> u64;

    #[link_name = "VarModule__is_flag"]
    fn VarModule__is_flag(object: *mut BattleObject, what: i32) -> bool;

    #[link_name = "VarModule__set_int"]
    fn VarModule__set_int(object: *mut BattleObject, what: i32, val: i32);

    #[link_name = "VarModule__set_float"]
    fn VarModule__set_float(object: *mut BattleObject, what: i32, val: f32);

    #[link_name = "VarModule__set_int64"]
    fn VarModule__set_int64(object: *mut BattleObject, what: i32, val: u64);

    #[link_name = "VarModule__set_flag"]
    fn VarModule__set_flag(object: *mut BattleObject, what: i32, val: bool);

    #[link_name = "VarModule__off_flag"]
    fn VarModule__off_flag(object: *mut BattleObject, what: i32);

    #[link_name = "VarModule__on_flag"]
    fn VarModule__on_flag(object: *mut BattleObject, what: i32);

    #[link_name = "VarModule__countdown_int"]
    fn VarModule__countdown_int(object: *mut BattleObject, what: i32, min: i32) -> bool;

    #[link_name = "VarModule__add_int"]
    fn VarModule__add_int(object: *mut BattleObject, what: i32, amount: i32);

    #[link_name = "VarModule__sub_int"]
    fn VarModule__sub_int(object: *mut BattleObject, what: i32, amount: i32);

    #[link_name = "VarModule__inc_int"]
    fn VarModule__inc_int(object: *mut BattleObject, what: i32);

    #[link_name = "VarModule__dec_int"]
    fn VarModule__dec_int(object: *mut BattleObject, what: i32);

    #[link_name = "VarModule__add_float"]
    fn VarModule__add_float(object: *mut BattleObject, what: i32, amount: f32);

    #[link_name = "VarModule__sub_float"]
    fn VarModule__sub_float(object: *mut BattleObject, what: i32, amount: f32);

    #[link_name = "VarModule__set_vec2"]
    fn VarModule__set_vec2(object: *mut BattleObject, what: i32, val: Vector2f);

    #[link_name = "VarModule__set_vec3"]
    fn VarModule__set_vec3(object: *mut BattleObject, what: i32, val: Vector3f);

    #[link_name = "VarModule__set_vec4"]
    fn VarModule__set_vec4(object: *mut BattleObject, what: i32, val: Vector4f);

    #[link_name = "VarModule__get_vec2"]
    fn VarModule__get_vec2(object: *mut BattleObject, what: i32) -> Vector2f;

    #[link_name = "VarModule__get_vec3"]
    fn VarModule__get_vec3(object: *mut BattleObject, what: i32) -> Vector3f;

    #[link_name = "VarModule__get_vec4"]
    fn VarModule__get_vec4(object: *mut BattleObject, what: i32) -> Vector4f;
}

#[allow(non_snake_case)]
/// An additional module to be used with Smash's `BattleObject` class. This handles storing and retrieving primitive variables
/// that you want to associate with a specific object (such as associating a gimmick timer with mario or dk)
pub mod VarModule {
    use super::*;
    /// Resets all integers that are within the instance array.
    pub const RESET_INSTANCE_INT:   u8 = 0b00000001;
    /// Resets all 64-bit values that are within the instance array
    pub const RESET_INSTANCE_INT64: u8 = 0b00000010;
    /// Resets all floats that are within the instance array
    pub const RESET_INSTANCE_FLOAT: u8 = 0b00000100;
    /// Resets all flags that are within the instance array (default is `false`)
    pub const RESET_INSTANCE_FLAG:  u8 = 0b00001000;

    /// Resets all integers that are within the status array
    pub const RESET_STATUS_INT:   u8 = 0b00010000;
    /// Resets all 64-bit values that are within the status array
    pub const RESET_STATUS_INT64: u8 = 0b00100000;
    /// Resets all floats that are within the status array
    pub const RESET_STATUS_FLOAT: u8 = 0b01000000;
    /// Resets all flags that are within the status array
    pub const RESET_STATUS_FLAG:  u8 = 0b10000000;

    /// Resets all integers
    pub const RESET_INT:   u8 = RESET_INSTANCE_INT | RESET_STATUS_INT;
    /// Resets all 64-bit values
    pub const RESET_INT64: u8 = RESET_INSTANCE_INT64 | RESET_STATUS_INT64;
    /// Resets all floats
    pub const RESET_FLOAT: u8 = RESET_INSTANCE_FLOAT | RESET_STATUS_FLOAT;
    /// Resets all flags
    pub const RESET_FLAG:  u8 = RESET_INSTANCE_FLAG | RESET_STATUS_FLAG;

    /// Resets all values in the instance array
    pub const RESET_INSTANCE: u8 = 0xF;
    /// Resets all values in the status array
    pub const RESET_STATUS:   u8 = 0xF0;
    /// Resets all values
    pub const RESET_ALL:      u8 = 0xFF;

    /// Checks if the object has `VarModule`
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    pub fn has_var_module(object: *mut BattleObject) -> bool {
        unsafe {
            VarModule__has_var_module(object)
        }
    }

    /// Resets various `VarModule` arrays depending on the mask
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `mask` - A mask of the reset values to determine what to reset
    pub fn reset(object: *mut BattleObject, mask: u8) {
        unsafe {
            VarModule__reset(object, mask)
        }
    }

    /// Retrieves an integer
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to retrieve
    /// # Returns
    /// The variable requested
    pub fn get_int(object: *mut BattleObject, what: i32) -> i32 {
        unsafe {
            VarModule__get_int(object, what)
        }
    }

    /// Retrieves a float
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to retrieve
    /// # Returns
    /// The variable requested
    pub fn get_float(object: *mut BattleObject, what: i32) -> f32 {
        unsafe {
            VarModule__get_float(object, what)
        }
    }

    /// Retrieves a 64-bit value
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to retrieve
    /// # Returns
    /// The variable requested
    pub fn get_int64(object: *mut BattleObject, what: i32) -> u64 {
        unsafe {
            VarModule__get_int64(object, what)
        }
    }

    /// Retrieves a flag
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to retrieve
    /// # Returns
    /// The variable requested
    pub fn is_flag(object: *mut BattleObject, what: i32) -> bool {
        unsafe {
            VarModule__is_flag(object, what)
        }
    }

    /// Sets an integer value
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to set
    /// * `val` - The value to set the variable to
    pub fn set_int(object: *mut BattleObject, what: i32, val: i32) {
        unsafe {
            VarModule__set_int(object, what, val)
        }
    }

    /// Sets a float value
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to set
    /// * `val` - The value to set the variable to
    pub fn set_float(object: *mut BattleObject, what: i32, val: f32) {
        unsafe {
            VarModule__set_float(object, what, val)
        }
    }

    /// Sets a 64-bit value
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to set
    /// * `val` - The value to set the variable to
    pub fn set_int64(object: *mut BattleObject, what: i32, val: u64) {
        unsafe {
            VarModule__set_int64(object, what, val)
        }
    }

    /// Sets a flag
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to set
    /// * `val` - The value to set the variable to
    pub fn set_flag(object: *mut BattleObject, what: i32, val: bool) {
        unsafe {
            VarModule__set_flag(object, what, val)
        }
    }

    /// Sets a flag to false
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to set
    /// # Note
    /// This method is equivalent to `VarModule::set_flag(object, what, false)`
    pub fn off_flag(object: *mut BattleObject, what: i32) {
        unsafe {
            VarModule__off_flag(object, what)
        }
    }

    /// Sets a flag to true
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to set
    /// # Note
    /// This method is equivalent to `VarModule::set_flag(object, what, true)`
    pub fn on_flag(object: *mut BattleObject, what: i32) {
        unsafe {
            VarModule__on_flag(object, what)
        }
    }

    /// Countdowns an integer
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to count down
    /// * `min` - The minimum value that variable should be before we are done counting down
    /// # Returns
    /// * `true` - `what` was less than `min` before or after decrementing
    /// * `false` - `what` remains greater than or equal to `min` after decrementing
    pub fn countdown_int(object: *mut BattleObject, what: i32, min: i32) -> bool {
        unsafe {
            VarModule__countdown_int(object, what, min)
        }
    }

    /// Adds a value to an integer
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to add to
    /// * `amount` - The value to add to the variable
    pub fn add_int(object: *mut BattleObject, what: i32, amount: i32) {
        unsafe {
            VarModule__add_int(object, what, amount)
        }
    }

    /// Subtracts a value from an integer
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to subtract from
    /// * `amount` - The value to subtract from the variable
    pub fn sub_int(object: *mut BattleObject, what: i32, amount: i32) {
        unsafe {
            VarModule__sub_int(object, what, amount)
        }
    }

    /// Increments an integer
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to increment
    /// # Note
    /// This is functionally equivalent to `VarModule::add_int(object, what, 1)`
    pub fn inc_int(object: *mut BattleObject, what: i32) {
        unsafe {
            VarModule__inc_int(object, what)
        }
    }

    /// Decrements an integer
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to decrement
    /// # Note
    /// This is functionally equivalent to `VarModule::sub_int(object, what, 1)`
    pub fn dec_int(object: *mut BattleObject, what: i32) {
        unsafe {
            VarModule__dec_int(object, what)
        }
    }

    /// Adds a value to float
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to add on to
    /// * `amount` - The amount to add to the variable
    pub fn add_float(object: *mut BattleObject, what: i32, amount: f32) {
        unsafe {
            VarModule__add_float(object, what, amount)
        }
    }

    /// Subtracts a value from a float
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - The variable to subtract from
    /// * `amount` - The amount to subtract from the variable
    pub fn sub_float(object: *mut BattleObject, what: i32, amount: f32) {
        unsafe {
            VarModule__sub_float(object, what, amount)
        }
    }

    /// Sets a 2-dimensional vector
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - Where to start setting the vector
    /// * `val` - The vector to set
    /// # Panics
    /// This function requires that the last 3 bytes of `what` are less than `0xFFF`
    pub fn set_vec2(object: *mut BattleObject, what: i32, val: Vector2f) {
        unsafe {
            VarModule__set_vec2(object, what, val)
        }
    }

    /// Sets a 3-dimensional vector
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - Where to start setting the vector
    /// * `val` - The vector to set
    /// # Panics
    /// This function requires that the last 3 bytes of `what` are less than `0xFFE`
    pub fn set_vec3(object: *mut BattleObject, what: i32, val: Vector3f) {
        unsafe {
            VarModule__set_vec3(object, what, val)
        }
    }

    /// Sets a 4-dimensional vector
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - Where to start setting the vector
    /// * `val` - The vector to set
    /// # Panics
    /// This function requires that the last 3 bytes of `what` are less than `0xFFD`
    pub fn set_vec4(object: *mut BattleObject, what: i32, val: Vector4f) {
        unsafe {
            VarModule__set_vec4(object, what, val)
        }
    }

    /// Gets a 2-dimensional vector
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - Where to start setting the vector
    /// # Returns
    /// The 2-dimensional vector starting at the value specified
    /// # Panics
    /// This function requires that the last 3 bytes of `what` are less than `0xFFF`
    pub fn get_vec2(object: *mut BattleObject, what: i32) -> Vector2f {
        unsafe {
            VarModule__get_vec2(object, what)
        }
    }

    /// Gets a 3-dimensional vector
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - Where to start setting the vector
    /// # Returns
    /// The 3-dimensional vector starting at the value specified
    /// # Panics
    /// This function requires that the last 3 bytes of `what` are less than `0xFFE`
    pub fn get_vec3(object: *mut BattleObject, what: i32) -> Vector3f {
        unsafe {
            VarModule__get_vec3(object, what)
        }
    }

    /// Gets a 4-dimensional vector
    /// # Arguments
    /// * `object` - The owning `BattleObject` instance
    /// * `what` - Where to start setting the vector
    /// # Returns
    /// The 4-dimensional vector starting at the value specified
    /// # Panics
    /// This function requires that the last 3 bytes of `what` are less than `0xFFD`
    pub fn get_vec4(object: *mut BattleObject, what: i32) -> Vector4f {
        unsafe {
            VarModule__get_vec4(object, what)
        }
    }
}