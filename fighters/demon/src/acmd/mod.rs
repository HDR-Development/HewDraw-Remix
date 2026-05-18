use super::*;

mod jabs;
mod ground;
mod tilts;
mod smashes;
mod aerials;
mod specials;
mod stand;
mod step;
mod squat;
mod throws;
mod other;

pub fn install(agent: &mut Agent) {
    jabs::install(agent);
    ground::install(agent);
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);
    stand::install(agent);
    step::install(agent);
    squat::install(agent);
    throws::install(agent);
    other::install(agent);
}