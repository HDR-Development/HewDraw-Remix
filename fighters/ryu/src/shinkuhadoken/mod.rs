use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("ryu_shinkuhadoken");
    acmd::install(agent);
    agent.install();
}