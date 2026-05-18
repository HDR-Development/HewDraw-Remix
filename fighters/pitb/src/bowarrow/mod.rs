use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("pitb_bowarrow");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}