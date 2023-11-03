use super::*;

mod special_hi;
mod special_s;

pub fn install() {
    special_s::install();
    special_hi::install();
}