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
    other::install();
    smashes::install();
    specials::install();
    throws::install();
    ground::install();
}
