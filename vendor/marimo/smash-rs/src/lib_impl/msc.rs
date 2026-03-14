use crate::*;

/// The structure to represent an MSC command.
/// 
/// Despite being mostly abstracted away via module and lua calls, MSC commands are
/// still used in ultimate, and you can find remnants of them throughout the fighter/weapon/item
/// code and the lua consts. For example, the following is an MSC command:
/// ```rs
/// pub fn unable_cancel_status(fighter: &mut L2CFighterCommon) {
///     fighter.clear_lua_stack();
///     fighter.push_lua_stack(L2CValue::new(app::msc_cmd::CANCEL_UNABLE_CANCEL_STATUS));
///     app::sv_module_access::cancel(fighter.lua_state);
///     fighter.pop_lua_stack(1);
/// }
/// ```
/// 
/// These commands will get sent off to the respective handler in these structures.
#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x10]
pub struct MscCommand {
    #[offset = 0x0] payload: *mut MscCommandPayload,
    #[offset = 0x8] output_count: u32
}

impl MscCommand {
    /// Gets the payload of the command
    pub fn payload<'a>(&'a self) -> &'a MscCommandPayload {
        unsafe {
            &*self.payload
        }
    }

    /// Gets the number of outputs this command returned.
    /// 
    /// ### Notes
    /// The number of outputs is an undefined quantity until after the command
    /// has been handled properly.
    pub fn get_output_count(&self) -> usize {
        self.output_count as usize
    }
}

/// The fundamental data type for [luac](https://www.lua.org/source/5.1/lua.h.html)
/// 
/// This type is similar to the [`lib::L2CValue`], however it isn't used in any exported APIs by the executable(s).
#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x10]
pub struct TValue {
    #[offset = 0x0] data: u64,
    #[offset = 0x8] tag_type: u32
}

/// The structure to represent the command request for a [`MscCommand`]
/// 
/// It comes packaged with everything the receiver needs to know about the command.
#[repr(C)]
#[derive(TypeAssert)]
#[size = 0x18]
pub struct MscCommandPayload {
    #[offset = 0x00] args: *const TValue,
    #[offset = 0x08] arg_count: u32,
    _padding: u32,
    #[offset = 0x10] lua_state: *mut lua_State
}

impl MscCommandPayload {
    /// Returns the arguments of the payload if the field is populated.
    /// 
    /// ### Notes
    /// The `args` field doesn't always need to be populated, as the default behavior is
    /// likely to first check if `lua_state` is `null`, and if not to pull the args off of
    /// its stack.
    pub fn default_args<'a>(&'a self) -> Option<&'a [TValue]> {
        if self.args.is_null() || self.arg_count == 0 {
            None
        } else {
            Some(unsafe { std::slice::from_raw_parts(self.args, self.arg_count as usize) })
        }
    }

    /// Returns the number of arguments in this command
    pub fn arg_count(&self) -> usize {
        self.arg_count as usize
    }

    /// Returns the `lua_State` which owns this command
    pub fn lua_state(&self) -> *mut lua_State {
        self.lua_state
    }
}