use super::*;
use crate::globals::*;

mod ready;

pub fn install(agent: &mut Agent) {
    ready::install(agent);
}