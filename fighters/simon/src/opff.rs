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
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            if boma.is_cat_flag(Cat2::FallJump)
                && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
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

// Simon Cross Fast Fall
unsafe fn cross_ff(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat2: i32, stick_y: f32) {
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2]) {
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            VarModule::on_flag(boma.object(), vars::simon::status::AIR_CROSS);
            if boma.is_cat_flag(Cat2::FallJump)
                && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
        if fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.is_prev_situation(*SITUATION_KIND_AIR){
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
    }
}

// Turn off air_cross flag if in the air and not in Cross
unsafe fn air_cross_air_off(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        if !fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2]) {
            if VarModule::is_flag(boma.object(), vars::simon::status::AIR_CROSS) {
                VarModule::off_flag(boma.object(), vars::simon::status::AIR_CROSS);
            }
        }
    }
}

// Land cancel Cross if used in the air and fallen to the ground
unsafe fn land_cancel_cross(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_situation(*SITUATION_KIND_GROUND) && VarModule::is_flag(boma.object(), vars::simon::status::AIR_CROSS) {
        VarModule::off_flag(boma.object(), vars::simon::status::AIR_CROSS);
        //StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);

        // Current FAF in motion list is 42, frame is 0 indexed so subtract a frame
        let special_s1_cancel_frame_ground = 41.0;
        // 12F of landing lag plus one extra frame to subtract from the FAF to actually get that amount of lag
        let landing_lag = 13.0;
        if MotionModule::frame(fighter.module_accessor) < (special_s1_cancel_frame_ground - landing_lag) {
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, special_s1_cancel_frame_ground - landing_lag, true, true, false);
        }
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    holy_water_ac(fighter, boma, id, status_kind, situation_kind, cat[0], frame);
    //dair_cancels(fighter, boma, motion_kind, id, status_kind, situation_kind, cat[0], frame);
    axe_ff(boma, status_kind, situation_kind, cat[1], stick_y);
    cross_ff(fighter, boma, cat[1], stick_y);
    air_cross_air_off(fighter, boma);
    land_cancel_cross(fighter, boma);
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