use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("miigunner_bottomshoot");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}