use super::*;

//FIGHTER_STATUS_KIND_SPECIAL_LW

unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lua_state = fighter.lua_state_agent;
    WorkModule::off_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_CONTINUE_MOT);
    main_setup(fighter);
    let mut l2c_agent = L2CAgent::new(lua_state);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_int(0x20cbc92683));
    l2c_agent.push_lua_stack(&mut L2CValue::I32(1));
    l2c_agent.push_lua_stack(&mut L2CValue::I32(*FIGHTER_LOG_DATA_INT_ATTACK_NUM_KIND));
    l2c_agent.push_lua_stack(&mut L2CValue::I32(*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_10 - 1));
    sv_battle_object::notify_event_msc_cmd(lua_state);
    l2c_agent.pop_lua_stack(1);
    l2c_agent.clear_lua_stack();
    l2c_agent.push_lua_stack(&mut L2CValue::new_int(0x3a40337e2c));
    l2c_agent.push_lua_stack(&mut L2CValue::I32(*FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_10 - 1));
    sv_battle_object::notify_event_msc_cmd(lua_state);
    l2c_agent.pop_lua_stack(1);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_main_loop as *const () as _))
}

unsafe extern "C" fn special_lw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut num = -1;

    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw1") || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw1") {
        if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            //println!("Kinesis activation");
            VarModule::on_flag(fighter.battle_object, vars::miiswordsman::status::SPECIAL_LW1_ATTACK_TRIGGER);
            fighter.change_status(
                L2CValue::I32(*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT),
                L2CValue::Bool(false)
            );
            return 1.into()
        }
    }
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_lw3") || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_lw3") {
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            //println!("Swordfighter gon' give it to ya");
            fighter.change_status(
                L2CValue::I32(*FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW3_END),
                L2CValue::Bool(false)
            );
            return 1.into()
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false {
            if fighter.sub_air_check_fall_common().get_bool() == false {
                if fighter.global_table[0x17].get_i32() != *SITUATION_KIND_GROUND
                || fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR {
                    if fighter.global_table[0x17].get_i32() != *SITUATION_KIND_GROUND {
                        if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
                            main_setup(fighter);
                        }
                    }
                }
                if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_APPLY_POWERUP_MOTION_RATE) == false {
                    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_APPLIED_POWERUP_MOTION_RATE) {
                        MotionModule::set_rate(fighter.module_accessor,1.0);
                        WorkModule::off_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_APPLIED_POWERUP_MOTION_RATE);
                    }
                }
                else {
                    if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_APPLIED_POWERUP_MOTION_RATE) == false {
                        let rate = WorkModule::get_param_float(fighter.module_accessor,hash40("param_special_lw"),hash40("lw1_motion_rate"));
                        MotionModule::set_rate(fighter.module_accessor,rate);
                        WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_APPLIED_POWERUP_MOTION_RATE);
                    }
                }
                if MotionModule::is_end(fighter.module_accessor) {
                    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
                        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(),false.into());
                    }
                    if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
                        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(),false.into());
                    }
                }
                counter_setup(fighter);
                num = 0;
            }
        }
        if num == -1 { num = 1; }
    }
    L2CValue::I32(num)
}

unsafe extern "C" fn main_setup(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_CONTINUE_MOT) == false {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_air_lw1"),0.0,1.0,false,0.0,false,false);
            WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor,Hash40::new("special_air_lw1"),-1.0,1.0,0.0);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor,*FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
        GroundModule::correct(fighter.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_CONTINUE_MOT) == false {
            MotionModule::change_motion(fighter.module_accessor,Hash40::new("special_lw1"),0.0,1.0,false,0.0,false,false);
            WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor,Hash40::new("special_lw1"),-1.0,1.0,0.0);
        }
    }
}

unsafe extern "C" fn counter_setup(fighter: &mut L2CFighterCommon) {
    if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        let attack_power = WorkModule::get_float(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_WORK_FLOAT_ATTACK_POWER);
        if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_LW1_HIT {
            return;
        }
        if attack_power > 0.0 {
            let part = (AttackModule::part_size(fighter.module_accessor) as i32) - 1;
            if -1 < part {
                let mut box_num = -1;
                while box_num <= part {
                    if AttackModule::is_attack(fighter.module_accessor,box_num + 1, false) {
                        AttackModule::set_power(fighter.module_accessor,box_num + 1, attack_power, false);
                    }
                    box_num += 1;
                }
            }
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_SHIELD_CHK) == false {
            if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_SHIELD) == false {
                return;
            }
            ShieldModule::set_status(fighter.module_accessor,0,app::ShieldStatus(*SHIELD_STATUS_NONE),*FIGHTER_MIISWORDSMAN_SHIELD_GROUP_KIND_COUNTER_GUARD);
            WorkModule::off_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_SHIELD_CHK);
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_SHIELD) == false {
                return;
            }
            ShieldModule::set_status(fighter.module_accessor,0,app::ShieldStatus(*SHIELD_STATUS_NORMAL),*FIGHTER_MIISWORDSMAN_SHIELD_GROUP_KIND_COUNTER_GUARD);
            WorkModule::on_flag(fighter.module_accessor,*FIGHTER_MIISWORDSMAN_STATUS_COUNTER_FLAG_SHIELD_CHK);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_SPECIAL_LW, special_lw_main);
}