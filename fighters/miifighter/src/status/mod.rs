use super::*;
use globals::*;
// status script import

mod special_lw1;

pub fn install(agent: &mut Agent) {
    special_lw1::install(agent);
}