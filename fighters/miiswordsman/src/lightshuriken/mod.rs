use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("miiswordsman_lightshuriken");
    acmd::install(agent);
    agent.install();
}