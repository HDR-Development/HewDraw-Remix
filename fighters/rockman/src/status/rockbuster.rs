use super::*;

mod shoot_wait;
mod shoot_walk;
mod shoot_turn;
mod shoot_jump_squat;
mod shoot_jump;
mod shoot_air;
mod shoot_landing;
pub mod helper;

pub fn install(agent: &mut Agent) {
    shoot_wait::install(agent);
    shoot_walk::install(agent);
    shoot_turn::install(agent);
    shoot_jump_squat::install(agent);
    shoot_jump::install(agent);
    shoot_air::install(agent);
    shoot_landing::install(agent);
}