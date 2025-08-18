use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("kamui_dragonhand");
    acmd::install(agent);
    agent.install();
}