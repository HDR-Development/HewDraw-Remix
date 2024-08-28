use super::*;
use globals::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("purin_disarmingvoice");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}