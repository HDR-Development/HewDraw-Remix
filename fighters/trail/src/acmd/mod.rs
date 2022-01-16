use super::*;
mod aerials;
mod tilts;
mod ground;

pub fn install() {
    aerials::install();
    tilts::install();
    ground::install();
}