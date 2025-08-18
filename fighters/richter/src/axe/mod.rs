use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("richter_axe");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}
