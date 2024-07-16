use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

use vars::daisy::instance::*;

unsafe fn wall_bounce(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP {
        let lr = PostureModule::lr(boma);
        let frame = MotionModule::frame(boma) as i32;
        let mut touch_wall = false;
        if lr > 0.0 {
            touch_wall = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT as u32);
        } else {
            touch_wall = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT as u32);
        };
        if touch_wall && (1..25).contains(&frame) {
            VarModule::on_flag(boma.object(), vars::peach::instance::IS_WALLBOUNCE);
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_HIT_END, true);
        }
    }
}

unsafe fn up_special_freefall_land_cancel(fighter: &mut L2CFighterCommon) {
    if fighter.is_prev_status(*FIGHTER_STATUS_KIND_FALL_SPECIAL)
    && fighter.is_status(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING, false);
    }
}

// transition daisy into unique animations for her third jump
unsafe fn triple_jump_motion(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_motion_one_of(&[Hash40::new("jump_aerial_f"), Hash40::new("jump_aerial_b")])
    && boma.get_num_used_jumps() == boma.get_jump_count_max() {
        if fighter.is_motion(Hash40::new("jump_aerial_f")) {
            MotionModule::change_motion(boma, Hash40::new("jump_aerial_f2"), 0.0, 1.0, false, 0.0, false, false);
        } else {
            MotionModule::change_motion(boma, Hash40::new("jump_aerial_b2"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
}

// have daisy always hold the racket for forward smash
unsafe fn racket_visibility(fighter: &mut L2CFighterCommon) {
    if (fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_S4) && fighter.motion_frame() < 40.0)
    || fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD) {
        VarModule::on_flag(fighter.object(), ATTACK_S4_RACKET_ACTIVE);
        if PostureModule::lr(fighter.boma()) == 1.0 {
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketmflip"), true);
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketm"), false);
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("panm"), false);
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("clubm"), false);
        } else {
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketm"), true);
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketmflip"), false);
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("panmflip"), false);
            ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("clubmflip"), false);
        }
    } else if VarModule::is_flag(fighter.object(), ATTACK_S4_RACKET_ACTIVE) {
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketm"), false);
        ModelModule::set_mesh_visibility(fighter.boma(), Hash40::new("racketmflip"), false);
        VarModule::off_flag(fighter.object(), ATTACK_S4_RACKET_ACTIVE);
    }
}

// various methods for handling daisy's crystal models and effects
unsafe fn crystal_handling(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    // statuses to allow the crystal gauntlet
    let is_gauntlet_status = fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        //*FIGHTER_PEACH_STATUS_KIND_SPECIAL_HI_AIR_END
    ]);

    if ArticleModule::is_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR) {
        let article = ArticleModule::get_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR);
        let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(article_id);
        // toggle the correct fist mesh depending on the direction daisy is facing
        if PostureModule::lr(boma) == 1.0 {
            ModelModule::set_mesh_visibility(article_boma, Hash40::new("daisy_glove"), false);
            ModelModule::set_mesh_visibility(article_boma, Hash40::new("daisy_gloveleft"), true);
        } else {
            ModelModule::set_mesh_visibility(article_boma, Hash40::new("daisy_glove"), true);
            ModelModule::set_mesh_visibility(article_boma, Hash40::new("daisy_gloveleft"), false);
        }
        
        // remove the gauntlet if not in an allowed status
        if !is_gauntlet_status {
            ArticleModule::remove_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
}

// handles the hit team for daisy's vegetable item, allowing it to be hit around
unsafe fn set_vegetable_team(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if ItemModule::get_have_item_kind(fighter.boma(), 0) == *ITEM_KIND_DAISYDAIKON {
        let item_id = ItemModule::get_have_item_id(boma, 0) as u32;
        let item_boma = sv_battle_object::module_accessor(item_id);
        VarModule::set_int(boma.object(), vars::daisy::instance::VEGETABLE_ID, item_id as i32);
        let team = TeamModule::hit_team_no(boma) as i32;
        TeamModule::set_team(item_boma, team, true);
        TeamModule::set_hit_team(item_boma, team);
        HitModule::sleep(item_boma, true); // disable hurtbox when holding
    } else {
        let item_id = VarModule::get_int(boma.object(), vars::daisy::instance::VEGETABLE_ID) as u32;
        let item_boma = sv_battle_object::module_accessor(item_id);
        // bool for if daisy is the owner of a carrot before affecting it, preventing ownership jank in daisy dittos
        let is_owner = TeamModule::hit_team_no(item_boma) == TeamModule::hit_team_no(boma);
        // measure how far the item is from daisy
        let x_distance = PostureModule::pos_x(boma) - PostureModule::pos_x(item_boma);
        let y_distance = (PostureModule::pos_y(boma) + 10.0) - PostureModule::pos_y(item_boma);
        // make sure the item is far enough away from daisy to prevent oddities with hitting herself
        let is_separated = x_distance.abs() > 15.0 || y_distance.abs() > 15.0;
        
        // toggle the hurtbox if an opponent is holding the carrot
        if StatusModule::status_kind(item_boma) == *ITEM_STATUS_KIND_HAVE {
            HitModule::sleep(item_boma, true);
        } else {
            HitModule::sleep(item_boma, false);
        }

        if TeamModule::hit_team_no(item_boma) as i32 != -1
        && is_owner && is_separated {
            //println!("changing {} hit team to universal", TeamModule::hit_team_no(item_boma));
            TeamModule::set_hit_team(item_boma, -1);
        }

        let current_damage = DamageModule::damage(item_boma, 0);
        let prev_damage = VarModule::get_float(boma.object(), vars::daisy::instance::VEGETABLE_DAMAGE);
        if current_damage > prev_damage && is_separated { 
            //println!("current {}, prev {}", current_damage, prev_damage);
            if !AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) {
                let num_players = Fighter::get_fighter_entry_count();
                let mut opponent_team = 7;
                for i in 0..num_players{
                    let opponent_boma = sv_battle_object::module_accessor(Fighter::get_id_from_entry_id(i));
                    if AttackModule::is_infliction(opponent_boma, *COLLISION_KIND_MASK_HIT) {
                        opponent_team = TeamModule::hit_team_no(opponent_boma) as i32;
                        //println!("Hit by opponent on team {}", opponent_team);
                        break;
                    }
                }
                //println!("Changing to team {}", opponent_team);
                TeamModule::set_team(item_boma, opponent_team, false);
                StatusModule::change_status_force(item_boma, *ITEM_STATUS_KIND_THROW, true); // resets the throw status so the hitbox doesn't clear
            } else {
                //println!("Hit by Daisy");
                let team = TeamModule::hit_team_no(boma) as i32;
                TeamModule::set_team(item_boma, team, false);
                StatusModule::change_status_force(item_boma, *ITEM_STATUS_KIND_THROW, true); // resets the throw status so the hitbox doesn't clear
            }
            VarModule::set_float(boma.object(), vars::daisy::instance::VEGETABLE_DAMAGE, current_damage);
        } else if current_damage != prev_damage {
            // handle the variable in other scenarios. mostly on item despawn
            VarModule::set_float(boma.object(), vars::daisy::instance::VEGETABLE_DAMAGE, current_damage);
        }
    }

    // also hide the carrot (and other items) when daisy is shielding
    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_GUARD,
        *FIGHTER_STATUS_KIND_GUARD_ON
    ]) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT,
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

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    wall_bounce(boma, status_kind);
    //up_special_freefall_land_cancel(fighter);
    triple_jump_motion(fighter, boma);
    racket_visibility(fighter);
    crystal_handling(fighter, boma);
    set_vegetable_team(fighter, boma);
    fastfall_specials(fighter);
}

pub unsafe extern "C" fn daisy_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    daisy_frame(fighter)
}

pub unsafe fn daisy_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, daisy_frame_wrapper);
}