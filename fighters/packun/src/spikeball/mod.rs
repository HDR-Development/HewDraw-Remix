use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("packun_spikeball");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}