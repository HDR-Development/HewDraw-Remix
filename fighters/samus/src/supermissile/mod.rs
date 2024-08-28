use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("samus_supermissile");
    acmd::install(agent);
    agent.install();
}