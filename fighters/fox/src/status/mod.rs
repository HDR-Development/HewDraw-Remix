use super::*;

mod special_s;
mod special_hi;
mod special_lw;

pub fn install(agent: &mut Agent) {
    special_s::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
}