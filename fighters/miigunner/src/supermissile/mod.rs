use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("miigunner_supermissile");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}