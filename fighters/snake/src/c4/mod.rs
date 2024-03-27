use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("snake_c4");
    acmd::install(agent);
    agent.install();
}