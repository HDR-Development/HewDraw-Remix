use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("ike_sword");
    acmd::install(agent);
    agent.install();
}
