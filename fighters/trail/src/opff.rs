// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn jab_2_ftilt_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_motion(Hash40::new("attack_12")) {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if boma.is_cat_flag(Cat1::AttackS3)
               && (WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) || WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)) {
                if !boma.is_in_hitlag() {
                    VarModule::on_flag(boma.object(), vars::trail::instance::ATTACK_12_ENABLE_S3_COMBO);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, false);
                }
            }
        }
    }
}

// lets sora bounce upwards upon landing down smash
unsafe fn attack_lw4_rebound(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_LW4)
    && (18.0..20.0).contains(&boma.motion_frame())
    && AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        VarModule::on_flag(boma.object(), vars::trail::instance::ATTACK_LW4_REBOUND);
        KineticModule::clear_speed_energy_id(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::add_speed(boma, &Vector3f::new(0.0, 1.5, 0.0));
        boma.change_status_req(*FIGHTER_STATUS_KIND_CLIFF_JUMP2, true);
    }
}

unsafe fn nair_fair_momentum_handling(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    // Fair/nair's external velocity setting might be handled via an on hit event or smth as I could not locate them in the status scripts, once we find those and edit as appropriate we should come back and remove this functionality here
    // - Calc
    if boma.is_motion(Hash40::new("attack_air_n")){
        WorkModule::on_flag(boma, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_N_FLAG_HIT_SPEED_Y);
    }

    // Fair momentum handling now moved to OPFF since params that affect both nair and fair's momentum on hit were standardized to give nair regular momentum
    if boma.is_status(*FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F){
        if boma.is_motion(Hash40::new("attack_air_f")) || boma.is_motion(Hash40::new("attack_air_f2")){
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD){
                let initial_x_mul = 0.35;
                let control_energy = KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) as *mut smash::app::KineticEnergy;
                smash::app::lua_bind::KineticEnergy::mul_speed(control_energy, &Vector3f::new(0.1, 1.0, 1.0));
                // println!("is_infliction triggered!");
            }

            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD){
                //let x_mul = WorkModule::get_param_float(boma, hash40("param_private"), hash40("attack_air_hit_speed_max_x_mul"))
                // Max airspeed multiplier
                let max_x_mul = 0.65;
                // Max air accel adjustment
                let max_accel_x_adjustment = -0.35;
                let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
                let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
                let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
                let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
                let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
                let facing = PostureModule::lr(boma);

                let stick_x = ControlModule::get_stick_x(boma);
                let stick_threshold = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("walk_stick_x"));
                if stick_x.abs() > stick_threshold {
                    // Apply acceleration opposite to your current drift to mimic vanilla's accel reduction on hit after fair
                    KineticModule::add_speed(boma, &Vector3f::new(max_accel_x_adjustment * (air_accel_x_add * stick_x.signum() + air_accel_x_mul * stick_x) * facing, 0.0, 0.0));
                }

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, max_x_mul * air_speed_x_stable, -1.0 * air_speed_y_stable);
                app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
                fighter.clear_lua_stack();

                fighter.clear_lua_stack();
                // println!("is_infliction_status triggered! setting limit speed to... {}", air_speed_x_stable * max_x_mul);
                // println!("fall speed is... {}", air_speed_y_stable);
            }
        }
    }
}

unsafe fn magic_handling(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    // firaga airdodge cancel
    if boma.is_status(*FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N1_SHOOT)
    && boma.is_motion(Hash40::new("special_air_n1"))
    && boma.motion_frame() > 2.0 {
        boma.check_airdodge_cancel();
    }
    // thundaga land cancel
    if boma.is_status(*FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N3) {
        let landing_lag = 12.0; // 11F of landing lag plus one extra frame to subtract from the FAF to actually get that amount of lag
        fighter.check_land_cancel(Some(landing_lag));
    }
    // blizzaga jump cancel
    if (boma.is_status(*FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N2)
    && boma.motion_frame() > 12.0) {
        boma.check_jump_cancel(false, false, false);
    }

    // handles the cooldown timer between casting spells
    if VarModule::get_int(boma.object(), vars::trail::instance::SPECIAL_N_MAGIC_TIMER) > 0 {
        VarModule::dec_int(boma.object(), vars::trail::instance::SPECIAL_N_MAGIC_TIMER);

        // cycles and enables magic on the last frame of the cooldown window
        if VarModule::get_int(boma.object(), vars::trail::instance::SPECIAL_N_MAGIC_TIMER) == 1 {
            let trail = fighter.global_table[0x4].get_ptr() as *mut Fighter;

            WorkModule::off_flag(boma,  *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);

            // 0x2100000C is needed by FighterSpecializer_Trail::change_magic
            // for it to work properly
            if WorkModule::is_flag(boma,  0x2100000C) {
                FighterSpecializer_Trail::change_magic(trail);
            }
            else {
                WorkModule::on_flag(boma,  0x2100000C);
                FighterSpecializer_Trail::change_magic(trail);
                WorkModule::off_flag(boma,  0x2100000C);
            }

            VarModule::off_flag(boma.object(), vars::trail::instance::DISABLE_SPECIAL_N);
        }
    }
}

// handles the speed and disappearance of blizzaga effects
unsafe fn flower_frame(boma: &mut BattleObjectModuleAccessor) {
    if ArticleModule::is_exist(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER) {
        let article = ArticleModule::get_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER);
        let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(article_id);
        if MotionModule::motion_kind(article_boma) == hash40("special_n2") {
            let blizz_frame = MotionModule::frame(article_boma) as i32;
            if blizz_frame == 1 {
                MotionModule::set_rate(article_boma, 1.1);
            }
            if (12..64).contains(&blizz_frame)
            && !boma.is_status(*FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N2) {
                MotionModule::set_rate(article_boma, 1.7);
            }
            if (65..90).contains(&blizz_frame) {
                MotionModule::set_rate(article_boma, 1.1);
                ArticleModule::remove_exist(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER, app::ArticleOperationTarget(0));
            }
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && (fighter.is_status(*FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N3)
        || (fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) && fighter.status_frame() > 10))
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn initialize_magic(fighter: &mut L2CFighterCommon) {
    if VarModule::is_flag(fighter.battle_object, vars::common::instance::IS_INIT) {
        return;
    }
    let magic_kind = fighter.get_int(*FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND);
    let trail = fighter.global_table[0x4].get_ptr() as *mut Fighter;
    // if magic_kind == *FIGHTER_TRAIL_SPECIAL_N_MAGIC_KIND_FIRE {
        fighter.on_flag(*FIGHTER_TRAIL_STATUS_SPECIAL_N1_FLAG_CHANGE_MAGIC);
        FighterSpecializer_Trail::change_magic(trail); // cycles to thunder
        fighter.on_flag(*FIGHTER_TRAIL_STATUS_SPECIAL_N1_FLAG_CHANGE_MAGIC);
        FighterSpecializer_Trail::change_magic(trail); // cycles to "blizzard", which is now fire
        VarModule::on_flag(fighter.battle_object, vars::common::instance::IS_INIT);
    // }
}

pub unsafe fn moveset(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    jab_2_ftilt_cancel(boma);
    nair_fair_momentum_handling(fighter, boma);
    attack_lw4_rebound(boma);
    magic_handling(fighter, boma);
    flower_frame(boma);
    fastfall_specials(fighter);
    initialize_magic(fighter);
}

pub extern "C" fn trail_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		trail_frame(fighter);
    }
}

pub unsafe fn trail_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        // let status_kind = if info.status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N || info.status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F { // status kind checks for nair/fair
        //     *FIGHTER_STATUS_KIND_ATTACK_AIR
        // } else {
        //     info.status_kind
        // };
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, trail_frame_wrapper);
}