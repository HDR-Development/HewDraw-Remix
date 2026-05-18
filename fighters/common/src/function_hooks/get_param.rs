use super::*;
use globals::*;
// Addresses, offsets, and inline hooking
use skyline::hooks::{getRegionAddress, Region, InlineCtx};
use utils::game_modes::CustomMode;

pub fn install() {
    skyline::install_hooks!(
        //get_offset,
        //get_inline_offset,
        get_param_int_hook,
        get_param_float_hook,
        //get_item_backtrace_inline,
    );
    //skyline::nro::add_hook(item_nro_hook);
}

// #[skyline::hook(offset=0x720540)]
// unsafe fn get_offset(arg0: u64, arg1: u64) {
//     static mut ONCE: bool = true;
//     if ONCE {
//         ONCE = false;
//         //debug::dump_trace();
//     }
//     original!()(arg0, arg1);
// }

// #[skyline::hook(offset=0x1f8810c, inline)]
// unsafe fn get_inline_offset(ctx: &InlineCtx) {
//     static mut ONCE: bool = true;
//     if ONCE {
//         ONCE = false;
//         println!("{:#x}", ctx.registers[3].x.as_ref() - getRegionAddress(Region::Text) as u64);
//     }
// }

#[skyline::hook(offset=0x4E53A0)]
pub unsafe fn get_param_int_hook(x0: u64, x1: u64, x2 :u64) -> i32 {
    let mut boma = *((x0 as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
    let boma_reference = &mut *boma;
    let fighter_kind = boma_reference.kind();

    // For fighters
    if boma_reference.is_fighter() {
        match utils::game_modes::get_custom_mode() {
            Some(modes) => {
                if modes.contains(&CustomMode::Smash64Mode) {
                    if x1 == hash40("landing_heavy_frame") {
                        return 4;
                    }
                }
                else if modes.contains(&CustomMode::RivalsOfAetherMode) {
                    // no SDI for RoA mode
                    if x1 == hash40("common") && x2 == hash40("hit_stop_delay_flick_max_count") {
                        return 0;
                    }

                    // universal 5F jumpsquat for RoA mode
                    if x1 == hash40("jump_squat_frame") {
                        return 5;
                    }
                }
            },
            _ => {}
        }

        if x2 == hash40("just_shield_precede_extension") {
            return 1000;
        }
    
        // if x2 == hash40("continue_just_shield_count") {
        // }
    }

    // For articles
    else if boma_reference.is_weapon() {
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

        if fighter_kind == *WEAPON_KIND_PACKUN_SPIKEBALL {
            if VarModule::is_flag(boma_reference.object(), vars::packun_spikeball::instance::ENABLE_EXPLODE) {
                if x1 == hash40("param_spikeball") { 
                    if x2 == hash40("hop_life") {
                        return 105;
                    }
                }
            }
        }

        else if fighter_kind == *WEAPON_KIND_LUCARIO_AURABALL {
            if x1 == hash40("param_auraball") {
                if VarModule::is_flag(boma_reference.object(), vars::lucario::instance::IS_POWERED_UP) {
                    if x2 == hash40("life") {
                        return 180;
                    }
                }
            }
        }
    }

    original!()(x0, x1, x2)
}

#[skyline::hook(offset=0x4E53E0)]
pub unsafe fn get_param_float_hook(x0 /*boma*/: u64, x1 /*param_type*/: u64, x2 /*param_hash*/: u64) -> f32 {
    let mut boma = *((x0 as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
    let boma_reference = &mut *boma;
    let fighter_kind = boma_reference.kind();

    // For fighters
    if boma_reference.is_fighter() {

        match utils::game_modes::get_custom_mode() {
            Some(modes) => {
                if modes.contains(&CustomMode::Smash64Mode) {
                    if x2 == hash40("shield_setoff_add") {
                        return 4.0;
                    }
            
                    if x2 == hash40("shield_setoff_mul") {
                        return 1.62;
                    }
            
                    if x1 == hash40("air_speed_y_stable") {
                        return original!()(x0, x1, x2) * 0.8;
                    }
            
                    if x1 == hash40("air_accel_y") {
                        return original!()(x0, x1, x2) * 0.8;
                    }
            
                    if x1 == hash40("damage_fly_top_air_accel_y") {
                        return original!()(x0, x1, x2) * 0.8;
                    }
            
                    if x1 == hash40("damage_fly_top_speed_y_stable") {
                        return original!()(x0, x1, x2) * 0.8;
                    }
            
                    if x1 == hash40("dive_speed_y") {
                        return original!()(x0, x1, x2) * 0.8;
                    }
            
                    if x1 == hash40("landing_frame") {
                        return 4.0;
                    }
            
                    if x1 == hash40("landing_attack_air_frame_n") {
                        return 4.0;
                    }
            
                    if x1 == hash40("landing_attack_air_frame_f") {
                        return 4.0;
                    }
            
                    if x1 == hash40("landing_attack_air_frame_b") {
                        return 4.0;
                    }
            
                    if x1 == hash40("landing_attack_air_frame_hi") {
                        return 4.0;
                    }
            
                    if x1 == hash40("landing_attack_air_frame_lw") {
                        return 4.0;
                    }
                }
                else if modes.contains(&CustomMode::RivalsOfAetherMode) {

                    // RoA mode universal 13F airdodge landing lag
                    if x1 == hash40("param_motion") && x2 == hash40("landing_frame_escape_air_slide_max") {
                        return 13.0;
                    }

                    // RoA mode universal 4F heavy landing lag
                    if x1 == hash40("landing_frame") {
                        VarModule::set_float(boma_reference.object(), vars::common::instance::LANDING_LAG_FOR_RIVALS_MODE, 4.0);
                        return 4.0;
                    }

                    // RoA mode whifflag on aerials
                    if [
                        hash40("landing_attack_air_frame_n"),
                        hash40("landing_attack_air_frame_f"),
                        hash40("landing_attack_air_frame_b"),
                        hash40("landing_attack_air_frame_hi"),
                        hash40("landing_attack_air_frame_lw"),
                    ].contains(&x1) {
                        let landing_lag = 2.0 + original!()(x0, x1, x2); // base landing lag is original landing lag + 2
                        let prev_inflict_status = VarModule::get_int(boma_reference.object(), vars::common::instance::PREV_STATUS_INFLICT_STATUS);
                        if prev_inflict_status & *COLLISION_KIND_MASK_HIT != 0 {
                            // on hit, landing lag is reduced by one third and can go no lower than 4F
                            let reduced_landing_lag = (landing_lag * 0.6667).floor().max(4.0);
                            VarModule::set_float(boma_reference.object(), vars::common::instance::LANDING_LAG_FOR_RIVALS_MODE, reduced_landing_lag);
                            return reduced_landing_lag;
                        }
                        VarModule::set_float(boma_reference.object(), vars::common::instance::LANDING_LAG_FOR_RIVALS_MODE, landing_lag);
                        return landing_lag;
                    }
                }
            },
            _ => {}
        }

        if x2 == hash40("damage_fly_correction_max") {
            if VarModule::is_flag(boma_reference.object(), vars::common::instance::ENABLE_FRAME_DATA_DEBUG) {
                return 15.0;
            }
        }

        // Coupled with "landing_heavy" change in change_motion hook
        // Because we start heavy landing anims on f3 rather than f1, we need to push back the heavy landing FAF by 2 frames so it is accurate to the defined per-character param
        if x1 == hash40("landing_frame") {
            return original!()(x0, hash40("landing_frame"), 0) + 2.0;
        }

        else if fighter_kind == *FIGHTER_KIND_DIDDY {
            if x1 == hash40("param_special_hi") {
                if x2 == hash40("special_hi_jet_ang_f_max") {
                    if WorkModule::get_int(boma, *FIGHTER_DIDDY_STATUS_SPECIAL_HI_WORK_INT_SITUATION) == *SITUATION_KIND_GROUND {
                        return 5.0;
                    }
                }
            }
        }

        else if fighter_kind == *FIGHTER_KIND_DONKEY {
            // handle reduction of the tumble threshold for DK when in barrel carry
            if x2 == hash40("damage_level3") {
                let status = boma_reference.status();
                // if you are in any of the FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_* statuses,
                // reduce the dumble threshold.
                if (481..=489).contains(&boma_reference.status()) {
                    return original!()(x0, x1, x2) * 0.5;
                }
            }
        }

        // Ken aerial hadouken modified offsets for aerial version
        else if fighter_kind == *FIGHTER_KIND_KEN {
            if VarModule::is_flag(boma_reference.object(), vars::shotos::instance::SPECIAL_N_HADOKEN_AIR) {
                if x1 == hash40("param_special_n") {
                    if x2 == hash40("shoot_x") {
                        return 11.0;
                    } else if x2 == hash40("shoot_y") {
                        return 6.0;
                    }
                }
            }
        }

        else if fighter_kind == *FIGHTER_KIND_LUCARIO {
            if x1 == hash40("param_special_hi")
            && x2 == hash40("rush_speed") {
                let rate = VarModule::get_float(boma_reference.object(), vars::lucario::instance::SPECIAL_HI_MOTION_RATE);
                if rate > 0.0 {
                    let original = original!()(x0, x1, x2);
                    return original * rate;
                }
            } 
        }

        // Ironball uses a custom kinetic type and I can't find the offset where these values are set
        else if fighter_kind == *FIGHTER_KIND_MIIFIGHTER
        || (fighter_kind == *FIGHTER_KIND_KIRBY && WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == 0x48) {
            if x1 == hash40("param_special_n") {
                if x2 == hash40("n1_throw_angle") {
                    return VarModule::get_float(boma_reference.object(), vars::miifighter::status::SPECIAL_N1_ANGLE);
                }
                if x2 == hash40("n1_start_speed_x") {
                    return VarModule::get_float(boma_reference.object(), vars::miifighter::status::SPECIAL_N1_SPEED);
                }
            }
        }
        
        else if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN {
            // not found within the special_hi status scripts
            if x1 == hash40("param_special_hi") {
                if VarModule::is_flag(boma_reference.object(), vars::common::instance::IS_HEAVY_ATTACK){
                    if x2 == hash40("hi2_rush_speed") {
                        return 3.0;
                    }
                }
            }
        }

        else if fighter_kind == *FIGHTER_KIND_MIIGUNNER {
            // not found within the special_hi status scripts
            if x1 == hash40("param_special_hi") && x2 == hash40("hi1_first_jump_y_speed") {
                let charge = VarModule::get_float(boma_reference.object(), vars::miigunner::status::ATTACK_CHARGE);
                let base_y_speed = ParamModule::get_float(boma_reference.object(), ParamType::Agent, "param_special_hi1.base_y_speed");
                let charge_y_speed_mul = ParamModule::get_float(boma_reference.object(), ParamType::Agent, "param_special_hi1.charge_y_speed_mul");
                let charge_y_speed_div = ParamModule::get_float(boma_reference.object(), ParamType::Agent, "param_special_hi1.charge_y_speed_div");
                return base_y_speed + (charge_y_speed_mul * charge) / charge_y_speed_div;
            }
        }
        	
        else if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU {
            //println!("Ivysaur");
            if x1 == hash40("param_special_s") && x2 == hash40("shoot_angle") {
                //println!("Razor angle");
                return ControlModule::get_stick_y(boma) * 25.0;
            }
        }
        
    }

    // For articles
    else if boma_reference.is_weapon() {

        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

        if fighter_kind == *WEAPON_KIND_SNAKE_TRENCHMORTAR_BULLET {
            if x1 == hash40("param_trenchmortarbullet") && x2 == hash40("speed_x") {
                return ControlModule::get_stick_x(boma) / 1.5 * PostureModule::lr(boma);
            }
        }

        else if fighter_kind == *WEAPON_KIND_DEDEDE_GORDO {
            if x1 == hash40("param_gordo") && x2 == hash40("shot_start_angle") {
                return 20.0 * ControlModule::get_stick_y(owner_module_accessor);
            }
        }

        else if fighter_kind == *WEAPON_KIND_PICKEL_FISHINGROD {
            if x1 == hash40("param_fishingrod") {
                if x2 == hash40("shoot_angle") {
                    if ControlModule::get_stick_y(owner_module_accessor) < 0.0 {
                        return 48.0 + (ControlModule::get_stick_y(owner_module_accessor) * 30.0);
                    }
                    else {
                        return 48.0;
                    }
                }
            }
        }

        else if fighter_kind == *WEAPON_KIND_PACKUN_SPIKEBALL {
            if VarModule::is_flag(boma_reference.object(), vars::packun_spikeball::instance::ENABLE_EXPLODE) {
                if x1 == hash40("param_spikeball") {
                    if x2 == hash40("hop_speed_x") {
                        return 0.0;
                    }
                    else if x2 == hash40("hop_speed_y") {
                        return 0.0;
                    }
                    else if x2 == hash40("hop_clear_attack_speed") && MotionModule::motion_kind(boma_reference) == hash40("explode") {
                        return -0.1;
                    }
                }
            }
            else if VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
                if x1 == hash40("param_spikeball") {
                    if x2 == hash40("shoot_speed_x_max") {
                        return 1.5;
                    }
                    else if x2 == hash40("shoot_speed_y_max") {
                        return 1.3;
                    }
                    else if x2 == hash40("shoot_speed_x_min") {
                        return 0.5;
                    }
                    else if x2 == hash40("shoot_speed_y_min") {
                        return 0.4;
                    }
                }
            }
        }

        else if fighter_kind == *WEAPON_KIND_LUCARIO_AURABALL {
            if x1 == hash40("param_auraball") {
                if VarModule::is_flag(boma_reference.object(), vars::lucario::instance::IS_POWERED_UP) {
                    if x2 == hash40("min_speed") {
                        return 0.7;
                    }
                    if x2 == hash40("max_speed") {
                        return 0.7;
                    }
                }
            }
        }
    }

    original!()(x0, x1, x2)
}

// #[skyline::hook(offset=0x165d0b0, inline)]
// unsafe fn get_item_backtrace_inline(ctx: &InlineCtx) {
//     ::utils::dump_trace!(0x0);
// }

// fn item_nro_hook(info: &skyline::nro::NroInfo) {
//     if info.name == "item" {
//         unsafe {
//             //println!("Module Base: {:#x}", (*info.module.ModuleObject).module_base);
//             EXPLOSIONBOMB_ADDRESS += (*info.module.ModuleObject).module_base;
//             skyline::install_hook!(get_explosionbomb_hook);
//         }
//     }
// }

// static mut EXPLOSIONBOMB_ADDRESS: u64 = 0x8c4940;//0x8c493c;

// #[skyline::hook(replace=EXPLOSIONBOMB_ADDRESS, inline)]
// pub unsafe fn get_explosionbomb_hook(ctx: &InlineCtx) {
//     let agent = (ctx.registers[21].x()) as *mut L2CAgentBase;
//     //println!("Agent: {}", (*agent).kind());
//     let lua_state = (*agent).lua_state_agent;
//     let item_boma = sv_system::battle_object_module_accessor(lua_state);
//     let owner_id = WorkModule::get_int(item_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
//     if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_SHEIK {
// 		let sheik = utils::util::get_battle_object_from_id(owner_id);
//         let item_id = item_boma.battle_object_id;
//         VarModule::set_int(sheik, vars::sheik::status::GRENADE_OBJECT_ID, item_id as i32);
//         //let sheik_boma = &mut *(*zelda).module_accessor;
//         //let value = VarModule::get_float(sheik, vars::sheik::status::GRENADE_GRAVITY);
//         //ctx.registers_f[0].set_s(value);
//     }
// }