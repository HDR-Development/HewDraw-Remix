// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// symbol-based call for the pikachu/pichu characters' common opff
extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}
 
unsafe fn bowser_bomb_jc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G].contains(&status_kind) {
        if frame > 20.0 && frame < 31.0 {
            if situation_kind == *SITUATION_KIND_AIR {
                boma.check_jump_cancel(false, false);
            }
        }
    }
}

// Ground Bowser Bomb jump drift
unsafe fn ground_bowser_bomb_jump_drift(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_x: f32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_LW_G].contains(&status_kind) {
        if frame > 14.0 && frame < 31.0 {
            if stick_x != 0.0 {
                let motion_vec = x_motion_vec(1.25, stick_x);
                KineticModule::add_speed_outside(boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
            }
        }
    }
}

// Bowser Flame Cancel
unsafe fn flame_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        let cooleddown = VarModule::countdown_int(boma.object(), vars::koopa::instance::FIREBALL_COOLDOWN_FRAME, 0);
        if frame < 23.0 && !cooleddown {
            if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
                MotionModule::set_frame(boma, 22.0, true);
            }
        }
    }
}

// NokNok shell flag reset
// unsafe fn noknok_reset(boma: &mut BattleObjectModuleAccessor) {
//     if VarModule::is_flag(boma.object(), vars::koopa::instance::NOKNOK_SHELL) {
//         if boma.is_status_one_of(
//     &[*FIGHTER_STATUS_KIND_DEAD,
//             *FIGHTER_STATUS_KIND_REBIRTH,
//             *FIGHTER_STATUS_KIND_WIN,
//             *FIGHTER_STATUS_KIND_LOSE,
//             *FIGHTER_STATUS_KIND_ENTRY]) 
//         {
//             VarModule::off_flag(boma.object(), vars::koopa::instance::NOKNOK_SHELL);
//         }
//     }
// }

// TRAINING MODE
// NokNok shell flag reset via taunt
// unsafe fn noknok_training(boma: &mut BattleObjectModuleAccessor) {
//     if is_training_mode() {
//         if boma.is_status(*FIGHTER_STATUS_KIND_APPEAL) {
//             VarModule::off_flag(boma.object(), vars::koopa::instance::NOKNOK_SHELL);
//         }
//     }
// }

unsafe fn up_special_land_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL && StatusModule::prev_status_kind(boma, 1) == *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, true);
    }
}


unsafe fn fireball_cooldown(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    //Ignore cooldown during respawn,death,entry and nspecial
    if (&[
        *FIGHTER_STATUS_KIND_ENTRY,*FIGHTER_STATUS_KIND_DEAD,*FIGHTER_STATUS_KIND_REBIRTH,
        *FIGHTER_STATUS_KIND_WIN,*FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_SPECIAL_N
    ]).contains(&status_kind) {
        return;
    }

    let cooleddown = VarModule::countdown_int(boma.object(), vars::koopa::instance::FIREBALL_COOLDOWN_FRAME, 0);
    let charged_effect =  VarModule::get_int(boma.object(), vars::koopa::instance::FIREBALL_EFFECT_ID);
    //If cooling down, remove ready effect
    if !cooleddown {
        if charged_effect > 0 {
            VarModule::set_int(boma.object(), vars::koopa::instance::FIREBALL_EFFECT_ID,0);
            if EffectModule::is_exist_effect(boma, charged_effect as u32) {
                EffectModule::kill(boma, charged_effect as u32, false,false);
            }
        }
        return;
    }
    //Otherwise, spawn effect if effect does not exist
    else if (charged_effect <= 0 
    || !EffectModule::is_exist_effect(boma, charged_effect as u32))
    {
        if (charged_effect <= 0){
            gimmick_flash(boma);
        }
        let pos = &Vector3f{x: 0.0, y: 1.0, z: 0.0};
        let rot = &Vector3f{x: 180.0, y: 0.0, z: 50.0};
        let handle = EffectModule::req_follow(boma, Hash40::new("koopa_breath_m_fire"), Hash40::new("jaw"), pos, rot, 1.0, true, 0, 0, 0, 0, 0, false, false) as u32;
        VarModule::set_int(boma.object(), vars::koopa::instance::FIREBALL_EFFECT_ID,handle as i32);
    }
}

// opff for handling the "excellent" punch 
unsafe fn koopa_ex_punch(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD) {
        if fighter.status_frame() == 51 { // indicates start of "excellent" frame window
            VarModule::on_flag(fighter.battle_object, vars::koopa::instance::IS_EXCELLENT_PUNCH);
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_level_up"), Hash40::new("handr"), 3, 0, 0, 0, 0, 0, 0.4, true);
            macros::LAST_EFFECT_SET_RATE(fighter, 3.0);
        } else if fighter.status_frame() == 58 { // window ends
            VarModule::off_flag(fighter.battle_object, vars::koopa::instance::IS_EXCELLENT_PUNCH);
        }
    }
    if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_S4) {
        if VarModule::is_flag(fighter.battle_object, vars::koopa::instance::IS_EXCELLENT_PUNCH) {
            if VarModule::is_flag(fighter.battle_object, vars::koopa::status::PUNCH_CAN_ZOOM) && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                SlowModule::set_whole(fighter.module_accessor, 8, 40);
                macros::CAM_ZOOM_IN_arg5(fighter, 2.0, 0.0, 1.8, 0.0, 0.0);
                macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_XL);
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_fire"), Hash40::new("handr"), 3, 0, 0, 0, 0, 0, 1.0, true);
                PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
                PLAY_SE(fighter, Hash40::new("se_koopa_final06")); // excellent sfx
                VarModule::off_flag(fighter.battle_object, vars::koopa::status::PUNCH_CAN_ZOOM);
            }
        }
    }
}

unsafe fn ex_punch_effect_reset(fighter: &mut L2CFighterCommon) {
    if fighter.is_prev_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD])
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE,
        *FIGHTER_STATUS_KIND_DAMAGE_AIR,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U,
        *FIGHTER_STATUS_KIND_DAMAGE_FALL ]) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_explosion_sign"), false, false);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A,
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

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    bowser_bomb_jc(boma, status_kind, situation_kind, cat[0], frame);
    ground_bowser_bomb_jump_drift(boma, status_kind, stick_x, frame);
    flame_cancel(boma, status_kind, situation_kind, frame);
    fireball_cooldown(boma,status_kind);
    koopa_ex_punch(fighter);
    ex_punch_effect_reset(fighter);
    fastfall_specials(fighter);
    // noknok_reset(boma);
    //up_special_land_cancel(boma, status_kind);
    // noknok_training(boma);
    fastfall_specials(fighter);
}


#[utils::macros::opff(FIGHTER_KIND_KOOPA )]
pub fn koopa_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		koopa_frame(fighter)
    }
}

pub unsafe fn koopa_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}