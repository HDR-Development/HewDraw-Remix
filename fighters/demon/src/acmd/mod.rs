use super::*;
mod aerials;
mod jabs;
mod ground;
mod other;
mod smashes;
mod specials;
mod throws;
mod tilts;

pub fn install() {
    aerials::install();
    jabs::install();
    ground::install();
    other::install();
    smashes::install();
    specials::install();
    throws::install();
    tilts::install();
}