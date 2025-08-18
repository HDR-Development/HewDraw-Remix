use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("koopajr_cannonball");
    acmd::install(agent);
    agent.install();
}