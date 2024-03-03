use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("rockman_airshooter");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}