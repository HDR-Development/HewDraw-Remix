use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("murabito_clayrocket");
    acmd::install(agent);
    agent.install();
}