use super::*;
use globals::*;
// status script import

mod special_lw;

pub fn install(agent: &mut Agent) {
    special_lw::install(agent);
}