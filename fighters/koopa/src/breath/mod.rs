use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("koopa_breath");
    acmd::install(agent);
    agent.install();
}