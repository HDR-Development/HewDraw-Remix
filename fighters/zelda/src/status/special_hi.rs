use super::*;

pub unsafe extern "C" fn special_hi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    create_arrow_eff(fighter);
    0.into()
}

unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let effect = VarModule::get_int(fighter.battle_object, vars::common::status::WARP_EFF_HANDLER) as u32;
    if effect != 0 {
        EffectModule::kill(fighter.module_accessor, effect, true, true);
        VarModule::set_int(fighter.battle_object, vars::common::status::WARP_EFF_HANDLER, 0);
    }
    smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
}

unsafe extern "C" fn special_hi_2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_3 {
        //clear buffer
        ControlModule::reset_trigger(fighter.module_accessor);
        //re-uses waveland window logic
        let init_speed_y = VarModule::get_float(fighter.battle_object, vars::common::status::TELEPORT_INITIAL_SPEED_Y); //teleport direction
        let pos = *PostureModule::pos(fighter.module_accessor); //top bone (bottom of ecb w/o shifting)
        let hip_pos = &mut Vector3f::zero();
        ModelModule::joint_global_position(fighter.module_accessor, Hash40::new("hip"), hip_pos, false); //position of hip bone
        let bot_snap = &Vector2f::new(pos.x, pos.y - 1.5);
        let top_snap = &Vector2f::new(pos.x, hip_pos.y);
        let ground_pos_any = &mut Vector2f::zero();
        let ground_pos_stage = &mut Vector2f::zero();
        let is_touch_any = GroundModule::line_segment_check(fighter.module_accessor, top_snap, bot_snap, &Vector2f::zero(), ground_pos_any, true);
        let is_touch_stage = GroundModule::line_segment_check(fighter.module_accessor, top_snap, bot_snap, &Vector2f::zero(), ground_pos_stage, false);
        let can_snap = !(is_touch_any == 0 as *const *const u64 || (is_touch_stage != 0 as *const *const u64 && init_speed_y > 0.0)); //avoid snapping to stage from below
        if can_snap {
            PostureModule::set_pos(fighter.module_accessor, &Vector3f::new(pos.x, ground_pos_any.y + 0.1, pos.z));
            GroundModule::attach_ground(fighter.module_accessor, false);
            StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), false);
            PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
    }
    0.into()
}

//stolen from sora code
unsafe extern "C" fn create_arrow_eff(fighter: &mut L2CFighterCommon) {
    let handle: u32 = VarModule::get_int(fighter.battle_object, vars::common::status::WARP_EFF_HANDLER) as u32;
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let vector = fighter.Vector2__create(stick_x.into(), stick_y.into());
    let length = fighter.Vector2__length(vector.clone());
    let threshold = 0.5;
    let mut angle= vector["y"].get_f32().atan2(vector["x"].get_f32()).to_degrees();
    if angle < 0.0 { angle += 360.0 }
    if length.get_f32() < threshold {angle = 90.0;}//sets angle to 90 if in stick deadzone
    let eff_pos = get_arrow_pos(fighter, angle.into());
    //println!("x{}, y{}, degree{}", stick_x, stick_y, angle);
    if handle != 0 {
        EffectModule::set_pos(fighter.module_accessor, handle, &Vector3f{x: eff_pos.x, y: eff_pos.y, z: 0.0});
        EffectModule::set_rot(fighter.module_accessor, handle, &Vector3f{x: 0.0, y: 0.0, z: angle - 90.0});
    } else {
        let effect = EffectModule::req(fighter.module_accessor, Hash40::new("sys_direction2"), &Vector3f{x: eff_pos.x, y: eff_pos.y, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0.4, 0, -1, false, 0);
        EffectModule::set_rot(fighter.module_accessor, effect as u32, &Vector3f{x: 0.0, y: 0.0, z: angle - 90.0});
        let team_color = FighterUtil::get_team_color(fighter.module_accessor);
        let effect_team_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
        EffectModule::set_rgb_partial_last(fighter.module_accessor, effect_team_color.x, effect_team_color.y, effect_team_color.z);
        VarModule::set_int(fighter.battle_object, vars::common::status::WARP_EFF_HANDLER, effect as i32);
    }
}

unsafe extern "C" fn get_arrow_pos(fighter: &mut L2CFighterCommon, angle: L2CValue) -> Vector2f {
    let pos = PostureModule::pos(fighter.module_accessor);
    let rad = angle.get_f32().to_radians();
    let scale = PostureModule::scale(fighter.module_accessor);
    let dist = 12.5;
    let dist_scaled = dist * scale;
    let x_pos = rad.cos() * dist_scaled + (*pos).x;
    let y_pos = rad.sin() * dist_scaled + (*pos).y;
    let y_offset = 9.25; //try to get center of fighter
    let y_pos = y_offset * scale + y_pos;
    Vector2f{x: x_pos, y: y_pos}
}


pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_exec);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_end);
    agent.status(End, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_HI_2, special_hi_2_end);
}