use smash::app::lua_bind::*;
use smash::app::*;
use smash::phx::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use utils::consts::*;
use utils::ext::*;
use utils::*;
use utils::game_modes::CustomMode;
use smashline::*;

use globals::*;

#[skyline::hook(offset = 0xf13ddc, inline)]
unsafe fn steve_parry_stuff_fix(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[0].x.as_ref() == 0x1D {
        *((ctx as *mut _ as *mut u8).add(0x100).add(0x98) as *mut u32) = 0x1;
    }
}

// #[skyline::hook(offset = 0x641814, inline)]
// unsafe fn shield_damage_analog(ctx: &skyline::hooks::InlineCtx) {
//     let boma =
//         *(*ctx.registers[0].x.as_ref() as *const u64).add(1) as *mut BattleObjectModuleAccessor;
//     let current_shield = WorkModule::get_float(boma, 6);
//     let attack_power = *(*ctx.registers[19].x.as_ref() as *const f32).add(0xf730 / 4);
//     let analog = InputModule::get_analog_for_guard((*boma).object());
//     let damage_mul = WorkModule::get_param_float(
//         boma,
//         smash::hash40("common"),
//         smash::hash40("shield_damage_mul"),
//     );
//     let damage_mul = if analog > 0.0 && analog < 1.0 {
//         damage_mul + 0.2 * (1.0 - analog)
//     } else {
//         damage_mul
//     };
//     WorkModule::set_float(boma, current_shield - attack_power * damage_mul, 6);
// }

// #[skyline::hook(offset = 0x6285f0, inline)]
// unsafe fn shield_pushback_analog(ctx: &skyline::hooks::InlineCtx) {
//     let fighter = *ctx.registers[19].x.as_ref();
//     let boma = *(fighter as *const u64).add(4);
//     let attack_module: u64 = *(boma as *const u64).add(0xa0 / 8);
//     let transactor_count: u64 = *(attack_module as *const u64).add(0x20 / 8);
//     let transactors: u64 = *(attack_module as *const u64).add(0x28 / 8);

//     let mul = WorkModule::get_param_float(
//         boma as _,
//         smash::hash40("common"),
//         smash::hash40("shield_rebound_speed_mul"),
//     );

//     for x in 0..transactor_count {
//         let transactor = transactors + 720 * x;
//         let p_list = *(transactor as *const u64).add(608 / 8);
//         if p_list == 0 {
//             continue;
//         }

//         let mut current = *(p_list as *const u64);
//         while current != p_list && current != 0 {
//             if *(current as *const u8).add(47) == 2 {
//                 let battle_object_id = *(current as *const u32).add(36 / 4);
//                 let object = utils::util::get_battle_object_from_id(battle_object_id);
//                 let analog = InputModule::get_analog_for_guard(object);
//                 let mul = if analog > 0.0 && analog < 1.0 {
//                     mul * analog * 0.1
//                 } else {
//                     mul
//                 };
//                 std::arch::asm!("fmov s0, w8", in("w8") mul);
//                 return;
//             }

//             current = *(current as *const u64);
//         }
//     }
//     std::arch::asm!("fmov s0, w8", in("w8") mul);
// }

pub fn install() {
    smashline::Agent::new("fighter")
        .on_start(fighter_reset)
        .on_line(Main, turbo_mode)
        .on_line(Main, hitfall_mode)
        .on_line(Main, airdash_mode)
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
        // shield_damage_analog,
        // shield_pushback_analog
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

pub extern "C" fn turbo_mode(fighter: &mut L2CFighterCommon) {
    unsafe {
        match utils::game_modes::get_custom_mode() {
            Some(modes) => {
                if modes.contains(&CustomMode::TurboMode) {
                    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                        // enable turbo behavior
                        CancelModule::enable_cancel(fighter.boma());
                        //println!("enabled cancelling!");

                        if fighter.is_situation(*SITUATION_KIND_GROUND) {
                            fighter.sub_wait_ground_check_common(false.into());
                        } else {
                            fighter.sub_air_check_fall_common();
                        }
                    }
                }
            },
            _ => {}
        }
    }
}

pub extern "C" fn hitfall_mode(fighter: &mut L2CFighterCommon) {
    unsafe {
        match utils::game_modes::get_custom_mode() {
            Some(modes) => {
                if modes.contains(&CustomMode::HitfallMode) {
                    fighter.check_hitfall();
                }
            },
            _ => {}
        }
    }
}

pub extern "C" fn airdash_mode(fighter: &mut L2CFighterCommon) {
    unsafe {
        match utils::game_modes::get_custom_mode() {
            Some(modes) => {
                if modes.contains(&CustomMode::AirdashMode) {
                    check_airdash(fighter);
                }
            },
            _ => {}
        }
    }
}

unsafe fn check_airdash(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_ESCAPE_AIR) {
        if fighter.status_frame() < 1 {
            let speed_x = KineticModule::get_sum_speed_x(fighter.boma(), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let speed_y = KineticModule::get_sum_speed_y(fighter.boma(), *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            // lets make sure not to divide by zero
            let speed_x_adjust = if speed_x == 0.0 {
                0.01
            }
            else {
                0.0
            };
            let angle = (speed_y/(speed_x + speed_x_adjust)).atan();

            let pos = Vector3f { x: 0., y: 3., z: 0.};
            let mut rot = Vector3f { x:0., y:0., z: (90. + 180. * angle/3.14159)};

            if speed_x > 0. {
                EffectModule::req_on_joint(fighter.boma(), Hash40::new("sys_whirlwind_r"), Hash40::new("top"),
                &pos, &rot, 0.75, &Vector3f{x:0.0, y:0.0, z:0.0}, &Vector3f{x:0.0, y:0.0, z:0.0}, false, 0, 0, 0);
            }
            else{
                rot = Vector3f { x:0., y:0., z: (-90. + 180. * angle/3.14159)};
                EffectModule::req_on_joint(fighter.boma(), Hash40::new("sys_whirlwind_l"), Hash40::new("top"),
                &pos, &rot, 0.75, &Vector3f{x:0.0, y:0.0, z:0.0}, &Vector3f{x:0.0, y:0.0, z:0.0}, false, 0, 0, 0);
            }
        }

        CancelModule::enable_cancel(fighter.boma());
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            fighter.sub_air_check_fall_common();
        }
    }
}
