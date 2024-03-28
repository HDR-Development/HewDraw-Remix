use super::*;
use globals::*;
// status script import

mod appeal;
mod special_s;
mod special_lw;

pub fn install(agent: &mut Agent) {
    appeal::install(agent);
    special_s::install(agent);
    special_lw::install(agent);
}