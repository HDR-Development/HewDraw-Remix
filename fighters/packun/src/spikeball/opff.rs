// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub extern "C" fn spikeball_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let boma = weapon.boma();
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        if weapon.motion_frame() == 2.0 && VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE) == 1 {
            VarModule::on_flag(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SHOULD_EXPLODE);
            // println!("bomb");
        }
        else if weapon.motion_frame() == 2.0 && VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE) != 1 {
            VarModule::off_flag(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SHOULD_EXPLODE);
            // println!("not bomb");
        }
        let status_kind = StatusModule::status_kind(weapon.module_accessor);
        let motion_kind = MotionModule::motion_kind(weapon.module_accessor);
        if [*FIGHTER_KIND_PACKUN, *FIGHTER_KIND_KIRBY].contains(&owner_module_accessor.kind()) {
            if weapon.is_status(*WEAPON_PACKUN_SPIKEBALL_STATUS_KIND_WAIT) || weapon.is_status(*WEAPON_PACKUN_SPIKEBALL_STATUS_KIND_HOP) {
                /* if VarModule::is_flag(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SHOULD_EXPLODE) && weapon.status_frame() == 2 {
                    println!("will bomb");
                } */
                if VarModule::is_flag(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SHOULD_EXPLODE) && weapon.status_frame() >= 80 && motion_kind != hash40("explode") {
                    WorkModule::off_flag(boma, *WEAPON_PACKUN_SPIKEBALL_STATUS_HOP_WORK_FLAG_CLEARED_ATTACK);
                    MotionModule::change_motion(weapon.module_accessor, Hash40::new("explode"), 0.0, 1.0, false, 0.0, false, false);
                    // println!("is bomb");
                }
            }
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, spikeball_frame);
}