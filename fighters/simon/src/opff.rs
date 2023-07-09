// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn holy_water_ac(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if frame > 20.0 {
            boma.check_airdodge_cancel();
        }
    }
}

unsafe fn axe_ff(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(boma) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
}

unsafe fn dair_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, motion_kind: u64, id: usize, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    //Act out of it much faster on hit so you can actually followup on people with good DI
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR
    && motion_kind == hash40("attack_air_lw2")
    && !StatusModule::is_changing(fighter.module_accessor)
    {
        if frame > 15.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}


pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    holy_water_ac(fighter, boma, id, status_kind, situation_kind, cat[0], frame);
    dair_cancels(fighter, boma, motion_kind, id, status_kind, situation_kind, cat[0], frame);
    axe_ff(boma, status_kind, situation_kind, cat[1], stick_y);
}

#[utils::macros::opff(FIGHTER_KIND_SIMON )]
pub fn simon_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		simon_frame(fighter)
    }
}

pub unsafe fn simon_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}