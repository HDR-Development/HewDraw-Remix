use super::*;

mod build;

pub fn install(agent: &mut Agent) {
    build::install(agent);
}