use super::*;
mod catchcut;
mod catchdash;

pub fn install() {
    catchcut::install();
    catchdash::install();
}