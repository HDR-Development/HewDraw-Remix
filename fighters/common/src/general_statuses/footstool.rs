// status imports
use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_transition_group_check_air_tread_jump
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_transition_group_check_air_tread_jump)]
pub unsafe fn sub_transition_group_check_air_tread_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cont;
    if fighter.global_table[0x30].get_bool() == false {
        cont = false;
    }
    else {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(fighter.global_table[0x30].get_ptr());
        cont = callable(fighter).get_bool();
    }
    if cont == false {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            // Uncomment this to re-enable jump inputs to footstool.
            // Currently I believe an issue is you won't be able to do a
            // high footstool jump, which I am still investigating, but for now the
            // basic functionality is there.
            // if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0 {
            //     if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP)
            //     && ControlModule::is_enable_flick_jump(fighter.module_accessor) {
            //         let do_footstool;
            //         if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME) != 0 {
            //             do_footstool = false;
            //         }
            //         else {
            //             let tread_speed_y = fighter.FL_sub_fighter_float_next_tread_speed_y().get_f32();
            //             let tread_jump_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("tread_jump_speed_limit"));
            //             if !(tread_jump_speed_limit <= tread_speed_y) {
            //                 do_footstool = false;
            //             }
            //             else {
            //                 fighter.clear_lua_stack();
            //                 lua_args!(fighter, 0x21bfbd3f83u64);
            //                 smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
            //                 do_footstool = fighter.pop_lua_stack(1).get_bool();
            //             }
            //         }
            //         if do_footstool {
            //             fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), true.into());
            //             return true.into();
            //         }
            //     }
            // }
            // if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_JUMP_TRIGGER != 0
            // ||
            
            if fighter.is_cat_flag(Cat2::AppealAll) {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_BUTTON) {
                    let do_footstool;
                    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_NO_TREAD_FRAME) != 0 {
                        do_footstool = false;
                    }
                    else {
                        let tread_speed_y = fighter.FL_sub_fighter_float_next_tread_speed_y().get_f32();
                        let tread_jump_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("tread_jump_speed_limit"));
                        if !(tread_jump_speed_limit <= tread_speed_y) {
                            do_footstool = false;
                        }
                        else {
                            fighter.clear_lua_stack();
                            lua_args!(fighter, 0x21bfbd3f83u64);
                            smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                            do_footstool = fighter.pop_lua_stack(1).get_bool();
                        }
                    }
                    if do_footstool {
                        fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), true.into());
                        return true.into();
                    }
                }
            }
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TREAD_JUMP_NO_TRIGGER) {
                fighter.clear_lua_stack();
                lua_args!(fighter, 0x21bfbd3f83u64, true);
                smash::app::sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
                if fighter.pop_lua_stack(1).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_TREAD_JUMP.into(), false.into());
                    return true.into();
                }
            }
        }
    }
    else {
        return true.into();
    }
    false.into()
}
