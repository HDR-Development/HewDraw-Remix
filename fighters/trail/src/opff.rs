// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// Blizzaga turnaround handling
unsafe fn special_n2_turnaround_handling(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if fighter.is_status (*FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N2) {
        let facing = PostureModule::lr(fighter.module_accessor);
        let stick_x = fighter.stick_x();
        if stick_x * facing < 0.0 {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
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
    

unsafe fn jab_2_ftilt_cancel(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64) {
    if [*FIGHTER_STATUS_KIND_ATTACK].contains(&status_kind) && motion_kind == hash40("attack_12") {
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            if boma.is_cat_flag(Cat1::AttackS3)
               && (WorkModule::is_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) || WorkModule::is_flag(boma, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)) {
                if !boma.is_in_hitlag() {
                    VarModule::on_flag(boma.object(), vars::trail::instance::ATTACK_12_INTO_S3);
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3,false);
                }
            }
        }
    }
    if ![*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_S3].contains(&status_kind) {
        VarModule::off_flag(boma.object(), vars::trail::instance::ATTACK_12_INTO_S3);
    }
}

// Fair 2 -> aerial cancel
//unsafe fn fair_cancels(boma: &mut BattleObjectModuleAccessor, cat1: i32, status_kind: i32, situation_kind: i32, motion_kind: u64) {
    // Check for aerial attack inputs during fair 2
    //if status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F && motion_kind == hash40("attack_air_f2") /*&& WorkModule::is_flag(boma, *FIGHTER_TRAIL_STATUS_ATTACK_AIR_F_FLAG_ENABLE_COMBO)*/ {
     //   if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
         //   if compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N
              //                      | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3
                 //                   | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4
                 //                   | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3
                  //                  | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4)
               // && ControlModule::get_attack_air_kind(boma) != *FIGHTER_COMMAND_ATTACK_AIR_KIND_F
               // && ControlModule::get_attack_air_kind(boma) != *FIGHTER_COMMAND_ATTACK_AIR_KIND_B {
               // if !boma.is_in_hitlag() {
                //    VarModule::on_flag(boma.object(), vars::trail::instance::COMBO_PLUS_AIR);
               //     StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
             //   }
          //  }
       // }
  //  }
//}

// Magic cancels
unsafe fn magic_cancels(boma: &mut BattleObjectModuleAccessor) {
    // Fire airdodge cancel
    if boma.is_status(*FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N1_SHOOT) && boma.is_motion(Hash40::new("special_air_n1")) && MotionModule::frame(boma) > 2.0 {
        //DamageModule::add_damage(boma, 1.0, 0);
        boma.check_airdodge_cancel();
    }
    // Thunder land cancel
    if boma.is_status(*FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N3)
    && boma.is_situation(*SITUATION_KIND_GROUND)
    && boma.is_prev_situation(*SITUATION_KIND_AIR)
    {
        // Current FAF in motion list is 70, frame is 0 indexed so subtract a frame
        let special_n_fire_cancel_frame_ground = 69.0;
        // 11F of landing lag plus one extra frame to subtract from the FAF to actually get that amount of lag
        let landing_lag = 12.0;
        if MotionModule::frame(boma) < (special_n_fire_cancel_frame_ground - landing_lag) {
            MotionModule::set_frame_sync_anim_cmd(boma, special_n_fire_cancel_frame_ground - landing_lag, true, true, true);
        }
    }
    // Blizzard jump cancel
    if (boma.is_status(*FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N2)
        && MotionModule::frame(boma) > 12.0) {
                boma.check_jump_cancel(false, false);
                WorkModule::off_flag(boma,  *FIGHTER_TRAIL_INSTANCE_WORK_ID_FLAG_MAGIC_SELECT_FORBID);
                WorkModule::on_flag(boma,  *FIGHTER_TRAIL_STATUS_SPECIAL_N2_FLAG_CHANGE_MAGIC);
    }
}

// Actionability after hitting aerial sweep
unsafe fn aerial_sweep_hit_actionability(boma: &mut BattleObjectModuleAccessor) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if boma.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI){
        if MotionModule::frame(boma) > 35.0 {
            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                VarModule::on_flag(boma.object(), vars::trail::status::UP_SPECIAL_HIT);
                VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
            }
        }
        if VarModule::is_flag(boma.object(), vars::trail::status::UP_SPECIAL_HIT) && MotionModule::frame(boma) > MotionModule::end_frame(boma) - 10.0 {
            if boma.get_num_used_jumps() < boma.get_jump_count_max(){
                WorkModule::inc_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
            }
            boma.change_status_req(*FIGHTER_STATUS_KIND_FALL, true);
        }
    }
}

unsafe fn side_special_hit_check(fighter: &mut smash::lua2cpp::L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, id: usize) {
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
            VarModule::on_flag(boma.object(), vars::trail::status::UP_SPECIAL_TO_SIDE_SPECIAL);
        }
        else {
            VarModule::off_flag(boma.object(), vars::trail::status::UP_SPECIAL_TO_SIDE_SPECIAL);
        }
    }
    if status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_ATTACK {
        if !VarModule::is_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT) {
            VarModule::on_flag(boma.object(), vars::common::instance::SIDE_SPECIAL_CANCEL_NO_HIT);
        }
        if fighter.global_table[CURRENT_FRAME].get_i32() == 1 {
            VarModule::off_flag(boma.object(), vars::trail::status::SIDE_SPECIAL_HIT);
            VarModule::off_flag(boma.object(), vars::trail::status::STOP_SIDE_SPECIAL);
        }
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
        && !fighter.is_in_hitlag()
        && (WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("attack_num")) - 1) > WorkModule::get_int(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT) {
            VarModule::on_flag(boma.object(), vars::trail::status::SIDE_SPECIAL_HIT);
            if !VarModule::is_flag(boma.object(), vars::trail::status::UP_SPECIAL_TO_SIDE_SPECIAL)
            && fighter.check_jump_cancel(false, false) {
                return;
            }
        }
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
            VarModule::on_flag(boma.object(), vars::trail::status::STOP_SIDE_SPECIAL);
        }
    }
    if status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_SEARCH {
        if fighter.global_table[CURRENT_FRAME].get_i32() == 1 {
            VarModule::off_flag(boma.object(), vars::trail::status::IS_SIDE_SPECIAL_INPUT);
        }
        if compare_mask(ControlModule::get_command_flag_cat(boma, 0), *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY) {
            VarModule::on_flag(boma.object(), vars::trail::status::IS_SIDE_SPECIAL_INPUT);
        }
        if VarModule::is_flag(boma.object(), vars::trail::status::SIDE_SPECIAL_HIT)
        && WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("attack_num")) > WorkModule::get_int(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT) {
            if !VarModule::is_flag(boma.object(), vars::trail::status::UP_SPECIAL_TO_SIDE_SPECIAL)
            && fighter.check_jump_cancel(false, false) {
                return;
            }
        }
    }
    if status_kind == *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END {
        if !VarModule::is_flag(boma.object(), vars::trail::status::STOP_SIDE_SPECIAL)
        && WorkModule::get_param_int(boma, hash40("param_special_s"), hash40("attack_num")) > WorkModule::get_int(boma, *FIGHTER_TRAIL_STATUS_SPECIAL_S_INT_ATTACK_COUNT)
        && fighter.global_table[CURRENT_FRAME].get_i32() == 15 {
            VarModule::off_flag(boma.object(), vars::trail::status::STOP_SIDE_SPECIAL);
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
            }
            else {
                fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
            }
            return;
        }
    }
}

// handles the speed and disappearance of blizzaga effects
unsafe fn flower_frame(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if ArticleModule::is_exist(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER) {
        let article = ArticleModule::get_article(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER);
        let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(article_id);
        let article_motion = MotionModule::motion_kind(article_boma);
        if article_motion == hash40("special_n2") {
            let blizz_frame = MotionModule::frame(article_boma) as i32;
            if blizz_frame == 1 {
                MotionModule::set_rate(article_boma, 1.1);
            }
            if (12..64).contains(&blizz_frame)
            && status_kind != *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N2 {
                MotionModule::set_rate(article_boma, 1.7);;
            }
            if (65..90).contains(&blizz_frame) {
                MotionModule::set_rate(article_boma, 1.1);
                ArticleModule::remove_exist(boma, *FIGHTER_TRAIL_GENERATE_ARTICLE_FLOWER, app::ArticleOperationTarget(0));
            }
        }
    }
}

// properly cycles Sora's HUD to fire in training mode on reset
unsafe fn training_cycle(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) { 
    if VarModule::is_flag(boma.object(), vars::trail::instance::CYCLE_MAGIC)
    && smash::app::sv_information::is_ready_go() {
        let magic_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TRAIL_INSTANCE_WORK_ID_INT_SPECIAL_N_MAGIC_KIND);
        let trail = fighter.global_table[0x4].get_ptr() as *mut Fighter;
        if magic_kind == *FIGHTER_TRAIL_SPECIAL_N_MAGIC_KIND_FIRE 
        && frame > 3.0 {
            WorkModule::on_flag(fighter.boma(), *FIGHTER_TRAIL_STATUS_SPECIAL_N1_FLAG_CHANGE_MAGIC);
            FighterSpecializer_Trail::change_magic(trail); // cycles to thunder
        } else if magic_kind == *FIGHTER_TRAIL_SPECIAL_N_MAGIC_KIND_THUNDER
        && frame > 4.0 {
            FighterSpecializer_Trail::change_magic(trail); // cycles to "blizzard", which is now fire
            VarModule::off_flag(boma.object(), vars::trail::instance::CYCLE_MAGIC);
        }
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && ( fighter.is_status_one_of(&[
        *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_N3,
        *FIGHTER_TRAIL_STATUS_KIND_SPECIAL_S_END,
        ]) 
        || (fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) && fighter.status_frame() > 10) )
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
    jab_2_ftilt_cancel(boma, cat[0], status_kind, situation_kind, motion_kind);
    //fair_cancels(boma, cat[0], status_kind, situation_kind, motion_kind);
    side_special_hit_check(fighter, boma, status_kind, situation_kind, id);
    nair_fair_momentum_handling(fighter, boma);
    magic_cancels(boma);
    aerial_sweep_hit_actionability(boma);
    flower_frame(boma, status_kind);
    training_cycle(fighter, boma, frame);
    fastfall_specials(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_TRAIL)]
pub fn trail_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		trail_frame(fighter);
    }
}

pub unsafe fn trail_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        let status_kind = if info.status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N || info.status_kind == *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F { // status kind checks for nair/fair
            *FIGHTER_STATUS_KIND_ATTACK_AIR
        } else {
            info.status_kind
        };
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}