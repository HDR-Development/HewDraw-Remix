use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("murabito_firework");
    acmd::install(agent);
    agent.install();
}