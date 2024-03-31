use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("ness_yoyohead");
    acmd::install(agent);
    agent.install();
}