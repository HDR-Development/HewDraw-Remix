use super::*;

mod status;

pub fn install() {
    let agent = &mut Agent::new("brave_flash");
    status::install(agent);
    agent.install();
}