use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("jack_doyle");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}