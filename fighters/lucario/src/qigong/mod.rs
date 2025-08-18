use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("lucario_qigong");
    acmd::install(agent);
    agent.install();
}