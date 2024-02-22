use super::*;
mod aerials;
mod copyspecials;
mod tilts;
mod other;
mod smashes;
mod specials;
mod throws;
mod ground;

pub fn install() {
    aerials::install();
    tilts::install();
    other::install();
    smashes::install();
    specials::install();
    throws::install();
    ground::install();
}
