use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("mewtwo_bindball");
    acmd::install(agent);
    agent.install();
}