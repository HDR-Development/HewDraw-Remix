use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("bayonetta_specialn_bullet");
    acmd::install(agent);
    agent.install();
}