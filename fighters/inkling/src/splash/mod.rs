use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("inkling_splash");
    acmd::install(agent);
    agent.install();
}