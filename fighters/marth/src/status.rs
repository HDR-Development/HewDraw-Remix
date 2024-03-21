use super::*;
use globals::*;

mod special_s;

pub fn install(agent: &mut Agent) {
    special_s::install(agent);
}
