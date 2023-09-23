use super::*;

mod special_n;
mod special_s;


pub fn install() {
    special_n::install();
    special_s::install();
}