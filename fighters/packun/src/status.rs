use super::*;

mod special_hi;
mod spikeball_start;
mod poisonbreath_start;

pub fn install() {
    special_hi::install();
    spikeball_start::install();
    poisonbreath_start::install();
}