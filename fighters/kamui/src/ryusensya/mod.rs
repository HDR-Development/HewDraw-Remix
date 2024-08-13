use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("kamui_ryusensya");
    acmd::install(agent);
    agent.install();
}