use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("snake_nikitamissile");
    acmd::install(agent);
    agent.install();
}