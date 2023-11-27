use super::*;
pub mod acmd;
pub mod status;

pub fn install(is_runtime: bool) {
    acmd::install();
    status::install();
}