use super::*;
mod aerials;
mod tilts;
mod other;
mod smashes;
mod specials;
mod ground;

pub fn install() {
    aerials::install();
    tilts::install();
    other::install();
    smashes::install();
    specials::install();
    ground::install();
}