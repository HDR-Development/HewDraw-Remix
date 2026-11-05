use super::*;

// Fighter/Weapon vtable replacements and instruction patches live in each agent's own `vtable.rs`
// (installed through smashline's `replace_fighter_vtable` / `replace_weapon_vtable`). Only inline
// hooks and functions that are not vtable entries remain here.

mod bayonetta;
mod brave;
mod krool;
mod lucario;
mod pickel;
mod ptrainer;
mod rockman;
mod shotos;
mod shulk;

mod dolly_burst;

mod weapon;

pub fn install() {
    bayonetta::install();
    brave::install();
    krool::install();
    lucario::install();
    pickel::install();
    ptrainer::install();
    rockman::install();
    shotos::install();
    shulk::install();

    dolly_burst::install();

    weapon::install();
}
