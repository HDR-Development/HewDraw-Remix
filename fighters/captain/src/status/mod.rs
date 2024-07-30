use super::*;
use globals::*;
// status script import

mod special_lw;
mod special_s;

pub fn install(agent: &mut Agent) {
    special_lw::install(agent);
    special_s::install(agent);
}