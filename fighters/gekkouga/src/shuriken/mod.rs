use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("gekkouga_shuriken");
    acmd::install(agent);
    agent.install();
}