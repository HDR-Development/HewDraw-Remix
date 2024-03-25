use super::*;
use globals::*;

mod special_n;
mod special_hi;

pub fn install(agent: &mut Agent) {
    special_n::install(agent);
    special_hi::install(agent);
}