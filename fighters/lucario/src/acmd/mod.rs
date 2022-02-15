use super::*;
mod aerials;
mod ground;
mod other;
mod smashes;
mod specials;
mod tilts;

pub fn install() {
    aerials::install();
    tilts::install();
    other::install();
    smashes::install();
    specials::install();
    ground::install();
}