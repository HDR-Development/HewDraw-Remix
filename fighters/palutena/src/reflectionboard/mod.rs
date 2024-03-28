use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("palutena_reflectionboard");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}