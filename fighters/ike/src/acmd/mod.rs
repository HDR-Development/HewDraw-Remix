use super::*;
mod aerials;
mod ground;
mod other;
mod smashes;
mod specials;
mod throws;
mod tilts;


pub fn install() {
    aerials::install();
    tilts::install();
    other::install();
    smashes::install();
    specials::install();
    throws::install();
    ground::install();
}
