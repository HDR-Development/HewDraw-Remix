use super::*;
mod aerials;
mod ground;
mod other;
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install(agent: &mut Agent) {
    aerials::install(agent: &mut Agent);
    tilts::install(agent: &mut Agent);
    other::install(agent: &mut Agent);
    smashes::install(agent: &mut Agent);
    specials::install(agent: &mut Agent);
    throws::install(agent: &mut Agent);
    ground::install(agent: &mut Agent);
}