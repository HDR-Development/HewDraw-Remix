use super::*;

mod dein_move;

pub fn install(agent: &mut Agent) {
    dein_move::install(agent);
}