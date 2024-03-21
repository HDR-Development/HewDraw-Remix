use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("yoshi_tamago");
    acmd::install(agent);
    agent.install();
}