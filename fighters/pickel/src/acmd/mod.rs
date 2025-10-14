use super::*;

mod ground;
mod tilts;
mod smashes;
mod aerials;
mod specials;
mod throws;
mod other;

// shorthand for referencing steve's different MATERIAL_KIND constants
const WOOD: i32 = 0x1;
const STONE: i32 = 0x2;
const IRON: i32 = 0x3;
const GOLD: i32 = 0x4;
const DIAMOND: i32 = 0x6;

pub fn install(agent: &mut Agent) {
    ground::install(agent);
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);
    throws::install(agent);
    other::install(agent);
}