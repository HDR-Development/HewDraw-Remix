use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("rockman_chargeshot");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}