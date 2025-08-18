use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("wiifit_hulahoop");
    acmd::install(agent);
    agent.install();
}