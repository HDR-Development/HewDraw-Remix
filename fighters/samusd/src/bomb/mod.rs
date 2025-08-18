use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("samusd_bomb");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}