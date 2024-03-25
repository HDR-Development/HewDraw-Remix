use super::*;
use smashline::*;

mod special_s;

pub fn install(agent: &mut Agent) {
    special_s::install(agent);
}