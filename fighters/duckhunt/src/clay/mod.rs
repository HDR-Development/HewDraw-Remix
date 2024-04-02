use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("duckhunt_clay");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}