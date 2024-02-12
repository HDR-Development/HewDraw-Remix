use super::*;

mod special_n;
mod special_hi;
mod edge_fire_fly;

pub fn install() {
    special_n::install();
    special_hi::install();
    edge_fire_fly::install();
}