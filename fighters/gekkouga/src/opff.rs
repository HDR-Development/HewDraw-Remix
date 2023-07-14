// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn max_water_shuriken_dc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_MAX_SHOT {
        if frame > 12.0 {
            boma.check_dash_cancel();
        }
    }
}

// Greninja Shadow Sneak Smash Attack Cancel
unsafe fn shadow_sneak_smash_attack_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_ATTACK {
        if boma.status_frame() < 6 {
            if situation_kind == *SITUATION_KIND_GROUND {
                if boma.is_cat_flag(Cat1::AttackS4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, false);
                }
                if boma.is_cat_flag(Cat1::AttackHi4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
                }
                if boma.is_cat_flag(Cat1::AttackLw4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
                }
            }
        }
    }
}

// Dair Jump Cancel
unsafe fn dair_jc(boma: &mut BattleObjectModuleAccessor, situation_kind: i32, cat1: i32, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("attack_air_lw") {
        if !boma.is_in_hitlag() {
            if frame > 31.0 {
                if situation_kind == *SITUATION_KIND_AIR {
                    boma.check_jump_cancel(false);
                }
            }
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_SHOT,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_MAX_START,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_N_MAX_SHOT,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_ATTACK,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_WALL_DAMAGE,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_ATTACK,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_LW_BOUND
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    max_water_shuriken_dc(boma, status_kind, situation_kind, cat[0], frame);
    shadow_sneak_smash_attack_cancel(boma, status_kind, situation_kind, cat[0], frame);
    //dair_jc(boma, situation_kind, cat[0], motion_kind, frame);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_GEKKOUGA )]
pub fn gekkouga_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		gekkouga_frame(fighter)
    }
}

pub unsafe fn gekkouga_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}