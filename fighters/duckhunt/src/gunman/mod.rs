use super::*;

mod acmd;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("duckhunt_gunman");
    acmd::install(agent);
    opff::install(agent);
    agent.install();
}