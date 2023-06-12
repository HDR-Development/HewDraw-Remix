use super::*;

mod special_n_cancel;
mod special_s;
mod special_hi;
 
pub fn install() {
    special_n_cancel::install();
    special_s::install();
    special_hi::install();
}

pub fn add_statuses() {
    special_n_cancel::install();
}