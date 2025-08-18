use super::*;

mod status;

pub fn install() {
    let agent = &mut Agent::new("plizardon_rock");
    status::install(agent);
    agent.install();
}