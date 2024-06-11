use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("inkling_inkbullet");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}