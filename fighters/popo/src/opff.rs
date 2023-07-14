// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn nana_couple_indicator(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, situation_kind: i32, motion_kind: u64, frame: f32) {
    // if fighter.kind() != *FIGHTER_KIND_NANA 
    // || [*FIGHTER_STATUS_KIND_DEMO, 
    //     *FIGHTER_STATUS_KIND_ENTRY, 
    //     *FIGHTER_STATUS_KIND_REBIRTH, 
    //     *FIGHTER_STATUS_KIND_WIN, 
    //     *FIGHTER_STATUS_KIND_LOSE, 
    //     *FIGHTER_STATUS_KIND_DEAD,
    //     *FIGHTER_STATUS_KIND_STANDBY].contains(&status_kind) {
    //     if VarModule::is_flag(boma.object(), vars::iceclimbers::instance::IS_SEPARATED) {
    //         VarModule::off_flag(boma.object(), vars::iceclimbers::instance::IS_SEPARATED);
    //         let effect_handle = VarModule::get_int(boma.object(), vars::iceclimbers::instance::SEPARATED_EFFECT) as u32;
    //         if EffectModule::is_exist_effect(boma, effect_handle) {
    //             EffectModule::kill(boma, effect_handle, true, true);
    //             println!("killing effect");
    //         } else {
    //             println!("failed to kill effect");
    //         }
    //     }
    //     return;
    // }

    // let pos_z = PostureModule::pos_z(boma);
    // // This if else block lets us only kill the effect when necessary
    // if pos_z == 0.0 {
    //     if !VarModule::is_flag(boma.object(), vars::iceclimbers::instance::IS_SEPARATED) {
    //         VarModule::on_flag(boma.object(), vars::iceclimbers::instance::IS_SEPARATED);
    //     }
    // } else {
    //     if VarModule::is_flag(boma.object(), vars::iceclimbers::instance::IS_SEPARATED) {
    //         VarModule::off_flag(boma.object(), vars::iceclimbers::instance::IS_SEPARATED);
    //         let effect_handle = VarModule::get_int(boma.object(), vars::iceclimbers::instance::SEPARATED_EFFECT) as u32;
    //         if EffectModule::is_exist_effect(boma, effect_handle) {
    //             EffectModule::kill(boma, effect_handle, true, true);
    //             println!("killing effect");
    //         } else {
    //             println!("failed to kill effect");
    //         }
    //     }
    // }

    // // This if block creates thew effects when flag is set
    // if VarModule::is_flag(boma.object(), vars::iceclimbers::instance::IS_SEPARATED) {
    //     EffectModule::
    //     let effect = EffectModule::req_follow(boma, Hash40::new("sys_falling_smoke"), Hash40::new("neck"), &Vector3f::zero(), &Vector3f::zero(), 1.1, true, 0, 0, 0, 0, 0, true, true) as u32;
    //     EffectModule::set_rgb(boma, effect, 1.0, 0.85, 0.85);
    //     VarModule::set_int(boma.object(), vars::iceclimbers::instance::SEPARATED_EFFECT, effect as i32);
    // }
}

unsafe fn dair_bounce(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, motion_kind: u64, frame: f32) {
    if (motion_kind == hash40("attack_air_lw") || motion_kind == hash40("attack_air_lw_nana"))
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && frame < 55.0
    {
        MotionModule::set_frame_sync_anim_cmd(boma, 54.0, true, true, false);
        AttackModule::clear_all(boma);

        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            SET_SPEED_EX(fighter, 0, 1.7, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            if fighter.kind() == *FIGHTER_KIND_POPO {
                PLAY_SE(fighter, Hash40::new("vc_popo_attack04"));
            } else if fighter.kind() == *FIGHTER_KIND_NANA {
                PLAY_SE(fighter, Hash40::new("vc_nana_attack04"));
            } 
        } else if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            SET_SPEED_EX(fighter, 0, 0.75, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            if fighter.kind() == *FIGHTER_KIND_POPO {
                PLAY_SE(fighter, Hash40::new("vc_popo_attack04"));
            } else if fighter.kind() == *FIGHTER_KIND_NANA {
                PLAY_SE(fighter, Hash40::new("vc_nana_attack04"));
            } 
        }
    }
}

// Ice Climbers Spotdodge Desync
unsafe fn spotdodge_desync(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if boma.kind() == *FIGHTER_KIND_NANA {
        if ![*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_CLIMB].contains(&status_kind){
            InputModule::disable_persist(boma.object());
        } else if [*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_STATUS_KIND_CLIFF_ESCAPE, *FIGHTER_STATUS_KIND_CLIFF_CLIMB].contains(&StatusModule::status_kind_next(boma)) {
            InputModule::enable_persist(boma.object());
        }
    }
}

pub static mut nana_boma: [u64; 8] = [0; 8];

unsafe fn get_nana_boma(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize) {
    if boma.kind() == *FIGHTER_KIND_NANA {
        let mut nana_boma_deez = boma;
        nana_boma[id] = (&mut *nana_boma_deez as *mut BattleObjectModuleAccessor) as u64;
    }
}

static mut effect_on: bool = false;
static mut nana_pos_x: f32 = 0.0;
static mut nana_pos_y: f32 = 0.0;

unsafe fn nana_death_effect(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {

    if boma.kind() == *FIGHTER_KIND_POPO {
        if status_kind == *FIGHTER_STATUS_KIND_STANDBY {
            effect_on = true;
            nana_pos_x = PostureModule::pos_x(nana_boma[id] as *mut BattleObjectModuleAccessor);
            nana_pos_y = PostureModule::pos_y(nana_boma[id] as *mut BattleObjectModuleAccessor);
        }
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH && fighter.status_frame() <= 1 && effect_on {
            let pos =  Vector3f {x: nana_pos_x, y: nana_pos_y, z: 0.0};
            let rot =  Vector3f {x: 0.0, y: 0.0, z: 0.0};
            EffectModule::req(boma, Hash40::new("sys_recovery"), &pos, &rot, 1.0, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32, 0, true, 0);
            effect_on = false;
        }
    }
}

unsafe fn voluntary_sopo(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    if fighter.kind() != *FIGHTER_KIND_POPO {
        return;
    }

    let nana = nana_boma[id] as *mut BattleObjectModuleAccessor;
    if VarModule::is_flag(boma.object(), vars::iceclimbers::instance::IS_VOLUNTARY_SOPO) {
        if (*nana).is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH]) {
            StatusModule::change_status_request(nana, *FIGHTER_STATUS_KIND_STANDBY, false);
        }
        return;
    }

    if fighter.is_prev_status_one_of(&[*FIGHTER_STATUS_KIND_ENTRY])
    && fighter.is_button_on(Buttons::Guard) 
    && fighter.is_button_on(Buttons::Special) 
    && fighter.is_button_on(Buttons::AppealLw) {
        VarModule::on_flag(boma.object(), vars::iceclimbers::instance::IS_VOLUNTARY_SOPO);
        StatusModule::change_status_request(nana, *FIGHTER_STATUS_KIND_DEAD, false);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_FAIL,
        *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_POPO_STATUS_KIND_SPECIAL_HI_JUMP_PARTNER
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // nothing lol
}


#[no_mangle]
pub unsafe extern "Rust" fn ice_climbers_common(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        ice_climbers_moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub unsafe fn ice_climbers_moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    spotdodge_desync(boma, status_kind);
    get_nana_boma(fighter, boma, id);
    nana_death_effect(fighter, boma, id, status_kind, frame);
    dair_bounce(fighter, boma, motion_kind, frame);
    voluntary_sopo(fighter, boma, id, status_kind, frame);
    nana_couple_indicator(fighter, boma, id, status_kind, situation_kind, motion_kind, frame);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_POPO )]
pub fn popo_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		popo_frame(fighter);
        ice_climbers_common(fighter);
    }
}

pub unsafe fn popo_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}