use super::*;

mod special_hi;
mod special_lw;
mod special_n;

pub fn install(agent: &mut Agent) {
    special_hi::install(agent);
    special_lw::install(agent);
    special_n::install(agent);
}