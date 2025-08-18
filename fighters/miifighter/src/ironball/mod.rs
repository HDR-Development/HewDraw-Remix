use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("miifighter_ironball");
    acmd::install(agent);
    agent.install();
}