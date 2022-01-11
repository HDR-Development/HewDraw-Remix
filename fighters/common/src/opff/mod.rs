use crate::opff_import::*;
use smash::app::{self, lua_bind::*, sv_system, sv_kinetic_energy};
use smash::phx::*;
use smash::hash40;
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::lua2cpp::L2CFighterCommon;
use smash::lua2cpp::L2CFighterBase;							 

pub mod ledges;
pub mod shields;
pub mod physics;
pub mod tech;
pub mod tech_cleanup;
pub mod cancels;
pub mod var_resets;
pub mod gentleman;
pub mod momentum_transfer_line;
//pub mod shotos;
//pub mod magic;
pub mod gimmick;
pub mod floats;
pub mod other;

/*
pub fn install() {
    // acmd::add_custom_hooks!(sys_line_system_control_fighter_hook);
    smashline::install_agent_frames!(sys_line_system_control_fighter_hook);
    smashline::install_agent_frames!(sys_line_system_control_hook);
    
}
*/

pub struct FrameInfo {
    pub lua_state: u64,
    pub agent: *mut L2CAgent,
    pub boma: *mut smash::app::BattleObjectModuleAccessor,
    pub fighter_kind: i32,
    pub status_kind: i32,
    pub situation_kind: i32,
    pub motion_kind: smash::phx::Hash40,
    pub cur_frame: f32,
    pub frame: f32,
    pub cat: [i32; 4],
    pub facing: f32,
    pub stick_x: f32,
    pub stick_y: f32,
    pub id: usize
}

impl FrameInfo {
    pub unsafe fn update_and_get(fighter: &mut L2CFighterCommon) -> Option<Self> {
        let lua_state = fighter.lua_state_agent;
        let boma = sv_system::battle_object_module_accessor(lua_state);
        let id = hdr::get_player_number(boma);
        if !(0..8).contains(&id) {
            return None;
        }
        let cat1 = ControlModule::get_command_flag_cat(boma, 0);
        let cat2 = ControlModule::get_command_flag_cat(boma, 1);
        let cat3 = ControlModule::get_command_flag_cat(boma, 2);
        let cat4 = ControlModule::get_command_flag_cat(boma, 3);
        let cur_frame = MotionModule::frame(boma);
        VarModule::set_int(fighter.battle_object,common::COSTUME_SLOT_NUMBER,WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR));
        Some(Self {
            lua_state: lua_state,
            agent: fighter as *mut L2CFighterCommon as *mut L2CAgent,
            boma: boma as *mut smash::app::BattleObjectModuleAccessor,
            fighter_kind: get_kind(boma),
            status_kind: StatusModule::status_kind(boma),
            situation_kind: StatusModule::situation_kind(boma),
            motion_kind: Hash40::new_raw(MotionModule::motion_kind(boma)),
            cur_frame: cur_frame,
            frame: cur_frame + 1.0,
            cat: [cat1, cat2, cat3, cat4],
            facing: PostureModule::lr(boma),
            stick_x: ControlModule::get_stick_x(boma),
            stick_y: ControlModule::get_stick_y(boma),
            id: id
        })
    }
}

pub struct WeaponFrameInfo {
    pub lua_state: u64,
    pub agent: *mut L2CAgent,
    pub boma: *mut smash::app::BattleObjectModuleAccessor,
    pub weapon_kind: i32,
    pub status_kind: i32,
    pub situation_kind: i32,
    pub motion_kind: smash::phx::Hash40,
    pub cur_frame: f32,
    pub frame: f32,
    pub cat: [i32; 4],
    pub facing: f32,
    pub stick_x: f32,
    pub stick_y: f32,
    pub id: usize
}

impl WeaponFrameInfo {
    pub unsafe fn weapon_update_and_get(weapon: &mut L2CFighterBase) -> Option<Self> {
        let lua_state = weapon.lua_state_agent;
        let boma = sv_system::battle_object_module_accessor(lua_state);
        let id = hdr::get_player_number(boma);
        if !(0..8).contains(&id) {
            return None;
        }
        let cat1 = ControlModule::get_command_flag_cat(boma, 0);
        let cat2 = ControlModule::get_command_flag_cat(boma, 1);
        let cat3 = ControlModule::get_command_flag_cat(boma, 2);
        let cat4 = ControlModule::get_command_flag_cat(boma, 3);
        let cur_frame = MotionModule::frame(boma);
        VarModule::set_int(weapon.battle_object,common::COSTUME_SLOT_NUMBER,WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR));
        Some(Self {
            lua_state: lua_state,
            agent: weapon as *mut L2CFighterBase as *mut L2CAgent,
            boma: boma as *mut smash::app::BattleObjectModuleAccessor,
            weapon_kind: get_kind(boma),
            status_kind: StatusModule::status_kind(boma),
            situation_kind: StatusModule::situation_kind(boma),
            motion_kind: Hash40::new_raw(MotionModule::motion_kind(boma)),
            cur_frame: cur_frame,
            frame: cur_frame + 1.0,
            cat: [cat1, cat2, cat3, cat4],
            facing: PostureModule::lr(boma),
            stick_x: ControlModule::get_stick_x(boma),
            stick_y: ControlModule::get_stick_y(boma),
            id: id
        })
    }
}

/*
This function runs exactly once per every fighter loaded into a match, every frame. I.E.  5 players in a match = 5 times per frame
Use this instead of get_command_flag_cat
*/

//      This is a special case function (I.E. don't use this as an exmaple for hooking).
//         It doesn't need a hook or return value because all that is handled in the ACMD crate.
// lol, lmao - blujay

// general per-frame fighter-level hooks
pub unsafe fn fighter_common_opff(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        let boma = &mut *info.boma;
        if get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            /* Logic for when game "sessions" begin/end */
            handle_game_resets(boma, fighter);
            //Update and handle GLOBAL_FRAME_COUNT
            global_frame_count::update_global_frame_counter(boma, info.status_kind);
            //movesets
            moveset_edits(fighter, &info);
            // visualizer::training_mode_hitbox_visualizer_control(boma);
            // visualizer::sys_line(fighter);
        }
    }
}


pub unsafe fn weapon_common_opff(weapon: &mut L2CFighterBase) {
    if let Some(info) = WeaponFrameInfo::weapon_update_and_get(weapon) {
        let boma = smash::app::sv_system::battle_object_module_accessor(weapon.lua_state_agent);
        if get_category(boma) == *BATTLE_OBJECT_CATEGORY_WEAPON {
            /* Logic for when game "sessions" begin/end */
            //handle_game_resets_weapon(boma, weapon);
            //Update and handle GLOBAL_FRAME_COUNT
            global_frame_count::update_global_frame_counter(boma, info.status_kind);
            //movesets
            weapon_edits(weapon, &info);
        }
    }
}

pub unsafe fn moveset_edits(fighter: &mut L2CFighterCommon, info: &FrameInfo) {
    let boma = &mut *info.boma;

    // fighter.set_cliff_xlu_frame(10.0.into());


    // General Engine Edits
    // physics::run(fighter, info.lua_state, &mut *info.agent, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    // shields::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    // tech::run(fighter, info.lua_state, &mut *info.agent, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing, info.frame);
    // tech_cleanup::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing, info.frame);
    // cancels::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    // ledges::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    // var_resets::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    // gentleman::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    // //magic::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    // gimmick::run(boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    // other::run(fighter, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    // momentum_transfer_line::run(fighter, info.lua_state, &mut *info.agent, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
    // Function hooks are in src/hooks/function_hooks

    // Character Moveset Changes
    // moveset_changes::run(boma, id, cat, status_kind, situation_kind, motion_kind, fighter_kind, stick_x, stick_y, facing, frame);
    floats::run(fighter, boma, info.cat, info.status_kind, info.situation_kind, info.fighter_kind, info.stick_x, info.stick_y, info.facing);
}

pub unsafe fn weapon_edits(fighter: &mut L2CFighterBase, info: &WeaponFrameInfo) {
    let boma = &mut *info.boma;

}

/* Notes on is_ready_go and the logic here

is_ready_go returns true when you (the player) have control over your character.
by creating two statics and comparing them we can determine when the game switches from a state
where you don't have control of the character (menus, loading, even training mode reset, anything that isn't technically "ingame")
we can determine the "start" (or end) of a match/game/gameplay session

In addition, is_ready go returns false for a few frames at the beginning of loading into training mode. It also returns false for the duration of the
Ready.... Go! sequence at the beginning of a match.

*/
unsafe fn handle_game_resets(boma: &mut app::BattleObjectModuleAccessor, fighter: &mut L2CFighterCommon) {
    let id = hdr::get_player_number(boma);
    //static vars don't get re-initialized if they've already been
    static mut last_ready_go: [bool;8] = [false;8];
    static mut is_ready_go: [bool;8] = [true;8];

    is_ready_go[id] = hdr::is_ready_go();

    //THIS BLOCK RUNS WHEN A "SESSION" ENDS
    if !is_ready_go[id] && last_ready_go[id]
    {
        //println!("---------------- GAME END --------------");
    }
    //THIS BLOCK RUNS WHEN A "SESSION" BEGINS
    else if is_ready_go[id] && !last_ready_go[id]
    {
        //println!("---------------- GAME START --------------");
        VarModule::reset(fighter.battle_object, VarModule::RESET_ALL);

        //~~replacing char-specific status scripts... kinda a hacky way, but its not too bad lul. We "should" find a good place to hook to put this in, but like this works perfectly fine for now~~
        // IT HAS BEEN DONE, HACKY NO MORE
        //crate::status_scripts::character_specific::character_specific_status_script_replacements(app::utility::get_kind(boma), &fighter.agent);


        // Initialize base dash and run speeds at the start of the match
        //println!("Initializing base dash and run speeds...");
        //VarModule::set_float(get_battle_object_from_accessor(boma), vars::common::BASE_DASH_SPEED, value_here)  WorkModule::get_param_float(boma, hash40("dash_speed"), 0);
        //VarModule::set_float(get_battle_object_from_accessor(boma), vars::common::BASE_RUN_SPEED_MAX, value_here)  WorkModule::get_param_float(boma, hash40("run_speed_max"), 0);

        //Momentum transfer helper
        //Initialize ratio using base values (jump_speed_x_max / run_speed_max) once at beginning of match
        let ratio = (WorkModule::get_param_float(boma, hash40("jump_speed_x_max"), 0) / WorkModule::get_param_float(boma, hash40("run_speed_max"), 0));
        VarModule::set_float(fighter.battle_object, common::MP_SPEED_RATIO, ratio);

    }
    last_ready_go[id] = is_ready_go[id];
}
