use super::*;
use globals::*;
// status script import

mod special_n;

pub fn install(agent: &mut Agent) {
    special_n::install(agent);
}