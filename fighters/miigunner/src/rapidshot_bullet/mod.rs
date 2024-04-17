use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("miigunner_rapidshot_bullet");
    acmd::install(agent);
    agent.install();
}