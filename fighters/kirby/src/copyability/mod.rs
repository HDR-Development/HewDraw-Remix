use super::*;

mod acmd;
mod opff;

pub fn install() {
    acmd::install();
    opff::install();
}