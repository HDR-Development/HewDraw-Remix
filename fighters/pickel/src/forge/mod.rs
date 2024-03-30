use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("pickel_forge");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}