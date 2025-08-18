use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("ken_shinryuken");
    acmd::install(agent);
    agent.install();
}