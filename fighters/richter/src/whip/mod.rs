use super::*;

mod aerials;
mod ground;
mod other;
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install() {
    let agent = &mut Agent::new("richter_whip");
    aerials::install(agent);
    ground::install(agent);
    other::install(agent);
    smashes::install(agent);
    specials::install(agent);
    throws::install(agent);
    tilts::install(agent);
    agent.install();
}