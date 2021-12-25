#[macro_use]
extern crate lazy_static;

mod offsets;

#[cfg(feature = "import-only")]
mod modules_import;

#[cfg(not(feature = "import-only"))]
mod modules;

pub mod singletons;
pub mod utils;

use std::path::{PathBuf, Path};

#[cfg(feature = "import-only")]
pub use modules_import::*;

#[cfg(not(feature = "import-only"))]
pub use modules::*;

#[cfg(not(feature = "import-only"))]
pub fn init() {
    modules::init();

    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>"
            }
        };

        let err_msg = format!("thread has panicked at '{}', {}", msg, location);
        show_error(
            69,
            "Skyline plugin as panicked! Please open the details and send a screenshot to the developer, then close the game.\n",
            err_msg.as_str()
        );
    }));
}

fn show_error(code: u32, message: &str, details: &str) {
    use skyline::nn::{err, settings};

    let mut message_bytes = String::from(message).into_bytes();
    let mut details_bytes = String::from(details).into_bytes();

    if message_bytes.len() >= 2048 {
        message_bytes.truncate(2044);
        message_bytes.append(&mut String::from("...\0").into_bytes());
    }

    if details_bytes.len() >= 2048 {
        details_bytes.truncate(2044);
        details_bytes.append(&mut String::from("...\0").into_bytes());
    }

    unsafe {
        let error = err::ApplicationErrorArg::new_with_messages(
            code,
            core::str::from_utf8(&message_bytes).unwrap().as_bytes().as_ptr(),
            core::str::from_utf8(&details_bytes).unwrap().as_bytes().as_ptr(),
            &settings::LanguageCode_Make(settings::Language_Language_English)
        );

        err::ShowApplicationError(&error);
    }
}