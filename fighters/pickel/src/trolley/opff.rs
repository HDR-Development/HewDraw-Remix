// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

pub unsafe extern "C" fn pickel_trolley_frame(agent: &mut smash::lua2cpp::L2CFighterBase) {
    let boma = agent.boma();
    let owner_id = WorkModule::get_int(agent.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_PICKEL {
        let pickel = utils::util::get_battle_object_from_id(owner_id);
        let pickel_boma = &mut *(*pickel).module_accessor;
        // Burn double jump when jumping out of Minecart
        if boma.is_situation(*SITUATION_KIND_AIR)
        && pickel_boma.is_status(*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_JUMP) {
            if MotionModule::frame(pickel_boma) <= 1.0
            && pickel_boma.get_num_used_jumps() < pickel_boma.get_jump_count_max() {
                WorkModule::inc_int(pickel_boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
        }
        // Restore double jump when landing with Minecart
        if boma.is_situation(*SITUATION_KIND_GROUND)
        && pickel_boma.is_status(*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_DRIVE) {
            if pickel_boma.get_num_used_jumps() >= pickel_boma.get_jump_count_max() {
                WorkModule::dec_int(pickel_boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pickel_trolley_frame);
}
