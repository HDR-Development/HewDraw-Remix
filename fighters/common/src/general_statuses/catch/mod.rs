use super::*;
mod catchcut;
mod catchdash;
mod catchattack;

pub fn install() {
    catchcut::install();
    catchdash::install();
    catchattack::install();
}