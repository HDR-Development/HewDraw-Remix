// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn dragon_fang_shot_dash_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT && frame > 7.0 {
        boma.check_dash_cancel();
    }
}

unsafe fn pin_drop_waveland(fighter: &mut L2CFighterCommon, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    let boma = fighter.boma();
    if status_kind == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_END {
        if !fighter.is_in_hitlag() 
        && frame >= 12.0 {
            fighter.check_airdodge_cancel();
        } 
    }
}

unsafe fn bair_boost_detection(boma: &mut BattleObjectModuleAccessor){
    if boma.get_aerial() == Some(AerialKind::Bair) {
        if boma.is_cat_flag(Cat1::AttackS4){
            VarModule::on_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
        }
        else{
            VarModule::off_flag(boma.object(), vars::common::instance::IS_HEAVY_ATTACK);
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //dragon_fang_shot_dash_cancel(boma, status_kind, situation_kind, cat[0], frame);
    bair_boost_detection(boma);
    pin_drop_waveland(fighter, status_kind, situation_kind, cat[0], frame);
}

#[utils::macros::opff(FIGHTER_KIND_KAMUI )]
pub fn kamui_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		kamui_frame(fighter)
    }
}

pub unsafe fn kamui_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}