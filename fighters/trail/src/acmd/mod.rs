use super::*;
mod aerials;
mod tilts;
mod ground;
mod other;
mod specials;
mod smashes;
mod throws;

pub fn install(agent: &mut Agent) {
    aerials::install(agent);
    tilts::install(agent);
    other::install(agent);
    smashes::install(agent);
    specials::install(agent);
    throws::install(agent);
    ground::install(agent);
}
