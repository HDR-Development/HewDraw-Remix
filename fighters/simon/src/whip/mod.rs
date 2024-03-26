use super::*;

mod aerials;
mod smashes;
mod tilts;

pub fn install() {
    let agent = &mut Agent::new("simon_whip");
    aerials::install(agent);
    smashes::install(agent);
    tilts::install(agent);
    agent.install();
}