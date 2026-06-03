use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("reflet_gigafire");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}