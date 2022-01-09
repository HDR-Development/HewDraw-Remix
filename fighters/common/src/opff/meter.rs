use smash::app::BattleObjectModuleAccessor;
use smash::lua2cpp::L2CFighterCommon;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::{Vector2f, Vector3f, Vector4f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::phx::Hash40;
use smash::hash40;


use crate::utils::hdr;
use crate::vars::*;

use smash_script::macros::*;

use hdr_modules::consts::{*, globals::*};
use hdr_modules::*;

use super::gimmick;

pub unsafe fn get_meter_count(boma: &mut BattleObjectModuleAccessor) -> i32 {
    meter_counter[hdr::get_player_number(boma)]
}

pub unsafe fn get_meter_level(boma: &mut BattleObjectModuleAccessor) -> i32 {
    meter_level[hdr::get_player_number(boma)]
}

// Graphical routines for flashing upon gaining a meter level
pub unsafe fn meter_flash(boma: &mut BattleObjectModuleAccessor) {
    let id = hdr::get_player_number(boma);
    // Vector4f{r, g, b, a};
    // Meter gain color
    let cbm_vec1 = Vector4f{x: 0.22, y: 0.22, z: 0.65, w: 0.68};
    let cbm_vec2 = Vector4f{x: 0.22, y: 0.22, z: 0.65, w: 0.00};
    ColorBlendModule::set_main_color(boma, &cbm_vec1, &cbm_vec2, 1.0, 0.5, 10, true);

    meter_gain_glow_timer[id] = 1;

    let pos_meter1 = Vector3f{x: 0.0, y: 3.0, z: 0.0};
    let pos_meter2 = Vector3f{x: 0.0, y: 1.0, z: 0.0};
    let random_vec2 = Vector3f{x: 0.1, y: 0.1, z: 0.5}; // Has to do with color blending perhaps?
    EffectModule::kill_kind(boma, Hash40::new("sys_damage_curse"), false, true);
    EffectModule::kill_kind(boma, Hash40::new("sys_v_smoke_a"), false, true);
    EffectModule::req_on_joint(boma, Hash40::new("sys_damage_curse"), Hash40::new("top"), &pos_meter1, &hdr::DEFAULT_VECTOR3, 0.5, &random_vec2, &random_vec2, false, 0, 0, 0);
    EffectModule::req_on_joint(boma, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), &pos_meter2, &hdr::DEFAULT_VECTOR3, 0.5, &random_vec2, &random_vec2, false, 0, 0, 0);
}

pub unsafe fn meter_level_up(boma: &mut BattleObjectModuleAccessor) {
    if get_meter_level(boma) >= 1 {
        meter_flash(boma);
    }
}

pub unsafe fn update_meter_level(fighter: &mut L2CAgentBase, boma: &mut BattleObjectModuleAccessor) {
    let id = hdr::get_player_number(boma);
    let prev_meter_level = meter_level[id];
    let fighter_kind = smash::app::utility::get_kind(boma);
    let count = meter_counter[id];

    // Change this into a loop with meter_levelX
    if count < meter_level1 {
        meter_level[id] = 0;
    } else if count < meter_level2 {
        meter_level[id] = 1;
    } else if count < meter_level3 {
        meter_level[id] = 2;
    } else if count < meter_level4 {
        meter_level[id] = 3;
    } else if count < meter_level5 {
        meter_level[id] = 4;
    } else if count < meter_level6 {
        meter_level[id] = 5;
    } else if count < meter_level7 {
        meter_level[id] = 6;
    } else if count < meter_level8 {
        meter_level[id] = 7;
    } else if count < meter_level9 {
        meter_level[id] = 8;
    } else if count < meter_level10 {
        meter_level[id] = 9;
    } else {
        meter_level[id] = 10;
    }

    if [*FIGHTER_KIND_DOLLY, *FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN].contains(&fighter_kind) {
        if meter_level[id] > prev_meter_level || prev_meter_level > meter_level[id] {
            meter_gain_glow_timer[id] = 1;
            WorkModule::set_flag(boma, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
            meter_display(fighter, boma, 0, prev_meter_level);
        }
    }
}

pub unsafe fn taunt_meter_display(fighter: &mut L2CAgentBase, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let level = get_meter_level(boma);

    if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)
    || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_LW)
    || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L)
    || ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
        WorkModule::set_flag(boma, false, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        meter_display(fighter, boma, 0, level);
    }

    if ControlModule::check_button_release(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)
    || ControlModule::check_button_release(boma, *CONTROL_PAD_BUTTON_APPEAL_LW)
    || ControlModule::check_button_release(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L)
    || ControlModule::check_button_release(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
        WorkModule::set_flag(boma, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
        EffectModule::kill_kind(boma, Hash40::new("sys_starrod_bullet"), false, true);
    }
}

pub unsafe fn meter_display(fighter: &mut L2CAgentBase, boma: &mut BattleObjectModuleAccessor, status_kind: i32, prev_meter_level: i32) {
    let mut gain = true;
    if prev_meter_level >= get_meter_level(boma) {
        gain = false;
    }
    let mut level = get_meter_level(boma);
    if !gain {
        level = prev_meter_level;
    }
    let mut fx_pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};

    EffectModule::kill_kind(boma, Hash40::new("sys_starrod_bullet"), false, true);
    for i in 1..level {
        if i < 6 {
            fx_pos = Vector3f{x: -15.0 + (5.0*i as f32), y: 22.0, z: -15.0 + (5.0*i as f32)};  // bottom row
        }
        else {
            fx_pos = Vector3f{x: -15.0 + (5.0*(i-5) as f32), y: 27.0, z: -15.0 + (5.0*(i-5) as f32)};  // top row
        }
        EffectModule::req_follow(boma, Hash40::new("sys_starrod_bullet"), Hash40::new("top"), &fx_pos, &hdr::DEFAULT_VECTOR3, 0.3, false, 0, 0, 0, 0, 0, false, false);
        if !gain && get_meter_level(boma) <= i {
            // burned meter bars signified by smaller, transparent stars
            let scale = Vector3f{x: 0.25, y: 0.25, z: 0.25};
            LAST_EFFECT_SET_ALPHA(fighter, 0.15);
            EffectModule::set_scale_last(boma, &scale);
        }
    }
    if gain {
        // gained meter bar signified by bigger, light blue star
        let scale = Vector3f{x: 0.4, y: 0.4, z: 0.4};
        LAST_EFFECT_SET_COLOR(fighter, 0.1, 0.7, 3.0);
        EffectModule::set_scale_last(boma, &scale);
    }
}

pub unsafe fn set_meter(fighter: &mut L2CAgentBase, boma: &mut BattleObjectModuleAccessor, meter_amount: i32) {
    let id = hdr::get_player_number(boma);
    if meter_amount < 0 {
        meter_counter[id] = 0;
    } else if meter_counter[id] > meter_max {
        meter_counter[id] = meter_max;
    } else {
        meter_counter[id] = meter_amount;
    }
    // Update the meter level after updating the meter count
    update_meter_level(fighter, boma);
}

pub unsafe fn add_meter(fighter: &mut L2CAgentBase, boma: &mut BattleObjectModuleAccessor, meter_amount: i32) {
    let id = hdr::get_player_number(boma);
    if meter_counter[id] - meter_amount < meter_max {
        meter_counter[id] += meter_amount;
    } else {
        meter_counter[id] = meter_max;
    }
    // Update the meter level after updating the meter count
    update_meter_level(fighter, boma);
}

pub unsafe fn drain_meter(fighter: &mut L2CAgentBase, boma: &mut BattleObjectModuleAccessor, meter_amount: i32) {
    let id = hdr::get_player_number(boma);
    if meter_counter[id] - meter_amount < 0 {
        meter_counter[id] = 0;
    } else {
        meter_counter[id] -= meter_amount;
    }
    // Update the meter level after udpating the meter count
    update_meter_level(fighter, boma);
}

pub unsafe fn meter_gain(boma: &mut BattleObjectModuleAccessor, meter_amount: i32) {
    let id = hdr::get_player_number(boma);
    meter_to_gain[id] = meter_amount;
}

// Used to reset the meter gain flag in between multihits
pub unsafe fn meter_gain_multihit(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, meter_amount: i32) {
    let id = hdr::get_player_number(boma);
    meter_gained_current_status[id] = false;
    meter_to_gain[id] = meter_amount;
}

// Meter Add function; always running
pub unsafe fn meter_add(fighter: &mut L2CAgentBase, boma: &mut BattleObjectModuleAccessor) {
    let id = hdr::get_player_number(boma);
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !meter_gained_current_status[id] {
        add_meter(fighter, boma, meter_to_gain[id]);
        meter_gained_current_status[id] = true;
    }
}

pub unsafe fn use_meter_level(fighter: &mut L2CAgentBase, boma: &mut BattleObjectModuleAccessor, level_amount: i32) {
    // Use array instead
    let x = match level_amount {
        1 => meter_level1,
        2 => meter_level2,
        3 => meter_level3,
        4 => meter_level4,
        5 => meter_level5,
        6 => meter_level6,
        7 => meter_level7,
        8 => meter_level8,
        9 => meter_level9,
        10 => meter_level10,
        _ => 0
    };
    drain_meter(fighter, boma, x);
}

// Reset meter upon match end
pub unsafe fn meter_reset(fighter: &mut L2CAgentBase, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if [*FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
            EffectModule::kill_kind(boma, Hash40::new("sys_starrod_bullet"), false, true);
            set_meter(fighter, boma, 0);
    }
}

// Reset meter glow
pub unsafe fn meter_glow_timer_counting(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let id = hdr::get_player_number(boma);
    if meter_gain_glow_timer[id] > 0 && meter_gain_glow_timer[id] < 121 { // 150/5 = 30F
        if meter_gain_glow_timer[id] > 119 {
            WorkModule::set_flag(boma, true, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);
            ColorBlendModule::cancel_main_color(boma, 0);
            EffectModule::kill_kind(boma, Hash40::new("sys_damage_curse"), false, true);
            EffectModule::kill_kind(boma, Hash40::new("sys_v_smoke_a"), false, true);
            EffectModule::kill_kind(boma, Hash40::new("sys_starrod_bullet"), false, true);
            meter_gain_glow_timer[id] = 0;
        } else {
            meter_gain_glow_timer[id] += 1;
        }
    }
    if [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_ENTRY].contains(&status_kind) {
        meter_gain_glow_timer[id] = 0;
    }
}


pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    let mut agent_base = fighter.fighter_base.agent_base;
    meter_reset(&mut agent_base, boma, status_kind);
    if [*FIGHTER_KIND_DOLLY, *FIGHTER_KIND_RYU, *FIGHTER_KIND_KEN].contains(&fighter_kind) {
        meter_add(&mut agent_base, boma);
        taunt_meter_display(&mut agent_base, boma, status_kind);
        meter_glow_timer_counting(fighter, boma, status_kind);
    }
    if [*FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_GANON, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_REFLET].contains(&fighter_kind) {
        meter_glow_timer_counting(fighter, boma, status_kind);
    }
    gimmick::gimmick_ready_glow_timer_counting(boma, status_kind);
}
