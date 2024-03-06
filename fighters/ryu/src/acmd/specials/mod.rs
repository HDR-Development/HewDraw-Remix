use super::*;
mod special_command;
mod special_hi;
mod special_lw;
mod special_n;
mod special_s;

pub fn install() {
    special_command::install();
    special_hi::install();
    special_lw::install();
    special_n::install();
    special_s::install();
}