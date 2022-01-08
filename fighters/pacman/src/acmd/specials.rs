
use smash::app::{sv_system, sv_animcmd::{frame, wait}, self, lua_bind::*};
use smash::phx::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::utility::*;
use smash_script::*;
use smashline::*;
use smashline::*;
use smash_script::macros::*;
use crate::utils::hdr;
use crate::vars::*;

#[acmd_script( agent = "pacman", script = "game_specialnshoot" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_special_n_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
            let charge_level = WorkModule::get_int(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
            if charge_level <= 0 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANCHERRY), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 1 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANSTRAWBERRY), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 3 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANORANGE), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 5 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANAPPLE), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 7 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANMELON), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 9 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANBOSS), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 11 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANBELL), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 12 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANKEY), 0, 0, false, false);   
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            //WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_FLAG_THROW);
        }
        
    }
    
}

#[acmd_script( agent = "pacman", script = "game_specialairnshoot" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_special_air_n_shoot_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
            let charge_level = WorkModule::get_int(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
            if charge_level <= 0 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANCHERRY), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 1 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANSTRAWBERRY), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 3 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANORANGE), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 5 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANAPPLE), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 7 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANMELON), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 9 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANBOSS), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 11 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANBELL), 0, 0, false, false);
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            else if charge_level <= 12 {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_PACMANKEY), 0, 0, false, false);   
                WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
            }
            //WorkModule::on_flag(boma, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_REMOVE_ITEM);
        }
        else{
            WorkModule::on_flag(boma, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_FLAG_THROW);
        }
        
    }
    
}


#[acmd_script( agent = "pacman", script = "game_speciallwfailure" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_special_lw_failure_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.4);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 270, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.6, /*SDI*/ 1.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "pacman", script = "game_specialairlwfailure" , category = ACMD_GAME , low_priority)]
unsafe fn pacman_special_air_lw_failure_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.4);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 7.5, /*Angle*/ 270, /*KBG*/ 70, /*FKB*/ 0, /*BKB*/ 20, /*Size*/ 10.0, /*X*/ 0.0, /*Y*/ 3.5, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.6, /*SDI*/ 1.5, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_ON, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_M, /*SFXType*/ *COLLISION_SOUND_ATTR_PUNCH, /*Type*/ *ATTACK_REGION_PUNCH);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        pacman_special_n_shoot_game,
        pacman_special_air_n_shoot_game,
        pacman_special_lw_failure_game,
        pacman_special_air_lw_failure_game,
    );
}

