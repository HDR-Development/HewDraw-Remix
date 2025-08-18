use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("tantan_punch1");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}