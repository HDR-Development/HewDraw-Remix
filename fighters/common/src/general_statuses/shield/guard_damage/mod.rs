use super::*;
mod end;
mod main;
mod pre;

mod exec;
mod exit;
mod init;

pub fn install() {
    pre::install();
    main::install();
    end::install();

    init::install();
    exec::install();
    exit::install();
}
