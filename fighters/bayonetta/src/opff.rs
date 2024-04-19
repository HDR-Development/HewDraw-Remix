use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn aerial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_motion_one_of(&[Hash40::new("attack_air_n"), Hash40::new("attack_air_hi"), Hash40::new("attack_air_lw"), Hash40::new("attack_air_f3"), Hash40::new("attack_air_b")])
    && VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) 
    && !fighter.is_in_hitlag() {
        let mut new_status = 0;
        let mut is_input_cancel = false;
        if fighter.is_cat_flag(Cat1::SpecialN) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_SPECIAL_N;
        } else if fighter.is_cat_flag(Cat1::SpecialHi) {
            if !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI) {
                is_input_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_SPECIAL_HI;
            }
        } else if fighter.is_cat_flag(Cat1::SpecialS) {
            if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) < 2 {
                is_input_cancel = true;
                new_status = *FIGHTER_STATUS_KIND_SPECIAL_S;
            }
        }
        if !fighter.is_motion(Hash40::new("attack_air_lw")) {fighter.check_airdodge_cancel(); }
        if is_input_cancel && VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) < 2 {
            StatusModule::change_status_force(boma, new_status, true);
        } //special cancel
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
    if StopModule::is_stop(boma) || StopModule::is_damage(boma) {
        if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED) > 1 {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 1);
            VarModule::off_flag(fighter.battle_object, vars::common::instance::SIDE_SPECIAL_CANCEL);
            VarModule::off_flag(fighter.battle_object, vars::common::instance::UP_SPECIAL_CANCEL);
        }
    }
    //resets hitflag
    if VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) {
        if StatusModule::is_changing(boma) && !boma.is_status_one_of(&[*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP, statuses::bayonetta::SPECIAL_S_EDGE]) {
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
    //hit-flag to filter bullets
    if !VarModule::is_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT) && VarModule::get_int(fighter.battle_object, vars::common::instance::LAST_ATTACK_HITBOX_ID) < 6 {
        //hit cancel moves (filter shield)
        if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_AIR, *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_U, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP]) {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {VarModule::on_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT); }
        } else { //other moves
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {VarModule::on_flag(fighter.battle_object, vars::bayonetta::instance::IS_HIT); }
        }
    }
}

unsafe fn forward_tilt(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("attack_s3_s")) && MotionModule::frame(boma) >= 28.0 {
        if boma.is_cat_flag(Cat1::AttackHi3 | Cat1::SpecialN | Cat1::SpecialHi) {//vert kick
            MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_s3_s3"), 0.0, 1.0, false, 0.0, false, false);
        } else if boma.is_cat_flag(Cat1::AttackS3 | Cat1::AttackN) { //side kick
            MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_s3_s2"), 0.0, 1.0, false, 0.0, false, false);
        } else if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) && boma.is_button_on(Buttons::Attack) {
            MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_s3_s3"), 0.0, 1.0, false, 0.0, false, false);
        }//hold
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
    aerial_cancels(fighter, boma);
    reset_flags(fighter, boma);
    resources(fighter, boma);
    forward_tilt(boma);
    bat_within_air_motion(fighter);
    fastfall_specials(fighter);
}

pub unsafe extern "C" fn bayonetta_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    bayonetta_frame(fighter);
}

pub unsafe fn bayonetta_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, bayonetta_frame_wrapper);
}