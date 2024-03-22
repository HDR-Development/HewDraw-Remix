use super::*;

mod aerials;
mod tilts;
mod smashes;
mod other;
mod ground;
mod specials;
mod throws;

pub fn install(agent: &mut Agent) {
    aerials::install(agent);
    tilts::install(agent);
    smashes::install(agent);
    other::install(agent);
    ground::install(agent);
    specials::install(agent);
    throws::install(agent);
}