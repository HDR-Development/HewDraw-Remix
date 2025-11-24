use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("murabito_sprout");
    acmd::install(agent);
    agent.install();
}