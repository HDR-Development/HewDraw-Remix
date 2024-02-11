use super::*;

mod special_n;
mod special_s;
mod attacks3;

pub fn install() {
    special_n::install();
    special_s::install();
    attacks3::install();
}
