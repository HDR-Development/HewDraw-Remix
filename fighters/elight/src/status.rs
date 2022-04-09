use super::*;

mod special_hi_attack;
mod special_hi_jump;
mod special_hi_finish;
mod special_hi;

pub fn install() {
    special_hi_attack::install();
    special_hi_jump::install();
    special_hi_finish::install();
    special_hi::install();
}