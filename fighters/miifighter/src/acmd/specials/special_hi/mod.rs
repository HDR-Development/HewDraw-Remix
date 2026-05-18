use super::*;

mod special_hi1;
mod special_hi2;
mod special_hi3;

pub fn install(agent: &mut Agent) {;
    special_hi1::install(agent);
    special_hi2::install(agent);
    special_hi3::install(agent);
}