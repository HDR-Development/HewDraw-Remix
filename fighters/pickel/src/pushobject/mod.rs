use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("pickel_pushobject");
    acmd::install(agent);
    agent.install();
}