use super::*;

#[skyline::hook( offset = 0xf10fe0 )]
pub unsafe fn check_material_attack_air_lw_generate(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    let iron_count = WorkModule::get_int(module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_IRON);
    let mut iron_needed = WorkModule::get_param_int(module_accessor, hash40("param_private"), 0x188e0b0db2);
    let status = StatusModule::status_kind(module_accessor);
    if status == *FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_START {
        iron_needed += 2;
    }
    iron_count >= iron_needed
}

pub fn install() {
    skyline::install_hooks!(
        check_material_attack_air_lw_generate
    );
    
}