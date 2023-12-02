use super::*;
use smash::lib::lua_const::*;

mod aerials;
mod other;
mod smashes;
mod specials;
mod throws;

pub fn install() {
    aerials::install();
    other::install();
    smashes::install();
    specials::install();
    throws::install();
}