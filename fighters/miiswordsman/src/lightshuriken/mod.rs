use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("miiswordsman_lightshurken");
    acmd::install(agent);
    agent.install();
}