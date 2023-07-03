use super::*;

mod attack;
mod batwithin;
mod escape;
mod special_n;


pub fn install() {
    attack::install();
    batwithin::install();
    escape::install();
    special_n::install();
}