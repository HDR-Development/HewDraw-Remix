use super::*;

mod attack_air_b;
mod attack_air_f;
mod attack_air_hi;
mod attack_air_lw;
mod attack_air_n;

pub fn install(agent: &mut Agent) {
    attack_air_b::install(agent);
    attack_air_f::install(agent);
    attack_air_hi::install(agent);
    attack_air_lw::install(agent);
    attack_air_n::install(agent);
}