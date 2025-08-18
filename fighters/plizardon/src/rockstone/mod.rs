use super::*;

mod status;
mod acmd;

pub fn install() {
    let agent = &mut Agent::new("plizardon_rockstone");
    status::install(agent);
    acmd::install(agent);
    agent.install();
}