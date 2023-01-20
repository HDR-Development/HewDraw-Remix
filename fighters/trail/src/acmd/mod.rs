use super::*;
mod aerials;
mod tilts;
mod ground;
mod other;
mod specials;
mod smashes;
mod throws;

pub fn install() {
    aerials::install();
    tilts::install();
    ground::install();
    other::install();
    specials::install();
    smashes::install();
    throws::install();
}