use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("inkling_splashbomb");
    acmd::install(agent);
    agent.install();
}