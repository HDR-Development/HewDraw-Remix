use super::*;

mod acmd;

pub fn install() {
    let agent = &mut Agent::new("snake_cypher");
    acmd::install(agent);
    agent.install();
}