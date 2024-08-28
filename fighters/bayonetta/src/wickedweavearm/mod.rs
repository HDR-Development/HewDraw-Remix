use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("bayonetta_wickedweavearm");
    acmd::install(agent);
    agent.install();
}