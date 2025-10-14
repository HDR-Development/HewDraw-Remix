// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;


unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH) {
        let sword_scale = Vector3f{x: 0.7, y: 1.0, z: 1.0};
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("swordl1"), &sword_scale);
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("swordr1"), &sword_scale);
    }
    else {
        let long_sword_scale = Vector3f{x: 0.95, y: 1.0, z: 1.0};
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("swordl1"), &long_sword_scale);
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("swordr1"), &long_sword_scale);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_CHARGE,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_S_SHOOT,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_EDGE_STATUS_KIND_SPECIAL_LW_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    sword_length(boma);
    fastfall_specials(fighter);
}

pub extern "C" fn edge_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		edge_frame(fighter)
    }
}

pub unsafe fn edge_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, edge_frame_wrapper);
}