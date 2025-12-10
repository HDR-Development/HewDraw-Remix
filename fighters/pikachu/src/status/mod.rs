use super::*;
use globals::*;
// status script import
mod special_hi;
mod special_s;

pub fn install(agent: &mut Agent) {
    special_hi::install(agent);
    special_s::install(agent);
}