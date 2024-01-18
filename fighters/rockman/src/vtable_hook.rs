use super::{vl, *};
use smash_rs::app::{WorkId, work_ids, transition_groups, transition_terms};

#[skyline::hook(offset = 0x107e950)]
pub unsafe extern "C" fn rockman_vtable_func(vtable: u64, fighter: &mut smash::app::Fighter) {
    let object = &mut fighter.battle_object;
    let module_accessor = object.module_accessor;
    let status = StatusModule::status_kind(module_accessor);
    let battle_object_slow = singletons::BattleObjectSlow() as *mut u8;
    let is_not_slow = (*battle_object_slow.add(0x8) == 0 || *(battle_object_slow as *const u32) == 0)
    && !StopModule::is_stop(module_accessor) && !SlowModule::is_skip(module_accessor);
    if is_not_slow {
        if ![
            *FIGHTER_STATUS_KIND_SPECIAL_LW,
            *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_LW_SHOOT
        ].contains(&status)
        && WorkModule::is_flag(module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD) {
            WorkModule::dec_int(module_accessor, 0x100000c3); // FIGHTER_ROCKMAN_INSTANCE_WORK_ID_INT_SPECIAL_LW_HOLD_FRAME
            if WorkModule::is_flag(module_accessor, 0x200000e1) { // FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_ENABLE_SHOOT
                if WorkModule::get_int(module_accessor, 0x100000c3) <= 0 {
                    LinkModule::send_event_nodes(
                        module_accessor,
                        *LINK_NO_ARTICLE,
                        Hash40::new_raw(0x2435e7c874),
                        0
                    );
                    ArticleModule::remove(module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
                    FighterSpecializer_Rockman::set_leafshield(module_accessor, false);
                }
                else if WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)
                && ControlModule::get_button(module_accessor) >> 1 & 1 != 0 {
                    StatusModule::change_status_request(module_accessor, *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_LW_SHOOT, true);
                }
            }
        }

        // New stuff for Charge Shot

        if !rockman_valid_charging_state(module_accessor)
        || WorkModule::is_flag(module_accessor, *FIGHTER_ROCKMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_LEAFSHIELD) {
            rockman_kill_charge(module_accessor, object);
        }
        else if !VarModule::is_flag(object, vars::rockman::instance::CHARGE_SHOT_CHARGING) {
            if ControlModule::get_button(module_accessor) >> 1 & 1 != 0 {
                VarModule::on_flag(object, vars::rockman::instance::CHARGE_SHOT_CHARGING);
            }
        }
        else {
            if ControlModule::get_button(module_accessor) >> 1 & 1 == 0 {
                if !VarModule::is_flag(object, vars::rockman::instance::CHARGE_SHOT_PLAYED_FX) {
                    rockman_kill_charge(module_accessor, object);
                }
                else if !VarModule::is_flag(object, vars::rockman::instance::CHARGE_SHOT_RELEASE) {
                    VarModule::set_int(object, vars::rockman::instance::CHARGE_SHOT_RELEASE_FRAME, vl::private::CHARGE_SHOT_RELEASE_FRAME);
                    VarModule::on_flag(object, vars::rockman::instance::CHARGE_SHOT_RELEASE);
                }
            }
            
            if VarModule::get_int(object, vars::rockman::instance::CHARGE_SHOT_RELEASE_FRAME) >= 0 {
                VarModule::dec_int(object, vars::rockman::instance::CHARGE_SHOT_RELEASE_FRAME);
            }
            if VarModule::is_flag(object, vars::rockman::instance::CHARGE_SHOT_PLAYED_FX)
            && (
                VarModule::is_flag(object, vars::rockman::instance::CHARGE_SHOT_RELEASE)
                && VarModule::get_int(object, vars::rockman::instance::CHARGE_SHOT_RELEASE_FRAME) == 0
            ) {
                rockman_kill_charge(module_accessor, object);
            }
        }
        if VarModule::is_flag(object, vars::rockman::instance::CHARGE_SHOT_CHARGING) {
            let charge_frame = VarModule::get_int(object, vars::rockman::instance::CHARGE_SHOT_FRAME);
            if charge_frame < vl::private::CHARGE_SHOT_MAX_FRAME + 1 {
                VarModule::inc_int(object, vars::rockman::instance::CHARGE_SHOT_FRAME);
            }
            let charge_frame = VarModule::get_int(object, vars::rockman::instance::CHARGE_SHOT_FRAME);
            if charge_frame == vl::private::CHARGE_SHOT_MAX_FRAME {
                FighterUtil::flash_eye_info(module_accessor);
                EffectModule::req_follow(
                    module_accessor,
                    Hash40::new("rockman_chargeshot_max"),
                    Hash40::new("hip"),
                    &Vector3f::zero(),
                    &Vector3f::zero(),
                    0.75,
                    false,
                    0,
                    0,
                    0,
                    0,
                    0,
                    false,
                    false
                );
            }
            if charge_frame == vl::private::CHARGE_SHOT_CLEAR_INPUT_FRAME {
                ControlModule::clear_command_one(module_accessor, 0, *FIGHTER_PAD_CMD_CAT1_SPECIAL_N);
            }
            if charge_frame > vl::private::CHARGE_SHOT_DELAY_CHARGE_FRAME {
                if !VarModule::is_flag(object, vars::rockman::instance::CHARGE_SHOT_PLAYED_FX) {
                    SoundModule::play_se(module_accessor, Hash40::new("se_rockman_smash_s02"), true, false, false, false, enSEType(0));
                    EffectModule::req_follow(
                        module_accessor,
                        Hash40::new("rockman_chargeshot_hold"),
                        Hash40::new("handl"),
                        &Vector3f{x: 1.0, y: 0.0, z: 0.0},
                        &Vector3f::zero(),
                        0.4,
                        false,
                        0,
                        0,
                        0,
                        0,
                        0,
                        false,
                        false
                    );
                    let eff_handle = EffectModule::get_last_handle(module_accessor) as u32;
                    VarModule::set_int(object, vars::rockman::instance::CHARGE_SHOT_EFF_HANDLE, eff_handle as i32);
                    VarModule::on_flag(object, vars::rockman::instance::CHARGE_SHOT_PLAYED_FX);
                }
            }
        }
    }
    original!()(vtable, fighter);
}

// pub unsafe fn is_damage_check(module_accessor: *mut BattleObjectModuleAccessor, is_prev: bool) -> bool {
//     let status : i32;
//     if is_prev {
//         status = StatusModule::prev_status_kind(module_accessor, 0);
//     }
//     else {
//         status = StatusModule::status_kind(module_accessor);
//     }
//     if FighterStopModuleImpl::is_damage_stop(module_accessor)
//     || WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CAPTURE_YOSHI)
//     || WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_GROUND)
//     || WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_GANON_SPECIAL_S_DAMAGE_FALL_AIR)
//     || (*FIGHTER_STATUS_KIND_CAPTURE_PULLED..=*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status)
//     || (*FIGHTER_STATUS_KIND_DOWN..=*FIGHTER_STATUS_KIND_LAY_DOWN).contains(&status)
//     || (*FIGHTER_STATUS_KIND_DOWN_DAMAGE..=*FIGHTER_STATUS_KIND_BIND).contains(&status)
//     || (*FIGHTER_STATUS_KIND_SLIP..=*FIGHTER_STATUS_KIND_SLIP_WAIT).contains(&status)
//     || (*FIGHTER_STATUS_KIND_TREAD_DAMAGE..=*FIGHTER_STATUS_KIND_ICE_JUMP).contains(&status)
//     || (*FIGHTER_STATUS_KIND_LINK_FINAL..=*FIGHTER_STATUS_KIND_PIT_FALL).contains(&status)
//     || (*FIGHTER_STATUS_KIND_SWALLOWED..=*FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI).contains(&status)
//     || (*FIGHTER_STATUS_KIND_CATCHED_REFLET..=*FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND).contains(&status)
//     || status == *FIGHTER_STATUS_KIND_GIMMICK_EATEN
//     || (*FIGHTER_STATUS_KIND_CAPTURE_ITEM..=*FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP).contains(&status)
//     || (*FIGHTER_STATUS_KIND_FINAL_VISUAL_ATTACK_OTHER..=*FIGHTER_STATUS_KIND_RIDLEY_FINAL_TARGET_END).contains(&status)
//     || (*FIGHTER_STATUS_KIND_CATCHED_RIDLEY..=*FIGHTER_STATUS_KIND_STABBED_DAMAGE).contains(&status)
//     || (*FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED..=*FIGHTER_STATUS_KIND_SWING_GAOGAEN_FAILURE).contains(&status)
//     || (*FIGHTER_STATUS_KIND_SHEIK_FINAL_CAPTURE..=*FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS).contains(&status)
//     || (*FIGHTER_STATUS_KIND_SIMON_FINAL_TARGET_START..=*FIGHTER_STATUS_KIND_YOSHI_FINAL_TARGET_END).contains(&status)
//     || (*FIGHTER_STATUS_KIND_SUICIDE_BOMB..=*FIGHTER_STATUS_KIND_TANTAN_FINAL_TARGET_END).contains(&status)
//     || (*FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD..=*FIGHTER_STATUS_KIND_EDGE_FINAL_TARGET_END).contains(&status)
//     || (*FIGHTER_STATUS_KIND_CAPTURE_TRAIL_KEYHOLE..=*FIGHTER_STATUS_KIND_TRAIL_FINAL_TARGET_END).contains(&status) {
//         return true;
//     }
//     false
// }

unsafe fn rockman_valid_charging_state(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    if WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N) {
        return true;
    }
    // if is_damage_check(module_accessor, false) {
    //     return false;
    // }
    let status = StatusModule::status_kind(module_accessor);
    ![
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_MISS_FOOT,
        *FIGHTER_STATUS_KIND_STANDBY,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_STATUS_KIND_FINAL,
        *FIGHTER_ROCKMAN_STATUS_KIND_SPECIAL_LW_SHOOT,
        *FIGHTER_ROCKMAN_STATUS_KIND_FINAL_INHALE,
        *FIGHTER_ROCKMAN_STATUS_KIND_FINAL_SCENE01,
        *FIGHTER_ROCKMAN_STATUS_KIND_FINAL_SCENE02,
        *FIGHTER_ROCKMAN_STATUS_KIND_FINAL_END
    ].contains(&status)
}

unsafe fn rockman_kill_charge(module_accessor: *mut BattleObjectModuleAccessor, object: *mut BattleObject) {
    VarModule::off_flag(object, vars::rockman::instance::CHARGE_SHOT_CHARGING);
    VarModule::off_flag(object, vars::rockman::instance::CHARGE_SHOT_PLAYED_FX);
    VarModule::off_flag(object, vars::rockman::instance::CHARGE_SHOT_RELEASE);
    VarModule::set_int(object, vars::rockman::instance::CHARGE_SHOT_RELEASE_FRAME, 0);
    SoundModule::stop_se(module_accessor, Hash40::new("se_rockman_smash_s02"), 0);
    VarModule::set_int(object, vars::rockman::instance::CHARGE_SHOT_FRAME, 0);
    let eff_handle = VarModule::get_int(object, vars::rockman::instance::CHARGE_SHOT_EFF_HANDLE) as u32;
    if EffectModule::is_exist_effect(module_accessor, eff_handle) {
        EffectModule::kill(module_accessor, eff_handle, true, true);
    }
}

#[skyline::hook(offset = 0x1083bcc, inline)]
unsafe fn rockman_do_leafshield_things_disable(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *ctx.registers[19].x.as_ref() as *mut BattleObjectModuleAccessor;
    FighterSpecializer_Rockman::set_leafshield(module_accessor, false);
}

#[skyline::hook(offset = 0x10838c0, inline)]
unsafe fn rockman_do_leafshield_things_enable(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *ctx.registers[19].x.as_ref() as *mut BattleObjectModuleAccessor;
    FighterSpecializer_Rockman::set_leafshield(module_accessor, true);
}

const LEAFSHIELD_DISABLE_GROUPS: [WorkId; 5] = [
    // transition_groups::CHECK_GROUND_SPECIAL,
    // transition_groups::CHECK_AIR_SPECIAL,
    transition_groups::CHECK_GROUND_ESCAPE,
    // transition_groups::CHECK_AIR_ESCAPE,
    transition_groups::CHECK_GROUND_GUARD,
    transition_groups::CHECK_GROUND_ATTACK,
    // transition_groups::CHECK_GROUND_CATCH,
    transition_groups::CHECK_AIR_ATTACK,
    transition_groups::CHECK_AIR_CLIFF
];

const LEAFSHIELD_DISABLE_INDIVI: [WorkId; 8] = [
    // transition_terms::CONT_DASH,
    // transition_terms::CONT_TURN_DASH,
    transition_terms::CONT_ATTACK_DASH,
    transition_terms::CONT_CATCH_DASH,
    transition_terms::CONT_CATCH_TURN,
    // transition_terms::CONT_ATTACK_HI4_START,
    // transition_terms::CONT_ATTACK_LW4_START,
    transition_terms::CONT_SPECIAL_N,
    transition_terms::CONT_SPECIAL_S,
    transition_terms::CONT_SPECIAL_HI,
    transition_terms::CONT_CLIFF_ATTACK,
    transition_terms::CONT_CLIFF_ESCAPE
];

#[skyline::hook(replace = FighterSpecializer_Rockman::set_leafshield)]
unsafe extern "C" fn set_leafshield(module_accessor: *mut smash_rs::app::BattleObjectModuleAccessor, set_shield: bool) {
    let work = (*module_accessor).work();
    work.set_flag(set_shield, work_ids::fighter::rockman::instance::SPECIAL_LW_LEAFSHIELD);
    work.set_flag(set_shield, work_ids::fighter::rockman::instance::SPECIAL_LW_ENABLE_SHOOT);
    if !set_shield {
        work.set_int(0, work_ids::fighter::rockman::instance::SPECIAL_LW_HOLD_FRAME);
        for x in LEAFSHIELD_DISABLE_GROUPS.iter() {
            work.unable_transition_term_forbid_group_indivi(*x);
        }
        for x in LEAFSHIELD_DISABLE_INDIVI.iter() {
            work.unable_transition_term_forbid_indivi(*x);
        }
        // work.enable_transition_term_forbid(transition_terms::CONT_SPECIAL_LW);
        // if (*module_accessor).status().status_kind() < 0x27 {
        //     for x in LEAFSHIELD_DISABLE_GROUPS.iter() {
        //         work.enable_transition_term_group(*x);
        //     }
        //     for x in LEAFSHIELD_DISABLE_INDIVI.iter() {
        //         work.enable_transition_term(*x);
        //     }
        // }
    }
    else {
        let hold_frame = work.get_param_int(smash_rs::phx::Hash40::new("param_special_lw"), smash_rs::phx::Hash40::new("hold_frame"));
        work.set_int(hold_frame, work_ids::fighter::rockman::instance::SPECIAL_LW_HOLD_FRAME);
        for x in LEAFSHIELD_DISABLE_GROUPS.iter() {
            work.enable_transition_term_forbid_group_indivi(*x);
        }
        for x in LEAFSHIELD_DISABLE_INDIVI.iter() {
            work.enable_transition_term_forbid_indivi(*x);
        }
    }
}

// #[skyline::hook(offset = 0x1085d40)]
// pub unsafe extern "C" fn rockman_airshooter_init(article: &mut smash::app::Article, fighter: &mut smash::app::FighterModuleAccessor) -> u64 {
//     let ret = original!()(article, fighter);
//     println!("Initializing Air Shooter");
//     let object_id = fighter.battle_object_module_accessor.battle_object_id;
//     let object = MiscModule::get_battle_object_from_id(object_id);
//     let num = VarModule::get_int(object, vars::rockman::status::AIR_SHOOTER_NUM);
//     VarModule::set_int(&mut article.battle_object as *mut BattleObject, rockman_airshooter::instance::int::NUM, num);
//     ret
// }

pub fn install(is_runtime: bool) {
    if is_runtime {
        return;
    }
    // Forces the original Leaf Shield handler to not run so we can run the custom one.
    skyline::patching::Patch::in_text(0x107ea84).data(0x1400001Eu32);
    // Removes the check that forces the removal of Leaf Shield if you are not within certain statuses.
    skyline::patching::Patch::in_text(0x107ff4c).data(0x14000007u32);

    // Disable's the manual checks so it can use FighterSpecializer_Rockman::is_leafshield instead.
    // Disable
    skyline::patching::Patch::in_text(0x1083bcc).nop();
    skyline::patching::Patch::in_text(0x1083bec).nop();
    skyline::patching::Patch::in_text(0x1083c08).nop();
    skyline::patching::Patch::in_text(0x1083c1c).nop();
    skyline::patching::Patch::in_text(0x1083c30).nop();
    skyline::patching::Patch::in_text(0x1083c4c).nop();
    skyline::patching::Patch::in_text(0x1083c60).nop();
    skyline::patching::Patch::in_text(0x1083c74).nop();
    skyline::patching::Patch::in_text(0x1083c88).nop();
    skyline::patching::Patch::in_text(0x1083c9c).nop();
    skyline::patching::Patch::in_text(0x1083cb0).nop();
    skyline::patching::Patch::in_text(0x1083cc4).nop();
    // Enable
    skyline::patching::Patch::in_text(0x10838c0).nop();
    skyline::patching::Patch::in_text(0x10838e0).nop();
    skyline::patching::Patch::in_text(0x1083908).nop();
    skyline::patching::Patch::in_text(0x1083924).nop();
    skyline::patching::Patch::in_text(0x1083938).nop();
    skyline::patching::Patch::in_text(0x108394c).nop();
    skyline::patching::Patch::in_text(0x1083968).nop();
    skyline::patching::Patch::in_text(0x108397c).nop();
    skyline::patching::Patch::in_text(0x1083990).nop();
    skyline::patching::Patch::in_text(0x10839a4).nop();
    skyline::patching::Patch::in_text(0x10839b8).nop();
    skyline::patching::Patch::in_text(0x10839cc).nop();

    // Patches which status to compare to for Metal Blade.
    skyline::patching::Patch::in_text(0x1080264).data(0x7107741Fu32);

    skyline::install_hooks!(
        rockman_vtable_func,
        rockman_do_leafshield_things_disable,
        rockman_do_leafshield_things_enable,
        set_leafshield,
        // rockman_airshooter_init
    );
}