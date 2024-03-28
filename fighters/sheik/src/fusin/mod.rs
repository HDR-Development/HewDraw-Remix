use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("sheik_fusin");
    acmd::install(agent);
    agent.install();
}