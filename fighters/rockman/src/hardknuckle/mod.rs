use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("rockman_hardknuckle");
    acmd::install(agent);
    agent.install();
}