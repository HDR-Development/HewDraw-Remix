use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("samus_missile");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}