use super::*;
use globals::*;
// status script import

mod rebirth;
mod special_hi;

pub fn install(agent: &mut Agent) {
    rebirth::install(agent);
    special_hi::install(agent);
}
