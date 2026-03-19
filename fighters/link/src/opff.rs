// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && ( fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2,
        ])
        || (fighter.is_motion(Hash40::new("special_air_hi")) && fighter.motion_frame() > 60.0) )
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

unsafe fn bomb_cancel(fighter: &mut smash::lua2cpp::L2CFighterCommon)  { 
    if VarModule::is_flag(fighter.battle_object, vars::link::status::ENABLE_SPECIAL_LW_CANCEL)
    && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
    && ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_LINKBOMB)
    && fighter.is_cat_flag(Cat1::SpecialLw) {
        // let bomb_exists = ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_LINKBOMB);
        // println!("Bomb Exists: {}", bomb_exists);
        fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_LW, true); {
    }
}
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    fastfall_specials(fighter);
    bomb_cancel(fighter);
}

pub extern "C" fn link_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		link_frame(fighter);
    };
}

pub unsafe fn link_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, link_frame_wrapper);
}