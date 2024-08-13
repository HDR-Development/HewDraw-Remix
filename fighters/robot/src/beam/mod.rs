use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("robot_beam");
    acmd::install(agent);
    agent.install();
}