use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("miigunner_bottomshoot");
    acmd::install(agent);
    agent.install();
}