use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("palutena_explosiveflame");
    acmd::install(agent);
    agent.install();
}