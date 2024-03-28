use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("murabito_bowlingball");
    acmd::install(agent);
    agent.install();
}