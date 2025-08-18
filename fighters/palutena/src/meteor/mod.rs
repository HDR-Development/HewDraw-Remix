use super::*;

mod acmd;
mod status;

pub fn install() {
    let agent = &mut Agent::new("palutena_meteor");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}