use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("shizue_fishingrod");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}