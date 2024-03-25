use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("miiswordsman_chakram");
    acmd::install(agent);
    agent.install();
}