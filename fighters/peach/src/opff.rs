use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);
 
unsafe fn float_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
        let prev_status_0 = StatusModule::prev_status_kind(boma, 0);
        let prev_status_1 = StatusModule::prev_status_kind(boma, 1);
        if [*FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START].contains(&prev_status_0)
            || [*FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START].contains(&prev_status_1) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
        }
    }
}
unsafe fn wall_bounce(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP {
        let lr = PostureModule::lr(boma);
        let pos_x = PostureModule::pos_x(boma);
        let pos_y = PostureModule::pos_y(boma);
        let dist = 6.0*lr;
        let frame = MotionModule::frame(boma) as i32;
        if 
            (
                GroundModule::ray_check(boma, &smash::phx::Vector2f{ x: pos_x, y: pos_y}, &Vector2f{ x: dist, y: 0.0}, false) == 1
                || GroundModule::ray_check(boma, &smash::phx::Vector2f{ x: pos_x, y: pos_y}, &Vector2f{ x: dist, y: 6.0}, false) == 1
            )
            && (1..25).contains(&frame){
                VarModule::on_flag(boma.object(), vars::peach::instance::IS_WALLBOUNCE);
                StatusModule::change_status_request_from_script(boma, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_HIT_END, true);
        }
    }else if status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_HIT_END {
        if VarModule::is_flag(boma.object(), vars::peach::instance::IS_WALLBOUNCE) {
            MotionModule::set_rate(boma, 0.6);
        }
    } else {
        VarModule::off_flag(boma.object(), vars::peach::instance::IS_WALLBOUNCE);
    }
}

pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    float_cancel(boma, status_kind);
    wall_bounce(boma, status_kind);
}

#[::utils::macros::opff(FIGHTER_KIND_PEACH )]
pub fn peach_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		peach_frame(fighter)
    }
}

pub unsafe fn peach_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}