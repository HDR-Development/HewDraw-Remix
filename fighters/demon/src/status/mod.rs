use super::*;
use globals::*;
// status script import

mod wait;
mod dash;
mod turn_dash;
mod landing;

mod attack;
mod attack_combo;
mod attack_air;

pub fn install(agent: &mut Agent) {
    wait::install(agent);
    dash::install(agent);
    turn_dash::install(agent);
    landing::install(agent);

    attack::install(agent);
    attack_combo::install(agent);
    attack_air::install(agent);
}