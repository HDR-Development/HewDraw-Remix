use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("miiswordsman_tornadoshot");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}