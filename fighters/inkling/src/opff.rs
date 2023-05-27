// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 

const INKLING_COLORS: [Vector3f;8] = [
    // used to tint the hitbox effects - have at least one value be 0, unless you want white ink. Values between 0 and 1.
    Vector3f {x:0.758027, y:0.115859, z:0.04},  // (orange)
    Vector3f {x:0.04, y:0.0608165, z:0.758027},  // (blue)
    Vector3f {x:0.79, y:0.504014, z:0.04},  // (yellow)
    Vector3f {x:0.347369, y:0.582004, z:0.04},  // (green)
    Vector3f {x:0.758027, y:0.0608165, z:0.273385},  // (pink)
    Vector3f {x:0.04, y:0.47948, z:0.388556},  // (sky blue)
    Vector3f {x:0.47948, y:0.04, z:0.582004},  // (violet)
    Vector3f {x:0.04, y:0.0462798, z:0.114017},  // (purple)
];

unsafe fn dair_splatter(boma: &mut BattleObjectModuleAccessor, motion_kind: u64, id: usize){
    if motion_kind == hash40("attack_air_lw") && AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
        let pos = Vector3f {x:0.,y:-2.,z:0.};
        let rot = Vector3f {x:0.,y:90.,z:0.};
        let handle2 = EffectModule::req_on_joint(boma,Hash40::new("inkling_blaster_muzzle"),Hash40::new("top"),&pos,&rot,2.2,&Vector3f::zero(), &Vector3f::zero(),false,0,0,0) as u32;
        let costumenum = VarModule::get_int(boma.object(), vars::common::instance::COSTUME_SLOT_NUMBER) as usize;
        EffectModule::set_rgb(boma, handle2, INKLING_COLORS[costumenum].x, INKLING_COLORS[costumenum].y, INKLING_COLORS[costumenum].z);
        EffectModule::set_rate_last(boma, 0.5);
    }
}

unsafe fn roller_jump_cancel(boma: &mut BattleObjectModuleAccessor){
    if boma.is_status(*FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_END) && boma.is_situation(*SITUATION_KIND_GROUND) && boma.status_frame() > 10 {
        boma.check_jump_cancel(true);
    }
    if boma.is_motion(Hash40::new("special_air_s_jump_end")){
        if MotionModule::frame(boma) > 6.0 {
            CancelModule::enable_cancel(boma);
        }
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    dair_splatter(boma, motion_kind, id);
    roller_jump_cancel(boma);
}

#[utils::macros::opff(FIGHTER_KIND_INKLING )]
pub fn inkling_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		inkling_frame(fighter);
    }
}

pub unsafe fn inkling_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}