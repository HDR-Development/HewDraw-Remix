use super::*;

unsafe extern "C" fn special_lw_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if !weapon.is_flag(*WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_FLAG_OUTFIELD_INVISIBLE) {
        VisibilityModule::set_whole(weapon.module_accessor, true);
    }
    let ptrainer = weapon.global_table[0x4].get_ptr() as *mut smash::app::Weapon;
    if smash::app::WeaponSpecializer_PTrainerPTrainer::request_change_pokemon(ptrainer) != 0 {
        weapon.set_int(*FIGHTER_COMMON_START_KIND_CHANGE, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_START_KIND);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("p_change"), 0.0, 1.0, false, 0.0, false, false);
        weapon.on_flag(*WEAPON_PTRAINER_PTRAINER_STATUS_WORK_FLAG_ON_CHANGE);
        sub_special_lw(weapon);
    }
    else {
        if !weapon.is_motion(Hash40::new("hold")) {
            MotionModule::change_motion(weapon.module_accessor, Hash40::new("hold"), 0.0, 1.0, false, 0.0, false, false);
        }
    }

    weapon.fastshift(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn sub_special_lw(weapon: &mut L2CWeaponCommon) {
    if !weapon.is_situation(*SITUATION_KIND_OUTFIELD) {
        if !weapon.is_flag(*WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_FLAG_MBALL_UPPER) {
            CameraModule::set_whole(weapon.module_accessor, true);
        }
    }
}

unsafe extern "C" fn special_lw_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if MotionModule::is_end(weapon.module_accessor) {
        weapon.change_status(WEAPON_PTRAINER_PTRAINER_STATUS_KIND_WAIT.into(), false.into());
        return 0.into();
    }
    if weapon.is_flag(*WEAPON_PTRAINER_PTRAINER_STATUS_WORK_FLAG_VOICE) {
        // move back to frame 11 to account for backward switch
        if weapon.status_frame() == 11 {
            if LinkModule::is_link(weapon.module_accessor, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) {
                let poke_parent_id = LinkModule::get_parent_object_id(weapon.module_accessor, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) as u32;
                let poke_object = utils::util::get_battle_object_from_id(poke_parent_id);
                if !poke_object.is_null() {
                    let poke_boma = &mut *(*poke_object).module_accessor;
                    if !ControlModule::check_button_on(poke_boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                        VarModule::on_flag(weapon.battle_object, vars::ptrainer::status::VOICE_FORWARD_SWITCH);
                    }
                }
            }
            play_voice(weapon);
        }
    }
    if VarModule::is_flag(weapon.battle_object, vars::ptrainer::status::CONTINUE_VOICE) {
        let voice_hash = weapon.get_int64(*WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_VOICE_ID);
        if !SoundModule::is_playing(weapon.module_accessor, voice_hash.as_hash40()) {
            play_voice_continue(
                weapon,
                VarModule::is_flag(weapon.battle_object, vars::ptrainer::status::VOICE_FORWARD_SWITCH),
                VarModule::is_flag(weapon.battle_object, vars::ptrainer::status::VOICE_USE_OUT_POKE_KIND)
            );
            VarModule::off_flag(weapon.battle_object, vars::ptrainer::status::CONTINUE_VOICE);
        }
    }

    return 0.into();
}

unsafe extern "C" fn play_voice(weapon: &mut L2CWeaponCommon) {
    let single_line = sv_math::rand(hash40("fighter"), 2) as i32;
    if single_line == 1 {
        let rand = sv_math::rand(hash40("fighter"), 6) as i32;
        let voice_hash = match rand {
            0 => {
                if LinkModule::is_link(weapon.module_accessor, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) {
                    let poke_parent_id = LinkModule::get_parent_object_id(weapon.module_accessor, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) as u32;
                    let poke_object = utils::util::get_battle_object_from_id(poke_parent_id);
                    if !poke_object.is_null() {
                        let poke_boma = &mut *(*poke_object).module_accessor;
                        let poke_boma_kind = poke_boma.kind();
                        if VarModule::is_flag(weapon.battle_object, vars::ptrainer::status::VOICE_FORWARD_SWITCH) {
                            if poke_boma_kind == *FIGHTER_KIND_PZENIGAME {
                                Hash40::new("vc_ptrainer_throw02")
                            }
                            else if poke_boma_kind == *FIGHTER_KIND_PFUSHIGISOU {
                                Hash40::new("vc_ptrainer_throw03")
                            }
                            else {
                                Hash40::new("vc_ptrainer_throw01")
                            }
                        }
                        else {
                            if poke_boma_kind == *FIGHTER_KIND_PZENIGAME {
                                Hash40::new("vc_ptrainer_throw03")
                            }
                            else if poke_boma_kind == *FIGHTER_KIND_PFUSHIGISOU {
                                Hash40::new("vc_ptrainer_throw01")
                            }
                            else {
                                Hash40::new("vc_ptrainer_throw02")
                            }
                        }
                    } else {
                        Hash40::new("vc_ptrainer_throw08")
                    }
                } else {
                    Hash40::new("vc_ptrainer_throw08")
                }
            },
            1 => Hash40::new("vc_ptrainer_throw04"),
            2 => Hash40::new("vc_ptrainer_throw05"),
            3 => Hash40::new("vc_ptrainer_throw06"),
            4 => Hash40::new("vc_ptrainer_throw07"),
            _ => {
                VarModule::on_flag(weapon.battle_object, vars::ptrainer::status::VOICE_USE_OUT_POKE_KIND);
                Hash40::new("vc_ptrainer_throw08")
            },
        };
        PLAY_SE(weapon, voice_hash);
    }
    else {
        VarModule::on_flag(weapon.battle_object, vars::ptrainer::status::CONTINUE_VOICE);
        let rand = sv_math::rand(hash40("fighter"), 6) as i32;
        let voice_hash = match rand {
            0 => Hash40::new("vc_ptrainer_throw05"),
            1 => Hash40::new("vc_ptrainer_throw06"),
            2 => Hash40::new("vc_ptrainer_throw07"),
            3 => {
                VarModule::on_flag(weapon.battle_object, vars::ptrainer::status::VOICE_USE_OUT_POKE_KIND);
                Hash40::new("vc_ptrainer_throw08")
            },
            4 => {
                VarModule::on_flag(weapon.battle_object, vars::ptrainer::status::VOICE_USE_OUT_POKE_KIND);
                Hash40::new("vc_ptrainer_throw09")
            },
            _ => {
                VarModule::on_flag(weapon.battle_object, vars::ptrainer::status::VOICE_USE_OUT_POKE_KIND);
                Hash40::new("vc_ptrainer_throw10")
            },
        };
        PLAY_SE(weapon, voice_hash);
        weapon.set_int64(voice_hash.hash as i64, *WEAPON_PTRAINER_PTRAINER_INSTANCE_WORK_ID_INT_VOICE_ID);
    }
    weapon.off_flag(*WEAPON_PTRAINER_PTRAINER_STATUS_WORK_FLAG_VOICE);
}

unsafe extern "C" fn play_voice_continue(weapon: &mut L2CWeaponCommon, is_forward: bool, use_out: bool) {
    if !LinkModule::is_link(weapon.module_accessor, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) {
        return;
    }
    let poke_parent_id = LinkModule::get_parent_object_id(weapon.module_accessor, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON) as u32;
    let poke_object = utils::util::get_battle_object_from_id(poke_parent_id);
    if poke_object.is_null() {
        return;
    }
    let poke_boma = &mut *(*poke_object).module_accessor;
    let poke_boma_kind = poke_boma.kind();
    if use_out {
        if is_forward {
            if poke_boma_kind == *FIGHTER_KIND_PZENIGAME {
                PLAY_SE(weapon, Hash40::new("vc_ptrainer_throw13"));
            }
            else if poke_boma_kind == *FIGHTER_KIND_PFUSHIGISOU {
                PLAY_SE(weapon, Hash40::new("vc_ptrainer_throw11"));
            }
            else {
                PLAY_SE(weapon, Hash40::new("vc_ptrainer_throw12"));
            }
        }
        else {
            if poke_boma_kind == *FIGHTER_KIND_PZENIGAME {
                PLAY_SE(weapon, Hash40::new("vc_ptrainer_throw12"));
            }
            else if poke_boma_kind == *FIGHTER_KIND_PFUSHIGISOU {
                PLAY_SE(weapon, Hash40::new("vc_ptrainer_throw13"));
            }
            else {
                PLAY_SE(weapon, Hash40::new("vc_ptrainer_throw11"));
            }
        }
    }
    else {
        if poke_boma_kind == *FIGHTER_KIND_PZENIGAME {
            PLAY_SE(weapon, Hash40::new("vc_ptrainer_throw11"));
        }
        else if poke_boma_kind == *FIGHTER_KIND_PFUSHIGISOU {
            PLAY_SE(weapon, Hash40::new("vc_ptrainer_throw12"));
        }
        else {
            PLAY_SE(weapon, Hash40::new("vc_ptrainer_throw13"));
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_PTRAINER_PTRAINER_STATUS_KIND_SPECIAL_LW, special_lw_main);
}

// vc_ptrainer_throw01 - squirtle!
// vc_ptrainer_throw02 - ivysaur!
// vc_ptrainer_throw03 - charziard!
// vc_ptrainer_throw04 - go
// vc_ptrainer_throw05 - go for it
// vc_ptrainer_throw06 - hang on
// vc_ptrainer_throw07 - now's your chance
// vc_ptrainer_throw08 - come back
// vc_ptrainer_throw09 - good job
// vc_ptrainer_throw10 - great job
// vc_ptrainer_throw11 - squirtle (end)
// vc_ptrainer_throw12 - ivysaur (end)
// vc_ptrainer_throw13 - charizard (end)