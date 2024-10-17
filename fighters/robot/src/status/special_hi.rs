use super::*;

use vars::robot::{
    instance::*,
    status::*
};

// FIGHTER_STATUS_KIND_SPECIAL_HI

unsafe extern "C" fn special_hi_pre(fighter: &mut L2CFighterCommon) -> L2CValue { 
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), // changed from ALWAYS_BOTH_SIDES
        true, 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false, // now lets people grab rob during the startup
        false,
        0,
        (*FIGHTER_STATUS_ATTR_INTO_DOOR | *FIGHTER_STATUS_ATTR_START_TURN) as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );

    return 0.into();
}

unsafe extern "C" fn special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int(fighter.battle_object, SPECIAL_HI_CHARGE_FRAME, 0);
    VarModule::set_float(fighter.battle_object, SPECIAL_HI_ROT_X, 0.0);

    if fighter.is_situation(*SITUATION_KIND_AIR) {
        VarModule::off_flag(fighter.battle_object, SPECIAL_HI_GROUND_START);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
    } else {
        VarModule::on_flag(fighter.battle_object, SPECIAL_HI_GROUND_START);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
    }

    let damage_statuses = &[
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL
    ];
    let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    let prev_status_kind_2 = StatusModule::prev_status_kind(fighter.module_accessor, 1);
    let is_prev_damage = damage_statuses.contains(&prev_status_kind) || damage_statuses.contains(&prev_status_kind_2);

    if !is_prev_damage {
        KineticModule::clear_speed_all(fighter.module_accessor);
        KineticModule::suspend_energy_all(fighter.module_accessor);
    } else {
        KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: 0.6, y: 0.2, z: 0.0}, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }

    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);

    let air_brake_x = sv_fighter_util::get_default_fighter_param_air_brake_x(fighter.lua_state_agent);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 8.0, 8.0);
    sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.0, 1.0);

    fighter.main_shift(special_hi_main_loop)
}

unsafe extern "C" fn special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let charge_frame = VarModule::get_int(fighter.battle_object, SPECIAL_HI_CHARGE_FRAME) as f32;
    VarModule::inc_int(fighter.battle_object, SPECIAL_HI_CHARGE_FRAME);
    
    // defines fuel consumption throughout the move
    let start_fuel = fighter.get_float(*FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
    let max_fuel = fighter.get_param_float("param_special_hi", "energy_max_frame");
    let fuel_increment = 2.0; // how much fuel is consumed by the charge per frame
    let min_cost = 20.0; // minimum amount of fuel consumed on use
    let required_fuel = (fuel_increment * charge_frame).clamp(min_cost, max_fuel);
    let remaining_fuel = (start_fuel - required_fuel).clamp(0.0, max_fuel);

    // handles rob's rotation during the charge
    let rot_x = VarModule::get_float(fighter.battle_object, SPECIAL_HI_ROT_X);
    if fighter.left_stick_x().abs() > 0.1 {   
        let rot_amount = 2.5; // how much rob rotates each frame
        let reverse = if fighter.is_stick_backward() { -1.0 } else { 1.0 };
        let direction = fighter.lr() * reverse; // determines the direction to rotate
        let angle = (rot_x + (rot_amount * direction)).clamp(-60.0, 60.0);
        PostureModule::set_rot(fighter.module_accessor, &Vector3f::new(angle * 0.3 * fighter.lr(), 0.0, 0.0), 0);
        VarModule::set_float(fighter.battle_object, SPECIAL_HI_ROT_X, angle);

        // changes direction if rotation crosses center threshold
        if rot_x == 0.0 && fighter.is_stick_backward() {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
    }
    // summon guide effect
    if rot_x != 0.0 { special_hi_guide_handler(fighter) };

    // default parameters for launch speed
    let mut launch_speed = Vector3f{
        x: 0.09 * rot_x.abs() * ((charge_frame - 18.0).clamp(0.0, 32.0) / 32.0),
        y: 0.5 - (0.025 * rot_x.abs()),
        z: 0.0
    };

    // force the full launch when the motion completes
    if MotionModule::is_end(fighter.module_accessor) {
        KineticModule::resume_energy_all(fighter.module_accessor);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

        PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));

        launch_speed.y = 3.75 - (0.025 * rot_x.abs());
        KineticModule::add_speed(fighter.module_accessor, &launch_speed);
        fighter.set_float(remaining_fuel, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);

        //println!("{}", launch_speed.x);
        fighter.change_status(FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP.into(), true.into());

        return 1.into();
    }

    let fuel_depleted = required_fuel >= start_fuel;
    if !fuel_depleted && fighter.is_button_on(Buttons::Special) {
        // continue charge as long as button is held and rob has fuel
        return 0.into();
    }

    // if we got to this point, we can assume the conditions have been met for commencing launch in some form
    KineticModule::resume_energy_all(fighter.module_accessor);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);

    let sfx =
        if charge_frame >= 20.0 { "se_common_bomb_l" }
        else if charge_frame >= 10.0 { "se_common_bomb_m" }
        else { "se_common_bomb_s" };

    if charge_frame >= 10.0 {
        launch_speed.y = (1.5 + (0.05 * charge_frame)) - (0.025 * rot_x.abs());
    }

    // launches/exits if rob ran out of fuel
    if fuel_depleted {
        if start_fuel > 0.0 { KineticModule::add_speed(fighter.module_accessor, &launch_speed); }
        fighter.set_float(0.0, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
        PLAY_SE(fighter, Hash40::new(sfx));

        //println!("{}", launch_speed.x);
        fighter.change_status(FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP.into(), true.into());

        return 1.into();
    } 

    // otherwise, launch with the amount of consumed fuel at the time of releasing the button
    if charge_frame >= 10.0 { // 10 frame minimum before launching
        KineticModule::add_speed(fighter.module_accessor, &launch_speed);
        fighter.set_float(remaining_fuel, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLOAT_BURNER_ENERGY_VALUE);
        PLAY_SE(fighter, Hash40::new(sfx));
    
        //println!("{}", launch_speed.x);
        fighter.change_status(FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP.into(), true.into());
    
        return 1.into();
    }

    return 0.into();
}
 

unsafe extern "C" fn special_hi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_ROBOT_STATUS_BURNER_FLAG_TRANSFORM_COMP);

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::resume_energy_all(fighter.module_accessor);

    let eff_handle = VarModule::get_int(fighter.battle_object, SPECIAL_HI_MARKER_EFFECT_HANDLE) as u32;
    if EffectModule::is_exist_effect(fighter.module_accessor, eff_handle) {
        EffectModule::kill(fighter.module_accessor, eff_handle, true, true);
        VarModule::set_int(fighter.battle_object, SPECIAL_HI_MARKER_EFFECT_HANDLE, 0);
    }

    return 0.into();
}

unsafe extern "C" fn arrow_guide_pos(fighter: &mut L2CFighterCommon, angle: L2CValue) -> Vector2f {
    let pos = PostureModule::pos(fighter.module_accessor);
    let rad = angle.get_f32().to_radians();
    let scale = PostureModule::scale(fighter.module_accessor);
    let dist = 20.0;
    let dist_scaled = dist * scale;
    let x_pos = rad.cos() * dist_scaled + (*pos).x;
    let y_pos = rad.sin() * dist_scaled + (*pos).y;
    let y_offset = 6.0;
    let y_pos = y_offset * scale + y_pos;
    Vector2f{x: x_pos, y: y_pos}
}

pub unsafe fn special_hi_guide_handler(fighter: &mut L2CFighterCommon) { // thanks wuboy <3
    let mut angle = (VarModule::get_float(fighter.battle_object, SPECIAL_HI_ROT_X) - 90.0) * -1.0;
    //println!("angle: {}", angle);

    let mut eff_handle = VarModule::get_int(fighter.battle_object, SPECIAL_HI_MARKER_EFFECT_HANDLE) as u32;
    let guide_pos = arrow_guide_pos(fighter, angle.into());
    if !EffectModule::is_exist_effect(fighter.module_accessor, eff_handle) {
        eff_handle = EffectModule::req(
            fighter.module_accessor,
            Hash40::new("sys_direction2"),
            &Vector3f{x: guide_pos.x, y: guide_pos.y, z: 0.0},
            &Vector3f{x: 0.0, y: 0.0, z: 0.0},
            1.0,
            0,
            -1,
            false,
            0
        ) as u32;
        VarModule::set_int(fighter.battle_object, SPECIAL_HI_MARKER_EFFECT_HANDLE, eff_handle as i32);
    } else {
        EffectModule::set_pos(fighter.module_accessor, eff_handle, &Vector3f{x: guide_pos.x, y: guide_pos.y, z: 0.0});
    }
    EffectModule::set_rot(fighter.module_accessor, eff_handle, &Vector3f{x: 0.0, y: 0.0, z: angle - 90.0});

    let team_color = FighterUtil::get_team_color(fighter.module_accessor);
    let effect_team_color = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
    EffectModule::set_rgb(fighter.module_accessor, eff_handle, effect_team_color.value[0], effect_team_color.value[1], effect_team_color.value[2]);
}

// FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP

unsafe extern "C" fn special_hi_keep_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_rise"), 0.0, 1.0, false, 0.0, false, false);

    let rot_x = VarModule::get_float(fighter.battle_object, SPECIAL_HI_ROT_X) * fighter.lr() * 0.8;
    PostureModule::set_rot(fighter.module_accessor, &Vector3f::new(rot_x, 0.0, 0.0), 0);

    fighter.main_shift(special_hi_keep_main_loop)
}

unsafe extern "C" fn special_hi_keep_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.motion_frame() >= 12.0 {
        if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR);
        }
    }

    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());

        return 1.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());

        return 1.into();
    }

    // // rotates rob to the stored rotation value over the course of a few frames
    // if (1..=4).contains(&fighter.status_frame()){
    //     let rot_x = VarModule::get_float(fighter.battle_object, SPECIAL_HI_ROT_X);
    //     let new_rot = 0.0 + (rot_x - ((rot_x / 4.0) * (5.0 - fighter.status_frame() as f32)));
    //     PostureModule::set_rot(fighter.module_accessor, &Vector3f::new(new_rot * fighter.lr(), 0.0, 0.0), 0);
    // }

    // interpolate back to upright position
    let current_rot = PostureModule::rot_x(fighter.module_accessor, 0);
    if (fighter.motion_frame() >= 45.0) && current_rot != 0.0 {
        let rot_amount = 0.125; // percent of remaining distance rotated each frame. will decrease exponentially
        let mut new_rot = current_rot - (current_rot * rot_amount);
        if (-1.0..1.0).contains(&new_rot) { new_rot = 0.0 }; // snap to 0 when close enough
        PostureModule::set_rot(fighter.module_accessor, &Vector3f::new(new_rot, 0.0, 0.0) ,0);
    }

    return 0.into();
}

unsafe extern "C" fn special_hi_keep_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_rot(fighter.module_accessor, &Vector3f::zero(), 0);
    VarModule::set_float(fighter.battle_object, SPECIAL_HI_ROT_X, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::resume_energy_all(fighter.module_accessor);

    0.into()
}

unsafe extern "C" fn stub_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_pre);
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_main);
    agent.status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, special_hi_end);
    agent.status(Main, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP, special_hi_keep_main);
    agent.status(End, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP, special_hi_keep_end);

    agent.status(Init, *FIGHTER_STATUS_KIND_SPECIAL_HI, stub_status);
    agent.status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, stub_status);
    agent.status(Init, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP, stub_status);
    agent.status(Exec, *FIGHTER_ROBOT_STATUS_KIND_SPECIAL_HI_KEEP, stub_status);
}