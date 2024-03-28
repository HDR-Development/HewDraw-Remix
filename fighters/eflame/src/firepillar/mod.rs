use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("eflame_firepillar");
    acmd::install(agent);
    agent.install();
}