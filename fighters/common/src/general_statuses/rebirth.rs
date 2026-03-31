use super::*;
use globals::*;


#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_rebirth_uniq_process_init)]
unsafe extern "C" fn sub_rebirth_uniq_process_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = PostureModule::lr(fighter.module_accessor);
    let kind = fighter.global_table[FIGHTER_KIND].get_i32();

    if !ParamModule::is_flag(fighter.object(), ParamType::Shared, "use_entry_anim_on_respawn") {
        return original!()(fighter);
    }

    match kind {
        0x0 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_DOKAN, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_DOKAN, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_DOKAN, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x1 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x3 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_TRANSPORTATION, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_TRANSPORTATION, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_TRANSPORTATION, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x6 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_WARPSTAR, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_WARPSTAR, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_WARPSTAR, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x7 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x8 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL, false, -1);
            
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL, Hash40::new("entry_r"), true, -1.0);
        },
        0x9 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_DOKAN, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_DOKAN, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_DOKAN, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0xB => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON, false, -1);

            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0xC => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PURIN_GENERATE_ARTICLE_MONSTERBALL, false, -1);
                        
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PURIN_GENERATE_ARTICLE_MONSTERBALL, Hash40::new("entry"), true, -1.0);
        },
        0xD => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, false, -1);
            
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x12 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_CAPSULEBLOCK, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_CAPSULEBLOCK, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_CAPSULEBLOCK, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x13 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_MONSTERBALL, false, -1);
                        
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_MONSTERBALL, Hash40::new("entry_r"), true, -1.0);
        },
        0x14 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_ARWING, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_ARWING, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_ARWING, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x16 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x1A => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ROY_GENERATE_ARTICLE_SWORD, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_ROY_GENERATE_ARTICLE_SWORD, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_ROY_GENERATE_ARTICLE_SWORD, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x1C => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_ENTRY, false, -1);
        },
        0x1D => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x20 => {
            // ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP, false, -1);
            
            // if lr == -1.0 {
            //     ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP, Hash40::new("entry_l"), true, -1.0);
            // } 
            // else {
            //     ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP, Hash40::new("entry_r"), true, -1.0);
            // }
        },
        0x21 => {
            let bike_hp = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("bike_hp"));
            WorkModule::set_float(fighter.module_accessor, bike_hp, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_BIKE_HP);

            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE, false, -1);

            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE, Hash40::new("entry_l"), true, -1.0);

            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE, Hash40::new("entry_r"), true, -1.0);
            }

            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_WARIO_INSTANCE_WORK_ID_FLAG_REMOVE_BIKE);
        },
        0x27 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x28 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_DOSEITABLE, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_DOSEITABLE, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_DOSEITABLE, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x2A => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_SHRINE, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_SHRINE, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_SHRINE, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x30 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_HOUSE, false, -1);

            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_HOUSE, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_HOUSE, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x32 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_BALANCEBOARD, false, -1);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_WIIBO, false, -1);
                                    
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_BALANCEBOARD, Hash40::new("entry_l"), true, -1.0);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_WIIBO, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_BALANCEBOARD, Hash40::new("entry_r"), true, -1.0);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_WIIBO, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x33 => {
            // ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO, false, -1);
        },
        0x34 => {
            let costume_slot = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
            if costume_slot != 5
            && costume_slot != 7 {
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_SWEATLITTLEMAC, false, -1);
                ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_THROWSWEAT, false, -1);
                                                
                if lr == -1.0 {
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_SWEATLITTLEMAC, Hash40::new("entry_l"), true, -1.0);                    
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_THROWSWEAT, Hash40::new("entry_l"), true, -1.0);
                } 
                else {
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_SWEATLITTLEMAC, Hash40::new("entry_r"), true, -1.0);
                    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_THROWSWEAT, Hash40::new("entry_r"), true, -1.0);
                }
            }
        },
        0x35 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MONSTERBALL, false, -1);
                        
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MONSTERBALL, Hash40::new("entry"), true, -1.0);
        },
        0x36 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GATE, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GATE, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GATE, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x37 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN, false, -1);
            
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN, Hash40::new("entry"), true, -1.0);
        },
        0x46 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_OFFICE, false, -1);

            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_OFFICE, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_OFFICE, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x47 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_GAOGAEN_GENERATE_ARTICLE_MONSTERBALL, false, -1);
                        
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_GAOGAEN_GENERATE_ARTICLE_MONSTERBALL, Hash40::new("entry"), true, -1.0);
        },
        0x4B => {
            // if lr == -1.0 {
            //     ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR, Hash40::new("entry_l"), true, -1.0);
            // } 
            // else {
            //     ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR, Hash40::new("entry_r"), true, -1.0);
            // }
        },
        0x4C => {
            // if lr == -1.0 {
            //     ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR, Hash40::new("entry_l"), true, -1.0);
            // } 
            // else {
            //     ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR, Hash40::new("entry_r"), true, -1.0);
            // }
        },
        0x56 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BATON, false, -1);
                        
            if lr == -1.0 {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BATON, Hash40::new("entry_l"), true, -1.0);
            } 
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BATON, Hash40::new("entry_r"), true, -1.0);
            }
        },
        0x58 => {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_ENTRYOBJECT, false, -1);
            
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_ENTRYOBJECT, Hash40::new("entry_r"), true, -1.0);
        },
        _ => {}
    }

    original!()(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_rebirth_common_pre)]
unsafe extern "C" fn sub_rebirth_common_pre(fighter: &mut L2CFighterCommon) {
    let lr = PostureModule::lr(fighter.module_accessor);
    let kind = fighter.global_table[FIGHTER_KIND].get_i32();

    CameraModule::reset_all(fighter.module_accessor);

    ControlModule::reset_trigger(fighter.module_accessor);
    ControlModule::clear_command(fighter.module_accessor, false);

    AreaModule::set_whole(fighter.module_accessor, false);
    VisibilityModule::set_whole(fighter.module_accessor, true);

    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1e61567377));

    PhysicsModule::set_swing_rebirth(fighter.module_accessor, true);

    let end_frame = if lr == -1.0 {
        MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("entry_l"))
    } else {
        MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("entry_r"))
    };

    let start_frame: f32 = match kind {
        0x4 => {
            end_frame - 75.0
        },
        0xA => {
            0.0
        },
        0xB => {
            end_frame - 61.0
        },
        0x1C => {
            70.0
        },
        0x1E => {
            45.0
        },
        0x1F => {
            45.0
        },
        0x21 => {
            15.0
        },
        0x22 => {
            end_frame - 75.0
        },
        0x2A => {
            end_frame - 80.0
        },
        0x30 => {
            54.0
        },
        0x32 => {
            end_frame - 75.0
        },
        0x33 => {
            0.0
        },
        0x46 => {
            54.0
        },
        _ => {
            (end_frame - 85.0).max(0.0)
        }
    };

    if start_frame > 45.0 {
        VarModule::on_flag(fighter.battle_object, vars::common::status::IGNORE_INITIAL_SOUND);
    }

    if [*FIGHTER_KIND_PZENIGAME,
        *FIGHTER_KIND_PFUSHIGISOU,
        *FIGHTER_KIND_PLIZARDON].contains(&kind)
    {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("respawn"), 0.0, 1.0, false, 0.0, false, false);
    }
    else if !ParamModule::is_flag(fighter.object(), ParamType::Shared, "use_entry_anim_on_respawn")
    || [*FIGHTER_KIND_PIKMIN].contains(&kind) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("down_stand_d"), 0.0, 0.0, false, 0.0, false, false);
    }
    else {
        if lr == -1.0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("entry_l"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("entry_r"), 0.0, 1.0, false, 0.0, false, false);
        }

        MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, start_frame, false, false, false);
    }

    match kind {
        0x0 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_DOKAN) {
                let article_boma = fighter.get_article_boma(*FIGHTER_MARIO_GENERATE_ARTICLE_DOKAN);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x1 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x2 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        }
        0x3 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_TRANSPORTATION) {
                let article_boma = fighter.get_article_boma(*FIGHTER_SAMUS_GENERATE_ARTICLE_TRANSPORTATION);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x6 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_WARPSTAR) {
                let article_boma = fighter.get_article_boma(*FIGHTER_KIRBY_GENERATE_ARTICLE_WARPSTAR);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x7 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING) {
                let article_boma = fighter.get_article_boma(*FIGHTER_FOX_GENERATE_ARTICLE_ARWING);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x8 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x9 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_DOKAN) {
                let article_boma = fighter.get_article_boma(*FIGHTER_LUIGI_GENERATE_ARTICLE_DOKAN);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0xB => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON) {
                let article_boma = fighter.get_article_boma(*FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0xC => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PURIN_GENERATE_ARTICLE_MONSTERBALL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_PURIN_GENERATE_ARTICLE_MONSTERBALL);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0xD => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR) {
                let article_boma = fighter.get_article_boma(*FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x12 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_CAPSULEBLOCK) {
                let article_boma = fighter.get_article_boma(*FIGHTER_MARIOD_GENERATE_ARTICLE_CAPSULEBLOCK);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x13 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_MONSTERBALL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_PICHU_GENERATE_ARTICLE_MONSTERBALL);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x14 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_ARWING) {
                let article_boma = fighter.get_article_boma(*FIGHTER_FALCO_GENERATE_ARTICLE_ARWING);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x16 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK) {
                let article_boma = fighter.get_article_boma(*FIGHTER_LUCINA_GENERATE_ARTICLE_MASK);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x1A => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ROY_GENERATE_ARTICLE_SWORD) {
                let article_boma = fighter.get_article_boma(*FIGHTER_ROY_GENERATE_ARTICLE_SWORD);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x1C => {            
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_ENTRY) {
                let article_boma = fighter.get_article_boma(*FIGHTER_GAMEWATCH_GENERATE_ARTICLE_ENTRY);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x1D => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE) {
                let article_boma = fighter.get_article_boma(*FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x20 => {
            // if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP) {
            //     let article_boma = fighter.get_article_boma(*FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP);
            //     MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            // }
        },
        0x21 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE) {
                let article_boma = fighter.get_article_boma(*FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x27 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x28 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_DOSEITABLE) {
                let article_boma = fighter.get_article_boma(*FIGHTER_LUCAS_GENERATE_ARTICLE_DOSEITABLE);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x2A => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_SHRINE) {
                let article_boma = fighter.get_article_boma(*FIGHTER_DEDEDE_GENERATE_ARTICLE_SHRINE);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x30 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_HOUSE) {
                let article_boma = fighter.get_article_boma(*FIGHTER_MURABITO_GENERATE_ARTICLE_HOUSE);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x32 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_BALANCEBOARD) {
                let article_boma = fighter.get_article_boma(*FIGHTER_WIIFIT_GENERATE_ARTICLE_BALANCEBOARD);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
            
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_WIIBO) {
                let article_boma = fighter.get_article_boma(*FIGHTER_WIIFIT_GENERATE_ARTICLE_WIIBO);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x33 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO) {
                ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));

                let article_boma = fighter.get_article_boma(*FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO);
                StatusModule::change_status_request_from_script(article_boma, *WEAPON_ROSETTA_TICO_STATUS_KIND_ENTRY, false);
            }
        },
        0x34 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_SWEATLITTLEMAC) {
                let article_boma = fighter.get_article_boma(*FIGHTER_LITTLEMAC_GENERATE_ARTICLE_SWEATLITTLEMAC);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }

            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_THROWSWEAT) {
                let article_boma = fighter.get_article_boma(*FIGHTER_LITTLEMAC_GENERATE_ARTICLE_THROWSWEAT);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x35 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MONSTERBALL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MONSTERBALL);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x36 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GATE) {
                let article_boma = fighter.get_article_boma(*FIGHTER_PALUTENA_GENERATE_ARTICLE_GATE);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x37 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN) {
                let article_boma = fighter.get_article_boma(*FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x46 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_OFFICE) {
                let article_boma = fighter.get_article_boma(*FIGHTER_SHIZUE_GENERATE_ARTICLE_OFFICE);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x47 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_GAOGAEN_GENERATE_ARTICLE_MONSTERBALL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_GAOGAEN_GENERATE_ARTICLE_MONSTERBALL);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x4B => {       
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR) {
                let article_boma = fighter.get_article_boma(*FIGHTER_POPO_GENERATE_ARTICLE_CONDOR);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x4C => {            
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR) {
                let article_boma = fighter.get_article_boma(*FIGHTER_POPO_GENERATE_ARTICLE_CONDOR);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x56 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BATON) {
                let article_boma = fighter.get_article_boma(*FIGHTER_MASTER_GENERATE_ARTICLE_BATON);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x58 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_ENTRYOBJECT) {
                let article_boma = fighter.get_article_boma(*FIGHTER_PICKEL_GENERATE_ARTICLE_ENTRYOBJECT);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x5A => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_DIVER) {
                let article_boma = fighter.get_article_boma(*FIGHTER_EFLAME_GENERATE_ARTICLE_DIVER);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        0x5B => {            
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_DIVER) {
                let article_boma = fighter.get_article_boma(*FIGHTER_ELIGHT_GENERATE_ARTICLE_DIVER);
                MotionModule::set_frame_sync_anim_cmd(article_boma, start_frame, false, false, false);
            }
        },
        _ => {}
    }

    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_rebirth_uniq_check(false.into());
    }

    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_sub_rebirth_uniq_check as *const () as _));

    GroundModule::set_ignore_boss(fighter.module_accessor, true);
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Rebirth_Main)]
unsafe extern "C" fn status_rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let kind = fighter.global_table[FIGHTER_KIND].get_i32();
    let cmd_cat2 = fighter.global_table[CMD_CAT2].get_i32();
    let pos = PostureModule::pos(fighter.module_accessor);
    let pos_x = (*pos).x;
    let pos_y = (*pos).y;
    let pos_z = (*pos).z;

    if fighter.sub_rebirth_common().get_bool() {
        return 1.into();
    }

    let rebirth_move_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("rebirth_move_frame"));
    let down_stand_d_end_frame = MotionModule::end_frame_from_hash(fighter.module_accessor, Hash40::new("down_stand_d")) as i32;
    if fighter.is_motion(Hash40::new("down_stand_d"))
    && fighter.global_table[CURRENT_FRAME].get_i32() == (rebirth_move_frame - 10) - down_stand_d_end_frame {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }

    if fighter.is_motion_one_of(&[Hash40::new("entry_l"), Hash40::new("entry_r"), Hash40::new("down_stand_d")]) {
        rebirth_motion_handler(fighter);
    }
    else {
        fighter.sub_wait_motion(false.into());
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_REBIRTH_FLAG_ENABLE_STRANS)
    && !fighter.is_motion_one_of(&[
        Hash40::new("appeal_hi_r"),
        Hash40::new("appeal_hi_l"),
        Hash40::new("appeal_s_r"),
        Hash40::new("appeal_s_l"),
        Hash40::new("appeal_lw_r"),
        Hash40::new("appeal_lw_l")
    ]) {
        if cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI != 0 {
            if lr >= 0.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_r"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_l"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0 || cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0 {
            if lr >= 0.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_s_l"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
        if cmd_cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW != 0 {
            if lr >= 0.0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_r"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_l"), 0.0, 1.0, false, 0.0, false, false);
            }
        }
    }

    match kind {
        0x0 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIO_GENERATE_ARTICLE_DOKAN) {
                let article_boma = fighter.get_article_boma(*FIGHTER_MARIO_GENERATE_ARTICLE_DOKAN);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x1 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x2 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_LINK_GENERATE_ARTICLE_PARASAIL);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        }
        0x3 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SAMUS_GENERATE_ARTICLE_TRANSPORTATION) {
                let article_boma = fighter.get_article_boma(*FIGHTER_SAMUS_GENERATE_ARTICLE_TRANSPORTATION);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x6 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KIRBY_GENERATE_ARTICLE_WARPSTAR) {
                let article_boma = fighter.get_article_boma(*FIGHTER_KIRBY_GENERATE_ARTICLE_WARPSTAR);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x7 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_ARWING) {
                let article_boma = fighter.get_article_boma(*FIGHTER_FOX_GENERATE_ARTICLE_ARWING);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x8 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_PIKACHU_GENERATE_ARTICLE_MONSTERBALL);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x9 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_DOKAN) {
                let article_boma = fighter.get_article_boma(*FIGHTER_LUIGI_GENERATE_ARTICLE_DOKAN);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0xB => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON) {
                let article_boma = fighter.get_article_boma(*FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0xC => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PURIN_GENERATE_ARTICLE_MONSTERBALL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_PURIN_GENERATE_ARTICLE_MONSTERBALL);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0xD => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR) {
                let article_boma = fighter.get_article_boma(*FIGHTER_PEACH_GENERATE_ARTICLE_KASSAR);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x12 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MARIOD_GENERATE_ARTICLE_CAPSULEBLOCK) {
                let article_boma = fighter.get_article_boma(*FIGHTER_MARIOD_GENERATE_ARTICLE_CAPSULEBLOCK);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x13 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_MONSTERBALL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_PICHU_GENERATE_ARTICLE_MONSTERBALL);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x14 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_FALCO_GENERATE_ARTICLE_ARWING) {
                let article_boma = fighter.get_article_boma(*FIGHTER_FALCO_GENERATE_ARTICLE_ARWING);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x16 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK) {
                let article_boma = fighter.get_article_boma(*FIGHTER_LUCINA_GENERATE_ARTICLE_MASK);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x1A => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ROY_GENERATE_ARTICLE_SWORD) {
                let article_boma = fighter.get_article_boma(*FIGHTER_ROY_GENERATE_ARTICLE_SWORD);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x1C => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_GAMEWATCH_GENERATE_ARTICLE_ENTRY) {
                let article_boma = fighter.get_article_boma(*FIGHTER_GAMEWATCH_GENERATE_ARTICLE_ENTRY);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x1D => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE) {
                let article_boma = fighter.get_article_boma(*FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x20 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP) {
                let article_boma = fighter.get_article_boma(*FIGHTER_SZEROSUIT_GENERATE_ARTICLE_GUNSHIP);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x21 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE) {
                let article_boma = fighter.get_article_boma(*FIGHTER_WARIO_GENERATE_ARTICLE_WARIOBIKE);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x27 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_DIDDY_GENERATE_ARTICLE_DKBARREL);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x28 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_DOSEITABLE) {
                let article_boma = fighter.get_article_boma(*FIGHTER_LUCAS_GENERATE_ARTICLE_DOSEITABLE);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x2A => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DEDEDE_GENERATE_ARTICLE_SHRINE) {
                let article_boma = fighter.get_article_boma(*FIGHTER_DEDEDE_GENERATE_ARTICLE_SHRINE);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x30 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_HOUSE) {
                let article_boma = fighter.get_article_boma(*FIGHTER_MURABITO_GENERATE_ARTICLE_HOUSE);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z - 10.0});
            }
        },
        0x32 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_BALANCEBOARD) {
                let article_boma = fighter.get_article_boma(*FIGHTER_WIIFIT_GENERATE_ARTICLE_BALANCEBOARD);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_WIIBO) {
                let article_boma = fighter.get_article_boma(*FIGHTER_WIIFIT_GENERATE_ARTICLE_WIIBO);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x33 => {
            if StatusModule::is_changing(fighter.module_accessor) {
                if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO) {
                    let article_boma = fighter.get_article_boma(*FIGHTER_ROSETTA_GENERATE_ARTICLE_TICO);
                    PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
                }
            }
        },
        0x34 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_SWEATLITTLEMAC) {
                let article_boma = fighter.get_article_boma(*FIGHTER_LITTLEMAC_GENERATE_ARTICLE_SWEATLITTLEMAC);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LITTLEMAC_GENERATE_ARTICLE_THROWSWEAT) {
                let article_boma = fighter.get_article_boma(*FIGHTER_LITTLEMAC_GENERATE_ARTICLE_THROWSWEAT);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x35 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MONSTERBALL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_GEKKOUGA_GENERATE_ARTICLE_MONSTERBALL);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x36 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PALUTENA_GENERATE_ARTICLE_GATE) {
                let article_boma = fighter.get_article_boma(*FIGHTER_PALUTENA_GENERATE_ARTICLE_GATE);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x37 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN) {
                let article_boma = fighter.get_article_boma(*FIGHTER_PACMAN_GENERATE_ARTICLE_BIGPACMAN);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x46 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SHIZUE_GENERATE_ARTICLE_OFFICE) {
                let article_boma = fighter.get_article_boma(*FIGHTER_SHIZUE_GENERATE_ARTICLE_OFFICE);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z - 10.0});
            }
        },
        0x47 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_GAOGAEN_GENERATE_ARTICLE_MONSTERBALL) {
                let article_boma = fighter.get_article_boma(*FIGHTER_GAOGAEN_GENERATE_ARTICLE_MONSTERBALL);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x4B => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR) {
                let article_boma = fighter.get_article_boma(*FIGHTER_POPO_GENERATE_ARTICLE_CONDOR);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x4C => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR) {
                let article_boma = fighter.get_article_boma(*FIGHTER_POPO_GENERATE_ARTICLE_CONDOR);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x56 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_MASTER_GENERATE_ARTICLE_BATON) {
                let article_boma = fighter.get_article_boma(*FIGHTER_MASTER_GENERATE_ARTICLE_BATON);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x58 => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_ENTRYOBJECT) {
                let article_boma = fighter.get_article_boma(*FIGHTER_PICKEL_GENERATE_ARTICLE_ENTRYOBJECT);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x5A => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_EFLAME_GENERATE_ARTICLE_DIVER) {
                let article_boma = fighter.get_article_boma(*FIGHTER_EFLAME_GENERATE_ARTICLE_DIVER);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        0x5B => {
            if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_DIVER) {
                let article_boma = fighter.get_article_boma(*FIGHTER_ELIGHT_GENERATE_ARTICLE_DIVER);
                PostureModule::set_pos(article_boma, &Vector3f{x: pos_x, y: pos_y, z: pos_z});
            }
        },
        _ => {}
    }

    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }

    if !fighter.global_table[IS_STOPPING].get_bool() {
        fighter.sub_rebirth_uniq_process_exec_fix_pos();
    }

    0.into()
}

unsafe extern "C" fn rebirth_motion_handler(fighter: &mut L2CFighterCommon) {
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("wait"), 0.0, 1.0, false, 0.0, false, false);
        
        fighter.sub_entry_remove_article();
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_bind_address_call_status_end_Rebirth)]
unsafe extern "C" fn bind_address_call_status_end_Rebirth(fighter: &mut L2CFighterCommon, _agent: &mut L2CAgent) -> L2CValue {
    fighter.status_end_Rebirth_common();

    fighter.sub_entry_remove_article();

    EffectModule::kill_all(fighter.module_accessor, *EFFECT_SUB_ATTRIBUTE_NONE as u32, true, true);

    0.into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_rebirth_uniq_process_init,
            sub_rebirth_common_pre,
            status_rebirth_main,
            bind_address_call_status_end_Rebirth
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}