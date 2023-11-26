use super::*;

mod special_s_jump;

mod special_hi_escape;
mod special_hi_damage;
 
pub fn install() {
    special_s_jump::install();

    special_hi_escape::install();
    special_hi_damage::install();
}
