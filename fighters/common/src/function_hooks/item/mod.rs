use super::*;

mod doll;
mod richterholywater;

pub fn install() {
    doll::install();
    richterholywater::install();
}