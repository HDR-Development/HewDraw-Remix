#[macro_use]
extern crate lazy_static;


mod offsets;

#[cfg(feature = "import-only")]
mod modules_import;

#[cfg(not(feature = "import-only"))]
mod modules;

pub mod singletons;
pub mod utils;

#[cfg(feature = "import-only")]
pub use modules_import::*;

#[cfg(not(feature = "import-only"))]
pub use modules::*;

#[cfg(not(feature = "import-only"))]
pub fn init() {
    modules::init();
}