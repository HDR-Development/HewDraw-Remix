use super::*;

mod aerials;
mod tilts;
mod other;
mod ground;

pub fn install() {
    aerials::install();
    tilts::install();
    other::install();
    ground::install();
}