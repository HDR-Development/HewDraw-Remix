use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("palutena_explosiveflame");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}