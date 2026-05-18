use super::*;

mod special_lw1;
mod special_lw2;
mod special_lw3;

pub fn install(agent: &mut Agent) {;
    special_lw1::install(agent);
    special_lw2::install(agent);
    special_lw3::install(agent);
}