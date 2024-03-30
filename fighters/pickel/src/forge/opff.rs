// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub unsafe extern "C" fn pickel_forge_frame(weapon: &mut smash::lua2cpp::L2CFighterBase){
    let boma = weapon.boma();
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_PICKEL {
        let pickel = utils::util::get_battle_object_from_id(owner_id);
        let pickel_boma = &mut *(*pickel).module_accessor;
        if pickel_boma.is_motion_one_of(&[ 
            Hash40::new("attack_air_lw"),
            Hash40::new("attack_air_lw_2"),
            Hash40::new("attack_air_lw_fall") ])
        && !boma.is_situation(*SITUATION_KIND_GROUND) 
        //&& !pickel_boma.is_status(*FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_START)
        && WorkModule::is_flag(boma, *WEAPON_PICKEL_FORGE_INSTANCE_WORK_ID_FLAG_UPDATE_ATTACK){
            MotionAnimcmdModule::call_script_single(boma, *FIGHTER_ANIMCMD_GAME, Hash40::new("game_fallattackride"), -1);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pickel_forge_frame);
}
