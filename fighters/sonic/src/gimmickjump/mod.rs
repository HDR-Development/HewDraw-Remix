use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("sonic_gimmickjump");
    acmd::install(agent);
    agent.install();
}