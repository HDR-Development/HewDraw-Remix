use super::*;
use globals::*;
// status script import

mod special_hi;
mod special_lw;
mod special_n;
mod special_s;

pub fn install(agent: &mut Agent) {
    special_hi::install(agent);
    special_lw::install(agent);
    special_n::install(agent);
    special_s::install(agent);
}