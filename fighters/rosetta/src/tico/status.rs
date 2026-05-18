use super::*;
use crate::globals::*;
use vars::rosetta::instance::*;

// WEAPON_ROSETTA_TICO_STATUS_KIND_SPECIAL_N_SHOOT

pub unsafe extern "C" fn special_n_shoot_main(weapon: &mut L2CFighterCommon) -> L2CValue {
    //pop forward
    let lr= weapon.lr();
    PostureModule::add_pos_2d(weapon.module_accessor, &Vector2f::new(3.8 * lr, 0.0));
    //vars
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let rosetta: *mut BattleObject = utils::util::get_battle_object_from_id(owner_id);
    let rosetta_boma: &mut BattleObjectModuleAccessor = &mut *(*rosetta).module_accessor;
    let agent: &mut L2CFighterCommon = util::get_fighter_common_from_accessor(rosetta_boma);
    let rosa_pos = *PostureModule::pos(agent.module_accessor);
    let loma_pos = *PostureModule::pos(weapon.module_accessor);
    let mut rosa_lr= agent.lr();
    let pos_diff = Vector2f::new(loma_pos.x - rosa_pos.x, loma_pos.y - rosa_pos.y);
    //disappear fx
    EFFECT(agent, Hash40::new("rosetta_escape"), Hash40::new("top"), 15.0 * rosa_lr, -1.0 + pos_diff.y, ( 3.8 * rosa_lr + pos_diff.x) * rosa_lr, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true);
    LAST_EFFECT_SET_RATE(agent, 1.25);
    //delete annoying blue glow
    weapon.clear_lua_stack();
    lua_args!(weapon, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("rosetta_tico_warp_line"), true, true);
    sv_module_access::effect(weapon.lua_state_agent);
    weapon.clear_lua_stack();
    lua_args!(weapon, MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND, Hash40::new("rosetta_tico_warp"), true, true);
    sv_module_access::effect(weapon.lua_state_agent);
    //anim
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_n_fly"), 0.0, 0.0, false, 0.0, false, false);
    //startup movement
    KineticModule::change_kinetic(weapon.module_accessor, *WEAPON_KINETIC_TYPE_NORMAL);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.5 *lr, 0.0);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -1.0, -1.0);
    //luma logic flags
    weapon.on_flag(*WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_FREE);
    weapon.on_flag(*WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_TARGET_MOVE);
    weapon.set_int(0, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    weapon.off_flag(*WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_FREE_DISABLE_JUMP);
    weapon.off_flag(*WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_TARGET_SAME_FLOOR);
    weapon.fastshift(L2CValue::Ptr(special_n_shoot_main_loop as *const () as _))
}

pub unsafe extern "C" fn special_n_shoot_main_loop(weapon: &mut L2CFighterCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let rosetta = utils::util::get_battle_object_from_id(owner_id);
    let rosetta_boma: &mut BattleObjectModuleAccessor = &mut *(*rosetta).module_accessor;
    let agent: &mut L2CFighterCommon = util::get_fighter_common_from_accessor(rosetta_boma);
    if weapon.status_frame() == 5 {//f5
        //invis
        JostleModule::set_status(weapon.module_accessor, false);
        VisibilityModule::set_whole(weapon.module_accessor, false);
        HitModule::set_status_all(weapon.module_accessor, app::HitStatus(*HIT_STATUS_OFF), 0);
        //distance and tp
        let lr = weapon.lr();
        let charge_level = VarModule::get_int(weapon.battle_object, vars::rosetta::instance::TICO_CHARGE_LEVEL) as f32;
        let distance_min = 10.0; //base distance 3 big squares
        let distance_add = 20.0; //20 units per 20f frames charged, max 9 big squares
        let distance = distance_min + (distance_add * charge_level); //5 charge tiers 
        let speed = distance / 6.0 * lr; //6 frames then disable movement
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed, 0.0);
        sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -1.0, -1.0);
    } else if weapon.status_frame() == 11 {//11, 7f before reappear
        //get pos for fx to spawn ig
        let rosa_pos = *PostureModule::pos(agent.module_accessor);
	    let loma_pos = *PostureModule::pos(weapon.module_accessor);
        let mut rosa_lr= agent.lr();
        let pos_diff = Vector2f::new(loma_pos.x - rosa_pos.x, loma_pos.y - rosa_pos.y);
        EFFECT(agent, Hash40::new("rosetta_escape"), Hash40::new("top"), 15.0 * rosa_lr, -2.95 + pos_diff.y,  ( 1.35 * rosa_lr + pos_diff.x) * rosa_lr, 0, 0, 0, 0.69, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(agent, 0.95);
        //stop it
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
        KineticModule::unable_energy_all(weapon.module_accessor);
        PLAY_SE(weapon, Hash40::new("vc_tico_angry"));//luma yell when re-appearing?
    } else if weapon.status_frame() == 18 { //18f
        weapon.change_status(WEAPON_ROSETTA_TICO_STATUS_KIND_SPECIAL_N_END.into(), true.into());
    }
    0.into()
}

// WEAPON_ROSETTA_TICO_STATUS_KIND_SPECIAL_N_END

pub unsafe extern "C" fn special_n_end_main(weapon: &mut L2CFighterCommon) -> L2CValue {
    //make visible
    JostleModule::set_status(weapon.module_accessor, true);
    VisibilityModule::set_whole(weapon.module_accessor, true);
    HitModule::set_status_all(weapon.module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    //anim frame
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_n_end"), 1.0, 1.0, false, 0.0, false, false); //skips to it popping up
    //slight movement on reappearance
    let lr: f32 = weapon.lr();
    KineticModule::change_kinetic(weapon.module_accessor, *WEAPON_KINETIC_TYPE_NORMAL);
    //KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.15 *lr, 0.0);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -1.0, -1.0);
    weapon.fastshift(L2CValue::Ptr(special_n_end_main_loop as *const () as _))
}

pub unsafe extern "C" fn special_n_end_main_loop(weapon: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(weapon.module_accessor) {
        weapon.change_status(WEAPON_ROSETTA_TICO_STATUS_KIND_FREE_WAIT.into(), true.into());
    }
    0.into()
}

unsafe extern "C" fn standby_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_RESET,
        0,
        app::GroundCliffCheckKind(*GROUND_CORRECT_KIND_AIR),
        false,
        0,
        0,
        0,
        0
    );
    return 0.into();
}

unsafe extern "C" fn standby_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let rosetta = utils::util::get_battle_object_from_id(owner_id);
    let rosetta_boma: &mut BattleObjectModuleAccessor = &mut *(*rosetta).module_accessor;
    let rosa_lr = rosetta_boma.lr();
    KineticModule::unable_energy_all(weapon.module_accessor);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("free_recall"), 0.0, 1.0, false, 0.0, false, false);
    weapon.shift(L2CValue::Ptr(standby_main_loop as *const () as _))
}//make lumi stop when recalled, waits to either die or be sent back

unsafe extern "C" fn standby_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let rosetta = utils::util::get_battle_object_from_id(owner_id);
    let rosetta_boma: &mut BattleObjectModuleAccessor = &mut *(*rosetta).module_accessor;
    if rosetta_boma.is_button_off(Buttons::Special) && !rosetta_boma.is_button_release(Buttons::Special) 
    || StopModule::is_damage(rosetta_boma)
    || !VarModule::is_flag(weapon.battle_object, TICO_SPAWN_HAS_SYNCED)
    {//if rosaliner is not holding special, she gets hit, or luma isn't ready do something
        weapon.change_status_req(*WEAPON_ROSETTA_TICO_STATUS_KIND_FREE_WAIT, false);
        return 1.into()
    }//hopefully the right status
    if HitModule::get_whole(weapon.module_accessor, 0) != *HIT_STATUS_NORMAL {
        weapon.change_status_req(statuses::rosetta_tico::POP, true);
    }
    //size scaling
    let frame = MotionModule::frame(weapon.module_accessor);
    let scale = 1.0 + (0.75 / 50.0 * frame.min(50.0));
    ModelModule::set_scale(weapon.module_accessor, scale);
    return 0.into();
}

unsafe extern "C" fn standby_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if weapon.global_table[STATUS_KIND].get_i32() != statuses::rosetta_tico::POP {
        ModelModule::set_scale(weapon.module_accessor, 1.0);
        HitModule::set_whole(weapon.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    return 0.into();
}

unsafe extern "C" fn pop_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_RESET,
        0,
        app::GroundCliffCheckKind(*GROUND_CORRECT_KIND_AIR),
        false,
        0,
        0,
        0,
        0
    );
    return 0.into();
}

unsafe extern "C" fn pop_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    JostleModule::set_status(weapon.module_accessor, false);
    HitModule::set_whole(weapon.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    KineticModule::unable_energy_all(weapon.module_accessor);
    weapon.shift(L2CValue::Ptr(pop_main_loop as *const () as _))
}

unsafe extern "C" fn pop_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let agent: &mut L2CFighterCommon = util::get_fighter_common_from_accessor(&mut *weapon.module_accessor);
    let frame = MotionModule::frame(weapon.module_accessor);
    let scale = 1.05 + (0.75 / 50.0 * frame.min(50.0));
    ModelModule::set_scale(weapon.module_accessor, scale);
    if MotionModule::is_end(weapon.module_accessor) {
        weapon.change_status_req(*WEAPON_ROSETTA_TICO_STATUS_KIND_DEAD, false);
    }

    return 0.into();
}

unsafe extern "C" fn pop_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    ModelModule::set_scale(weapon.module_accessor, 1.0);
	JostleModule::set_status(weapon.module_accessor, true);
    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_ROSETTA_TICO_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_main);
    agent.status(Main, *WEAPON_ROSETTA_TICO_STATUS_KIND_SPECIAL_N_END, special_n_end_main);

    agent.status(Pre, statuses::rosetta_tico::STANDBY, standby_pre);
    agent.status(Main, statuses::rosetta_tico::STANDBY, standby_main);
    agent.status(End, statuses::rosetta_tico::STANDBY, standby_end);

    agent.status(Pre, statuses::rosetta_tico::POP, pop_pre);
    agent.status(Main, statuses::rosetta_tico::POP, pop_main);
    agent.status(End, statuses::rosetta_tico::POP, pop_end);
}