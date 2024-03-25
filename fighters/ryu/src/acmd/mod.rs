use super::*;
mod aerials;
mod finals;
mod tilts;
mod other;
mod smashes;
mod specials;
mod throws;
mod ground;

pub fn install(agent: &mut Agent) {
    aerials::install(agent);
    finals::install(agent);
    tilts::install(agent);
    other::install(agent);
    smashes::install(agent);
    specials::install(agent);
    throws::install(agent);
    ground::install(agent);
}
