use smash::app::lua_bind::*;
use smash::app::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use utils::consts::*;
use utils::ext::*;
use utils::*;

use globals::*;

#[skyline::hook(offset = 0xf13ddc, inline)]
unsafe fn steve_parry_stuff_fix(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[0].x.as_ref() == 0x1D {
        *((ctx as *mut _ as *mut u8).add(0x100).add(0x98) as *mut u32) = 0x1;
    }
}

#[skyline::hook(offset = 0x641814, inline)]
unsafe fn shield_damage_analog(ctx: &skyline::hooks::InlineCtx) {
    let boma =
        *(*ctx.registers[0].x.as_ref() as *const u64).add(1) as *mut BattleObjectModuleAccessor;
    let current_shield = WorkModule::get_float(boma, 6);
    let attack_power = *(*ctx.registers[19].x.as_ref() as *const f32).add(0xf730 / 4);
    let analog = InputModule::get_analog_for_guard((*boma).object());
    let damage_mul = WorkModule::get_param_float(
        boma,
        smash::hash40("common"),
        smash::hash40("shield_damage_mul"),
    );
    let damage_mul = if analog > 0.0 && analog < 1.0 {
        damage_mul + 0.2 * (1.0 - analog)
    } else {
        damage_mul
    };
    WorkModule::set_float(boma, current_shield - attack_power * damage_mul, 6);
}

#[skyline::hook(offset = 0x6285f0, inline)]
unsafe fn shield_pushback_analog(ctx: &skyline::hooks::InlineCtx) {
    let fighter = *ctx.registers[19].x.as_ref();
    let boma = *(fighter as *const u64).add(4);
    let attack_module: u64 = *(boma as *const u64).add(0xa0 / 8);
    let transactor_count: u64 = *(attack_module as *const u64).add(0x20 / 8);
    let transactors: u64 = *(attack_module as *const u64).add(0x28 / 8);

    let mul = WorkModule::get_param_float(
        boma as _,
        smash::hash40("common"),
        smash::hash40("shield_rebound_speed_mul"),
    );

    for x in 0..transactor_count {
        let transactor = transactors + 720 * x;
        let p_list = *(transactor as *const u64).add(608 / 8);
        if p_list == 0 {
            continue;
        }

        let mut current = *(p_list as *const u64);
        while current != p_list && current != 0 {
            if *(current as *const u8).add(47) == 2 {
                let battle_object_id = *(current as *const u32).add(36 / 4);
                let object = utils::util::get_battle_object_from_id(battle_object_id);
                let analog = InputModule::get_analog_for_guard(object);
                let mul = if analog > 0.0 && analog < 1.0 {
                    mul * analog * 0.1
                } else {
                    mul
                };
                std::arch::asm!("fmov s0, w8", in("w8") mul);
                return;
            }

            current = *(current as *const u64);
        }
    }
    std::arch::asm!("fmov s0, w8", in("w8") mul);
}

pub fn install() {
    smashline::Agent::new("fighter")
        .on_start(fighter_reset)
        .install();
    // skyline::patching::Patch::in_text(0x6417f4).nop();
    // skyline::patching::Patch::in_text(0x6285d0).nop();
    skyline::install_hooks!(
        steve_parry_stuff_fix,
        set_hit_team_hook,
        set_hit_team_second_hook,
        set_team_second_hook,
        set_team_hook,
        set_team_owner_id_hook,
        ptrainer_swap_backwards_hook,
        ptrainer_stub_death_switch,
        // shield_damage_analog,
        // shield_pushback_analog
        //set_hit_team_hook,
        hero_rng_hook,
        psych_up_hit,
        donkey_link_event,
        krool_belly_damage_hook,
    );
}

#[skyline::hook(replace=TeamModule::set_hit_team)]
unsafe fn set_hit_team_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
    original!()(boma, arg2);
    if (boma.kind() == *ITEM_KIND_BARREL) {
        //println!("set hit team called for barrel: {:x}", arg2);
        return;
    }
}

#[skyline::hook(replace=TeamModule::set_hit_team_second)]
unsafe fn set_hit_team_second_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
    original!()(boma, arg2);
    if (boma.is_item()
    && boma.kind() == *ITEM_KIND_BARREL) {
        //println!("set hit team second called for barrel: {:x}", arg2);
        return;
    }
}
/// used to ignore setting the team for barrel. This resolves an issue
/// where, when someone throws barrel upwards or forwards, they are
/// able to be hit by their own barrel for 1 frame. This is here
/// because editing item statuses is not possible
#[skyline::hook(replace=TeamModule::set_team)]
unsafe fn set_team_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32, arg3: bool) {
    if (boma.is_item() 
      && boma.kind() == *ITEM_KIND_BARREL) {
        //println!("set team ignored for barrel: {:x}", arg2);
    } else {
        original!()(boma, arg2, arg3);
    }
}

#[skyline::hook(replace=TeamModule::set_team_second)]
unsafe fn set_team_second_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
    original!()(boma, arg2);
    // if (boma.is_item()
    // && boma.kind() == *ITEM_KIND_BARREL) {
    //     //println!("set team second called for barrel: {:x}", arg2);
    //     return;
    // }
}

#[skyline::hook(replace=TeamModule::set_team_owner_id)]
unsafe fn set_team_owner_id_hook(boma: &mut BattleObjectModuleAccessor, arg2: i32) {
    original!()(boma, arg2);
    if (boma.is_item()
    && boma.kind() == *ITEM_KIND_BARREL) {
        //println!("set team owner id called for barrel: {:x}", arg2);
        return;
    }
}

pub extern "C" fn fighter_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let ratio =
            (WorkModule::get_param_float(fighter.module_accessor, hash40("jump_speed_x_max"), 0)
                / WorkModule::get_param_float(fighter.module_accessor, hash40("run_speed_max"), 0));
        VarModule::set_float(
            fighter.battle_object,
            vars::common::instance::JUMP_SPEED_RATIO,
            ratio,
        );
        if fighter.kind() == *FIGHTER_KIND_KEN
            || fighter.kind() == *FIGHTER_KIND_RYU
            || fighter.kind() == *FIGHTER_KIND_DOLLY
        {
            MeterModule::reset(fighter.battle_object);
        }
    }

}

extern "C" {
    #[link_name = "_ZN3app24FighterSpecializer_Brave23special_lw_open_commandERNS_7FighterE"]
    fn special_lw_open_command();
}

extern "C" {
    #[link_name = "hero_rng_hook_impl"]
    fn hero_rng_hook_impl(fighter: *mut BattleObject);
}

extern "C" {
    #[link_name = "krool_belly_damage_hook_impl"]
    fn krool_belly_damage_hook_impl(damage: f32, fighter: *mut Fighter, unk: bool);
}

#[skyline::hook(replace = special_lw_open_command)]
pub unsafe fn hero_rng_hook(fighter: *mut BattleObject) {
    hero_rng_hook_impl(fighter);
}

#[skyline::hook(offset = 0x993ee0)]
pub unsafe extern "C" fn donkey_link_event(vtable: u64, fighter: &mut Fighter, event: &mut smash2::app::LinkEvent) -> u64 {
    // param_3 + 0x10
    if event.link_event_kind.0 == hash40("capture") {
        // println!("hi");
        let capture_event : &mut smash2::app::LinkEventCapture = std::mem::transmute(event);
        let module_accessor = fighter.battle_object.module_accessor;
        if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            // param_3[0x28]
            capture_event.result = true;
            // capture_event.constraint = false;
            // param_3 + 0x30
            capture_event.node = smash2::phx::Hash40::new("throw");
            StatusModule::change_status_request(module_accessor, *FIGHTER_STATUS_KIND_CATCH_PULL, false);
            return 0;
        }
        return 1;
    }
    original!()(vtable, fighter, event)
}

#[skyline::hook(offset = 0x853e10)]
pub unsafe fn psych_up_hit() {
    // do nothing
}

// #[skyline::hook(offset = 0xc050d8, inline)]
// pub unsafe fn krool_belly_toggle_hook(ctx: &mut skyline::hooks::InlineCtx) {
//     krool_belly_toggle_hook_impl(ctx);
// }

#[skyline::hook(offset = 0xc055f0)]
pub unsafe fn krool_belly_damage_hook(damage: f32, fighter: *mut Fighter, unk: bool) {
    krool_belly_damage_hook_impl(damage, fighter, unk);
}

#[skyline::hook(offset = 0x34ce8e4, inline)]
unsafe fn ptrainer_swap_backwards_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let object = *ctx.registers[20].x.as_ref() as *mut BattleObject;
    if VarModule::is_flag(object, vars::ptrainer::instance::IS_SWITCH_BACKWARDS) {
        let new = match *ctx.registers[8].x.as_ref() {
            0 => 1,
            1 => 2,
            2 => 0,
            _ => unreachable!()
        };

        *ctx.registers[8].x.as_mut() = new;
    }
}

#[skyline::hook(offset = 0xf96330)]
unsafe fn ptrainer_stub_death_switch() {}