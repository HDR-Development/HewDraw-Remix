use crate::opff_import::*;
use smash::app::{self, lua_bind::*, sv_animcmd, sv_system};
use smash::lib::{lua_const::*, L2CAgent};
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::{Hash40, Vector3f};

#[repr(C)]
struct HitboxData {
    pos1: Vector3f,
    pos2: Vector3f,
    last_frame: Vector3f,
    color: Vector3f,
    id: i32,
    bone: Hash40,
    radius: f32,
    angle: f32,
    frame: f32,
    is_cap: bool
}

extern "C" {
    fn enable_hitbox_vis(enable: bool);
    fn enable_hurtbox_vis(enable: bool);
    fn enable_special_vis(enable: bool);
    fn is_hitbox_vis() -> bool; 
    fn is_hurtbox_vis() -> bool;
    fn is_special_vis() -> bool;
}

const hitbox_vis_toggle_effect_pos: Vector3f = Vector3f {x: 7.0, y: 18.0, z: 0.0};
pub unsafe fn training_mode_hitbox_visualizer_control(boma: &mut app::BattleObjectModuleAccessor) {
    let id = hdr::get_player_number(boma);
    // Needs optimization
    if id == 0 && hdr::is_training_mode()
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)
        && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK) //need to hold left taunt and shield, and then press attack
        && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CATCH)
    {
        DEBUG = !DEBUG;
        MotionAnimcmdModule::set_sleep_effect(boma, DEBUG);
        EffectModule::req_on_joint(boma, Hash40::new("sys_sp_flash"), Hash40::new("top"), &hitbox_vis_toggle_effect_pos, &hdr::DEFAULT_VECTOR3, 1.5, &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, false, 0, 0, 0);
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
            //print!("[HDR] Manual Crash Triggered");
            //*(0 as *mut _) = 69; // lmao nice 😎
        }
        if !DEBUG {
            EffectModule::kill_kind(boma, Hash40::new("sys_ripstick_bullet"), true, true);
        }
    }
    if id == 0 && hdr::is_training_mode()
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L)
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)
        && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK)
        && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CATCH)
    {
        enable_hitbox_vis(!is_hitbox_vis());
    }
    if id == 0 && hdr::is_training_mode()
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R)
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)
        && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK)
        && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CATCH)
    {
        enable_hurtbox_vis(!is_hurtbox_vis());
    }
    if id == 0 && hdr::is_training_mode()
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW)
        && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)
        && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_ATTACK)
        && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_CATCH)
    {
        enable_special_vis(!is_special_vis());
    }
}


pub const ID_COLORS: &[Vector3f] = &[
    // used to tint the hitbox effects -- make sure that at least one component
    // is equal to 1.0
    Vector3f {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    }, // #ff0000 (red)
    Vector3f {
        x: 1.0,
        y: 0.4,
        z: 0.0,
    }, // #ff9900 (orange)
    Vector3f {
        x: 0.8,
        y: 1.0,
        z: 0.0,
    }, // #ccff00 (yellow)
    Vector3f {
        x: 0.2,
        y: 1.0,
        z: 0.2,
    }, // #00ff33 (green)
    Vector3f {
        x: 0.0,
        y: 0.8,
        z: 1.0,
    }, // #00ccff (sky blue)
    Vector3f {
        x: 0.4,
        y: 0.4,
        z: 1.0,
    }, // #6666ff (blue)
    Vector3f {
        x: 0.8,
        y: 0.0,
        z: 1.0,
    }, // #cc00ff (purple)
    Vector3f {
        x: 1.0,
        y: 0.2,
        z: 0.8,
    }, // #ff33cc (pink)
];
const MAX_EFFECTS_PER_HITBOX: i32 = 16; // max # of circles drawn for an extended hitbox

pub fn is_shielding(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        let status_kind = StatusModule::status_kind(module_accessor) as i32;
        (*FIGHTER_STATUS_KIND_GUARD_ON..=*FIGHTER_STATUS_KIND_GUARD_DAMAGE).contains(&status_kind)
    }
}

pub unsafe fn generate_hitbox_effects(
    module_accessor: &mut app::BattleObjectModuleAccessor,
    bone: u64,
    size: f32,
    x: f32,
    y: f32,
    z: f32,
    x2: Option<f32>,
    y2: Option<f32>,
    z2: Option<f32>,
    color: Vector3f,
) {
    let mut offset = Vector3f{x: x, y: y, z: z};
    let mut offset2 = Vector3f{x: x2.unwrap_or(x), y: y2.unwrap_or(y), z: z2.unwrap_or(z)};
    let mut pos = Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
    let mut pos2 = Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
    ModelModule::joint_global_position_with_offset(module_accessor, Hash40::new_raw(bone), &offset, &mut pos, false);
    ModelModule::joint_global_position_with_offset(module_accessor, Hash40::new_raw(bone), &offset2, &mut pos2, false);
    // pos.x += x;
    // pos.y += y;
    // pos.z += z;
    // draw_hitbox(pos.x, pos.y, pos.z, pos2.x, pos2.y, pos2.z, size);
    // let size_mult = 19.0 / 200.0;

    // let x_dist: f32;
    // let y_dist: f32;
    // let z_dist: f32;
    // let mut n_effects: i32;
    // if x2 == None && y2 == None && z2 == None {
    //     // && let lib::L2CValueType::Void = y2.val_type && let lib::L2CValueType::Void = z2.val_type {  // extended hitbox
    //     x_dist = 0.0;
    //     y_dist = 0.0;
    //     z_dist = 0.0;
    //     n_effects = 1;
    // } else {
    //     // non-extended hitbox
    //     x_dist = x2.unwrap_or(0.0) - x;
    //     y_dist = y2.unwrap_or(0.0) - y;
    //     z_dist = z2.unwrap_or(0.0) - z;
    //     let dist_sq: f32 = x_dist * x_dist + y_dist * y_dist + z_dist * z_dist;
    //     let dist = dist_sq.sqrt();
    //     n_effects = ((dist / (size * 1.75)) + 1.0).ceil() as i32; // just enough effects to form a continuous line
    //     if n_effects < 2 {
    //         n_effects = 2;
    //     } else if n_effects > MAX_EFFECTS_PER_HITBOX {
    //         n_effects = MAX_EFFECTS_PER_HITBOX;
    //     }
    // }

    // for i in 0..n_effects {
    //     let mut t = 0.0;
    //     if n_effects > 1 {
    //         t = (i as f32) / ((n_effects - 1) as f32);
    //     }
    //     let x_curr = x + x_dist * t;
    //     let y_curr = y + y_dist * t;
    //     let z_curr = z + z_dist * t;

    //     let pos = Vector3f {
    //         x: x_curr,
    //         y: y_curr,
    //         z: z_curr,
    //     };
    //     let zeros = Vector3f {
    //         x: 0.0,
    //         y: 0.0,
    //         z: 0.0,
    //     };

    //     if false { // is_fighter(module_accessor) {
    //         EffectModule::req_on_joint(
    //             module_accessor,
    //             Hash40::new("sys_shield"),
    //             Hash40::new_raw(bone),
    //             &pos,
    //             &zeros,
    //             size * size_mult,
    //             &zeros,
    //             &zeros,
    //             true,
    //             *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32
    //                 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32
    //                 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32,
    //             0,
    //             0,
    //         );
    //     } else {
    //         EffectModule::req_follow(
    //             module_accessor,
    //             Hash40::new("sys_shield"),
    //             Hash40::new_raw(bone),
    //             &pos,
    //             &zeros,
    //             size * size_mult,
    //             true,
    //             *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32
    //                 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32
    //                 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32,
    //             0,
    //             0,
    //             0,
    //             0,
    //             true,
    //             true,
    //         );
    //     }

    //     // set to hitbox ID color
    //     EffectModule::set_rgb_partial_last(module_accessor, color.x, color.y, color.z);

    //     // speed up animation by rate to remove pulsing effect
    //     EffectModule::set_rate_last(module_accessor, 8.0);
    // }
}

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr();
    };
}

pub fn sys_line(fighter: &mut L2CFighterCommon) {
unsafe {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    // Resume Effect AnimCMD incase we don't display hitboxes
    MotionAnimcmdModule::set_sleep_effect(module_accessor, false);
    
    if DEBUG && !hdr::is_training_mode() {
        DEBUG = false;
    }

    if !hdr::is_training_mode() {
        enable_hitbox_vis(false);
        enable_hurtbox_vis(false);
        enable_special_vis(false);
    }

    if !DEBUG || !hdr::is_training_mode() {
        // stop_follow(module_accessor);
        return;
    }
    // request_follow(module_accessor);

    let status_kind = StatusModule::status_kind(module_accessor) as i32;
    if (*FIGHTER_STATUS_KIND_CATCH..=*FIGHTER_STATUS_KIND_CATCH_TURN).contains(&status_kind) {
        return;
    }

    if is_shielding(module_accessor) {
        return;
    }

    // Pause Effect AnimCMD if hitbox visualization is active
    // Keep effects on for missed tech effect
    MotionAnimcmdModule::set_sleep_effect(module_accessor, status_kind != FIGHTER_STATUS_KIND_DOWN);

    EffectModule::set_visible_kind(module_accessor, Hash40::new("sys_shield"), false);
    EffectModule::kill_kind(module_accessor, Hash40::new("sys_shield"), false, true);
    // for i in 0..8 {
    //     if !AttackModule::is_attack(module_accessor, i, false) {
    //         continue;
    //     }

    //     let attack_data = *AttackModule::attack_data(module_accessor, i, false);
    //     let is_capsule = attack_data.x2 != 0.0 || attack_data.y2 != 0.0 || attack_data.z2 != 0.0;
    //     let mut x2 = None;
    //     let mut y2 = None;
    //     let mut z2 = None;
    //     if is_capsule {
    //         x2 = Some(attack_data.x2);
    //         y2 = Some(attack_data.y2);
    //         z2 = Some(attack_data.z2);
    //     }
    //     generate_hitbox_effects(
    //         module_accessor,
    //         attack_data.node, // joint
    //         attack_data.size,
    //         attack_data.x,
    //         attack_data.y,
    //         attack_data.z,
    //         x2,
    //         y2,
    //         z2,
    //         ID_COLORS[(i % 8) as usize],
    //     );
    // }
}
}

unsafe fn disable_effect(lua_state: u64, state: bool) {
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    MotionAnimcmdModule::set_sleep_effect(module_accessor, state);
}


#[skyline::hook(replace = sv_animcmd::EFFECT_FOLLOW)]
unsafe fn handle_effect_follow(lua_state: u64) {
    disable_effect(lua_state, DEBUG);
    original!()(lua_state)
}

#[skyline::hook(replace = GrabModule::set_rebound)]
pub unsafe fn handle_set_rebound(
    module_accessor: *mut app::BattleObjectModuleAccessor,
    rebound: bool,
) {
    if !DEBUG || !hdr::is_training_mode() {
        original!()(module_accessor, rebound);
        return;
    }

    if rebound != false {
        original!()(module_accessor, rebound);
        return;
    }

    // only if we're not shielding
    if is_shielding(module_accessor) {
        original!()(module_accessor, rebound);
        return;
    }

    EffectModule::set_visible_kind(module_accessor, Hash40::new("sys_shield"), false);
    EffectModule::kill_kind(module_accessor, Hash40::new("sys_shield"), false, true);

    original!()(module_accessor, rebound);
}

pub fn install() {
    skyline::install_hooks!(handle_effect_follow, handle_set_rebound);
}

