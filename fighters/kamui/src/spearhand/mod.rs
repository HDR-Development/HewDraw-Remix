use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("kamui_spearhand");
    acmd::install(agent);
    agent.install();
}