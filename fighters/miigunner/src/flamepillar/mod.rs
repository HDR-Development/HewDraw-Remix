use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("miigunner_flamepillar");
    acmd::install(agent);
    agent.install();
}