/// This crate is intended to be used during development to help speed up compile times
/// The `common` crate itself gets built and linked into the `fighters` crate, however 
/// all of the individual characters will depend on this crate, as changing the `common` crate can cause
/// lengthy compile times if we have to rebuild the entire plugin numerous times to test some changes

pub mod ext;
pub use ext::*;

pub mod djc;