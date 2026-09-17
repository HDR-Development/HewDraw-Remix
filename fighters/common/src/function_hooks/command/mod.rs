use super::*;

#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn check_backwards_dp_input(data: CommandInputFlags, class: &mut CommandInputState) -> bool {
    if data.back(class.lr as f32)
    || data.back_up(class.lr as f32) {
        return true;
    }

    false
}

mod c_236;
mod c_41236;
mod c_214;
mod c_623;
mod c_632;
mod c_21416;
mod c_21416_r;
mod c_236236;
mod c_236236_r;
mod c_623_nb;
mod c_623_strict;
mod c_623_ab_long;
mod c_323_catch;

pub fn install() {
    c_236::install();
    c_41236::install();
    c_214::install();
    c_623::install();
    c_632::install();
    c_21416::install();
    c_21416_r::install();
    c_236236::install();
    c_236236_r::install();
    c_623_nb::install();
    c_623_strict::install();
    c_623_ab_long::install();
    c_323_catch::install();
}
