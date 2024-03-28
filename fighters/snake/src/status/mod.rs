use super::*;
use globals::*;
// status script import

mod appeal;
mod attack_s4;
mod catch;
mod special_s;
mod rebirth;

pub fn install(agent: &mut Agent) {
    appeal::install(agent);
    attack_s4::install(agent);
    catch::install(agent);
    special_s::install(agent);
    rebirth::install(agent);
}