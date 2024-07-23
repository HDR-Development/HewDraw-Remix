use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

use vars::daisy::instance::*;

unsafe fn wall_bounce(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP) {
        let lr = PostureModule::lr(boma);
        let frame = MotionModule::frame(boma) as i32;
        let mut touch_wall = false;
        if lr > 0.0 {
            touch_wall = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT as u32);
        } else {
            touch_wall = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT as u32);
        };
        if touch_wall && (1..25).contains(&frame) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_HIT_END, true);
        }
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

unsafe fn crystal_handling(boma: &mut BattleObjectModuleAccessor) {
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
    }
}

// handles daisy's vegetable item, allowing it to be hit around
unsafe fn vegetable_handling(boma: &mut BattleObjectModuleAccessor) {
    if ItemModule::get_have_item_kind(boma, 0) == *ITEM_KIND_DAISYDAIKON {
        let item_id = ItemModule::get_have_item_id(boma, 0) as u32;
        let item_boma = sv_battle_object::module_accessor(item_id);
        VarModule::set_int(boma.object(), VEGETABLE_ID, item_id as i32);
        let team = TeamModule::hit_team_no(boma) as i32;
        TeamModule::set_team(item_boma, team, true);
        TeamModule::set_hit_team(item_boma, team);
        HitModule::sleep(item_boma, true); // disable hurtbox when holding
    } else {
        let item_id = VarModule::get_int(boma.object(), VEGETABLE_ID) as u32;
        let item_boma = sv_battle_object::module_accessor(item_id);
        // bool for if daisy is the owner of a carrot before affecting it, preventing ownership jank in daisy dittos
        let is_owner = TeamModule::hit_team_no(item_boma) == TeamModule::hit_team_no(boma);
        // measure how far the item is from daisy, used to prevent oddities with hitting herself
        let x_distance = PostureModule::pos_x(boma) - PostureModule::pos_x(item_boma);
        let y_distance = (PostureModule::pos_y(boma) + 10.0) - PostureModule::pos_y(item_boma);
        let is_separated = x_distance.abs() > 15.0 || y_distance.abs() > 15.0;
        
        // decides whether or not the hurtbox should be active
        if StatusModule::status_kind(item_boma) == *ITEM_STATUS_KIND_HAVE 
        || VarModule::is_flag(boma.object(), VEGETABLE_LOCKOUT) {
            HitModule::sleep(item_boma, true);
        } else {
            HitModule::sleep(item_boma, false);
        }

        if TeamModule::hit_team_no(item_boma) as i32 != -1
        && is_owner && is_separated {
            //println!("changing {} hit team to universal", TeamModule::hit_team_no(item_boma));
            TeamModule::set_hit_team(item_boma, -1);
        }

        // tracks the source of whatever hits the carrot, and swaps its team accordingly
        let current_damage = DamageModule::damage(item_boma, 0);
        let prev_damage = VarModule::get_float(boma.object(), VEGETABLE_DAMAGE);
        if current_damage > prev_damage { 
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
                //println!("ID {} changing to team {}", item_id, opponent_team);
                TeamModule::set_team(item_boma, opponent_team, false);
            } else {
                //println!("Hit by Daisy");
                let team = TeamModule::hit_team_no(boma) as i32;
                TeamModule::set_team(item_boma, team, false);
                //println!("ID {} changing to team {}", item_id, team);
            }
            StatusModule::change_status_force(item_boma, *ITEM_STATUS_KIND_THROW, true); // resets the throw status so the hitbox doesn't clear
            VarModule::set_float(boma.object(), VEGETABLE_DAMAGE, current_damage);
            if !VarModule::is_flag(boma.object(), VEGETABLE_LOCKOUT) {
                VarModule::set_int(boma.object(), VEGETABLE_LOCKOUT_FRAME, 10);
                VarModule::on_flag(boma.object(), VEGETABLE_LOCKOUT);
            }
        } else if current_damage != prev_damage {
            // handle the variable in other scenarios. mostly on item despawn
            VarModule::set_float(boma.object(), VEGETABLE_DAMAGE, current_damage);
        }
    }

    // handles the carrot hurtbox lockout window
    if VarModule::get_int(boma.object(), VEGETABLE_LOCKOUT_FRAME) > 0 {
        VarModule::dec_int(boma.object(), VEGETABLE_LOCKOUT_FRAME);
    } else {
        VarModule::off_flag(boma.object(), VEGETABLE_LOCKOUT);
    }

    // hide the carrot (and other items) when daisy is shielding
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_GUARD,
        *FIGHTER_STATUS_KIND_GUARD_ON
    ]) {
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
}

// i wonder what you taste like
unsafe fn appeal_special(boma: &mut BattleObjectModuleAccessor) {
    // transitions to the special taunt if the button is released within 2 frames
    if boma.is_motion_one_of(&[Hash40::new("appeal_s_l"), Hash40::new("appeal_s_r")])
    && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L)
    && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R)
    && boma.motion_frame() as i32 == 3 {
        MotionModule::change_motion_inherit_frame(boma, Hash40::new("appeal_s_special"), -1.0, 1.0, 0.0, false, false);
    }

    if ArticleModule::is_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO) {
        let article = ArticleModule::get_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO);
        let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(article_id);

        let idle_frames = 40; // how long the flower sticks around after it's done talking
        let yap_timer = VarModule::get_int(boma.object(), YAPPING_TIMER);
        if yap_timer > 0 { VarModule::dec_int(boma.object(), YAPPING_TIMER); }
        if (970..971).contains(&yap_timer) {
            let mut quote_data: [&str;2] = ["dummy", "dummy"];
            let mut yapping_frames = 0; // approximate amount of frames each line takes to complete
            let rng = app::sv_math::rand(hash40("fighter"), 25);
           match (rng as i32) { // assign data for what the flower will say based on rng
                 0 => { quote_data = ["se_daisy_appeal_x01_onward", "daisy_flower_bubble_01"]; yapping_frames = 84; },
                 1 => { quote_data = ["se_daisy_appeal_x02_company", "daisy_flower_bubble_02"]; yapping_frames = 82; },
                 2 => { quote_data = ["se_daisy_appeal_x03_great", "daisy_flower_bubble_03"]; yapping_frames = 76; },
                 3 => { quote_data = ["se_daisy_appeal_x04_everyday", "daisy_flower_bubble_04"]; yapping_frames = 95; },
                 4 => { quote_data = ["se_daisy_appeal_x05_feelsoff", "daisy_flower_bubble_05"]; yapping_frames = 173; },
                 5 => { quote_data = ["se_daisy_appeal_x06_focus", "daisy_flower_bubble_06"]; yapping_frames = 125; },
                 6 => { quote_data = ["se_daisy_appeal_x07_howyadoin", "daisy_flower_bubble_07"]; yapping_frames = 60; },
                 7 => { quote_data = ["se_daisy_appeal_x08_something", "daisy_flower_bubble_08"]; yapping_frames = 80; },
                 8 => { quote_data = ["se_daisy_appeal_x09_keeptrying", "daisy_flower_bubble_09"]; yapping_frames = 124; },
                 9 => { quote_data = ["se_daisy_appeal_x10_almost", "daisy_flower_bubble_10"]; yapping_frames = 94; },
                10 => { quote_data = ["se_daisy_appeal_x11_goodday", "daisy_flower_bubble_11"]; yapping_frames = 122; },
                11 => { quote_data = ["se_daisy_appeal_x12_newspecies", "daisy_flower_bubble_12"]; yapping_frames = 92; },
                12 => { quote_data = ["se_daisy_appeal_x13_taste", "daisy_flower_bubble_13"]; yapping_frames = 95; },
                13 => { quote_data = ["se_daisy_appeal_x14_nexttime", "daisy_flower_bubble_14"]; yapping_frames = 66; },
                14 => { quote_data = ["se_daisy_appeal_x15_ohhey", "daisy_flower_bubble_15"]; yapping_frames = 60; },
                15 => { quote_data = ["se_daisy_appeal_x16_party", "daisy_flower_bubble_16"]; yapping_frames = 125; },
                16 => { quote_data = ["se_daisy_appeal_x17_peaceful", "daisy_flower_bubble_17"]; yapping_frames = 94; },
                17 => { quote_data = ["se_daisy_appeal_x18_rooting", "daisy_flower_bubble_18"]; yapping_frames = 84; },
                18 => { quote_data = ["se_daisy_appeal_x19_sogood", "daisy_flower_bubble_19"]; yapping_frames = 92; },
                19 => { quote_data = ["se_daisy_appeal_x20_summoned", "daisy_flower_bubble_20"]; yapping_frames = 98; },
                20 => { quote_data = ["se_daisy_appeal_x21_surprise", "daisy_flower_bubble_21"]; yapping_frames = 62; },
                21 => { quote_data = ["se_daisy_appeal_x22_wellthen", "daisy_flower_bubble_22"]; yapping_frames = 40; },
                22 => { quote_data = ["se_daisy_appeal_x23_what", "daisy_flower_bubble_23"]; yapping_frames = 88; },
                23 => { quote_data = ["se_daisy_appeal_x24_where", "daisy_flower_bubble_24"]; yapping_frames = 65; },
                 _ => { quote_data = ["se_daisy_appeal_x25_yikes", "daisy_flower_bubble_25"]; yapping_frames = 36; }
            };
            ArticleModule::change_motion(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO, Hash40::new("catch_attack"), true, 0.0);
            SoundModule::play_se(boma, Hash40::new(quote_data[0]), true, false, false, false, app::enSEType(0));
            let effect_id = EffectModule::req_on_joint(article_boma, Hash40::new(quote_data[1]), Hash40::new("top"), &Vector3f::new(0.5, 11.25, 0.0), &Vector3f::zero(), 0.6, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
            VarModule::set_int(boma.object(), FLOWER_EFFECT_ID, effect_id as i32);
            VarModule::set_int(boma.object(), YAPPING_TIMER, (idle_frames + yapping_frames));
            VarModule::set_int(boma.object(), FLOWER_EFFECT_FRAMES, 5);
            VarModule::on_flag(boma.object(), START_FLOWER_EFFECT);
        }

        // handles the animation of the speech bubble effect
        let effect = VarModule::get_int(boma.object(), FLOWER_EFFECT_ID);
        let effect_frame = VarModule::get_int(boma.object(), FLOWER_EFFECT_FRAMES) as f32;
        if effect_frame as i32 > 0 { VarModule::dec_int(boma.object(), FLOWER_EFFECT_FRAMES); }
        // speech bubble entry
        if VarModule::is_flag(boma.object(), START_FLOWER_EFFECT) {
            if effect_frame > 0.0 {
                let mul = 1.0 - (effect_frame * 0.05);
                EffectModule::set_scale(boma, effect as u32, &Vector3f::new(1.4 * mul, 1.6 * mul, 1.4 * mul));
                EffectModule::set_alpha(boma, effect as u32, 1.0 - (0.2 * effect_frame));
            } else {
                EffectModule::set_scale(boma, effect as u32, &Vector3f::new(1.4, 1.6, 1.4));
                EffectModule::set_alpha(boma, effect as u32, 1.0);
                VarModule::off_flag(boma.object(), START_FLOWER_EFFECT);
            }
        }
        // speech bubble exit
        if VarModule::is_flag(boma.object(), END_FLOWER_EFFECT) {
            if effect_frame > 0.0 {
                let mul = 0.75 + (effect_frame * 0.05);
                EffectModule::set_scale(boma, effect as u32, &Vector3f::new(1.4 * mul, 1.6 * mul, 1.4 * mul));
                EffectModule::set_alpha(boma, effect as u32, (0.1 * effect_frame));
            } else {
                EffectModule::kill(boma, effect as u32, true, true);
                VarModule::off_flag(boma.object(), END_FLOWER_EFFECT);
            }
        }

        // when the dialogue is over, return to idle and remove the effect
        if yap_timer == idle_frames {
            ArticleModule::change_motion(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO, Hash40::new("catch_wait"), true, 0.0);
            VarModule::set_int(boma.object(), FLOWER_EFFECT_FRAMES, 5);
            VarModule::on_flag(boma.object(), END_FLOWER_EFFECT);
        }

        // removes the flower once all is said and done
        if yap_timer <= 0 {
            let exit_effect = EffectModule::req_on_joint(article_boma, Hash40::new("sys_erace_smoke"), Hash40::new("top"), &Vector3f::new(0.2, 4.5, 0.0), &Vector3f::zero(), 0.6, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
            EffectModule::set_rate(boma, exit_effect as u32, 1.0);
            ArticleModule::remove_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO, ArticleOperationTarget(0));
        }
    }
}

// unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
//     if !fighter.is_in_hitlag()
//     && !StatusModule::is_changing(fighter.module_accessor)
//     && fighter.is_status_one_of(&[
//         *FIGHTER_STATUS_KIND_SPECIAL_N,
//         *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT,
//         ]) 
//     && fighter.is_situation(*SITUATION_KIND_AIR) {
//         fighter.sub_air_check_dive();
//         if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
//             if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
//                 fighter.clear_lua_stack();
//                 lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
//                 let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

//                 fighter.clear_lua_stack();
//                 lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
//                 app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
//                 fighter.clear_lua_stack();
//                 lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
//                 app::sv_kinetic_energy::enable(fighter.lua_state_agent);

//                 KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
//             }
//         }
//     }
// }

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    wall_bounce(boma);
    triple_jump_motion(fighter, boma);
    crystal_handling(boma);
    vegetable_handling(boma);
    appeal_special(boma);
    //fastfall_specials(fighter);
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