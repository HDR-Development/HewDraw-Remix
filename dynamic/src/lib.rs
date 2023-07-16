#![allow(non_upper_case_globals)]
pub mod offsets;
pub mod util;
pub mod singletons;
pub mod consts;
pub mod ext;
mod modules;
pub mod frame_info;
pub mod game_modes;
pub mod ui;

#[macro_use]
extern crate modular_bitfield;

pub use hdr_macros::{
    export,
    import,
    import_noreturn,
    hash40
};

pub use hdr_macros as macros;

pub use modules::*;
pub use frame_info::*;