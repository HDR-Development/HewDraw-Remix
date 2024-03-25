use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("jack_doyle");
    acmd::install(agent);
    agent.install();
}