use super::*;

mod attacklw3;
mod attacks3;
mod special_n;
mod special_s;

pub fn install(agent: &mut Agent) {
    attacklw3::install(agent);
    attacks3::install(agent);
    special_n::install(agent);
    special_s::install(agent);
}