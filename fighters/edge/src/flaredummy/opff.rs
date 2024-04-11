// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub extern "C" fn shadowflare_orb_callback(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe { 
        if weapon.is_status(*WEAPON_EDGE_FLAREDUMMY_STATUS_KIND_FLY) {
            let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let sephiroth = utils::util::get_battle_object_from_id(owner_id);
            let sephiroth_boma = &mut *(*sephiroth).module_accessor;
            let sephiroth_status_kind = StatusModule::status_kind(sephiroth_boma);
            if sephiroth_status_kind == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH && AttackModule::is_infliction_status(sephiroth_boma, *COLLISION_KIND_MASK_HIT) {
                // Explode if Sephiroth hits the target marked with this set of orbs with Blade Dash
                let x_distance = PostureModule::pos_x(weapon.module_accessor) - PostureModule::pos_x(sephiroth_boma);
                let y_distance = PostureModule::pos_y(weapon.module_accessor) - PostureModule::pos_y(sephiroth_boma);
                let tolerance = 20.0;
                if x_distance.abs() <= tolerance && y_distance.abs() <= tolerance{
                    StatusModule::change_status_force(weapon.module_accessor, *WEAPON_EDGE_FLAREDUMMY_STATUS_KIND_TRY, false);
                }
            }
        }
    }
}

pub fn install(agent: &mut Agent) {
    //agent.on_line(Main, shadowflare_orb_callback);
}