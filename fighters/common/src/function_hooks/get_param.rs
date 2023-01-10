use super::*;
use globals::*;
// Addresses, offsets, and inline hooking
use skyline::hooks::{getRegionAddress, Region, InlineCtx};

pub fn install() {
    skyline::install_hooks!(
        //get_offset,
        //get_inline_offset,
        get_param_int_hook,
        get_param_float_hook,
    );
}

static INT_OFFSET: usize = 0x4e5380; // 12.0.0
static FLOAT_OFFSET: usize = 0x4e53C0; // 12.0.0

#[skyline::hook(offset=0x720540)]
unsafe fn get_offset(arg0: u64, arg1: u64) {
    static mut ONCE: bool = true;
    if ONCE {
        ONCE = false;
        //debug::dump_trace();
    }
    original!()(arg0, arg1);
}

#[skyline::hook(offset=0x1f8810c, inline)]
unsafe fn get_inline_offset(ctx: &InlineCtx) {
    static mut ONCE: bool = true;
    if ONCE {
        ONCE = false;
        println!("{:#x}", ctx.registers[3].x.as_ref() - getRegionAddress(Region::Text) as u64);
    }
}

#[skyline::hook(offset=INT_OFFSET)]
pub unsafe fn get_param_int_hook(x0: u64, x1: u64, x2 :u64) -> i32 {
    let mut boma = *((x0 as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
    let boma_reference = &mut *boma;
    let fighter_kind = boma_reference.kind();
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if boma_reference.is_fighter() {

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

    }

    original!()(x0, x1, x2)
}

#[skyline::hook(offset=FLOAT_OFFSET)]
pub unsafe fn get_param_float_hook(x0 /*boma*/: u64, x1 /*param_type*/: u64, x2 /*param_hash*/: u64) -> f32 {
    let mut boma = *((x0 as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
    let boma_reference = &mut *boma;
    let fighter_kind = boma_reference.kind();
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if boma_reference.is_fighter() {

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

        // Coupled with "landing_heavy" change in change_motion hook
        // Because we start heavy landing anims on f2 rather than f1, we need to push back the heavy landing FAF by 1 frame so it is accurate to the defined per-character param
        if x1 == hash40("landing_frame") {
            return original!()(x0, hash40("landing_frame"), 0) + 1.0;
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
                return 3.5 + 2.7 * VarModule::get_float(boma_reference.object(), vars::miigunner::status::CHARGE_ATTACK_LEVEL) / 29.0;
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
    
    }
    else if boma_reference.is_weapon() {

        // For articles
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

        if fighter_kind == *WEAPON_KIND_SNAKE_TRENCHMORTAR_BULLET {
            if x1 == hash40("param_trenchmortarbullet") && x2 == hash40("speed_x") {
                return ControlModule::get_stick_x(boma) / 1.5 * PostureModule::lr(boma);
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
    
    }

    original!()(x0, x1, x2)
}
