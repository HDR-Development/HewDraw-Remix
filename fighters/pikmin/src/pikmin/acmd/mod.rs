use super::*;
use smash::lib::lua_const::*;
use smash_script::macros::ATTACK_ABS;

mod aerials;
mod other;
mod smashes;
mod specials;
mod throws;

pub fn install(agent: &mut Agent) {
    aerials::install(agent);
    other::install(agent);
    smashes::install(agent);
    specials::install(agent);
    throws::install(agent);
}