use crate::*;

mod attack;
mod cancel;
mod damage;
mod item;
mod lua;
mod motion_animcmd;
mod posture;
mod status;
mod work;

pub use attack::*;
pub use cancel::*;
pub use damage::*;
pub use item::*;
pub use lua::*;
pub use motion_animcmd::*;
pub use posture::*;
pub use status::*;
pub use work::*;

#[repr(C)]
#[vtable_impl(Module)]
struct ModuleVTable {
    destructor: extern "C" fn(this: &mut Module),
    deleter: extern "C" fn(this: &mut Module),
    is_virtual: extern "C" fn(this: &Module) -> bool,
    handle_int_msc_cmd: extern "C" fn(this: &mut Module, command: &lib::MscCommand) -> lib::TValue,
    handle_float_msc_cmd:
        extern "C" fn(this: &mut Module, command: &lib::MscCommand) -> lib::TValue,
}

/// The fundamental base type for the majority of Smash Ultimate's modules.
///
/// Smash Ultimate takes its module system from Smash 4 and expands upon it.
///
/// Due to the nature of fighter/weapon code being written primarily in Lua, transpiled to C++,
/// and then compiled into relocatable character modules (the NRO files), many calls in Ultimate
/// are exported as symbols. However, the MSC environment still exists in Ultimate even though it
/// is wrapped up nicely in Lua and overall very hidden away.
///
/// Every module (for most objects there are ~50 modules) inherits from a base class (named here
/// simply as `Module`)lib:: and must support the following operations:
/// 1. Check if it is implemented or an empty module. If it is an empty module, `Module::is_virtual` will
/// return true.
/// 2. Handle MSC commands that will return an integer
/// 3. Handle MSC commands that will return a float
///
/// Not every module actually has an implementation which will handle the MSC commands, and some don't
/// really need to. That being said, when they do it can provide insights into their *non*-exported methods.
///
/// For example, all that is exported by the main executable for [`CancelModule`] is [`CancelModule::is_enable_cancel`]
/// and [`CancelModule::enable_cancel`]. You can access two more methods through the use of the following MSC commands
/// via `app::sv_module_access::cancel`:
/// * `MA_MSC_CMD_CANCEL_UNABLE_CANCEL`        -> [`CancelModule::unable_cancel`]
/// * `MA_MSC_CMD_CANCEL_UNABLE_CANCEL_STATUS` -> [`CancelModule::unable_cancel_status`]
///
/// In addition to the three operations specified above, every module also has the following methods:
/// * `initialize` - Called when the object is created/constructed
/// * `finalize` - Called when the object is destroyed
/// * `start_module` - Called when the object enters into play. For example, `Module::initialize` will be called
/// on Mario's fireballs at the start of the game, but `Module::start_module` will be called when they are summoned
/// during his neutral special, and `Module::end_module` will be called when they disappear
/// * `end_module` - Called when the object exits play.
///
/// These are usually the first methods in the Module's vtable immediately following the required implementations.
#[repr(C)]
pub struct Module {
    vtable: &'static ModuleVTable,
}
