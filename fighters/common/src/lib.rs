pub mod prelude {
    pub use smash;
    pub use skyline;
    pub use smash_script;
    pub use smashline;
    pub use utils;
}

use smash::app::{*, lua_bind::*};
use smash::lua2cpp::*;
use smash::lib::{*, lua_const::*};
use smash::phx::*;
use utils::{*, consts::*};

pub mod djc;