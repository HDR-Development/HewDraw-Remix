use super::*;

mod aerials;
mod tilts;
mod smashes;
mod other;
mod ground;
mod specials;
mod throws;

pub fn install() {
    aerials::install();
    tilts::install();
    smashes::install();
    other::install();
    ground::install();
    specials::install();
    throws::install();
}