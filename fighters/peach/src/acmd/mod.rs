use super::*;

mod aerials;
mod ground;
mod other;
mod smashes;
mod specials;
mod tilts;
mod throws;

pub fn install(agent: &mut Agent) {
    aerials::install(agent);
    ground::install(agent);
    other::install(agent);
    smashes::install(agent);
    specials::install(agent);
    tilts::install(agent);
    throws::install(agent);
}