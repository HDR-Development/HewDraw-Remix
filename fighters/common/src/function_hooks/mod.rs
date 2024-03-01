use super::*;
use crate::globals::*;
use std::arch::asm;
pub mod energy;
pub mod effect;
pub mod edge_slipoffs;
pub mod ledges;
pub mod get_param;
pub mod change_motion;
pub mod transition;
pub mod djcancel;
pub mod init_settings;
pub mod momentum_transfer;
pub mod hitstun;
pub mod change_status;
pub mod is_flag;
pub mod controls;
pub mod misc;
pub mod jumps;
pub mod stage_hazards;
pub mod set_fighter_status_data;
pub mod attack;
pub mod collision;
pub mod camera;
pub mod shotos;
pub mod aura;
pub mod sound;
mod fighterspecializer;
mod fighter_util;
mod vtables;

#[repr(C)]
pub struct TempModule {
    vtable: *const u64,
    // ...
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModuleAccessor {
    vtable: *const *const u64,
    battle_object_id: u32,
    padding: u32,
    event_manager: [u8; 0x28],
    posture_module: &'static TempModule,
    status_module: &'static TempModule,
    control_module: &'static TempModule,
    work_module: &'static TempModule,
    ground_module: &'static TempModule,
    camera_module: &'static TempModule,
    kinetic_module: &'static TempModule,
    color_blend_module: &'static TempModule,
    model_module: &'static TempModule,
    physics_module: &'static TempModule,
    motion_module: &'static TempModule,
    stop_module: &'static TempModule,
    article_module: &'static TempModule,
    attack_module: &'static TempModule,
    damage_module: &'static TempModule,
    hit_module: &'static TempModule,
    combo_module: &'static TempModule,
    area_module: &'static TempModule,
    item_module: &'static TempModule,
    link_module: &'static TempModule,
    team_module: &'static TempModule,
    search_module: &'static TempModule,
    unk1_module: &'static TempModule,
    turn_module: &'static TempModule,
    reflect_module: &'static TempModule,
    shield_module: &'static TempModule,
    reflector_module: &'static TempModule,
    absorber_module: &'static TempModule,
    jostle_module: &'static TempModule,
    catch_module: &'static TempModule,
    cancel_module: &'static TempModule,
    unk2_module: &'static TempModule,
    capture_module: &'static TempModule,
    effect_module: &'static TempModule,
    sound_module: &'static TempModule,
    visibility_module: &'static TempModule,
    grab_module: &'static TempModule,
    slope_module: &'static TempModule,
    shake_module: &'static TempModule,
    slow_module: &'static TempModule,
    unk3_module: &'static TempModule,
    shadow_module: &'static TempModule,
    motion_animcmd_module: &'static TempModule,
    lua_module: &'static TempModule,
    ink_paint_module: &'static TempModule
}

// Articles that should bypass running their MAIN status before KineticModule::UpdateEnergy and GroundCollision::process
const EXCEPTION_WEAPON_KINDS: [smash::lib::LuaConst ; 15] = [
    WEAPON_KIND_PICKEL_PLATE,
    WEAPON_KIND_MASTER_SWORD,
    WEAPON_KIND_LUCAS_HIMOHEBI,
    WEAPON_KIND_SZEROSUIT_WHIP,
    WEAPON_KIND_SZEROSUIT_WHIP2,
    WEAPON_KIND_SAMUS_GBEAM,
    WEAPON_KIND_SAMUSD_GBEAM,
    WEAPON_KIND_SHIZUE_FISHINGLINE,
    WEAPON_KIND_TOONLINK_HOOKSHOT,
    WEAPON_KIND_YOUNGLINK_HOOKSHOT,
    WEAPON_KIND_JACK_DOYLE,
    WEAPON_KIND_PICKEL_FORGE,
    WEAPON_KIND_PICKEL_TROLLEY,
    WEAPON_KIND_MARIO_FIREBALL,
    WEAPON_KIND_SHIZUE_CLAYROCKET
];

// For one reason or another, the below statuses/kinds do not play well with running before energy update/ground collision
// so they must be ran using vanilla's order of operations
unsafe fn skip_early_main_status(boma: *mut BattleObjectModuleAccessor, status_kind: i32) -> bool {
    if (*boma).is_fighter()
    && ( [*FIGHTER_STATUS_KIND_AIR_LASSO,
        *FIGHTER_STATUS_KIND_AIR_LASSO_REACH,
        *FIGHTER_STATUS_KIND_AIR_LASSO_HANG,
        *FIGHTER_STATUS_KIND_AIR_LASSO_REWIND,
        *FIGHTER_STATUS_KIND_ITEM_THROW,
        *FIGHTER_STATUS_KIND_ITEM_THROW_DASH,
        *FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY,
        *FIGHTER_STATUS_KIND_FINAL].contains(&status_kind)

        || ((*boma).kind() == *FIGHTER_KIND_RICHTER
            && [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind))
        || ((*boma).kind() == *FIGHTER_KIND_SIMON
            && [*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_STATUS_KIND_ATTACK_HI3, *FIGHTER_STATUS_KIND_ATTACK_S3, *FIGHTER_STATUS_KIND_ATTACK_HI4, *FIGHTER_STATUS_KIND_ATTACK_S4, *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind))
        || ((*boma).kind() == *FIGHTER_KIND_MASTER
            && [*FIGHTER_MASTER_STATUS_KIND_SPECIAL_N_MAX_SHOOT, *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind))
        || ((*boma).kind() == *FIGHTER_KIND_KIRBY
            && [*FIGHTER_KIRBY_STATUS_KIND_MASTER_SPECIAL_N_MAX_SHOOT].contains(&status_kind))
        || ((*boma).kind() == *FIGHTER_KIND_JACK
            && [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind))
        || ((*boma).kind() == *FIGHTER_KIND_PFUSHIGISOU
            && [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind))
        || ((*boma).kind() == *FIGHTER_KIND_KAMUI
            && [*FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B].contains(&status_kind))
        || ((*boma).kind() == *FIGHTER_KIND_INKLING
            && [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_DASH, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_RUN_TURN, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_WALK_TURN, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_S_END].contains(&status_kind))
        || ((*boma).kind() == *FIGHTER_KIND_DOLLY
            && [*FIGHTER_STATUS_KIND_FINAL, *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK].contains(&status_kind))
        || ((*boma).kind() == *FIGHTER_KIND_PICKEL
            && [*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WAIT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_FALL, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_FALL_AERIAL, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_LANDING, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK_BACK, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_SQUAT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_AERIAL, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_LANDING_LIGHT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_RIDE].contains(&status_kind))
        || ((*boma).kind() == *FIGHTER_KIND_MIIGUNNER
            && [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind))
        || ((*boma).kind() == *FIGHTER_KIND_MIIFIGHTER
            && [*FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S2_END, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S2_WEAK, *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_S2_ATTACK, *FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status_kind))
        || ((*boma).kind() == *FIGHTER_KIND_GEKKOUGA
            && [*FIGHTER_GEKKOUGA_STATUS_KIND_SPECIAL_S_ATTACK].contains(&status_kind))
        || ((*boma).kind() == *FIGHTER_KIND_LITTLEMAC
            && [*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START].contains(&status_kind)) )
    {
        return true;
    }

    if (*boma).is_weapon()
    && ( EXCEPTION_WEAPON_KINDS.iter().any(|x| **x == (*boma).kind() )
        || ((*boma).kind() == *WEAPON_KIND_TANTAN_SPIRALLEFT
            && [*WEAPON_TANTAN_SPIRALLEFT_STATUS_KIND_SPECIAL_HI_AIR, *WEAPON_TANTAN_SPIRALLEFT_STATUS_KIND_REACH].contains(&status_kind)) )
    {
        return true;
    }

    false
}

// This runs before GroundCollision::process
#[skyline::hook(offset = 0x3a7f70)]
unsafe fn before_collision(object: *mut BattleObject) {
    let boma = (*object).module_accessor;
    let module_accessor: *mut ModuleAccessor = std::mem::transmute((*object).module_accessor);
    let module_accessor = *module_accessor;

    if VarModule::has_var_module((*boma).object()) { VarModule::on_flag((*boma).object(), vars::common::instance::BEFORE_GROUND_COLLISION); }

    if skip_early_main_status(boma, StatusModule::status_kind(boma)) {
        return call_original!(object);
    }

    let stop_module__is_stop: extern "C" fn(*const TempModule) -> bool = std::mem::transmute(*(((module_accessor.stop_module.vtable as u64) + 0x88) as *const u64));
    let is_receiver_in_hitlag = stop_module__is_stop(module_accessor.stop_module);

    let battle_object_slow = utils::singletons::BattleObjectSlow() as *const u8;
    let is_slow = *((utils::singletons::BattleObjectSlow() as *const u8).add(0x8) as *const bool);

    let motion_animcmd_module__clean_coroutines: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.motion_animcmd_module.vtable as u64) + 0x50) as *const u64));
    motion_animcmd_module__clean_coroutines(module_accessor.motion_animcmd_module);

    if !is_slow || *battle_object_slow == 0 {
        let slow_module__update: extern "C" fn(*const TempModule, u64) -> bool = std::mem::transmute(*(((module_accessor.slow_module.vtable as u64) + 0x48) as *const u64));
        let is_attacker_in_hitlag = slow_module__update(module_accessor.slow_module, 1);

        let unk: i32 = if !is_attacker_in_hitlag {
            -1
        } else {
            -4
        };
        let unk2: i32 = if !is_receiver_in_hitlag {
            unk
        } else {
            24
        };

        let kinetic_module__update: extern "C" fn(*const TempModule, i32) = std::mem::transmute(*(((module_accessor.kinetic_module.vtable as u64) + 0x78) as *const u64));
        kinetic_module__update(module_accessor.kinetic_module, unk2);

        let motion_module__update: extern "C" fn(*const TempModule, u64, bool) = std::mem::transmute(*(((module_accessor.motion_module.vtable as u64) + 0xb8) as *const u64));
        motion_module__update(module_accessor.motion_module, 1, is_attacker_in_hitlag);

        let shake_module__update: extern "C" fn(*const TempModule, u64) = std::mem::transmute(*(((module_accessor.shake_module.vtable as u64) + 0x80) as *const u64));
        shake_module__update(module_accessor.shake_module, 1);

        let physics_module__update: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.physics_module.vtable as u64) + 0x50) as *const u64));
        physics_module__update(module_accessor.physics_module);

        let control_module__update: extern "C" fn(*const TempModule, bool) = std::mem::transmute(*(((module_accessor.control_module.vtable as u64) + 0x148) as *const u64));
        control_module__update(module_accessor.control_module, is_receiver_in_hitlag);

        let posture_module__update_vectors: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.posture_module.vtable as u64) + 0x158) as *const u64));
        posture_module__update_vectors(module_accessor.posture_module);

        let turn_module__update: extern "C" fn(*const TempModule, u64) = std::mem::transmute(*(((module_accessor.turn_module.vtable as u64) + 0x60) as *const u64));
        turn_module__update(module_accessor.turn_module, 1);

        let effect_module__update: extern "C" fn(*const TempModule, bool) = std::mem::transmute(*(((module_accessor.effect_module.vtable as u64) + 0x50) as *const u64));
        effect_module__update(module_accessor.effect_module, is_receiver_in_hitlag);

        let status_module__enable_lua_status: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.status_module.vtable as u64) + 0x70) as *const u64));
        status_module__enable_lua_status(module_accessor.status_module);

        let attack_module__update: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.attack_module.vtable as u64) + 0x48) as *const u64));
        attack_module__update(module_accessor.attack_module);

        let hit_module__update: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.hit_module.vtable as u64) + 0x50) as *const u64));
        hit_module__update(module_accessor.hit_module);

        let shield_module__clear_log_flag: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.shield_module.vtable as u64) + 0x48) as *const u64));
        shield_module__clear_log_flag(module_accessor.shield_module);

        let reflector_module__clear_log_flag: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.reflector_module.vtable as u64) + 0x48) as *const u64));
        reflector_module__clear_log_flag(module_accessor.reflector_module);

        let absorber_module__clear_log_flag: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.absorber_module.vtable as u64) + 0x48) as *const u64));
        absorber_module__clear_log_flag(module_accessor.absorber_module);

        let grab_module__update: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.grab_module.vtable as u64) + 0x48) as *const u64));
        grab_module__update(module_accessor.grab_module);

        let search_module__update: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.search_module.vtable as u64) + 0x48) as *const u64));
        search_module__update(module_accessor.search_module);

        let item_module__update: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.item_module.vtable as u64) + 0x48) as *const u64));
        item_module__update(module_accessor.item_module);

        let color_blend_module__update: extern "C" fn(*const TempModule, bool) = std::mem::transmute(*(((module_accessor.color_blend_module.vtable as u64) + 0x48) as *const u64));
        color_blend_module__update(module_accessor.color_blend_module, is_receiver_in_hitlag | is_attacker_in_hitlag);

        let ink_paint_module__update: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.ink_paint_module.vtable as u64) + 0x50) as *const u64));
        ink_paint_module__update(module_accessor.ink_paint_module);

        if !is_receiver_in_hitlag {
            if !is_attacker_in_hitlag {
                let combo_module__update: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.combo_module.vtable as u64) + 0x50) as *const u64));
                combo_module__update(module_accessor.combo_module);

                let status_module__call_line_system: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.status_module.vtable as u64) + 0x78) as *const u64));
                status_module__call_line_system(module_accessor.status_module);
            }
            else {
                let status_module__call_line_system_stop: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.status_module.vtable as u64) + 0x80) as *const u64));
                status_module__call_line_system_stop(module_accessor.status_module);
            }

            let status_module__call_lua_line_system: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.status_module.vtable as u64) + 0x88) as *const u64));
            status_module__call_lua_line_system(module_accessor.status_module);

            if !is_slow {
                let motion_module__update_motion: extern "C" fn(f32, *const TempModule, bool) = std::mem::transmute(*(((module_accessor.motion_module.vtable as u64) + 0xd8) as *const u64));
                motion_module__update_motion(1.0, module_accessor.motion_module, is_attacker_in_hitlag);
            }
            else {
                let motion_module__update_motion_slow: extern "C" fn(*const TempModule, bool) = std::mem::transmute(*(((module_accessor.motion_module.vtable as u64) + 0xd0) as *const u64));
                motion_module__update_motion_slow(module_accessor.motion_module, is_attacker_in_hitlag);
            }

            let status_module__call_lua_line_system_post: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.status_module.vtable as u64) + 0x90) as *const u64));
            status_module__call_lua_line_system_post(module_accessor.status_module);

            let unk3: i32 = if is_attacker_in_hitlag {
                -4
            } else {
                -1
            };

            // <HDR>

            if (*boma).is_fighter() {
                let motion_kind = MotionModule::motion_kind(boma);
                if motion_kind != hash40("invalid") {
                    let motion_module__update_motion_slow: extern "C" fn(*const TempModule, u64) -> u64 = std::mem::transmute(*(((module_accessor.motion_module.vtable as u64) + 0x680) as *const u64));
                    motion_module__update_motion_slow(module_accessor.motion_module, motion_kind);

                    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma, Hash40::new_raw(motion_kind), true);

                    if MotionModule::frame(boma) + 0.0001 < cancel_frame
                    || MotionModule::prev_frame(boma) + 0.0001 >= cancel_frame
                    {
                        let motion_kind_partial = MotionModule::motion_kind_partial(boma, 1);
                        if motion_kind_partial != hash40("invalid") {
                            let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(boma, Hash40::new_raw(motion_kind_partial), true);
                            if MotionModule::frame_partial(boma, 1) + 0.0001 >= cancel_frame
                            && MotionModule::prev_frame_partial(boma, 1) + 0.0001 < cancel_frame {
                                if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER)
                                && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ASSIST)
                                && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GENESISSET)
                                && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_KNOCKOUT) {
                                    CancelModule::enable_cancel(boma);
                                }
                            }
                        }
                    }
                    else {
                        if !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HAMMER)
                        && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_ASSIST)
                        && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_GENESISSET)
                        && !WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_KNOCKOUT) {
                            CancelModule::enable_cancel(boma);
                        }
                    }
                }
            }

            let slow_module__is_skip: extern "C" fn(*const TempModule) -> bool = std::mem::transmute(*(((module_accessor.slow_module.vtable as u64) + 0xb0) as *const u64));
            let is_skip = slow_module__is_skip(module_accessor.slow_module);

            // Calling this BEFORE kinetic energy updates and stage collision is processed allows us to bypass the "1f physics delay"
            run_main_status_original(module_accessor, false, is_skip);

            // </HDR>

            let kinetic_module__update_energy: extern "C" fn(*const TempModule, i32) = std::mem::transmute(*(((module_accessor.kinetic_module.vtable as u64) + 0x80) as *const u64));
            kinetic_module__update_energy(module_accessor.kinetic_module, unk3);

            if (*boma).is_fighter() {
                // <HDR>

                // Handles double traction while your grounded speed is influenced by knockback
                // if above max walk speed
                let mut damage_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut app::KineticEnergy;
                let damage_speed_x = app::lua_bind::KineticEnergy::get_speed_x(damage_energy);
                let damage_speed_y = app::lua_bind::KineticEnergy::get_speed_y(damage_energy);
                if damage_speed_x != 0.0
                && StatusModule::status_kind(boma) <= 0x1DB  // only affects common statuses
                && (*boma).is_situation(*SITUATION_KIND_GROUND) {
                    let speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL) - KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_GROUND) - KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_EXTERN);
                    let max_walk = WorkModule::get_param_float(boma, hash40("walk_speed_max"), 0);
                    let ground_brake = WorkModule::get_param_float(boma, hash40("ground_brake"), 0);

                    if speed_x.abs() >= max_walk {
                        let mut damage_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut app::KineticEnergyNormal;
                        let extra_traction = -1.0 * ground_brake * damage_speed_x.signum();
                        let vec2 = Vector2f{x: damage_speed_x + extra_traction, y: damage_speed_y};
                        app::lua_bind::KineticEnergyNormal::set_speed(damage_energy, &vec2);
                    }
                }

                // </HDR>

                let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x6212f0);
                let battle_object__update_movement: extern "C" fn(*mut app::Fighter, bool) = std::mem::transmute(func_addr);
                battle_object__update_movement(object as *mut app::Fighter, !is_receiver_in_hitlag);

                // Prevents jostle from pushing you off of edges
                // except if you are in knockdown (to allow for pratfall combos)
                if (*boma).is_situation(*SITUATION_KIND_GROUND)
                && !(*boma).is_status_one_of(&[
                    *FIGHTER_STATUS_KIND_DOWN,
                    *FIGHTER_STATUS_KIND_DOWN_CONTINUE,
                    *FIGHTER_STATUS_KIND_DOWN_WAIT,
                    *FIGHTER_STATUS_KIND_DOWN_WAIT_CONTINUE,
                    *FIGHTER_STATUS_KIND_DOWN_DAMAGE])
                && GroundModule::get_correct(boma) == *GROUND_CORRECT_KIND_GROUND
                && KineticModule::is_enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE) {
                    let main_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    let damage_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_DAMAGE);
                    let mut jostle_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE) as *mut app::KineticEnergy;
                    let jostle_energy_x = app::lua_bind::KineticEnergy::get_speed_x(jostle_energy);

                    if jostle_energy_x != 0.0
                    && (main_speed_x + damage_speed_x).abs() < jostle_energy_x.abs() {
                        // This check passes if the speed at which your character is moving due to general movement
                        // (dashing, running, walking, grounded knockback, shield pushback, etc.)
                        // is LESS than the speed at which jostle is pushing your character
                        GroundModule::correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                        VarModule::on_flag(object, vars::common::instance::TEMPORARY_CLIFF_STOP);
                    }
                }

            }
            else if (*boma).is_weapon() {
                let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x33a6140);
                let battle_object__update_movement: extern "C" fn(*mut app::Weapon, bool) = std::mem::transmute(func_addr);
                battle_object__update_movement(object as *mut app::Weapon, !is_receiver_in_hitlag);
            }
            else {
                let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3a8f70);
                let battle_object__update_movement: extern "C" fn(*mut BattleObject, bool) = std::mem::transmute(func_addr);
                battle_object__update_movement(object, !is_receiver_in_hitlag);
            }

            let damage_module__update: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.damage_module.vtable as u64) + 0x58) as *const u64));
            damage_module__update(module_accessor.damage_module);
        }
        else {
            let status_module__call_line_system_stop: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.status_module.vtable as u64) + 0x80) as *const u64));
            status_module__call_line_system_stop(module_accessor.status_module);

            let status_module__call_lua_line_system: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.status_module.vtable as u64) + 0x88) as *const u64));
            status_module__call_lua_line_system(module_accessor.status_module);

            // <HDR>

            let slow_module__is_skip: extern "C" fn(*const TempModule) -> bool = std::mem::transmute(*(((module_accessor.slow_module.vtable as u64) + 0xb0) as *const u64));
            let is_skip = slow_module__is_skip(module_accessor.slow_module);

            // Calling this BEFORE kinetic energy updates and stage collision is processed allows us to bypass the "1f physics delay"
            run_main_status_original(module_accessor, true, is_skip);

            // </HDR>

            let kinetic_module__update_energy: extern "C" fn(*const TempModule, i32) = std::mem::transmute(*(((module_accessor.kinetic_module.vtable as u64) + 0x80) as *const u64));
            kinetic_module__update_energy(module_accessor.kinetic_module, 24);

            if (*boma).is_fighter() {
                let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x6212f0);
                let battle_object__update_movement: extern "C" fn(*mut app::Fighter, bool) = std::mem::transmute(func_addr);
                battle_object__update_movement(object as *mut app::Fighter, !is_receiver_in_hitlag);

            }
            else if (*boma).is_weapon() {
                let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x33A6140);
                let battle_object__update_movement: extern "C" fn(*mut app::Weapon, bool) = std::mem::transmute(func_addr);
                battle_object__update_movement(object as *mut app::Weapon, !is_receiver_in_hitlag);
            }
            else {
                let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3a8f70);
                let battle_object__update_movement: extern "C" fn(*mut BattleObject, bool) = std::mem::transmute(func_addr);
                battle_object__update_movement(object, !is_receiver_in_hitlag);
            }
        }
    }
    else {
        let slow_module__update: extern "C" fn(*const TempModule, u64) -> bool = std::mem::transmute(*(((module_accessor.slow_module.vtable as u64) + 0x48) as *const u64));
        slow_module__update(module_accessor.slow_module, 0);

        let control_module__update_slow: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.control_module.vtable as u64) + 0x150) as *const u64));
        control_module__update_slow(module_accessor.control_module);

        let kinetic_module__update: extern "C" fn(*const TempModule, i32) = std::mem::transmute(*(((module_accessor.kinetic_module.vtable as u64) + 0x78) as *const u64));
        kinetic_module__update(module_accessor.kinetic_module, 8);

        let shake_module__update: extern "C" fn(*const TempModule, u64) = std::mem::transmute(*(((module_accessor.shake_module.vtable as u64) + 0x80) as *const u64));
        shake_module__update(module_accessor.shake_module, 0);

        let motion_module__update: extern "C" fn(*const TempModule, u64, bool) = std::mem::transmute(*(((module_accessor.motion_module.vtable as u64) + 0xb8) as *const u64));
        motion_module__update(module_accessor.motion_module, 0, false);

        let turn_module__update: extern "C" fn(*const TempModule, u64) = std::mem::transmute(*(((module_accessor.turn_module.vtable as u64) + 0x60) as *const u64));
        turn_module__update(module_accessor.turn_module, 0);

        let physics_module__update: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.physics_module.vtable as u64) + 0x50) as *const u64));
        physics_module__update(module_accessor.physics_module);

        let kinetic_module__update_energy: extern "C" fn(*const TempModule, i32) = std::mem::transmute(*(((module_accessor.kinetic_module.vtable as u64) + 0x80) as *const u64));
        kinetic_module__update_energy(module_accessor.kinetic_module, 8);

        if (*boma).is_fighter() {
            let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x6212f0);
            let battle_object__update_movement: extern "C" fn(*mut app::Fighter, bool) = std::mem::transmute(func_addr);
            battle_object__update_movement(object as *mut app::Fighter, false);

        }
        else if (*boma).is_weapon() {
            let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x33A6140);
            let battle_object__update_movement: extern "C" fn(*mut app::Weapon, bool) = std::mem::transmute(func_addr);
            battle_object__update_movement(object as *mut app::Weapon, false);
        }
        else {
            let func_addr = (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3a8f70);
            let battle_object__update_movement: extern "C" fn(*mut BattleObject, bool) = std::mem::transmute(func_addr);
            battle_object__update_movement(object, false);
        }
    }
    if is_slow {
        let slow_rate = lua_bind::BattleObjectSlow::rate(utils::singletons::BattleObjectSlow());
        if *battle_object_slow == 2 {
            if !is_receiver_in_hitlag {
                let motion_module__update_slow: extern "C" fn(f32, *const TempModule) = std::mem::transmute(*(((module_accessor.motion_module.vtable as u64) + 0xc8) as *const u64));
                motion_module__update_slow(slow_rate, module_accessor.motion_module);
            }
        }
        else if *battle_object_slow == 1 {
            if !is_receiver_in_hitlag {
                let motion_module__update_motion: extern "C" fn(f32, *const TempModule, bool) = std::mem::transmute(*(((module_accessor.motion_module.vtable as u64) + 0xd8) as *const u64));
                motion_module__update_motion(slow_rate, module_accessor.motion_module, true);
            }
        }
        else if *battle_object_slow == 0 {
            if !is_receiver_in_hitlag {
                let motion_module__something: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.motion_module.vtable as u64) + 0xc0) as *const u64));
                motion_module__something(module_accessor.motion_module);

                let motion_module__update_motion: extern "C" fn(f32, *const TempModule, bool) = std::mem::transmute(*(((module_accessor.motion_module.vtable as u64) + 0xd8) as *const u64));
                motion_module__update_motion(slow_rate, module_accessor.motion_module, true);
            }
            let posture_module__update_slow_speed: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.posture_module.vtable as u64) + 0x160) as *const u64));
            posture_module__update_slow_speed(module_accessor.posture_module);
        }
        let posture_module__update_slow_pos: extern "C" fn(*const TempModule, f32) = std::mem::transmute(*(((module_accessor.posture_module.vtable as u64) + 0x170) as *const u64));
        posture_module__update_slow_pos(module_accessor.posture_module, slow_rate);
    }
    let sound_module__something: extern "C" fn(*const TempModule, bool) = std::mem::transmute(*(((module_accessor.sound_module.vtable as u64) + 0x50) as *const u64));
    sound_module__something(module_accessor.sound_module, is_receiver_in_hitlag);

    let physics_module__update_rope_matrix: extern "C" fn(*const TempModule, bool, bool) = std::mem::transmute(*(((module_accessor.physics_module.vtable as u64) + 0x60) as *const u64));
    physics_module__update_rope_matrix(module_accessor.physics_module, true, false);
}

// This group of functions is normally run after KineticModule::UpdateEnergy and GroundCollision::process
// Calls MAIN status script, and associated functions
unsafe fn run_main_status_original(module_accessor: ModuleAccessor, is_receiver_in_hitlag: bool, is_skip: bool) {
    if !is_receiver_in_hitlag {
        let area_module__unk: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.area_module.vtable as u64) + 0x68) as *const u64));
        area_module__unk(module_accessor.area_module);

        let item_module__unk: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.item_module.vtable as u64) + 0x50) as *const u64));
        item_module__unk(module_accessor.item_module);
    }

    let physics_module__update_rope_matrix: extern "C" fn(*const TempModule, bool, bool) = std::mem::transmute(*(((module_accessor.physics_module.vtable as u64) + 0x60) as *const u64));
    physics_module__update_rope_matrix(module_accessor.physics_module, false, false);

    if !is_receiver_in_hitlag {
        let motion_module__unk: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.motion_module.vtable as u64) + 0x208) as *const u64));
        motion_module__unk(module_accessor.motion_module);
    }

    let ground_module__get_correct: extern "C" fn(*const TempModule) -> i32 = std::mem::transmute(*(((module_accessor.ground_module.vtable as u64) + 0x158) as *const u64));
    let ground_correct_kind = ground_module__get_correct(module_accessor.ground_module);

    if ground_correct_kind != *GROUND_CORRECT_KIND_NONE {
        let ground_module__is_touch: extern "C" fn(*const TempModule, i32) -> bool = std::mem::transmute(*(((module_accessor.ground_module.vtable as u64) + 0x4a0) as *const u64));
        let is_touch = ground_module__is_touch(module_accessor.ground_module, *GROUND_TOUCH_FLAG_DOWN);

        let effect_module__unk: extern "C" fn(*const TempModule, bool) = std::mem::transmute(*(((module_accessor.effect_module.vtable as u64) + 0x3e0) as *const u64));
        effect_module__unk(module_accessor.effect_module, is_touch);
    }

    let status_module__run_lua_status: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.status_module.vtable as u64) + 0x68) as *const u64));
    status_module__run_lua_status(module_accessor.status_module);

    let unk1_module__unk: extern "C" fn(*const TempModule, bool, bool) = std::mem::transmute(*(((module_accessor.unk1_module.vtable as u64) + 0x48) as *const u64));
    unk1_module__unk(module_accessor.unk1_module, is_receiver_in_hitlag, is_skip);

    let effect_module__unk: extern "C" fn(*const TempModule, u64) = std::mem::transmute(*(((module_accessor.effect_module.vtable as u64) + 0x58) as *const u64));
    effect_module__unk(module_accessor.effect_module, 1);
}

// This runs after KineticModule::UpdateEnergy and GroundCollision::process
#[skyline::hook(offset = 0x3a84e0)]
unsafe fn after_collision(object: *mut BattleObject) {
    let boma = (*object).module_accessor;
    let module_accessor: *mut ModuleAccessor = std::mem::transmute((*object).module_accessor);
    let module_accessor = *module_accessor;

    if VarModule::has_var_module((*boma).object()) { VarModule::off_flag((*boma).object(), vars::common::instance::BEFORE_GROUND_COLLISION); }

    if skip_early_main_status(boma, StatusModule::status_kind(boma)) {
        return call_original!(object);
    }

    // Resets flag which prevents jostle edge slipoffs for next frame
    if (*boma).is_fighter()
    && VarModule::is_flag(object, vars::common::instance::TEMPORARY_CLIFF_STOP) {
        GroundModule::correct(boma, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        VarModule::off_flag(object, vars::common::instance::TEMPORARY_CLIFF_STOP);
    }

    let stop_module__is_stop: extern "C" fn(*const TempModule) -> bool = std::mem::transmute(*(((module_accessor.stop_module.vtable as u64) + 0x88) as *const u64));
    let is_receiver_in_hitlag = stop_module__is_stop(module_accessor.stop_module);

    let slow_module__is_skip: extern "C" fn(*const TempModule) -> bool = std::mem::transmute(*(((module_accessor.slow_module.vtable as u64) + 0xb0) as *const u64));
    let is_skip = slow_module__is_skip(module_accessor.slow_module);

    let battle_object_slow = utils::singletons::BattleObjectSlow() as *const u8;
    let is_slow = *((utils::singletons::BattleObjectSlow() as *const u8).add(0x8) as *const bool);

    if !is_slow || *battle_object_slow == 2 {
        // <HDR>
        if (*boma).is_fighter()
        && VarModule::is_flag((*boma).object(), vars::common::instance::IS_IGNORED_STATUS_FRAME_0)
        && skip_early_main_status(boma, StatusModule::status_kind_next(boma))
        {
            VarModule::off_flag((*boma).object(), vars::common::instance::IS_IGNORED_STATUS_FRAME_0);

            let status_module__changeStatus: extern "C" fn(*const TempModule, i32) = std::mem::transmute(*(((module_accessor.status_module.vtable as u64) + 0x1f0) as *const u64));

            *(((module_accessor.status_module as *const TempModule as u64) + 0xf4) as *mut bool) = true;  // StatusModule::is_changing = true
            status_module__changeStatus(module_accessor.status_module, StatusModule::status_kind_next(boma));
            *(((module_accessor.status_module as *const TempModule as u64) + 0xf4) as *mut bool) = false;  // StatusModule::is_changing = false

            let unk1_module__unk: extern "C" fn(*const TempModule, bool, bool) = std::mem::transmute(*(((module_accessor.unk1_module.vtable as u64) + 0x48) as *const u64));
            unk1_module__unk(module_accessor.unk1_module, is_receiver_in_hitlag, is_skip);

            let effect_module__unk: extern "C" fn(*const TempModule, u64) = std::mem::transmute(*(((module_accessor.effect_module.vtable as u64) + 0x58) as *const u64));
            effect_module__unk(module_accessor.effect_module, 1);
        }
        else {
            // Reset airtime counter when your situation kind is changed, rather than when entering a landing status
            if (*boma).is_fighter()
            && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR
            && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
            {
                WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR);
            }

            // No need to check for motion kind changes if we are:
            //   1. Not currently detecting surface collision
            //   2. Neither on the ground nor in the air
            if GroundModule::get_correct(boma) != *GROUND_CORRECT_KIND_NONE
            && [*SITUATION_KIND_GROUND, *SITUATION_KIND_AIR].contains(&StatusModule::situation_kind(boma))
            && [*SITUATION_KIND_GROUND, *SITUATION_KIND_AIR].contains(&StatusModule::prev_situation_kind(boma))
            {

                let ground_module = *(boma as *mut BattleObjectModuleAccessor as *const u64).add(0x58 / 8);
                let ground_collision_info = *((ground_module + 0x28) as *mut *mut f32);

                let prev_collision_line_up = ((ground_collision_info as u64) + 0x190) as *mut GroundCollisionLine;
                let prev_collision_line_left = ((ground_collision_info as u64) + 0x1c0) as *mut GroundCollisionLine;
                let prev_collision_line_right = ((ground_collision_info as u64) + 0x1f0) as *mut GroundCollisionLine;
                let prev_collision_line_down = ((ground_collision_info as u64) + 0x220) as *mut GroundCollisionLine;

                let collision_line_up = ((ground_collision_info as u64) + 0x10) as *mut GroundCollisionLine;
                let collision_line_left = ((ground_collision_info as u64) + 0x40) as *mut GroundCollisionLine;
                let collision_line_right = ((ground_collision_info as u64) + 0x70) as *mut GroundCollisionLine;
                let collision_line_down = ((ground_collision_info as u64) + 0xa0) as *mut GroundCollisionLine;

                // This check passes only on the first frame you come into contact with/leave a surface (ground/wall/ceiling)
                // except when jumping, as the game already changes motion earlier on
                if ( (*(prev_collision_line_up as *mut u64) == 0 && *(collision_line_up as *mut u64) != 0)
                ||   (*(prev_collision_line_left as *mut u64) == 0 && *(collision_line_left as *mut u64) != 0)
                ||   (*(prev_collision_line_right as *mut u64) == 0 && *(collision_line_right as *mut u64) != 0)
                ||   (*(prev_collision_line_down as *mut u64) != *(collision_line_down as *mut u64))
                ||   (StatusModule::situation_kind(boma) != StatusModule::prev_situation_kind(boma)) )
                && !( (*boma).is_fighter()
                    && (*boma).status_frame() == 0
                    && ((*boma).is_status(*FIGHTER_STATUS_KIND_JUMP) || (*boma).is_prev_status(*FIGHTER_STATUS_KIND_JUMP))
                    && ((*(prev_collision_line_down as *mut u64) != *(collision_line_down as *mut u64)) != (StatusModule::situation_kind(boma) != StatusModule::prev_situation_kind(boma))) )
                {
                    // This runs the MAIN status once, ignoring sub-statuses, to ensure we change motion kind when coming into contact with a surface
                    // Otherwise, our motion kind will update a frame late (e.g. landing animation)
                    if VarModule::has_var_module((*boma).object()) { VarModule::on_flag((*boma).object(), vars::common::instance::CHECK_CHANGE_MOTION_ONLY); }
                    let status_module__run_lua_status: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.status_module.vtable as u64) + 0x68) as *const u64));
                    status_module__run_lua_status(module_accessor.status_module);
                    if VarModule::has_var_module((*boma).object()) {
                        VarModule::off_flag((*boma).object(), vars::common::instance::CHECK_CHANGE_MOTION_ONLY);
                        VarModule::off_flag((*boma).object(), vars::common::instance::FLUSH_EFFECT_ACMD);
                    }
                }
            }
        }
        // </HDR>

        if !is_receiver_in_hitlag || !is_skip {
            let status_module__call_line_fix_pos2: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.status_module.vtable as u64) + 0xa0) as *const u64));
            status_module__call_line_fix_pos2(module_accessor.status_module);
        }

        let status_module__call_line_fix_pos: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.status_module.vtable as u64) + 0xa8) as *const u64));
        status_module__call_line_fix_pos(module_accessor.status_module);

        let unk3_module__unk: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.unk3_module.vtable as u64) + 0x48) as *const u64));
        unk3_module__unk(module_accessor.unk3_module);

        let slope_module__unk: extern "C" fn(*const TempModule, u64) = std::mem::transmute(*(((module_accessor.slope_module.vtable as u64) + 0x60) as *const u64));
        slope_module__unk(module_accessor.slope_module, 1);
    }
    else {
        let physics_module__update_rope_matrix: extern "C" fn(*const TempModule, bool, bool) = std::mem::transmute(*(((module_accessor.physics_module.vtable as u64) + 0x60) as *const u64));
        physics_module__update_rope_matrix(module_accessor.physics_module, false, false);

        let unk1_module__unk: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.unk1_module.vtable as u64) + 0x50) as *const u64));
        unk1_module__unk(module_accessor.unk1_module);

        let effect_module__unk: extern "C" fn(*const TempModule, u64) = std::mem::transmute(*(((module_accessor.effect_module.vtable as u64) + 0x58) as *const u64));
        effect_module__unk(module_accessor.effect_module, 0);

        let status_module__call_both_line_fix_pos_slow: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.status_module.vtable as u64) + 0xb0) as *const u64));
        status_module__call_both_line_fix_pos_slow(module_accessor.status_module);

        let slope_module__unk: extern "C" fn(*const TempModule, u64) = std::mem::transmute(*(((module_accessor.slope_module.vtable as u64) + 0x60) as *const u64));
        slope_module__unk(module_accessor.slope_module, 0);
    }

    let physics_module__unk: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.physics_module.vtable as u64) + 0x58) as *const u64));
    physics_module__unk(module_accessor.physics_module);

    let physics_module__unk2: extern "C" fn(*const TempModule) = std::mem::transmute(*(((module_accessor.physics_module.vtable as u64) + 0x78) as *const u64));
    physics_module__unk2(module_accessor.physics_module);
}

#[skyline::hook(offset = 0x4debe0)]
unsafe fn status_module__change_status(status_module: *const u64, status_kind_next: i32) {
    let boma = *(status_module as *mut *mut BattleObjectModuleAccessor).add(1);

    if (*boma).is_fighter() {
        JostleModule::set_overlap_rate_mul(boma, 1.0);
    }

    if (*boma).is_fighter()
    && skip_early_main_status(boma, status_kind_next)
    && VarModule::is_flag((*boma).object(), vars::common::instance::BEFORE_GROUND_COLLISION) {
        VarModule::on_flag((*boma).object(), vars::common::instance::IS_IGNORED_STATUS_FRAME_0);
        *(((status_module as u64) + 0x9c) as *mut i32) = status_kind_next;  // setting StatusModule::status_kind_next
        return;
    }
    call_original!(status_module, status_kind_next);
}

// Only extra elec hitlag for hit character
#[skyline::hook(offset = 0x406824, inline)]
unsafe fn change_elec_hitlag_for_attacker(ctx: &mut skyline::hooks::InlineCtx) {
  let is_attacker = *ctx.registers[4].w.as_ref() & 1 == 0;
  if *ctx.registers[8].x.as_ref() == smash::hash40("collision_attr_elec") && is_attacker {
    *ctx.registers[8].x.as_mut() = smash::hash40("collision_attr_normal");
  }
}

pub fn install() {
    energy::install();
    effect::install();
    edge_slipoffs::install();
    ledges::install();
    get_param::install();
    change_motion::install();
    transition::install();
    djcancel::install();
    init_settings::install();
    hitstun::install();
    change_status::install();
    is_flag::install();
    controls::install();
    momentum_transfer::install();
    misc::install();
    //dash_dancing::install();
    jumps::install();
    stage_hazards::install();
    set_fighter_status_data::install();
    attack::install();
    collision::install();
    camera::install();
    shotos::install();
    aura::install();
    sound::install();
    fighterspecializer::install();
    fighter_util::install();
    vtables::install();

    unsafe {
        // Handles getting rid of the kill zoom
        const KILL_ZOOM_DATA: u32 = 0xD503201Fu32;
        skyline::patching::Patch::in_text(utils::offsets::kill_zoom_regular()).nop();
        skyline::patching::Patch::in_text(utils::offsets::kill_zoom_throw()).data(KILL_ZOOM_DATA);
        // Changes full hops to calculate vertical velocity identically to short hops
        skyline::patching::Patch::in_text(0x6d21a8).data(0x52800015u32);

        // removes phantoms
        skyline::patching::Patch::in_text(0x3e6d08).data(0x14000012u32);

        // Resets projectile lifetime on parry, rather than using remaining lifetime
        skyline::patching::Patch::in_text(0x33bdfd8).nop();
        skyline::patching::Patch::in_text(0x33bdfdc).data(0x2a0a03e1);

        // The following handles disabling the "Weapon Catch" animation for those who have it.
        // You will only enter the weapon catch animation if you are completely idle.
        // Link, Young Link, Toon Link
        skyline::patching::Patch::in_text(0xc29818).data(0x7100011F);
        // Simon and Richter
        skyline::patching::Patch::in_text(0x1195224).data(0x7100001F);
        // Krool and Pyra are in their respective modules.
        // Gives attacker less clank hitlag than defender
        skyline::patching::Patch::in_text(0x3e0b48).data(0x1E204160);
    }
    skyline::install_hooks!(
        before_collision,
        after_collision,
        status_module__change_status,
        change_elec_hitlag_for_attacker
    );
}
