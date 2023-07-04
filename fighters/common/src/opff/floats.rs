use super::*;
use smash::app::BattleObjectModuleAccessor;
use smash::lua2cpp::L2CFighterCommon;
use smash_script::macros::*;


// Robin, Dark Samus, Mewtwo float
pub unsafe fn extra_floats(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    let mut motion_value = 0.0;

    // Default to float option 0 upon match start/entry status kind
    if status_kind == *FIGHTER_STATUS_KIND_ENTRY {
        VarModule::set_int(boma.object(), vars::common::instance::FLOAT_STYLE, 0);
    }

    // Set the max float duration for the current character
    if fighter_kind == *FIGHTER_KIND_SAMUSD {
        VarModule::set_int(boma.object(), vars::common::instance::FLOAT_STYLE, 1);
        VarModule::set_int(boma.object(), vars::common::instance::FLOAT_DURATION,  50);
        motion_value = 1.0;
    }
    if fighter_kind == *FIGHTER_KIND_REFLET {
        VarModule::set_int(boma.object(), vars::common::instance::FLOAT_DURATION,999);
        motion_value = 0.0;
    }
    if fighter_kind == *FIGHTER_KIND_MEWTWO {
        VarModule::set_int(boma.object(), vars::common::instance::FLOAT_STYLE, 1);
        VarModule::set_int(boma.object(), vars::common::instance::FLOAT_DURATION, 60);
        motion_value = 1.0;
    }
    
    // Set float duration to 0F if not float option is selected
    if VarModule::get_int(boma.object(), vars::common::instance::FLOAT_STYLE) == 2 {
        VarModule::set_int(boma.object(), vars::common::instance::FLOAT_DURATION, 0);
    }
    // Activate float_pause aerial flag to prevent floats from being activated during aerials if
    // float option isnt 1
    if VarModule::get_int(boma.object(), vars::common::instance::FLOAT_STYLE) != 1 {
        if status_kind != *FIGHTER_STATUS_KIND_ATTACK_AIR {
            VarModule::set_flag(boma.object(), vars::common::instance::FLOAT_PAUSE_AERIAL, WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) == VarModule::get_int(boma.object(), vars::common::instance::FLOAT_DURATION));
        } else if WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY) {
            VarModule::off_flag(boma.object(), vars::common::instance::FLOAT_PAUSE_AERIAL);
        }
    }

    if situation_kind == *SITUATION_KIND_AIR
    && !boma.is_prev_status(*SITUATION_KIND_GROUND)
    //&& !StatusModule::is_changing(boma)
    {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY) {
            VarModule::on_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT);
        }
        // Prevent float from activating during aerials
        if VarModule::get_int(boma.object(), vars::common::instance::FLOAT_STYLE) != 1 && !WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY) {
            if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR && !CancelModule::is_enable_cancel(boma) {
                if VarModule::is_flag(boma.object(), vars::common::instance::FLOAT_PAUSE_AERIAL) {
                    VarModule::on_flag(boma.object(), vars::common::instance::AERIAL_NO_FLOAT);
                    WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME);
                }
            } else {
                if VarModule::is_flag(boma.object(), vars::common::instance::AERIAL_NO_FLOAT) {
                    WorkModule::set_int(boma, VarModule::get_int(boma.object(), vars::common::instance::FLOAT_DURATION), *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME);
                    VarModule::off_flag(boma.object(), vars::common::instance::AERIAL_NO_FLOAT);
                }
            }
        }
        // prevent Robin from activating float when out of Elwind magic
        if boma.kind() == *FIGHTER_KIND_REFLET
        && WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT) <= 0 {
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) > 0 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) < VarModule::get_int(boma.object(), vars::common::instance::FLOAT_DURATION) {
                app::FighterSpecializer_Reflet::set_flag_to_table(fighter.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
            WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY);
            return;
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
            if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUPERLEAF) 
            && !WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY)
            && !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP)
            {
                WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUPERLEAF);
            }
            // Immediately transition to fall/double jump fall when activating float
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) && boma.left_stick_y() < -0.66 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) > 0 {
                if [*FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3, *FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_PASS].contains(&status_kind) {
                    VarModule::on_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT);
                    ControlModule::reset_trigger(boma);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                } else if status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL {
                    VarModule::on_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT);
                    ControlModule::reset_trigger(boma);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
                } else if [hash40("walk_fall_l"), hash40("walk_fall_r"), hash40("run_fall_l"), hash40("run_fall_r")].contains(&MotionModule::motion_kind(boma)) {
                    MotionModule::change_motion(boma, Hash40::new("fall"), 0.0, 1.0, false, 0.0, false, false);
                } else if status_kind == *FIGHTER_STATUS_KIND_JUMP {
                    if StatusModule::is_changing(boma) { //peach ground-float mechanic
                        let pos = smash::phx::Vector3f { x: PostureModule::pos_x(boma), y: PostureModule::pos_y(boma) + 3.5, z: 0.0 };
                        PostureModule::set_pos(boma, &pos);
                    }
                    VarModule::on_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT);
                    ControlModule::reset_trigger(boma);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
                }
            }
            //else {
            //    // "superjump" bugfix
            //    if WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY) && status_kind == *FIGHTER_STATUS_KIND_JUMP && boma.status_frame() <= 1 {
            //        ControlModule::reset_trigger(boma);
            //        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_FALL, true);
            //    }
            //}
            // FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME will sometimes erroneously decrement once every 2 frames (effectively allowing for double the max float time); this overrides that
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) > 0 && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) < VarModule::get_int(boma.object(), vars::common::instance::FLOAT_DURATION) {
                if ([*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_STATUS_KIND_CLIFF_JUMP1, *FIGHTER_STATUS_KIND_CLIFF_JUMP2, *FIGHTER_STATUS_KIND_CLIFF_JUMP3].contains(&status_kind) || [*FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_KIND_FALL_AERIAL, *FIGHTER_STATUS_KIND_ATTACK_AIR].contains(&status_kind)) && !WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY) {
                    let fall_slowly_frame = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME);
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY);
                    VarModule::on_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT);
                    WorkModule::set_int(boma, fall_slowly_frame - 1, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME);
                }
            }

            // Omnidirectional float for Dark Samus and Mewtwo
            if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) > 0 
            && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) < VarModule::get_int(boma.object(), vars::common::instance::FLOAT_DURATION) 
            && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                if boma.left_stick_y() != 0.0 && VarModule::get_int(fighter.battle_object, vars::common::instance::FLOAT_TIMER) > 2 {
                    let mut motion_vec = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                    if boma.left_stick_y() > 0.0 {
                        motion_vec.y = motion_value;
                    } else {
                        motion_vec.y = -motion_value;
                    }
                    KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
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
            *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind) 
        || !WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY) 
        || boma.is_button_off(Buttons::Jump))
        && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) < VarModule::get_int(boma.object(), vars::common::instance::FLOAT_DURATION) 
        && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) >= 0 
        && !StatusModule::is_changing(boma)
        {
            WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME);
            WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY);
        }
    }

    // Reset Float Time
    if situation_kind == *SITUATION_KIND_GROUND
    || boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH])
    {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUPERLEAF);
        WorkModule::set_int(boma, VarModule::get_int(boma.object(), vars::common::instance::FLOAT_DURATION), *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME);
    }
}

pub unsafe fn float_effects(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, fighter_kind: i32) {
    if fighter_kind == *FIGHTER_KIND_SAMUSD || fighter_kind == *FIGHTER_KIND_MEWTWO {
        if VarModule::is_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT) {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::FLOAT_TIMER);
            let timer = VarModule::get_int(fighter.battle_object, vars::common::instance::FLOAT_TIMER);
            if timer == 1 {
                if fighter_kind == *FIGHTER_KIND_SAMUSD {
                    let pos1 = Vector3f{x: -2.0, y: 0.0, z: 0.0};
                    let pos2 = Vector3f{x: 2.0, y: 0.0, z: 0.5};
                    let pos3 = Vector3f{x: 0.0, y: 0.0, z: -0.5};
                    let pos4 = Vector3f{x: 2.0, y: 0.0, z: -0.5};
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), &pos1, &Vector3f::zero(), 2.5, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new_raw(0x09aee445d1), &pos2, &Vector3f::zero(), 2.0, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), &pos3, &Vector3f::zero(), 1.70000005, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), &Vector3f::zero(), &Vector3f::zero(), 2.0999999, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), &Vector3f::zero(), &Vector3f::zero(), 1.89999998, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), &Vector3f::zero(), &Vector3f::zero(), 2.0, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new_raw(0x0954eb78b2), &pos4, &Vector3f::zero(), 2.0, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), &Vector3f::zero(), &Vector3f::zero(), 1.70000005, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), &Vector3f::zero(), &Vector3f::zero(), 2.0999999, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), &Vector3f::zero(), &Vector3f::zero(), 1.89999998, true, 0, 0, 0, 0, 0, false, false);
                    EffectModule::req_follow(boma, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), &Vector3f::zero(), &Vector3f::zero(), 1.89999998, true, 0, 0, 0, 0, 0, false, false);
                }
                if fighter_kind == *FIGHTER_KIND_MEWTWO {
                    EffectModule::req_follow(boma, Hash40::new("mewtwo_final_aura"), Hash40::new("hip"), &Vector3f::zero(), &Vector3f::zero(), 1.25, true, 0, 0, 0, 0, 0, false, false);
                }
            } else if fighter_kind == *FIGHTER_KIND_MEWTWO {
                if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_SUPERLEAF_FALL_SLOWLY_FRAME) == 50  {
                // consume double jump on f10 of float
                    fighter.set_int(2, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                }
            }
        }
    }
    else {
        if WorkModule::is_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_FALL_SLOWLY) {
            VarModule::inc_int(fighter.battle_object, vars::common::instance::FLOAT_TIMER);
            let timer = VarModule::get_int(fighter.battle_object, vars::common::instance::FLOAT_TIMER);
            if boma.kind() == *FIGHTER_KIND_REFLET {
                if timer == 1 {
                    // Elwind book/UI visibility
                    let mut reflet_fighter = app::Fighter{battle_object: *(fighter.battle_object)};
                    app::FighterSpecializer_Reflet::change_hud_kind(&mut reflet_fighter, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND);
                    app::FighterSpecializer_Reflet::change_grimoire(fighter.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND);
                    WorkModule::set_int(boma, *FIGHTER_REFLET_MAGIC_KIND_EL_WIND, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);

                    // drain 3 bars of Elwind on float activation
                    let elwind_meter = WorkModule::get_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
                    let float_activation_spend = 2;
                    WorkModule::set_int(boma, elwind_meter - float_activation_spend, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);

                    // effects
                    EffectModule::req_follow(boma, Hash40::new("reflet_catch"), Hash40::new("top"), &Vector3f{x: 0.0, y: -6.0, z: -5.3}, &Vector3f::zero(), 0.7, true, 0, 0, 0, 0, 0, false, false);
                    LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.0);  // elwind green
                    EffectModule::req_follow(boma, Hash40::new("sys_aura_light"), Hash40::new("bookc"), &Vector3f::zero(), &Vector3f::zero(), 1.5, true, 0, 0, 0, 0, 0, false, false);
                    LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 0.078);  // elwind green
                }
                else if timer % 15 == 0 {  // every 15 frames
                    // drain 1 bar of Elwind
                    WorkModule::dec_int(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT);
                }
            }
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
        *FIGHTER_STATUS_KIND_DAMAGE_FALL].contains(&status_kind)) && VarModule::is_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT) {
        if fighter_kind == *FIGHTER_KIND_SAMUSD {
            EffectModule::kill_kind(boma, Hash40::new("samusd_win3_aura"), false, true);
        }
        if fighter_kind == *FIGHTER_KIND_MEWTWO {
            EffectModule::kill_kind(boma, Hash40::new("mewtwo_final_aura"), false, true);
        }
        if fighter_kind == *FIGHTER_KIND_REFLET {
            EffectModule::kill_kind(boma, Hash40::new("reflet_catch"), false, true);
            EffectModule::kill_kind(boma, Hash40::new("sys_aura_light"), false, true);
        }
        VarModule::off_flag(fighter.battle_object, vars::common::instance::OMNI_FLOAT);
        VarModule::set_int(fighter.battle_object, vars::common::instance::FLOAT_TIMER, 0);
    }
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    if [*FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_REFLET].contains(&fighter_kind) {
        extra_floats(fighter, boma, cat[0], status_kind, situation_kind, fighter_kind, stick_x, stick_y, facing);
        float_effects(fighter, boma, cat[0], status_kind, situation_kind, fighter_kind);
    }
}
