use super::*;

mod special_n;

pub fn install(agent: &mut Agent) {
    special_n::install(agent);
}