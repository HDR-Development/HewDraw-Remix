// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn piranhacopter_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI
    && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)
    && boma.status_frame() >= 30
    {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END, false);
    }
    if status_kind == *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END
    && boma.is_motion(Hash40::new("special_hi"))
    {
        if boma.is_prev_situation(*SITUATION_KIND_AIR)
        && boma.is_situation(*SITUATION_KIND_GROUND)
        {
            GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        if boma.is_prev_situation(*SITUATION_KIND_GROUND)
        && boma.is_situation(*SITUATION_KIND_AIR)
        {
            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
            GroundModule::correct(boma,GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        let stop_add_speed_y_frame = WorkModule::get_param_int(boma, hash40("param_special_hi"), hash40("stop_add_speed_y_frame"));
        if boma.status_frame() >= stop_add_speed_y_frame {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_LANDING, false);
        }
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

unsafe fn stance_head(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    // Enable meshes for stances
    // HeadA is the normal head
	// HeadB is the poison head
	// HeadS is the spike head
    if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 0 {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), false);
    }
    else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 1  {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), false);
    }
    else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2  {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heads"), true);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("headb"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("heada"), false);
    }
}

unsafe fn stance_init_effects(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.object(), vars::packun::instance::STANCE_INIT) {
        if !VarModule::is_flag(fighter.object(), vars::packun::status::CLOUD_COVER) {
            EFFECT(fighter, Hash40::new("sys_level_up"), Hash40::new("top"), -2, 10, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, true);
            if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 0 {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_grass_landing"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, false);
            }
            else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 1 {
                EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_max"), Hash40::new("top"), 0, 15.5, 0, 0, 0, 0, 1.2, false);
            }
            else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
                EFFECT_FOLLOW(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, false);
            }
        }
        VarModule::off_flag(fighter.object(), vars::packun::instance::STANCE_INIT);
    }
}

/// handle speed application
unsafe fn check_apply_speeds(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    
    // handle speed application once
    if VarModule::is_flag(fighter.object(), vars::packun::instance::STANCE_NEED_SET_SPEEDS) {
        if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 0 {
            apply_status_speed_mul(fighter, 1.0);
        } else if fighter.is_status_one_of(&[
            *FIGHTER_STATUS_KIND_ESCAPE_F,
            *FIGHTER_STATUS_KIND_ESCAPE_B,
            *FIGHTER_STATUS_KIND_SLIP_STAND_F,
            *FIGHTER_STATUS_KIND_SLIP_STAND_B,
            *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
            *FIGHTER_STATUS_KIND_PASSIVE_FB]) {
                apply_status_speed_mul(fighter, 1.0);
        } else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 1 {
            apply_status_speed_mul(fighter, 0.9);
        } else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
            apply_status_speed_mul(fighter, 0.84);
        }
        VarModule::off_flag(fighter.object(), vars::packun::instance::STANCE_NEED_SET_SPEEDS);
    }

    if fighter.status() != VarModule::get_int(fighter.object(), vars::packun::instance::STANCE_STATUS) {
        //println!("Status is changing!");
        VarModule::on_flag(fighter.object(), vars::packun::instance::STANCE_NEED_SET_SPEEDS);
        VarModule::set_int(fighter.object(), vars::packun::instance::STANCE_STATUS, fighter.status());
        //println!("new stance status: {}", VarModule::get_int(fighter.object(), vars::packun::instance::STANCE_STATUS));
    }

    // dash & momentum transfer speeds
    if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 1 {
        VarModule::set_float(fighter.object(), vars::common::instance::JUMP_SPEED_MAX_MUL, 1.0);

        // if you are initial dash, slow them down slightly
        if fighter.is_status(*FIGHTER_STATUS_KIND_DASH) {
            let motion_vec = Vector3f {
                x: -0.05 * PostureModule::lr(fighter.boma()) * (1.0 - (MotionModule::frame(fighter.boma()) / MotionModule::end_frame(fighter.boma()))),
                y: 0.0, 
                z: 0.0
            };
            //KineticModule::add_speed_outside(fighter.boma(), *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
    }

    else if VarModule::get_int(fighter.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
        VarModule::set_float(fighter.object(), vars::common::instance::JUMP_SPEED_MAX_MUL, 0.88);

        // if you are initial dash, slow them down slightly
        if fighter.is_status(*FIGHTER_STATUS_KIND_DASH) {
            let motion_vec = Vector3f {
                x: -0.15 * PostureModule::lr(fighter.boma()) * (1.0 - (MotionModule::frame(fighter.boma()) / MotionModule::end_frame(fighter.boma()))),
                y: 0.0, 
                z: 0.0
            };
            KineticModule::add_speed_outside(fighter.boma(), *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        }
    }
}

/// checks if stance should be reset due to match end
unsafe fn check_reset(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY]) {
            VarModule::set_int(fighter.object(), vars::packun::instance::CURRENT_STANCE, 0);
    }
}

/// applies the given multiplier to various speed stats of the given fighter. 
/// This should only be called once per status, or you will get some multiplicative effects
unsafe fn apply_status_speed_mul(fighter: &mut smash::lua2cpp::L2CFighterCommon, mul: f32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
    let og_speed_mul = app::sv_kinetic_energy::get_speed_mul(fighter.lua_state_agent);

    // set the X motion speed multiplier (where movement is baked into an anim)
    lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter.get_motion_energy(), og_speed_mul * mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_mul( fighter.get_controller_energy(), mul);

    // set the X motion accel multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_accel_add( fighter.get_controller_energy(), mul);

    // set the X speed max multiplier for control energy (used in the air, during walk, fall, etc)
    lua_bind::FighterKineticEnergyController::mul_x_speed_max(fighter.get_controller_energy(), mul);
}

unsafe fn sspecial_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE) == *FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_AIR_ESCAPE_AIR {
                WorkModule::set_int(boma, *FIGHTER_PACKUN_SPECIAL_S_CANCEL_TYPE_NONE, *FIGHTER_PACKUN_STATUS_SPECIAL_S_WORK_INT_CANCEL_TYPE);
                //ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }
        }
    }
}

unsafe fn ptooie_scale(boma: &mut BattleObjectModuleAccessor) {
    if VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
        VarModule::set_float(boma.object(), vars::packun::instance::PTOOIE_SCALE, 1.3);
    }
    else {
        VarModule::set_float(boma.object(), vars::packun::instance::PTOOIE_SCALE, 1.0);
    }
}

// Allows hold input to transition to rapid jab if in Putrid stance, and handles changed animations per stance
unsafe fn motion_handler(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("attack_13")) && VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 1 {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_100, false);
    }
    if boma.is_motion(Hash40::new("attack_s3_s")) { 
        if VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
            MotionModule::change_motion(boma, Hash40::new("attack_s3_s2"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 0 {
            MotionModule::change_motion(boma, Hash40::new("attack_s3_s_a"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if boma.is_motion(Hash40::new("attack_s4_s")) && VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
        let frame = if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD {24.0} else {0.0};
        MotionModule::change_motion(boma, Hash40::new("attack_s4_s_2"), frame, 1.0, false, 0.0, false, false);
    }
    if boma.is_motion(Hash40::new("attack_s4_hold")) && VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
        MotionModule::change_motion(boma, Hash40::new("attack_s4_hold_2"), 0.0, 1.0, false, 0.0, false, false);
    }
    if boma.is_motion(Hash40::new("attack_air_b")) && VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE) == 2 {
        MotionModule::change_motion(boma, Hash40::new("attack_air_b_s"), 0.0, 1.0, false, 0.0, false, false);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_FAILURE,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_N_HIT_END,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_END,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_SHOOT,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CANCEL,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_S_CHARGE,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_PACKUN_STATUS_KIND_SPECIAL_LW_FALL_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    piranhacopter_cancel(boma, status_kind, situation_kind, cat[0]);
	//spike_head_mesh_test(boma);
    sspecial_cancel(boma, status_kind, situation_kind);
    ptooie_scale(boma);
    stance_head(fighter);
    check_reset(fighter);
    check_apply_speeds(fighter);
    stance_init_effects(fighter);
    motion_handler(boma);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_PACKUN )]
pub fn packun_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		packun_frame(fighter);
    }
}

pub unsafe fn packun_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

#[weapon_frame( agent = WEAPON_KIND_PACKUN_POISONBREATH )]
pub fn poisonbreath_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let boma = weapon.boma();
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let owner_object = owner_module_accessor.object();
		let status_kind = StatusModule::status_kind(boma);
		let motion_kind = MotionModule::motion_kind(boma);
        if owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN {
            let pos_x = PostureModule::pos_x(boma);
            let pos_y = PostureModule::pos_y(boma);
            let packun_pos_x = PostureModule::pos_x(owner_module_accessor);
            let packun_pos_y = PostureModule::pos_y(owner_module_accessor);
            let scale = PostureModule::scale(boma);
            if ((pos_x - packun_pos_x).abs() < 12.0*scale) && 
                ((pos_y - packun_pos_y).abs() < 12.0*scale) && 
                pos_y != 0.0 {
                if owner_module_accessor.is_status(*FIGHTER_STATUS_KIND_APPEAL){
                    VarModule::on_flag(owner_object, vars::packun::status::CLOUD_COVER);
                }
                if VarModule::is_flag(owner_object, vars::packun::status::FLAME_ACTIVE) &&
                motion_kind != hash40("explode") {
                    //println!("Woo!");
                    MotionModule::change_motion(weapon.module_accessor, Hash40::new("explode"), 0.0, 1.0, false, 0.0, false, false);
                }
                if VarModule::is_flag(owner_object, vars::packun::status::BITE_START) &&
                motion_kind != hash40("explode") {
                    //println!("Woo!");
                    VarModule::on_flag(owner_object, vars::packun::status::BURST);
                    WorkModule::set_int(boma, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
                }
            }
		}
    }
}

#[weapon_frame( agent = WEAPON_KIND_PACKUN_SPIKEBALL )]
pub fn spikeball_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        let boma = weapon.boma();
        let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        if weapon.motion_frame() == 2.0 && VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE) == 1 {
            VarModule::on_flag(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SHOULD_EXPLODE);
            // println!("bomb");
        }
        else if weapon.motion_frame() == 2.0 && VarModule::get_int(owner_module_accessor.object(), vars::packun::instance::CURRENT_STANCE) != 1 {
            VarModule::off_flag(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SHOULD_EXPLODE);
            // println!("not bomb");
        }
        let status_kind = StatusModule::status_kind(weapon.module_accessor);
        let motion_kind = MotionModule::motion_kind(weapon.module_accessor);
        if owner_module_accessor.kind() == *FIGHTER_KIND_PACKUN {
            if weapon.is_status(*WEAPON_PACKUN_SPIKEBALL_STATUS_KIND_WAIT) || weapon.is_status(*WEAPON_PACKUN_SPIKEBALL_STATUS_KIND_HOP) {
                /* if VarModule::is_flag(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SHOULD_EXPLODE) && weapon.status_frame() == 2 {
                    println!("will bomb");
                } */
                if VarModule::is_flag(owner_module_accessor.object(), vars::packun::instance::PTOOIE_SHOULD_EXPLODE) && weapon.status_frame() >= 80 && motion_kind != hash40("explode") {
                    WorkModule::off_flag(boma, *WEAPON_PACKUN_SPIKEBALL_STATUS_HOP_WORK_FLAG_CLEARED_ATTACK);
                    MotionModule::change_motion(weapon.module_accessor, Hash40::new("explode"), 0.0, 1.0, false, 0.0, false, false);
                    // println!("is bomb");
                }
            }
        }
    }
}