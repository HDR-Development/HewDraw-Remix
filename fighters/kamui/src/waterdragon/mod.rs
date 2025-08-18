use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("kamui_waterdragon");
    acmd::install(agent);
    agent.install();
}