use super::*;
mod aerials;
mod tilts;
mod other;
mod smashes;
mod specials;
mod throws;
mod ground;
mod finals;

pub fn install(agent: &mut Agent) {
    aerials::install(agent);
    tilts::install(agent);
    other::install(agent);
    smashes::install(agent);
    specials::install(agent);
    throws::install(agent);
    ground::install(agent);
    finals::install(agent);
}
