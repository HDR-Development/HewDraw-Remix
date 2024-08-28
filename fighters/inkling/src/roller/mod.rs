use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("inkling_roller");
    acmd::install(agent);
    agent.install();
}