#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod consts;


mod ai_extras;
pub mod ai;
mod battle_object;
mod bosses;
pub mod camera;
pub mod debug;
mod events;
pub mod fighter;
pub mod global_parameter;
mod items;
mod module_accessors;
mod modules;
mod singletons;
pub mod smashball;
pub mod smashballheavy;
mod specializers;
pub mod stage;
pub mod sv_global_parameter;
pub mod sv_information;

pub use ai_extras::*;
pub use battle_object::*;
pub use bosses::*;
pub use consts::*;
pub use events::*;
pub use items::*;
pub use module_accessors::*;
pub use modules::*;
pub use singletons::*;
pub use specializers::*;

#[repr(C)]
pub struct FighterAIWeapon(u64);

pub type FighterEntryID = u32;

#[repr(C)]
pub struct FighterEntry {}

#[repr(C)]
pub struct FighterInformation {}