use super::*;

static mut DOLL_INITIALIZE_STATUS_OFFSET : usize = 0x649490;

#[skyline::hook(replace = DOLL_INITIALIZE_STATUS_OFFSET)]
unsafe extern "C" fn doll_initialize_status(item: &mut L2CAgent) -> L2CValue {
    let ret = original!()(item);
    TeamModule::set_hit_team(item.module_accessor, -1);
    TeamModule::set_team(item.module_accessor, -1, true);
    TeamModule::set_team_owner_id(item.module_accessor, 0x50000000);
    HitModule::set_no_team(item.module_accessor, true);
    WorkModule::on_flag(item.module_accessor, *ITEM_DOLL_INSTANCE_WORK_FLAG_FIRST_DAMAGE);
    ret
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "item" {
        unsafe {
            // Allows the doll to be hittable by Greninja upon spawning in, instead of at a delay.
            let item_offset = (*info.module.ModuleObject).module_base as usize;
            DOLL_INITIALIZE_STATUS_OFFSET += item_offset;
            skyline::install_hooks!(
                doll_initialize_status
            );
        }
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}