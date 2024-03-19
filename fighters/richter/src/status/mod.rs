use super::*;

mod attacklw3;
mod attacks3;
mod special_s;

pub fn install() {
    attacklw3::install();
    attacks3::install();
    special_s::install();
}