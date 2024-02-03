use super::*;
mod pre;
mod main;
mod end;

mod init;
pub mod exec;
mod exit;

pub fn install() {
    pre::install();
    main::install();
    end::install();

    init::install();
    exec::install();
    exit::install();
}
