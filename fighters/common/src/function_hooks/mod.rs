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


#[skyline::hook(offset = 0x3a82b0, inline)]
unsafe fn battleobject__call_update_movement(ctx: &skyline::hooks::InlineCtx) {
    let boma = *ctx.registers[23].x.as_ref() as *mut BattleObjectModuleAccessor;

    let stop_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x90 / 8) as *const u64;
    let vtable = *(stop_module as *const *const u64);
    let stop_module__is_stop: extern "C" fn(*const u64) -> bool = std::mem::transmute(*(((vtable as u64) + 0x88) as *const u64));
    let is_stop = stop_module__is_stop(stop_module);
    
    let slow_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x170 / 8) as *const u64;
    let vtable = *(slow_module as *const *const u64);
    let slow_module__is_skip: extern "C" fn(*const u64) -> bool = std::mem::transmute(*(((vtable as u64) + 0xb0) as *const u64));
    let is_skip = slow_module__is_skip(slow_module);

    let area_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0xc0 / 8) as *const u64;
    let vtable = *(area_module as *const *const u64);
    let area_module__something: extern "C" fn(*const u64) = std::mem::transmute(*(((vtable as u64) + 0x68) as *const u64));
    area_module__something(area_module);

    let item_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0xc8 / 8) as *const u64;
    let vtable = *(item_module as *const *const u64);
    let item_module__something: extern "C" fn(*const u64) = std::mem::transmute(*(((vtable as u64) + 0x50) as *const u64));
    item_module__something(item_module);
    
    let physics_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x80 / 8) as *const u64;
    let vtable = *(physics_module as *const *const u64);
    let physics_module__update_rope_matrix: extern "C" fn(*const u64, bool, bool) = std::mem::transmute(*(((vtable as u64) + 0x60) as *const u64));
    physics_module__update_rope_matrix(physics_module, false, false);

    let motion_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x88 / 8) as *const u64;
    let vtable = *(motion_module as *const *const u64);
    let motion_module__something: extern "C" fn(*const u64) = std::mem::transmute(*(((vtable as u64) + 0x208) as *const u64));
    motion_module__something(motion_module);

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

#[skyline::hook(offset = 0x3a8168, inline)]
unsafe fn battleobject__call_update_movement_stop(ctx: &skyline::hooks::InlineCtx) {
    let boma = *ctx.registers[23].x.as_ref() as *mut BattleObjectModuleAccessor;

    let stop_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x90 / 8) as *const u64;
    let vtable = *(stop_module as *const *const u64);
    let stop_module__is_stop: extern "C" fn(*const u64) -> bool = std::mem::transmute(*(((vtable as u64) + 0x88) as *const u64));
    let is_stop = stop_module__is_stop(stop_module);
    
    let slow_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x170 / 8) as *const u64;
    let vtable = *(slow_module as *const *const u64);
    let slow_module__is_skip: extern "C" fn(*const u64) -> bool = std::mem::transmute(*(((vtable as u64) + 0xb0) as *const u64));
    let is_skip = slow_module__is_skip(slow_module);

    let physics_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x80 / 8) as *const u64;
    let vtable = *(physics_module as *const *const u64);
    let physics_module__update_rope_matrix: extern "C" fn(*const u64, bool, bool) = std::mem::transmute(*(((vtable as u64) + 0x60) as *const u64));
    physics_module__update_rope_matrix(physics_module, false, false);

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

#[skyline::hook(offset = 0x3a85b4, inline)]
unsafe fn run_lua_status_hook(ctx: &skyline::hooks::InlineCtx) {
    let boma = *ctx.registers[22].x.as_ref() as *mut BattleObjectModuleAccessor;

    /*let slow_num = *((utils::singletons::BattleObjectSlow() as u64) as *mut u64);
    let slow_bool = *(((utils::singletons::BattleObjectSlow() as u64) + 2) as *mut bool);
    if slow_bool && slow_num != 2 {
        println!("slow?");
    }*/

    if (*boma).is_fighter()
    && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR
    && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
    {
        WorkModule::set_int(boma, 0, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR);
    }

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

    if *(prev_collision_line_up as *mut u64) == 0 && *(collision_line_up as *mut u64) != 0
    || *(prev_collision_line_left as *mut u64) == 0 && *(collision_line_left as *mut u64) != 0
    || *(prev_collision_line_right as *mut u64) == 0 && *(collision_line_right as *mut u64) != 0
    || *(prev_collision_line_down as *mut u64) == 0 && *(collision_line_down as *mut u64) != 0 {
        let stop_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x90 / 8) as *const u64;
        let vtable = *(stop_module as *const *const u64);
        let stop_module__is_stop: extern "C" fn(*const u64) -> bool = std::mem::transmute(*(((vtable as u64) + 0x88) as *const u64));
        let is_stop = stop_module__is_stop(stop_module);
        
        let slow_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x170 / 8) as *const u64;
        let vtable = *(slow_module as *const *const u64);
        let slow_module__is_skip: extern "C" fn(*const u64) -> bool = std::mem::transmute(*(((vtable as u64) + 0xb0) as *const u64));
        let is_skip = slow_module__is_skip(slow_module);

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
        if VarModule::has_var_module((*boma).object()) { VarModule::on_flag((*boma).object(), vars::common::instance::CHECK_CHANGE_MOTION_ONLY); }
        status_module__run_lua_status(status_module);
        if VarModule::has_var_module((*boma).object()) { VarModule::off_flag((*boma).object(), vars::common::instance::CHECK_CHANGE_MOTION_ONLY); }

        let something_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0xe8 / 8) as *const u64;
        let vtable = *(something_module as *const *const u64);
        let something_module__idk: extern "C" fn(*const u64, u64, u64) = std::mem::transmute(*(((vtable as u64) + 0x48) as *const u64));
        something_module__idk(something_module, is_stop as u64, is_skip as u64);
    
        let effect_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x140 / 8) as *const u64;
        let vtable = *(effect_module as *const *const u64);
        let effect_module__idk: extern "C" fn(*const u64, u64) = std::mem::transmute(*(((vtable as u64) + 0x58) as *const u64));
        effect_module__idk(effect_module, 1);
    }
}

#[skyline::hook(offset = 0x479380)]
unsafe fn kinetic_module__update_energy(kinetic_module: u64, arg2: u64) {
    let boma = *(kinetic_module as *mut *mut BattleObjectModuleAccessor).add(1);
    if VarModule::has_var_module((*boma).object()) && VarModule::is_flag((*boma).object(), vars::common::instance::IGNORE_ENERGY_UPDATE) {
        VarModule::off_flag((*boma).object(), vars::common::instance::IGNORE_ENERGY_UPDATE);
        return;
    }
    if VarModule::has_var_module((*boma).object()) { VarModule::off_flag((*boma).object(), vars::common::instance::IGNORE_ENERGY_UPDATE); }
    call_original!(kinetic_module, arg2);
}

#[skyline::hook(offset = 0x6cdc40)]
unsafe fn kinetic_module__reset_energy(kinetic_module: u64, arg2: u64, boma: &mut BattleObjectModuleAccessor) {
    if VarModule::has_var_module(boma.object()) { VarModule::on_flag(boma.object(), vars::common::instance::IGNORE_ENERGY_UPDATE); }
    call_original!(kinetic_module, arg2, boma)
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
        // These functions are run separately in run_lua_status_hook
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
        battleobject__call_update_movement,
        battleobject__call_update_movement_stop,
        run_lua_status_hook,
        kinetic_module__update_energy,
        kinetic_module__reset_energy
    );
}
