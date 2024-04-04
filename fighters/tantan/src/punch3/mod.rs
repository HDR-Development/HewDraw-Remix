use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("tantan_punch3");
    acmd::install(agent);
    agent.install();
}