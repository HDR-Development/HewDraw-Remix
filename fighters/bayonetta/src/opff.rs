use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn aerial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_motion_one_of(&[Hash40::new("attack_air_n"), Hash40::new("attack_air_hi"), Hash40::new("attack_air_lw"), Hash40::new("attack_air_f3"), Hash40::new("attack_air_b")]) //dont cancel with fair 1/2
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) //dont cancel on shield
    && !fighter.is_in_hitlag() //dont cancel during hitstop
    && !fighter.is_flag(*FIGHTER_BAYONETTA_STATUS_ATTACK_AIR_FLAG_SHOOTING) { //don't cancel DURING bullet arts
        let mut new_status = 0;
        let mut is_input_cancel = false;
        if fighter.is_cat_flag(Cat1::SpecialN) {
            is_input_cancel = true;
            new_status = *FIGHTER_STATUS_KIND_SPECIAL_N;
        } else if fighter.is_cat_flag(Cat1::SpecialHi) {
            if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USED_COUNT) < 2 {
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
    //reset fair state 
    if !fighter.is_situation(*SITUATION_KIND_AIR) {VarModule::set_int(boma.object(), vars::bayonetta::instance::FAIR_STATE, 0); }
    //reset dabk count
    if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) == 0 {VarModule::set_int(boma.object(), vars::bayonetta::instance::DABK_COUNT, 0); }
    //refresh hdr resources
    let resource = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED);
    let lag = fighter.get_float(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME);
    if lag < 1.0 { //same methd as vanilla
        //filters out lag-cancel techs
        if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) == 0 && fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USED_COUNT) == 0 {
            if fighter.is_situation(*SITUATION_KIND_AIR) && resource >= 1 {
                VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 1);
            } else { //only gives back 2nd resource
                VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::NUM_RECOVERY_RESOURCE_USED, 0);
            }
        }
    } else if resource >= 2 {
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI); //double check if need to set use count to 2 if jump restores it
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
        //fighter.set_int(2, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT);
        //fighter.set_int(2, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USED_COUNT);
    } //check vanilla lag counter
}

unsafe fn forward_tilt(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("attack_s3_s")) && MotionModule::frame(boma) >= 28.0 {
        if !boma.is_cat_flag(Cat1::AttackLw3 | Cat1::Catch) {
            if boma.is_cat_flag(Cat1::AttackHi3 | Cat1::SpecialN | Cat1::SpecialHi) {//vert kick
                MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_s3_s3"), 0.0, 1.0, false, 0.0, false, false);
            } else if boma.is_cat_flag(Cat1::AttackS3 | Cat1::AttackN) { //side kick
                MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_s3_s2"), 0.0, 1.0, false, 0.0, false, false);
            } else if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) && boma.is_button_on(Buttons::Attack) {
                MotionModule::change_motion(boma, smash::phx::Hash40::new("attack_s3_s3"), 0.0, 1.0, false, 0.0, false, false);
            }//hold
        }
    }
}

unsafe fn bat_within_air_motion(fighter: &mut L2CFighterCommon) {
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ITEM_SWING_S4_HOLD])
    && !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_WITCH_TIME) {
        fighter.set_int(0, *FIGHTER_BAYONETTA_STATUS_ATTACK_INT_WITCH_TIME_SMASH_HOLD);
    } //witch time charge nerf check frame 0 -> every frame
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
        statuses::bayonetta::SPECIAL_N_CANCEL,
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
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