use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("pickel_trolley");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}