use super::*;

mod dash;
mod special_s;

pub fn install() {
    dash::install();
    special_s::install();
}