use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("dedede_gordo");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}