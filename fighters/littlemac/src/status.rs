use super::*;

mod special_n_cancel;
mod special_s;
 
pub fn install() {
    special_n_cancel::install();
    special_s::install();
}

pub fn add_statuses() {
    special_n_cancel::install();
}