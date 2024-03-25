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

pub static mut NANA_BOMA: [u64; 8] = [0; 8];

unsafe fn get_nana_boma(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize) {
    if boma.kind() == *FIGHTER_KIND_NANA {
        let mut nana_boma_deez = boma;
        NANA_BOMA[id] = (&mut *nana_boma_deez as *mut BattleObjectModuleAccessor) as u64;
    }
}

static mut EFFECT_ON: bool = false;
static mut NANA_POS_X: f32 = 0.0;
static mut NANA_POS_Y: f32 = 0.0;

unsafe fn nana_death_effect(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {

    if boma.kind() == *FIGHTER_KIND_POPO {
        if status_kind == *FIGHTER_STATUS_KIND_STANDBY {
            EFFECT_ON = true;
            NANA_POS_X = PostureModule::pos_x(NANA_BOMA[id] as *mut BattleObjectModuleAccessor);
            NANA_POS_Y = PostureModule::pos_y(NANA_BOMA[id] as *mut BattleObjectModuleAccessor);
        }
        if status_kind == *FIGHTER_STATUS_KIND_REBIRTH && fighter.status_frame() <= 1 && EFFECT_ON {
            let pos =  Vector3f {x: NANA_POS_X, y: NANA_POS_Y, z: 0.0};
            let rot =  Vector3f {x: 0.0, y: 0.0, z: 0.0};
            EffectModule::req(boma, Hash40::new("sys_recovery"), &pos, &rot, 1.0, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32, 0, true, 0);
            EFFECT_ON = false;
        }
    }
}

unsafe fn voluntary_sopo(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, status_kind: i32, frame: f32) {
    if fighter.kind() != *FIGHTER_KIND_POPO {
        return;
    }

    let nana = NANA_BOMA[id] as *mut BattleObjectModuleAccessor;
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

// make nana face the same direction as popo during down smash
unsafe fn attacklw4_lr(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize) {
    if fighter.kind() != *FIGHTER_KIND_POPO {
        return;
    }

    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_LW4, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD]) {
        let nana = NANA_BOMA[id] as *mut BattleObjectModuleAccessor;
        let popo_lr = PostureModule::lr(boma);
        let nana_lr = PostureModule::lr(nana);
        if nana_lr != popo_lr {
            PostureModule::reverse_lr(nana);
            PostureModule::update_rot_y_lr(nana);
        }
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
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
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
    attacklw4_lr(fighter, boma, id);
    fastfall_specials(fighter);
}

pub extern "C" fn popo_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        ice_climbers_common(fighter);
    }
}

// Ice Climbers Cheer Cancel (Techy)
unsafe fn cheer_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_POPO_STATUS_KIND_THROW_NANA) {
        MotionModule::set_frame(fighter.module_accessor, MotionModule::end_frame(fighter.module_accessor), true);
        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
    }
}

pub extern "C" fn nana_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        ice_climbers_common(fighter);
        cheer_cancel(fighter);
    }
}

pub fn install_popo(agent: &mut Agent) {
    agent.on_line(Main, popo_frame_wrapper);
}

pub fn install_nana(agent: &mut Agent) {
    agent.on_line(Main, nana_frame_wrapper);
}
