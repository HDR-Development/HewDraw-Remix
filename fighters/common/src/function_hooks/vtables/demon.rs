use super::*;
use utils::ext::*;

extern "C" {
    #[link_name = "demon_on_attack_inner"]
    fn demon_on_attack_inner(vtable: u64, fighter: &mut Fighter, log: u64);
}

#[skyline::hook(offset = 0x932f50)]
pub unsafe extern "C" fn demon_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) {
    demon_on_attack_inner(vtable, fighter, log);

    original!()(vtable, fighter, log)
}

#[skyline::hook(offset = 0x934310)]
pub unsafe extern "C" fn demon_some_event(_vtable: u64, _fighter: &mut Fighter, event: u64) -> u64 {
    event
}

pub fn install() {
    skyline::install_hooks!(
        demon_on_attack,
        demon_some_event
    );
}
