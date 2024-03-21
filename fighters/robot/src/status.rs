use super::*;
use globals::*;

mod special_s;
mod special_hi;

pub fn install(agent: &mut Agent) {
    special_s::install(agent);
    special_hi::install(agent);
}
