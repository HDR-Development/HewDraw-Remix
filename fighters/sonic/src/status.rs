use super::*;

mod dash;
mod special_s;
mod special_lw;

pub fn install() {
    dash::install();
    special_s::install();
    special_lw::install();
}