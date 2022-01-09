use smash::app::BattleObjectModuleAccessor;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::{Vector2f, Vector3f, Vector4f};
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::phx::Hash40;
use smash::hash40;
use smash_script::macros::*;

use hdr_modules::consts::{*, globals::*};
use hdr_modules::*;

use crate::utils::hdr;
use crate::vars::*;

// Graphical routines for flashing upon changing float styles; repurposes meter glow stuff
pub unsafe fn float_flash(boma: &mut BattleObjectModuleAccessor, float_option: i32) {
    let mut f_red   = 0.0;
    let mut f_green = 0.0;
    let mut f_blue  = 0.0;

    if float_option == 0 {
        f_red = 0.65;
        f_green = 0.25;
        f_blue = 0.65;
    } else if float_option == 1 {
        f_red = 0.22;
        f_green = 0.7;
        f_blue = 0.22;
    } else {
        f_red = 0.22;
        f_green = 0.22;
        f_blue = 0.22;
    }

    let cbm_vec1 = Vector4f{x: f_red, y: f_green, z: f_blue, w: 0.72};
    let cbm_vec2 = Vector4f{x: f_red, y: f_green, z: f_blue, w: 0.05};
    ColorBlendModule::set_main_color(boma, &cbm_vec1, &cbm_vec2, 1.0, 0.5, 7, true);

    meter_gain_glow_timer[hdr::get_player_number(boma)] = 1;

    let pos_meter1 = Vector3f{x: 0.0, y: 3.0, z: 0.0};
    let pos_meter2 = Vector3f{x: 0.0, y: 1.0, z: 0.0};
    let rot2 = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let random_vec2 = Vector3f{x: 0.1, y: 0.1, z: 0.5};
    EffectModule::kill_kind(boma, Hash40::new("sys_damage_curse"), false, true);
    EffectModule::kill_kind(boma, Hash40::new("sys_v_smoke_a"), false, true);
    EffectModule::req_on_joint(boma, Hash40::new("sys_damage_curse"), Hash40::new("top"), &pos_meter1, &rot2, 0.5, &random_vec2, &random_vec2, false, 0, 0, 0);
    EffectModule::req_on_joint(boma, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), &pos_meter2, &rot2, 0.5, &random_vec2, &random_vec2, false, 0, 0, 0);
}

// Ganondorf, Robin, Dark Samus, Mewtwo float
pub unsafe fn extra_floats(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    let id = hdr::get_player_number(boma);
    let mut motion_value = 0.0;

    // Default to float option 0 upon match start/entry status kind
    if status_kind == *FIGHTER_STATUS_KIND_ENTRY {
        float_style[id] = 0;
    }

    // Choose float style via taunting and shield + directional input
    if status_kind == *FIGHTER_STATUS_KIND_APPEAL && MotionModule::frame(boma) < 50.0 {
        let guard_on = ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD);
        if guard_on && stick_y > 0.5 {
            float_style[id] = 0;
            float_flash(boma, 0);
            CancelModule::enable_cancel(boma);
        }
        if guard_on && stick_y < -0.5 {
            float_style[id] = 1;
            float_flash(boma, 1);
            CancelModule::enable_cancel(boma);
        }
        if guard_on && (stick_x < -0.5 || stick_x > 0.5) {
            float_style[id] = 2;
            float_flash(boma, 2);
            CancelModule::enable_cancel(boma);
        }
    }

    // Choose float style on respawn platform with respective taunt button
    if status_kind == *FIGHTER_STATUS_KIND_REBIRTH {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            float_style[id] = 0;
            float_flash(boma, 0);
        }
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW) {
            float_style[id] = 1;
            float_flash(boma, 1);
        }
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
            float_style[id] = 2;
            float_flash(boma, 2);
        }
    }

    // Track double jump frame for float leniency window
    if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
        double_jump_frame[id] = MotionModule::frame(boma);
    }

    // Set the max float duration for the current character
    if fighter_kind == *FIGHTER_KIND_SAMUSD {
        float_duration[id] = 50;
        motion_value = 1.0;
    }
    if fighter_kind == *FIGHTER_KIND_REFLET {
        float_duration[id] = 60;
        motion_value = 0.0;
    }
    if fighter_kind == *FIGHTER_KIND_GANON {
        float_duration[id] = 60;
        motion_value = 0.0;
    }
    if fighter_kind == *FIGHTER_KIND_MEWTWO {
        float_duration[id] = 60;
        motion_value = 1.0;
    }

    // Set float duration to 0F if not float option is selected
    if float_style[id] == 2 {
        float_duration[id] = 0;
    }

    // Activate float_pause aerial flag to prevent floats from being activated during aerials if
    // float option isnt 1
    if float_style[id] != 1 {
        if status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR {
            float_pause_aerial[id] = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) == float_duration[id];
        }
    }

    if situation_kind == *SITUATION_KIND_AIR {

        if WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY) {
            VarModule::on_flag(fighter.module_accessor, common::OMNI_FLOAT);
        }
        // Prevent float from activating during aerials
        if float_style[id] != 1 {
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
                if float_pause_aerial[id] {
                    aerial_no_float[id] = true;
                    WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME);
                }
            } else {
                if aerial_no_float[id] {
                    WorkModule::set_int(boma, float_duration[id], *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME);
                    aerial_no_float[id] = false;
                }
            }
        }

        // prevent float from activating during ledge jump
        if [*FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3].contains(&status_kind) {
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUPERLEAF);
        }

        if [*FIGHTER_STATUS_KIND_FALL,
            *FIGHTER_STATUS_KIND_FALL_AERIAL,
            *FIGHTER_STATUS_KIND_JUMP,
            *FIGHTER_STATUS_KIND_JUMP_AERIAL,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP3,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_PASS].contains(&status_kind) {
            // Activate float midair
            if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUPERLEAF) {
                WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUPERLEAF);
            }
            // Immediately transition to fall/double jump fall when activating float
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) && stick_y < -0.66 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) > 0 {
                if [*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_PASS].contains(&status_kind) {
                    VarModule::on_flag(fighter.module_accessor, common::OMNI_FLOAT);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                } else if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                    // Return double jump within leniency window
                    if double_jump_frame[id] <= 2.0 {
                        if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
                            WorkModule::set_int(boma, WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) - 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                        }
                    }
                    VarModule::on_flag(fighter.module_accessor, common::OMNI_FLOAT);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
                } else if [hash40("walk_fall_l"), hash40("walk_fall_r"), hash40("run_fall_l"), hash40("run_fall_r")].contains(&MotionModule::motion_kind(boma)) {
                    MotionModule::change_motion(boma, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
                }
            }
            else {
                // "superjump" bugfix
                if WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY) && status_kind == *FIGHTER_STATUS_KIND_JUMP && MotionModule::frame(boma) <= 1.0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }

            // FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME will sometimes erroneously decrement once every 2 frames (effectively allowing for double the max float time); this overrides that
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) > 0 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) < float_duration[id] {
                if (([*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3].contains(&status_kind) && MotionModule::frame(boma) > 1.0) || [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind)) && !WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY) {
                    let fall_slowly_frame = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME);
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY);
                    VarModule::on_flag(fighter.module_accessor, common::OMNI_FLOAT);
                    WorkModule::set_int(boma, fall_slowly_frame - 1, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME);
                }
            }

            // Omnidirectional float for Dark Samus and Mewtwo
            if fighter_kind == *FIGHTER_KIND_SAMUSD || fighter_kind == *FIGHTER_KIND_MEWTWO {
                if ([*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL].contains(&status_kind) && MotionModule::frame(boma) > 3.0) // Omnidirection float only after F3 of initiating the float
                    || (status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && (WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) < float_duration[id] && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) > 0)) /* If in aerial attack and float has been initiated and are current floating */ {
                    if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) > 0 {
                        if stick_y != 0.0 {
                            if !VarModule::is_flag(fighter.module_accessor, common::OMNI_FLOAT) { VarModule::on_flag(fighter.module_accessor, common::OMNI_FLOAT); }
                            let mut motion_vec = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                            if stick_y > 0.0 {
                                motion_vec.y = motion_value;
                            } else {
                                motion_vec.y = -motion_value;
                            }
                            KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
                        }
                    }
                }
            }
        }

        // One float per airtime
        if (![*FIGHTER_STATUS_KIND_FALL,
            *FIGHTER_STATUS_KIND_FALL_AERIAL,
            *FIGHTER_STATUS_KIND_JUMP,
            *FIGHTER_STATUS_KIND_JUMP_AERIAL,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
            *FIGHTER_STATUS_KIND_CLIFF_JUMP3,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind) || !WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY) || !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) < float_duration[id] {
            WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY);
        }

    }

    // Reset Float Time
    if situation_kind == *SITUATION_KIND_GROUND {
        WorkModule::set_int(boma, float_duration[id], *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME);
    }
    
}

pub unsafe fn float_effects(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if fighter_kind == *FIGHTER_KIND_SAMUSD || fighter_kind == *FIGHTER_KIND_MEWTWO {
        if VarModule::is_flag(fighter.module_accessor, common::OMNI_FLOAT) {
            let mut timer = VarModule::get_int(fighter.module_accessor, common::FLOAT_TIMER);
            if timer == 1 {
                if fighter_kind == *FIGHTER_KIND_SAMUSD {
                    let pos1 = Vector3f{x: -2.0, y: 0.0, z: 0.0};
                    let pos2 = Vector3f{x: 2.0, y: 0.0, z: 0.5};
                    let pos3 = Vector3f{x: 0.0, y: 0.0, z: -0.5};
                    let pos4 = Vector3f{x: 2.0, y: 0.0, z: -0.5};
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), &pos1, &hdr::DEFAULT_VECTOR3, 2.5, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new_raw(0x09aee445d1), &pos2, &hdr::DEFAULT_VECTOR3, 2.0, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), &pos3, &hdr::DEFAULT_VECTOR3, 1.70000005, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 2.0999999, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 1.89999998, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 2.0, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new_raw(0x0954eb78b2), &pos4, &hdr::DEFAULT_VECTOR3, 2.0, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 1.70000005, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 2.0999999, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 1.89999998, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 1.89999998, true, 0, 0, 0, 0, 0, false, false);
                }
                if fighter_kind == *FIGHTER_KIND_MEWTWO {
                    EffectModule::req_follow(boma, Hash40::new("mewtwo_final_aura"), Hash40::new("hip"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 1.25, true, 0, 0, 0, 0, 0, false, false);
                }
            }
            VarModule::set_int(fighter.module_accessor, common::FLOAT_TIMER, timer + 1);
        }
    }
    else {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY) {
            let mut timer = VarModule::get_int(fighter.module_accessor, common::FLOAT_TIMER);

            if timer == 1 {
                if fighter_kind == *FIGHTER_KIND_GANON {
                    EffectModule::req_follow(boma, Hash40::new("ganon_entry_aura"), Hash40::new("kneer"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 0.75, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("ganon_entry_aura"), Hash40::new("footr"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 0.75, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("ganon_entry_aura"), Hash40::new("kneel"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 0.75, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("ganon_entry_aura"), Hash40::new("footl"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 0.75, true, 0, 0, 0, 0, 0, false, false);
                }
                if fighter_kind == *FIGHTER_KIND_REFLET {
                    EffectModule::req_follow(boma, Hash40::new("reflet_catch"), Hash40::new("top"), &Vector3f{x: 0.0, y: -6.0, z: -5.3}, &hdr::DEFAULT_VECTOR3, 0.7, true, 0, 0, 0, 0, 0, false, false);
                    LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.0);  // elwind green
                    EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("bookc"), &hdr::DEFAULT_VECTOR3, &hdr::DEFAULT_VECTOR3, 1.5, true, 0, 0, 0, 0, 0, false, false);
                    LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.078);  // elwind green
                }
            }
            VarModule::set_int(fighter.module_accessor, common::FLOAT_TIMER, timer + 1);
        }
    }

    if (situation_kind == *SITUATION_KIND_GROUND || WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) == 0
    || ![*FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL,
        *FIGHTER_STATUS_KIND_JUMP,
        *FIGHTER_STATUS_KIND_JUMP_AERIAL,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP1,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP2,
        *FIGHTER_STATUS_KIND_CLIFF_JUMP3,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind)) && VarModule::is_flag(fighter.module_accessor, common::OMNI_FLOAT) {
        if fighter_kind == *FIGHTER_KIND_SAMUSD {
            EffectModule::kill_kind(boma, Hash40::new("samusd_win3_aura"), false, true);
        }
        if fighter_kind == *FIGHTER_KIND_MEWTWO {
            EffectModule::kill_kind(boma, Hash40::new("mewtwo_final_aura"), false, true);
        }
        if fighter_kind == *FIGHTER_KIND_GANON {
            EffectModule::kill_kind(boma, Hash40::new("ganon_entry_aura"), false, true);
        }
        if fighter_kind == *FIGHTER_KIND_REFLET {
            EffectModule::kill_kind(boma, Hash40::new("reflet_catch"), false, true);
            EffectModule::kill_kind(boma, Hash40::new("sys_aura_light"), false, true);
        }
        VarModule::off_flag(fighter.module_accessor, common::OMNI_FLOAT);
        VarModule::set_int(fighter.module_accessor, common::FLOAT_TIMER, 0);
    }
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    if [*FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_GANON, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_REFLET].contains(&fighter_kind) {
        extra_floats(fighter, boma, cat[0], status_kind, situation_kind, fighter_kind, stick_x, stick_y, facing);
        float_effects(fighter, boma, cat[0], status_kind, situation_kind, fighter_kind);
    }
}
