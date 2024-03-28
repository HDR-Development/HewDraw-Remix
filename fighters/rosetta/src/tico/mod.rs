use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("rosetta_tico");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}