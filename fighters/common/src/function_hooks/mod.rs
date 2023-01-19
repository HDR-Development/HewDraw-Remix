use super::*;
use crate::globals::*;
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
pub mod directional_influence;
pub mod hitstun;
pub mod change_status;
pub mod is_flag;
pub mod controls;
pub mod jumps;
pub mod stage_hazards;
pub mod set_fighter_status_data;
pub mod attack;
pub mod collision;
pub mod camera;


/* Moves that should bypass the momentum logic (in terms of the jump status script) */
const EARLY_MAIN_STATUS_RUN_EXCEPTION_KINDS: [smash::lib::LuaConst ; 14] = [
    WEAPON_KIND_PICKEL_PLATE,
    WEAPON_KIND_SIMON_WHIP,
    WEAPON_KIND_RICHTER_WHIP,
    WEAPON_KIND_MASTER_SWORD,
    WEAPON_KIND_LUCAS_HIMOHEBI,
    WEAPON_KIND_SZEROSUIT_WHIP,
    WEAPON_KIND_SZEROSUIT_WHIP2,
    WEAPON_KIND_SAMUS_GBEAM,
    WEAPON_KIND_SAMUSD_GBEAM,
    WEAPON_KIND_SHIZUE_FISHINGLINE,
    WEAPON_KIND_JACK_WIREROPE,
    WEAPON_KIND_PFUSHIGISOU_VINE,
    WEAPON_KIND_TOONLINK_HOOKSHOT,
    WEAPON_KIND_YOUNGLINK_HOOKSHOT
];

// This group of functions is normally run after KineticModule::UpdateEnergy and GroundCollision::process
// Calls MAIN status script, and associated functions
unsafe fn run_main_status_original(boma: *mut BattleObjectModuleAccessor, is_stop: bool, is_skip: bool) {
    if !is_stop {
        let area_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0xc0 / 8) as *const u64;
        let vtable = *(area_module as *const *const u64);
        let area_module__something: extern "C" fn(*const u64) = std::mem::transmute(*(((vtable as u64) + 0x68) as *const u64));
        area_module__something(area_module);

        let item_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0xc8 / 8) as *const u64;
        let vtable = *(item_module as *const *const u64);
        let item_module__something: extern "C" fn(*const u64) = std::mem::transmute(*(((vtable as u64) + 0x50) as *const u64));
        item_module__something(item_module);
    }

    let physics_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x80 / 8) as *const u64;
    let vtable = *(physics_module as *const *const u64);
    let physics_module__update_rope_matrix: extern "C" fn(*const u64, bool, bool) = std::mem::transmute(*(((vtable as u64) + 0x60) as *const u64));
    physics_module__update_rope_matrix(physics_module, false, false);

    if !is_stop {
        let motion_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x88 / 8) as *const u64;
        let vtable = *(motion_module as *const *const u64);
        let motion_module__something: extern "C" fn(*const u64) = std::mem::transmute(*(((vtable as u64) + 0x208) as *const u64));
        motion_module__something(motion_module);
    }

    let ground_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x58 / 8) as *const u64;
    let vtable = *(ground_module as *const *const u64);
    let ground_module__get_correct: extern "C" fn(*const u64) -> i32 = std::mem::transmute(*(((vtable as u64) + 0x158) as *const u64));
    let is_correct = ground_module__get_correct(ground_module);

    if is_correct != 0 {
        let ground_module__get_touch_pos: extern "C" fn(*const u64, u64) = std::mem::transmute(*(((vtable as u64) + 0x4a0) as *const u64));
        ground_module__get_touch_pos(ground_module, 8);

        let effect_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x140 / 8) as *const u64;
        let vtable = *(effect_module as *const *const u64);
        let effect_module__idk: extern "C" fn(*const u64) = std::mem::transmute(*(((vtable as u64) + 0x3e0) as *const u64));
        effect_module__idk(effect_module);
    }

    let status_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x40 / 8) as *const u64;
    let vtable = *(status_module as *const *const u64);
    let status_module__run_lua_status: extern "C" fn(*const u64) = std::mem::transmute(*(((vtable as u64) + 0x68) as *const u64));
    status_module__run_lua_status(status_module);

    let something_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0xe8 / 8) as *const u64;
    let vtable = *(something_module as *const *const u64);
    let something_module__idk: extern "C" fn(*const u64, u64, u64) = std::mem::transmute(*(((vtable as u64) + 0x48) as *const u64));
    something_module__idk(something_module, is_stop as u64, is_skip as u64);

    let effect_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x140 / 8) as *const u64;
    let vtable = *(effect_module as *const *const u64);
    let effect_module__idk: extern "C" fn(*const u64, u64) = std::mem::transmute(*(((vtable as u64) + 0x58) as *const u64));
    effect_module__idk(effect_module, 1);
}

// This runs immediately before KineticModule::UpdateEnergy is called
#[skyline::hook(offset = 0x3a82b0, inline)]
unsafe fn kinetic_module__call_update_energy_hook(ctx: &skyline::hooks::InlineCtx) {
    let boma = *ctx.registers[23].x.as_ref() as *mut BattleObjectModuleAccessor;

    if (*boma).is_weapon()
    && EARLY_MAIN_STATUS_RUN_EXCEPTION_KINDS.iter().any(|x| **x == (*boma).kind() ) {
        return;
    }

    let stop_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x90 / 8) as *const u64;
    let vtable = *(stop_module as *const *const u64);
    let stop_module__is_stop: extern "C" fn(*const u64) -> bool = std::mem::transmute(*(((vtable as u64) + 0x88) as *const u64));
    let is_stop = stop_module__is_stop(stop_module);
    
    let slow_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x170 / 8) as *const u64;
    let vtable = *(slow_module as *const *const u64);
    let slow_module__is_skip: extern "C" fn(*const u64) -> bool = std::mem::transmute(*(((vtable as u64) + 0xb0) as *const u64));
    let is_skip = slow_module__is_skip(slow_module);

    // Calling this BEFORE kinetic energy updates and stage collision is processed allows us to bypass the "1f physics delay"
    run_main_status_original(boma, is_stop, is_skip);
}

// This runs immediately before KineticModule::UpdateEnergy is called, during hitlag
#[skyline::hook(offset = 0x3a8168, inline)]
unsafe fn kinetic_module__call_update_energy_stop_hook(ctx: &skyline::hooks::InlineCtx) {
    let boma = *ctx.registers[23].x.as_ref() as *mut BattleObjectModuleAccessor;

    if (*boma).is_weapon()
    && EARLY_MAIN_STATUS_RUN_EXCEPTION_KINDS.iter().any(|x| **x == (*boma).kind() ) {
        return;
    }

    let stop_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x90 / 8) as *const u64;
    let vtable = *(stop_module as *const *const u64);
    let stop_module__is_stop: extern "C" fn(*const u64) -> bool = std::mem::transmute(*(((vtable as u64) + 0x88) as *const u64));
    let is_stop = stop_module__is_stop(stop_module);
    
    let slow_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x170 / 8) as *const u64;
    let vtable = *(slow_module as *const *const u64);
    let slow_module__is_skip: extern "C" fn(*const u64) -> bool = std::mem::transmute(*(((vtable as u64) + 0xb0) as *const u64));
    let is_skip = slow_module__is_skip(slow_module);

    // Calling this BEFORE kinetic energy updates and stage collision is processed allows us to bypass the "1f physics delay"
    run_main_status_original(boma, is_stop, is_skip);
}

// This runs after KineticModule::UpdateEnergy and GroundCollision::process
#[skyline::hook(offset = 0x3a85b4, inline)]
unsafe fn run_lua_status_hook(ctx: &skyline::hooks::InlineCtx) {
    let boma = *ctx.registers[22].x.as_ref() as *mut BattleObjectModuleAccessor;
    
    let stop_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x90 / 8) as *const u64;
    let vtable = *(stop_module as *const *const u64);
    let stop_module__is_stop: extern "C" fn(*const u64) -> bool = std::mem::transmute(*(((vtable as u64) + 0x88) as *const u64));
    let is_stop = stop_module__is_stop(stop_module);
    
    let slow_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x170 / 8) as *const u64;
    let vtable = *(slow_module as *const *const u64);
    let slow_module__is_skip: extern "C" fn(*const u64) -> bool = std::mem::transmute(*(((vtable as u64) + 0xb0) as *const u64));
    let is_skip = slow_module__is_skip(slow_module);

    if (*boma).is_weapon()
    && EARLY_MAIN_STATUS_RUN_EXCEPTION_KINDS.iter().any(|x| **x == (*boma).kind() ) {
        run_main_status_original(boma, is_stop, is_skip);
        return;
    }

    // Reset airtime counter when your situation kind is changed, rather than when entering a landing status
    if (*boma).is_fighter()
    && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR
    && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
    {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR);
    }

    // No need to check for motion kind changes if we are:
    // 1. Not currently detecting surface collision
    // 2. Neither on the ground nor in the air
    if GroundModule::get_correct(boma) == *GROUND_CORRECT_KIND_NONE
    || ![*SITUATION_KIND_GROUND, *SITUATION_KIND_AIR].contains(&StatusModule::situation_kind(boma))
    {
        return;
    }

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

    // This check passes only on the first frame you come into contact with a surface (ground/wall/ceiling)
    if *(prev_collision_line_up as *mut u64) == 0 && *(collision_line_up as *mut u64) != 0
    || *(prev_collision_line_left as *mut u64) == 0 && *(collision_line_left as *mut u64) != 0
    || *(prev_collision_line_right as *mut u64) == 0 && *(collision_line_right as *mut u64) != 0
    || *(prev_collision_line_down as *mut u64) == 0 && *(collision_line_down as *mut u64) != 0 {
        // This runs the MAIN status once, ignoring sub-statuses, to ensure we change motion kind when coming into contact with a surface
        // Otherwise, our motion kind will update a frame late (e.g. landing animation)
        if VarModule::has_var_module((*boma).object()) { VarModule::on_flag((*boma).object(), vars::common::instance::CHECK_CHANGE_MOTION_ONLY); }
        run_main_status_original(boma, is_stop, is_skip);
        if VarModule::has_var_module((*boma).object()) { VarModule::off_flag((*boma).object(), vars::common::instance::CHECK_CHANGE_MOTION_ONLY); }
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
    jumps::install();
    stage_hazards::install();
    set_fighter_status_data::install();
    attack::install();
    collision::install();
    camera::install();

    unsafe {
        // Handles getting rid of the kill zoom
        const KILL_ZOOM_DATA: u32 = 0xD503201Fu32;
        skyline::patching::Patch::in_text(utils::offsets::kill_zoom_regular()).nop();
        skyline::patching::Patch::in_text(utils::offsets::kill_zoom_throw()).data(KILL_ZOOM_DATA);
        // Changes full hops to calculate vertical velocity identically to short hops
        skyline::patching::Patch::in_text(0x6d2188).data(0x52800015u32);        

        // removes phantoms
        skyline::patching::Patch::in_text(0x3e6ce8).data(0x14000012u32);
        
        // Stubs MAIN status execution functions
        // These functions are run conditionally in run_main_status_original
        skyline::patching::Patch::in_text(0x3a8518).nop();
        skyline::patching::Patch::in_text(0x3a8528).nop();
        skyline::patching::Patch::in_text(0x3a8540).nop();
        skyline::patching::Patch::in_text(0x3a8568).nop();
        skyline::patching::Patch::in_text(0x3a859c).nop();
        skyline::patching::Patch::in_text(0x3a85b0).nop();
        skyline::patching::Patch::in_text(0x3a85c0).nop();
        skyline::patching::Patch::in_text(0x3a85d8).nop();
        skyline::patching::Patch::in_text(0x3a85f0).nop();
    }
    skyline::install_hooks!(
        kinetic_module__call_update_energy_hook,
        kinetic_module__call_update_energy_stop_hook,
        run_lua_status_hook,
    );
}
