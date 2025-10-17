use super::*;

pub static mut RICHTER_HOLYWATER_BORN: usize = 0x758e00;
pub static mut RICHTER_HOLYWATER_BORN_LOOP: usize = 0x759600;

extern "C" {
    #[link_name = "richter_holywater_born_inner"]
    pub fn richter_holywater_born_inner(item: &mut L2CAgent) -> L2CValue;

    #[link_name = "richter_holywater_born_loop_inner"]
    pub fn richter_holywater_born_loop_inner(item: &mut L2CAgent) -> L2CValue;
}

#[skyline::hook(replace = RICHTER_HOLYWATER_BORN)]
unsafe extern "C" fn richter_holywater_born(item: &mut L2CAgent) -> L2CValue {
    richter_holywater_born_inner(item)
}

#[skyline::hook(replace = RICHTER_HOLYWATER_BORN_LOOP)]
unsafe extern "C" fn richter_holywater_born_loop(item: &mut L2CAgent) -> L2CValue {
    original!()(item);
    richter_holywater_born_loop_inner(item)
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "item" {
        unsafe {
            let base = (*info.module.ModuleObject).module_base as usize;
            RICHTER_HOLYWATER_BORN += base;
            RICHTER_HOLYWATER_BORN_LOOP += base;
            skyline::install_hooks!(
                richter_holywater_born,
                richter_holywater_born_loop
            );
        }
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}