use common::opff_import::*;
use super::*;
use globals::*;
use common::opff::*;
 
unsafe fn special_s_article_fix(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, frame: f32) {
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_LINK_STATUS_KIND_SPECIAL_S2].contains(&status_kind) {
        if frame <= 1.0 {
            special_projectile_spawned[id] = false;
        }
    }
}

// Young Link Fire Arrow fast fall
unsafe fn fire_arrow_ff(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if hdr::compare_cat(cat2, *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP)
                && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Young Link Bomb pull B-Reverse
unsafe fn bomb_b_reverse(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if frame < 5.0 {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
                if frame > 1.0 && frame < 5.0 && !b_reversed[id] {
                    let b_reverse = Vector3f{x: -1.0, y: 1.0, z: 1.0};
                    KineticModule::mul_speed(boma, &b_reverse, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    b_reversed[id] = true;
                }
            }
        }
    }
}

// Bombchu Timer Count
unsafe fn bombchu_timer(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize) {
    let gimmick_timerr = VarModule::get_int(fighter.battle_object, vars::common::GIMMICK_TIMER);
    if gimmick_timerr > 0 && gimmick_timerr < 721 {
        // Bombchu Timer Reset
        if gimmick_timerr > 719 {
            VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, 0);
            gimmick_flash(boma);
        } else {
            VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, gimmick_timerr + 1);
        }
    }
}

// Bombchu Timer Death Reset
unsafe fn bombchu_reset(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) {
        VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, 0);
    }
}

// Training Mode Bombchu Timer taunt reset
unsafe fn bombchu_training(fighter: &mut L2CFighterCommon, id: usize, status_kind: i32) {
    if hdr::is_training_mode() {
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL {
            VarModule::set_int(fighter.battle_object, vars::common::GIMMICK_TIMER, 0);
        }
    }
}

// Lengthen sword
unsafe fn sword_length(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
	let long_sword_scale = Vector3f{x: 1.0, y: 1.1, z: 1.0};
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("sword"), &long_sword_scale);
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    special_s_article_fix(fighter, boma, id, status_kind, situation_kind, frame);
    fire_arrow_ff(fighter, boma, status_kind, situation_kind, cat[1], stick_y);
    bomb_b_reverse(fighter, boma, id, status_kind, stick_x, facing, frame);
    bombchu_timer(fighter, boma, id);
    bombchu_reset(fighter, id, status_kind);
    bombchu_training(fighter, id, status_kind);
	sword_length(fighter, boma);
    links::moveset(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);
}

#[utils::opff(FIGHTER_KIND_YOUNGLINK )]
pub fn younglink_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        fighter_common_opff(fighter);
		younglink_frame(fighter)
    }
}

pub unsafe fn younglink_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = crate::hooks::sys_line::FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}