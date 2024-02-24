use super::*;
use globals::*;

mod special_n;
mod special_s;
mod special_lw;


pub fn install() {
    special_n::install();
    special_s::install();
    special_lw::install();
}
