use super::*;
mod escape_air;
mod attack_air;
mod rebirth;

pub fn install(agent: &mut Agent) {
    escape_air::install();
    attack_air::install();
    rebirth::install();
}
