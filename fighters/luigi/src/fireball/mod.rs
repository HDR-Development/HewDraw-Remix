use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("luigi_fireball");
    acmd::install(agent);
    agent.install();
}