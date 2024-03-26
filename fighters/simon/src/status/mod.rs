use super::*;

mod attacklw3;
mod attacks3;

pub fn install(agent: &mut Agent) {
    attacklw3::install(agent);
    attacks3::install(agent);
}
