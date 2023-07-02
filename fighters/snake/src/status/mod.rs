use super::*;

mod appeal;
mod attack_s4;
mod catch;
mod special_n;
mod special_s;


pub fn install() {
    appeal::install();
    attack_s4::install();
    catch::install();
    special_n::install();
    special_s::install();
}