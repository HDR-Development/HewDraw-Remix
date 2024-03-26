use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("miigunner_supermissile");
    acmd::install(agent);
    agent.install();
}