use super::*;
use smashline::*;

mod special_hi;

pub fn install(agent: &mut Agent) {
    special_hi::install(agent);
}