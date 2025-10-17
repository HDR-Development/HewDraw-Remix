use super::*;
use globals::*;
// status script import

mod rebirth;
mod special_n;
mod special_s;
mod special_lw;

pub fn install(agent: &mut Agent) {
    rebirth::install(agent);
    special_n::install(agent);
    special_s::install(agent);
    special_lw::install(agent);
}