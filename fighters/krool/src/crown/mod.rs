use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("krool_crown");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}