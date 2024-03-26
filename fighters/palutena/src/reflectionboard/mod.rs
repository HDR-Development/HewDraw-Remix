use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("palutena_reflectionboard");
    acmd::install(agent);
    agent.install();
}