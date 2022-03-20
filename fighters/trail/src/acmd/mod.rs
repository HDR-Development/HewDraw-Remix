use super::*;
mod aerials;
mod tilts;
mod ground;
mod other;
mod specials;

pub fn install() {
    aerials::install();
    tilts::install();
    ground::install();
    other::install();
    specials::install();
}