use super::*;
mod aerials;
mod tilts;
mod ground;
mod other;

pub fn install() {
    aerials::install();
    tilts::install();
    ground::install();
    other::install();
}