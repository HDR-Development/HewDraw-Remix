use super::*;

mod special_hi;
mod special_hi_jump;

pub fn install() {
    special_hi::install();
    special_hi_jump::install();
}