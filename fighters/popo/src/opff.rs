// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 

// Ice Climbers Cheer Cancel (Techy)
unsafe fn cheer_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if boma.kind() == *FIGHTER_KIND_NANA {
        if status_kind == *FIGHTER_POPO_STATUS_KIND_THROW_NANA {
            MotionModule::set_frame(boma, MotionModule::end_frame(boma), true);
            StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_WAIT, true);
        }
    }
}

// Ice Climbers Spotdodge Desync
unsafe fn spotdodge_desync(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if boma.kind() == *FIGHTER_KIND_NANA {
        if ![*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&status_kind){
            InputModule::disable_persist(boma.object());
        } else if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B].contains(&StatusModule::status_kind_next(boma)) {
            InputModule::enable_persist(boma.object());
        }
    }
}

// Clear JC grab flag
unsafe fn clear_jc_grab_flag(boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32) {
    if boma.kind() == *FIGHTER_KIND_POPO {
        //VarModule::set_flag(boma.object(), vars::common::POPO_JC_GRAB,
        //[*FIGHTER_STATUS_KIND_CATCH,
        //    *FIGHTER_STATUS_KIND_CATCH_PULL,
        //    *FIGHTER_STATUS_KIND_CATCH_WAIT].contains(&status_kind));
    }
}

pub static mut nana_boma: [u64; 8] = [0; 8];

unsafe fn get_nana_boma(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize) {
    if boma.kind() == *FIGHTER_KIND_NANA {
        let mut nana_boma_deez = boma;
        nana_boma[id] = (&mut *nana_boma_deez as *mut BattleObjectModuleAccessor) as u64;
    }
}

static mut effect_on: bool = false;
static mut nana_pos_x: f32 = 0.0;
static mut nana_pos_y: f32 = 0.0;

unsafe fn nana_death_effect(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {

    if boma.kind() == *FIGHTER_KIND_POPO {
        if status_kind == *FIGHTER_STATUS_KIND_STANDBY {
            effect_on = true;
            nana_pos_x = PostureModule::pos_x(nana_boma[id] as *mut BattleObjectModuleAccessor);
            nana_pos_y = PostureModule::pos_y(nana_boma[id] as *mut BattleObjectModuleAccessor);
        }
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH && MotionModule::frame(boma) <= 1.0 && effect_on {
            let pos =  Vector3f {x: nana_pos_x, y: nana_pos_y, z: 0.0};
            let rot =  Vector3f {x: 0.0, y: 0.0, z: 0.0};
            EffectModule::req(boma, Hash40::new("sys_recovery"), &pos, &rot, 1.0, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32, 0, true, 0);
            effect_on = false;
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // nothing lol
}


#[no_mangle]
pub unsafe extern "Rust" fn ice_climbers_common(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        ice_climbers_moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub unsafe fn ice_climbers_moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    cheer_cancel(fighter, boma, status_kind);
    spotdodge_desync(boma, status_kind);
    //clear_jc_grab_flag(boma, id, status_kind);
    get_nana_boma(fighter, boma, id);
    nana_death_effect(fighter, boma, id, status_kind, frame);
}

#[utils::macros::opff(FIGHTER_KIND_POPO )]
pub fn popo_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		popo_frame(fighter);
        ice_climbers_common(fighter);
    }
}

pub unsafe fn popo_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}