use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("reflet_thunder");
    acmd::install(agent);
    agent.install();
}
