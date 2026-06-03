use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

unsafe fn aerial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_motion_one_of(&[Hash40::new("attack_air_n"), Hash40::new("attack_air_f3"), Hash40::new("attack_air_b"), Hash40::new("attack_air_hi"), Hash40::new("attack_air_lw")]) {
        // dont cancel with fair 1/2
        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) { // dont cancel on shield
            WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
            WorkModule::enable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
            // cancel into second wtw/abk without prereq
            if VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT) < 2 {
                VarModule::on_flag(fighter.battle_object, vars::bayonetta::status::RECOVERY_RESOURCE_BYPASS_CHECK);
            }
        }
        if !fighter.is_in_hitlag()  // dont cancel during hitstop
        && !StopModule::is_stop(boma)
        && !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION) {  // don't cancel DURING bullet arts
            fighter.sub_transition_group_check_air_special();
            fighter.sub_transition_group_check_air_escape();
        }
    }
    if fighter.is_motion(Hash40::new("landing_air_lw")) {
        // hitstop checks dont work with dair..
        if AttackModule::is_attack(boma, 0, false) {
            WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
            WorkModule::unable_transition_term_group(boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
        } else {
            // cancel dair landing if either falling dair or landing hit made contact
            if VarModule::get_int(fighter.battle_object, vars::common::instance::PREV_STATUS_INFLICT_STATUS) & *COLLISION_KIND_MASK_HIT != 0 
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
            }
            fighter.sub_transition_group_check_ground_special();
        }

    }
}

unsafe fn reset_flags(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    // reset fair state 
    if !fighter.is_situation(*SITUATION_KIND_AIR) {VarModule::set_int(boma.object(), vars::bayonetta::instance::ATTACK_AIR_F_COUNT, 0); }
    // refresh hdr resources
    let resource = VarModule::get_int(fighter.battle_object, vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT);
    let lag = fighter.get_float(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLOAT_SPECIAL_LANDING_FRAME);
    //if lag < 1.0 {  // same methd as vanilla
        // filters out lag-cancel techs
    if fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_AIR_S_USED_COUNT) == 0
    && fighter.get_int(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_INT_SPECIAL_HI_USED_COUNT) == 0 {
        // reset added special use trackers
        fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
        fighter.off_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
        VarModule::set_int(boma.object(), vars::bayonetta::instance::SPECIAL_S_DABK_COUNT, 0);
        VarModule::set_int(boma.object(), vars::bayonetta::instance::SPECIAL_BULLET_ARTS_COUNT, 0);
        // give back 1 resource if burnt out
        if fighter.is_situation(*SITUATION_KIND_AIR) && resource >= 1 {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT, 1);
        } else {
            VarModule::set_int(fighter.battle_object, vars::bayonetta::instance::RECOVERY_RESOURCE_COUNT, 0);
        }
    }
    //} else 
    if resource >= 2 { // skip using HDR flags (vanilla seems to more thoroughly reset it?)
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_HI);
        fighter.on_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    }
}

unsafe fn bat_within_air_motion(fighter: &mut L2CFighterCommon) {
    let boma: &mut BattleObjectModuleAccessor = fighter.boma();
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ITEM_SWING_S4_HOLD])
    && !fighter.is_flag(*FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_WITCH_TIME) {
        fighter.set_int(0, *FIGHTER_BAYONETTA_STATUS_ATTACK_INT_WITCH_TIME_SMASH_HOLD);
    }  // witch time charge nerf check frame 0 -> every frame
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
        //*FIGHTER_STATUS_KIND_SPECIAL_N,
        //*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE,
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_FIRE,
        //*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_END,
        //statuses::bayonetta::SPECIAL_N_CANCEL,
        //*FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_HI_JUMP,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_D_HIT,
        *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_AIR_S_WALL_END]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    aerial_cancels(fighter, boma);
    reset_flags(fighter, boma);
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