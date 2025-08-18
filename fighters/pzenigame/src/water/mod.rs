use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("pzenigame_water");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}