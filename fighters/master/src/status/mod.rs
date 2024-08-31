use super::*;
use globals::*;
// status script import

mod rebirth;
mod special_s;

pub fn install(agent: &mut Agent) {
    rebirth::install(agent);
    special_s::install(agent);
}