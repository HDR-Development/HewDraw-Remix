use super::*;
use globals::*;
use smash::app::lua_bind::CancelModule::is_enable_cancel;
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn aerial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if (fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR) || fighter.is_motion(Hash40::new("attack_air_f3")))
    && !fighter.is_motion_one_of(&[Hash40::new("attack_air_n_hold"), Hash40::new("attack_air_hi_hold"), Hash40::new("attack_air_lw_hold")])
    && VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) 
    && !CancelModule::is_enable_cancel(boma) {
        let mut new_status = 0;
        let mut is_input_cancel = false;
        if fighter.is_cat_flag(Cat1::SpecialN) {fighter.change_status_req(*FIGHTER_STATUS_KIND_SPECIAL_N, false);
        } else if fighter.is_cat_flag(Cat1::SpecialS) {
            if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) < 2 && !VarModule::is_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL) {
                is_input_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_SPECIAL_S;
            }
        } else if fighter.is_cat_flag(Cat1::SpecialHi) {
            if !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI) && !VarModule::is_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL) {
                is_input_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_SPECIAL_HI;
            }
        } 
        if !fighter.is_in_hitlag() {
            fighter.check_airdodge_cancel();
            if is_input_cancel {//special cancel
                fighter.change_status_req(new_status, false);
            } 
        }
    }
}

unsafe fn nspecial_mechanics(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE) { //PM-like neutral-b canceling
        if fighter.is_situation(*SITUATION_KIND_AIR) {
            if fighter.is_cat_flag(Cat1::AirEscape)  {
                ControlModule::reset_trigger(boma);
                StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_FALL, true);
                ControlModule::clear_command_one(fighter.module_accessor, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE);
            }//drift
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            sv_kinetic_energy!(controller_set_accel_x_mul, fighter, 0.04);
            sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.009);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.4, 0.0);
        }
    }
}

unsafe fn reset_flags(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    let boma = fighter.boma();
    if (StatusModule::is_situation_changed(boma) && !fighter.is_situation(*SITUATION_KIND_AIR))//checks for (re)spawn or grounded state
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD,
                                  *FIGHTER_STATUS_KIND_REBIRTH,
                                  *FIGHTER_STATUS_KIND_WIN,
                                  *FIGHTER_STATUS_KIND_LOSE,
                                  *FIGHTER_STATUS_KIND_ENTRY]){
        VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 0);
        VarModule::set_int(boma.object(), vars::bayonetta::instance::FAIR_STATE, 0);
    }
    //resets flags if hit
    if StopModule::is_stop(boma) {
        if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) > 1 {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 1);
            VarModule::off_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
            VarModule::off_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
        }
    } 
    //resets hitflag
    if VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) {
        if StatusModule::is_changing(boma) && !boma.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP) {
            VarModule::off_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT);
        }
    }
}

unsafe fn resources(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    //resource limit
    if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) >= 2 {
        VarModule::on_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
        VarModule::on_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
    }
    //hit-flag
    if !VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) && AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) && VarModule::get_int(fighter.battle_object, vars::common::instance::LAST_ATTACK_HITBOX_ID) < 6 {
        VarModule::on_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT);
    }
}

unsafe fn abk_angling(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U) {
        let facing = PostureModule::lr(boma);
        let anglestick = VarModule::get_float(fighter.battle_object, vars::bayonetta::status::ABK_ANGLE);
        joint_rotator(fighter, frame, Hash40::new("top"), Vector3f{x: -14.5*anglestick, y:90.0*facing, z:0.0}, 10.0, 15.0, 45.0, 55.0);
        if boma.status_frame() <= 8 { VarModule::set_float(fighter.battle_object, vars::bayonetta::status::ABK_ANGLE, boma.left_stick_y());}
        //trajectory
        else if boma.status_frame() <= 26 && !fighter.is_in_hitlag() {
            KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f::new( -0.41 * anglestick * facing, anglestick*0.67, 0.0));
        }
    }
}

unsafe fn forward_tilt(fighter: &mut L2CFighterCommon) {
    let boma = fighter.boma();
    if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_S3) {
        if fighter.is_flag(*FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
            && !fighter.is_cat_flag(Cat1::Catch)
            && fighter.is_button_on(Buttons::Attack) {
                StatusModule::change_status_force(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
            } //filter inputs, auto progress (could be done in status)
        }
    }
}

unsafe fn bat_within_air_motion(fighter: &mut L2CFighterCommon) {
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_LW_BATWITHIN) {
            KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.4, 0.0);
            sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 1.0);
        } else if fighter.is_status(*FIGHTER_BAYONETTA_STATUS_KIND_BATWITHIN) {
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, boma.left_stick_y() * 0.4);
            sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.4, 0.0);
        }
    }
}

unsafe fn joint_rotator(fighter: &mut L2CFighterCommon, frame: f32, joint: Hash40, rotation_amount: Vector3f, start_frame: f32, bend_frame: f32, return_frame: f32, straight_frame: f32) {
    let lua_state = fighter.lua_state_agent;
    let end_frame = MotionModule::end_frame(fighter.boma());
    let max_rotation = rotation_amount;
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    if frame >= start_frame && frame < return_frame {
        // this has to be called every frame, or you snap back to the normal joint angle
        // interpolate to the respective waist bend angle
        let calc_x_rotate = max_rotation.x * (frame / (bend_frame - start_frame));
        let calc_y_rotate = max_rotation.y * (frame / (bend_frame - start_frame));
        let calc_z_rotate = max_rotation.z * (frame / (bend_frame - start_frame));
        let mut x_rotation = 0.0;
        let mut y_rotation = 0.0;
        let mut z_rotation = 0.0;
        if max_rotation.x < 0.0 {
            x_rotation = calc_x_rotate.clamp(max_rotation.x, 0.0);
        }
        else {
            x_rotation = calc_x_rotate.clamp(0.0, max_rotation.x);
        }
        if max_rotation.y < 0.0 {
            y_rotation = calc_y_rotate.clamp(max_rotation.y, 0.0);
        }
        else {
            y_rotation = calc_y_rotate.clamp(0.0, max_rotation.y);
        }
        if max_rotation.z < 0.0 { 
            z_rotation = calc_z_rotate.clamp(max_rotation.z, 0.0);
        }
        else{
            z_rotation = calc_z_rotate.clamp(0.0, max_rotation.z);
        }
        rotation = Vector3f{x: x_rotation, y: y_rotation, z: z_rotation};
        ModelModule::set_joint_rotate(fighter.boma(), joint, &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    } else if frame >= return_frame && frame < straight_frame {
        // linear interpolate back to normal
        let calc_x_rotate = max_rotation.x *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let calc_y_rotate = max_rotation.y *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let calc_z_rotate = max_rotation.z *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let mut x_rotation = 0.0;
        let mut y_rotation = 0.0;
        let mut z_rotation = 0.0;
        if max_rotation.x < 0.0 {
            x_rotation = calc_x_rotate.clamp(max_rotation.x, 0.0);
        }
        else {
            x_rotation = calc_x_rotate.clamp(0.0, max_rotation.x);
        }
        if max_rotation.y < 0.0 {
            y_rotation = calc_y_rotate.clamp(max_rotation.y, 0.0);
        }
        else {
            y_rotation = calc_y_rotate.clamp(0.0, max_rotation.y);
        }
        if max_rotation.z < 0.0 { 
            z_rotation = calc_z_rotate.clamp(max_rotation.z, 0.0);
        }
        else{
            z_rotation = calc_z_rotate.clamp(0.0, max_rotation.z);
        }
        rotation = Vector3f{x: x_rotation, y: y_rotation, z: z_rotation};
        ModelModule::set_joint_rotate(fighter.boma(), joint, &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE,
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_FIRE,
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_HIT,
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END]) 
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

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    //aerial_cancels(fighter, boma);
    nspecial_mechanics(fighter, boma);
    reset_flags(fighter, boma);
    resources(fighter, boma);
    abk_angling(fighter, boma, frame);
    forward_tilt(fighter);
    bat_within_air_motion(fighter);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_BAYONETTA)]
pub unsafe fn bayonetta_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    bayonetta_frame(fighter);
}
pub unsafe fn bayonetta_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.frame);
    }
}