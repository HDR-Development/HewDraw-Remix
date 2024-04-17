use super::*;
use globals::*;
use std::arch::asm;
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
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if boma_reference.is_fighter() {

        match utils::game_modes::get_custom_mode() {
            Some(modes) => {
                if modes.contains(&CustomMode::Smash64Mode) {
                    if x1 == hash40("landing_heavy_frame") {
                        return 4;
                    }
                }
            },
            _ => {}
        }

        if x2 == hash40("just_shield_precede_extension") {
            return 1000;
        }
    
        if x2 == hash40("continue_just_shield_count") {
    
        }
    
        if fighter_kind == *FIGHTER_KIND_RYU {
            if VarModule::is_flag(boma_reference.object(), vars::shotos::instance::IS_USE_EX_SPECIAL) && x1 == hash40("param_special_s") && (x2 == hash40("loop_num_w") || x2 == hash40("loop_num_m") || x2 == hash40("loop_num_s") || x2 == hash40("loop_num_w") || x2 == hash40("air_loop_num_m") || x2 == hash40("air_air_loop_num_s")) {
                return 3;
            }
        }

        else if fighter_kind == *FIGHTER_KIND_PACKUN {
            if boma_reference.is_motion(Hash40::new("special_hi"))
            && !boma_reference.is_prev_situation(*SITUATION_KIND_AIR)
            && x1 == hash40("param_special_hi")
            && x2 == hash40("start_no_landing_frame") {
                return 999;
            }
        }

    }

    else if boma_reference.is_weapon() {

        // For articles
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

        if fighter_kind == *WEAPON_KIND_PACKUN_SPIKEBALL {
            if VarModule::is_flag(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SHOULD_EXPLODE) {
                if x1 == hash40("param_spikeball") { 
                    if x2 == hash40("hop_life") {
                        return 105;
                    }
                }
            }
            // else if VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
            //     if x1 == hash40("param_spikeball") { 
            //         if x2 == hash40("out_range_y") {
            //             return 45;
            //         }
            //     }
            // }
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
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

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
            },
            _ => {}
        }

        /*if x1 == hash40("air_speed_x_stable") {
            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_JUMP_SQUAT {
                if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_RUN {
                    return original!()(x0, hash40("jump_speed_x_max"), 0);
                }
            }
        }*/
        /*
        if x1 == hash40("critical_frame") {
            return 0.0;
        }
        if x1 == hash40("critical_zoom_rate") {
            return 0.0;
        }
        */

        // handle reduction of the tumble threshold for DK when in barrel carry
        if x2 == hash40("damage_level3") 
        && boma_reference.kind() == *FIGHTER_KIND_DONKEY
         {
            let status = boma_reference.status();

            // if you are in any of the FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_* statuses,
            // reduce the dumble threshold.
            if status >= 481 && status <= 489 {
                return original!()(x0, x1, x2) * 0.5;
            }
        }

        // Coupled with "landing_heavy" change in change_motion hook
        // Because we start heavy landing anims on f2 rather than f1, we need to push back the heavy landing FAF by 1 frame so it is accurate to the defined per-character param
        if x1 == hash40("landing_frame") {
            return original!()(x0, hash40("landing_frame"), 0) + 1.0;
        }

        // Ken aerial hadouken modified offsets for aerial version
        else if fighter_kind == *FIGHTER_KIND_KEN {
            if VarModule::is_flag(boma_reference.object(), vars::shotos::instance::IS_CURRENT_HADOKEN_AIR) {
                if x1 == hash40("param_special_n") {
                    if x2 == hash40("shoot_x") {
                        return 11.0;
                    } else if x2 == hash40("shoot_y") {
                        return 6.0;
                    }
                }
            }
        }
        
        else if fighter_kind == *FIGHTER_KIND_MIISWORDSMAN {
            //if MotionModule::motion_kind(&mut *boma) == hash40("special_hi1") || MotionModule::motion_kind(&mut *boma) == hash40("special_hi1_start"){
            if StatusModule::situation_kind(&mut *boma) == *SITUATION_KIND_GROUND{
                //println!("Stone");
                if x1 == hash40("param_special_hi") && x2 == hash40("hi1_jump_speed_mul") {
                    //println!("Scabbard");
                    return 0.65;
                }
            }
            if x1 == hash40("param_special_hi"){
                //if heavy_attack[hdr::get_player_number(owner_module_accessor)]{
                if VarModule::is_flag(boma_reference.object(), vars::common::instance::IS_HEAVY_ATTACK){
                    if x2 == hash40("hi2_rush_speed") {
                        return 3.0;
                    }
                }
            }
            else if x1 == hash40("param_private") {
                if x2 == hash40("final_wave_speed") {
                    if VarModule::is_flag(boma_reference.object(), vars::miiswordsman::status::WAVE_SPECIAL_N) {
                        return 2.0;
                    }
                }
                else if x2 == hash40("final_wave_scale_max") {
                    if VarModule::is_flag(boma_reference.object(), vars::miiswordsman::status::WAVE_SPECIAL_N) {
                        return 0.5;
                    }
                }
            }
        }
        	
        else if fighter_kind == *FIGHTER_KIND_PFUSHIGISOU {
            //println!("Ivysaur");
            if x1 == hash40("param_special_s") && x2 == hash40("shoot_angle") {
                //println!("Razor angle");
                return ControlModule::get_stick_y(boma) * 25.0;
            }
        }
        
        else if fighter_kind == *FIGHTER_KIND_MIIGUNNER {
            if x1 == hash40("param_special_hi") && x2 == hash40("hi1_first_jump_y_speed") {
                return 3.5 + (2.7 * VarModule::get_float(boma_reference.object(), vars::miigunner::status::CURRENT_CHARGE)) / 29.0;
            }
        }

        else if fighter_kind == *FIGHTER_KIND_SHEIK {
            if x1 == hash40("param_special_s") {
                if x2 == hash40("throw_angle") {
                    if ControlModule::get_stick_y(boma) > 0.0 {
                        return 10.0 + ControlModule::get_stick_y(boma) * 40.0;
                    }
                }
                if x2 == hash40("throw_speed") {
                    if ControlModule::get_stick_y(boma) > 0.0 {
                        return 4.0 - ControlModule::get_stick_y(boma) * 1.0;
                    }
                }
            }
        }
        // else if fighter_kind == *FIGHTER_KIND_PICKEL {
        //     if [*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WAIT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_FALL, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_FALL_AERIAL].contains(&StatusModule::status_kind(boma)) {
        //         if ControlModule::get_stick_x(boma) * PostureModule::lr(boma) > 0.5 {
        //             if x1 == hash40("param_special_n") && x2 == hash40("generate_pickelobject_offset_x") {
        //                 return 10.5;
        //             }
        //         }
        //         if ControlModule::get_stick_y(boma) > 0.5 {
        //             if x1 == hash40("param_special_n") && x2 == hash40("generate_pickelobject_offset_y") {
        //                 return 22.5;
        //             }
        //         }
        //         else if ControlModule::get_stick_y(boma) < -0.5 {
        //             if x1 == hash40("param_special_n") && x2 == hash40("generate_pickelobject_offset_y") {
        //                 return -7.5;
        //             }
        //         }
        //         else {
        //             if ControlModule::get_stick_x(boma) * PostureModule::lr(boma) > 0.5 {
        //                 if x1 == hash40("param_special_n") && x2 == hash40("generate_pickelobject_offset_y") {
        //                     return 3.5;
        //                 }
        //             }
        //         }
        //     }
        //     if [*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK_BACK].contains(&StatusModule::status_kind(boma)) {
        //         if ControlModule::get_stick_x(boma) * PostureModule::lr(boma) > 0.5 {
        //             if x1 == hash40("param_special_n") && x2 == hash40("generate_pickelobject_offset_x") {
        //                 return 9.5;
        //             }
        //         }
        //         if ControlModule::get_stick_y(boma) > 0.5 {
        //             if x1 == hash40("param_special_n") && x2 == hash40("generate_pickelobject_offset_y") {
        //                 return 22.5;
        //             }
        //         }
        //         else if ControlModule::get_stick_y(boma) < 0.0 {
        //             if x1 == hash40("param_special_n") && x2 == hash40("generate_pickelobject_offset_y") {
        //                 return -7.5;
        //             }
        //         }
        //         else {
        //             if ControlModule::get_stick_x(boma) * PostureModule::lr(boma) > 0.5 {
        //                 if x1 == hash40("param_special_n") && x2 == hash40("generate_pickelobject_offset_y") {
        //                     return 3.5;
        //                 }
        //             }
        //         }
        //     }
        // }
        else if fighter_kind == *FIGHTER_KIND_DIDDY {
            if x1 == hash40("param_special_hi") {
                if x2 == hash40("special_hi_jet_ang_f_max") {
                    if WorkModule::get_int(boma, *FIGHTER_DIDDY_STATUS_SPECIAL_HI_WORK_INT_SITUATION) == *SITUATION_KIND_GROUND {
                        return 5.0;
                    }
                }
            }
        }
    
    }
    else if boma_reference.is_weapon() {

        // For articles
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

        if fighter_kind == *WEAPON_KIND_SNAKE_TRENCHMORTAR_BULLET {
            if x1 == hash40("param_trenchmortarbullet") && x2 == hash40("speed_x") {
                return ControlModule::get_stick_x(boma) / 1.5 * PostureModule::lr(boma);
            }
        }

        else if fighter_kind == *WEAPON_KIND_DEDEDE_GORDO {
            if x1 == hash40("param_gordo") && x2 == hash40("shot_start_angle"){
                return 20.0 * ControlModule::get_stick_y(owner_module_accessor);
            }
        }

        // Frieza death ball on M2 aerial Shadow Ball
        /*
        if fighter_kind == *WEAPON_KIND_MEWTWO_SHADOWBALL {
            if x1 == hash40("param_shadowball") && x2 == hash40("angle") {
                //println!("Owner module accessor entry ID: {}", WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID));
                if StatusModule::situation_kind(owner_module_accessor) == *SITUATION_KIND_AIR {
                    //println!("Mewtwo is airborne");
                    return -45.0;
                }
            }
        }
        */
        else if fighter_kind == *WEAPON_KIND_PICKEL_FISHINGROD{
            if x1 == hash40("param_fishingrod"){
                if x2 == hash40("shoot_angle"){
                    if ControlModule::get_stick_y(owner_module_accessor) < 0.0{
                        return 48.0 + (ControlModule::get_stick_y(owner_module_accessor) * 30.0);
                    }
                    else{
                        return 48.0;
                    }
                }
            }
        }
        else if fighter_kind == *WEAPON_KIND_MIISWORDSMAN_TORNADOSHOT {
            if x1 == hash40("param_tornadoshot"){
                if x2 == hash40("life") {
                    //if heavy_attack[hdr::get_player_number(owner_module_accessor)]{
                    if VarModule::is_flag(owner_module_accessor.object(), vars::common::instance::IS_HEAVY_ATTACK){
                        return 70.0;
                    }
                }
                else if x2 == hash40("speed_x") {
                    //if heavy_attack[hdr::get_player_number(owner_module_accessor)]{
                    if VarModule::is_flag(owner_module_accessor.object(), vars::common::instance::IS_HEAVY_ATTACK){
                        return 1.5;
                    }
                }
                else if x2 == hash40("accel_x") {
                    //if heavy_attack[hdr::get_player_number(owner_module_accessor)]{
                    if VarModule::is_flag(owner_module_accessor.object(), vars::common::instance::IS_HEAVY_ATTACK){
                        return -0.025;
                    }
                }
            }
        }
    
        else if fighter_kind == *WEAPON_KIND_MIIGUNNER_GRENADELAUNCHER {
            if x1 == hash40("param_grenadelauncher") {
                if x2 == hash40("angle") {
                    let charge = VarModule::get_float(owner_module_accessor.object(), vars::miigunner::instance::GRENADE_CHARGE);
                    return 34.0 + charge;
                }
            }
        }

        else if fighter_kind == *WEAPON_KIND_PACKUN_SPIKEBALL {
            if VarModule::is_flag(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SHOULD_EXPLODE) {
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
                    // if x2 == hash40("attack_mul") {
                    //     return 1.0;
                    // }
                    // if x2 == hash40("angle") {
                    //     return VarModule::get_float(boma_reference.object(), vars::lucario::status::SPECIAL_N_ANGLE);
                    // }
                    if x2 == hash40("min_speed") {
                        return 0.7;
                    }
                    if x2 == hash40("max_speed") {
                        return 0.7;
                    }
                }
            }
        }

        else if fighter_kind == *WEAPON_KIND_RICHTER_AXE {
            if x1 == hash40("param_axe") {
                if (&[
                    hash40("throw_angle"),
                    hash40("throw_angle_stick_front"),
                    hash40("throw_angle_stick_back")
                ]).contains(&x2)
                && owner_module_accessor.is_situation(*SITUATION_KIND_AIR) {
                    return -42.0;
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
//     let agent = (*ctx.registers[21].x.as_ref()) as *mut L2CAgentBase;
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
//         //asm!("fmov s0, w8", in("w8") value);
//     }
// }