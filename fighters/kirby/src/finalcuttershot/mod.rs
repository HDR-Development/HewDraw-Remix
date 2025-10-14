use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("kirby_finalcuttershot");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}