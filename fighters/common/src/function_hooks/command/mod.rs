use super::*;

mod c_623_nb;
mod c_623_strict;
mod c_623_ab_long;
mod c_623_a;
mod c_323_catch;

pub fn install() {
    c_623_nb::install();
    c_623_strict::install();
    c_623_ab_long::install();
    c_623_a::install();
    c_323_catch::install();
}
