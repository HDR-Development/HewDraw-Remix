use super::*;

mod shoot_wait;
mod shoot_walk;
mod shoot_turn;
mod shoot_jump_squat;
mod shoot_jump;
mod shoot_air;
mod shoot_landing;
pub mod helper;

pub fn install() {
    shoot_wait::install();
    shoot_walk::install();
    shoot_turn::install();
    shoot_jump_squat::install();
    shoot_jump::install();
    shoot_air::install();
    shoot_landing::install();
}