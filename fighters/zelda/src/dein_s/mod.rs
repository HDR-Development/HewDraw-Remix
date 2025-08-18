use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("zelda_dein_s");
    acmd::install(agent);
    agent.install();
}