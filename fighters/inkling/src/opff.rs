use ::common::opff_import::*;
use super::*;
use globals::*;
use ::common::opff::*;
 
unsafe fn inkling_moveset(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, id: usize){
    if motion_kind == hash40("attack_air_lw") && AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
        let pos = Vector3f {x:0.,y:-2.,z:0.};
        let rot = Vector3f {x:0.,y:90.,z:0.};
        let handle2 = EffectModule::req_on_joint(boma,Hash40::new("inkling_blaster_muzzle"),Hash40::new("top"),&pos,&rot,2.2,&Vector3f::zero(), &Vector3f::zero(),false,0,0,0) as u32;
        let costumenum = costumeslotnumber[id] as usize;
        EffectModule::set_rgb(boma, handle2, INKLING_COLORS[costumenum].x, INKLING_COLORS[costumenum].y, INKLING_COLORS[costumenum].z);
        EffectModule::set_rate_last(boma, 0.5);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    inkling_moveset(boma, motion_kind, id);
}

#[utils::opff(FIGHTER_KIND_INKLING )]
pub fn inkling_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		inkling_frame(fighter)
    }
}

pub unsafe fn inkling_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}