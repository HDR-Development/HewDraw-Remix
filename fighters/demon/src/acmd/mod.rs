use super::*;
mod aerials;
mod jabs;
mod ground;
mod other;
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install(agent: &mut Agent) {
    aerials::install(agent);
    jabs::install(agent);
    ground::install(agent);
    other::install(agent);
    smashes::install(agent);
    specials::install(agent);
    throws::install(agent);
    tilts::install(agent);
}