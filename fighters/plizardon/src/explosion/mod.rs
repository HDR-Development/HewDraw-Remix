use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("plizardon_explosion");
    acmd::install(agent);
    agent.install();
}