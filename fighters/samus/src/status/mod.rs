use super::*;
use globals::*;
// status script import

mod special_s;

pub fn install(agent: &mut Agent) {
    special_s::install(agent);
}