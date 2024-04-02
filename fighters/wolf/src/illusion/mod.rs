use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("wolf_illusion");
    acmd::install(agent);
    agent.install();
}