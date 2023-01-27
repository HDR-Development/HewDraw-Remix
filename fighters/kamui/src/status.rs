use super::*;

mod special_lw;
mod special_s;

pub fn install() {
    special_lw::install();
    special_s::install();
}
