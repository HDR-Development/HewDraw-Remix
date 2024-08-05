// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

use vars::pickel::{
    instance::*,
    status::*
};

unsafe fn material_handling(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    // wait 2 frames before letting the material table advance, preventing any jumps in entries
    if !VarModule::is_flag(boma.object(), SHOULD_CYCLE_MATERIAL) {
        if VarModule::get_int(boma.object(), MINING_TIMER) == 0 {
            VarModule::on_flag(boma.object(), SHOULD_CYCLE_MATERIAL);
        } else {
            VarModule::dec_int(boma.object(), MINING_TIMER);
        }
    }

    // check to see if steve (or anything) is currently hitting the table
    let mut table_is_hit = false;
    if ArticleModule::is_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE) {
        let table_hp = ArticleModule::get_float(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE, *WEAPON_PICKEL_TABLE_INSTANCE_WORK_ID_FLOAT_HP);
        let prev_table_hp = VarModule::get_float(boma.object(), TABLE_HP_TRACKER);
        if table_hp < prev_table_hp {
            table_is_hit = true;
            VarModule::set_float(boma.object(), TABLE_HP_TRACKER, table_hp);
        } else if table_hp != prev_table_hp { 
            VarModule::set_float(boma.object(), TABLE_HP_TRACKER, table_hp);
        }
    }

    // advance the index towards gold/diamond if steve lands an attack
    // additionally, if steve is about to mine a rare materal, his mining hand will faintly glow
    let index = VarModule::get_int(boma.object(), MATERIAL_INDEX) as i32;
    let hand_effect = VarModule::get_int(boma.object(), MATERIAL_EFFECT_HANDLER);
    if ![29, 49, 59, 89, 99].contains(&index) { // gold and diamond entries
        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) 
        && !table_is_hit { // index will not advance if steve is hitting his crafting table
            VarModule::inc_int(boma.object(), MATERIAL_INDEX);
            // let next = VarModule::get_int(boma.object(), MATERIAL_INDEX) + 1;
            // println!("next entry: {}", next);
        }
        let hit_articles: [i32;5] = [ // articles that are allowed to advance the index on hit
            *FIGHTER_PICKEL_GENERATE_ARTICLE_FIRE,
            *FIGHTER_PICKEL_GENERATE_ARTICLE_MELT,
            *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE,
            *FIGHTER_PICKEL_GENERATE_ARTICLE_TROLLEY,
            *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD ];
        for weapon in hit_articles {
            if ArticleModule::is_exist(boma, weapon) 
            && VarModule::is_flag(boma.object(), SHOULD_CYCLE_MATERIAL) {
                let article = ArticleModule::get_article(boma, weapon);
                let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
                let article_boma = sv_battle_object::module_accessor(article_id);
                if AttackModule::is_infliction(article_boma, *COLLISION_KIND_MASK_HIT)
                && !table_is_hit {
                    VarModule::inc_int(boma.object(), MATERIAL_INDEX);
                    VarModule::off_flag(boma.object(), SHOULD_CYCLE_MATERIAL);
                    VarModule::set_int(boma.object(), MINING_TIMER, 5);
                    // let next = VarModule::get_int(boma.object(), MATERIAL_INDEX) + 1;
                    // println!("next entry: {}", next);
                }
            }
        }
        if hand_effect > 0 {
            VarModule::set_int(boma.object(), MATERIAL_EFFECT_HANDLER, 0);
            if EffectModule::is_exist_effect(boma, hand_effect as u32) {
                EffectModule::kill(boma, hand_effect as u32, false, false);
            }
        }
    } else if (hand_effect <= 0 || !EffectModule::is_exist_effect(boma, hand_effect as u32)) {
        let handle = EffectModule::req_follow(boma, Hash40::new("sys_status_all_up"), Hash40::new("haver"), &Vector3f::zero(), &Vector3f::zero(), 0.28, false, 0, 0, 0, 0, 0, false, false) as u32;
        EffectModule::set_rate(boma, handle, 0.5);
        if [49, 99].contains(&index) { // color effect blue if it will be a diamond
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.45, 0.94);
        }
        VarModule::set_int(boma.object(), MATERIAL_EFFECT_HANDLER, handle as i32);
    }
}

// hitstun-related effects
unsafe fn hitstun_handling(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    let timer = VarModule::get_int(boma.object(), HITSTUN_TIMER);
    if timer > 0 { VarModule::dec_int(boma.object(), HITSTUN_TIMER); }
    let current_damage = DamageModule::damage(boma, 0);
    let prev_damage = VarModule::get_float(boma.object(), DAMAGE_TRACKER);
    if current_damage > prev_damage { // steve will glow red when taking damage
        if current_damage > (prev_damage + 10.0) // 10.0% threshold for sfx to play
        && timer == 0 { // sound will play if it has been 55 frames since last damaged
            PLAY_SE(fighter, Hash40::new("se_pickel_landing_high_place"));
        }
        VarModule::set_float(boma.object(), DAMAGE_TRACKER, current_damage);
        let vec1 = Vector4f{ x: 0.85, y: 0.85, z: 0.85, w: 0.2};
        let vec2 = Vector4f{ x: 0.9907, y: 0.02, z: 0.0251, w: 0.8};
        ColorBlendModule::set_main_color(boma, &vec1, &vec2, 0.21, 2.2, 5, true);
        VarModule::set_int(boma.object(), HITSTUN_TIMER, 55);
    } else if current_damage < prev_damage { // in the case steve is healed, or respawns
        VarModule::set_float(boma.object(), DAMAGE_TRACKER, current_damage);
    }
    if timer == 30 { // red tint clears after 25 frames
        ColorBlendModule::cancel_main_color(boma, 0);
    }

    if fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_DAMAGE_FLY,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR
    ]) {
        // posing while being launched
        fighter.set_joint_rotate("neck", Vector3f {x: 35.0, y: 25.0, z: -50.0});
        fighter.set_joint_rotate("hip", Vector3f {x: -105.0, y: 0.0, z: 0.0});
        fighter.set_joint_rotate("shoulderl", Vector3f {x: 0.0, y: 35.0, z: 0.0});
        fighter.set_joint_rotate("shoulderr", Vector3f {x: 0.0, y: -35.0, z: 0.0});
        VarModule::on_flag(boma.object(), SHOULD_RESET_ROT);
    } else if fighter.is_status(*FIGHTER_STATUS_KIND_DAMAGE_FALL) {
        // rotation during tumble
        let rot_y = PostureModule::rot_y(boma, 0); 
        PostureModule::set_rot(boma, &Vector3f {x: 0.0, y: (rot_y + 6.0), z:0.0}, 0);
        fighter.set_joint_rotate("neck", Vector3f {x: 0.0, y: 0.0, z: -40.0});
        fighter.set_joint_rotate("shoulderl", Vector3f {x: 0.0, y: 15.0, z: 0.0});
        fighter.set_joint_rotate("shoulderr", Vector3f {x: 0.0, y: -15.0, z: 0.0});
        VarModule::on_flag(boma.object(), SHOULD_RESET_ROT);
    } else if PostureModule::rot_y(boma, 0) != 0.0
    && VarModule::is_flag(boma.object(), SHOULD_RESET_ROT) {
        PostureModule::set_rot(boma, &Vector3f {x: 0.0, y: 0.0, z:0.0}, 0);
        VarModule::off_flag(boma.object(), SHOULD_RESET_ROT);
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

const TABLE_RESPAWN_TIMER: i32 = vars::common::instance::GIMMICK_TIMER;

// steve table respawn
unsafe fn table_recreate(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let cooldown_frame = VarModule::get_int(boma.object(), TABLE_RESPAWN_TIMER);
    if cooldown_frame > 0 { VarModule::dec_int(boma.object(), TABLE_RESPAWN_TIMER); }
    
    // initiate effects for the cooldown indication
    if cooldown_frame == 25 {
        gimmick_flash(boma);
        LAST_EFFECT_SET_SCALE_W(fighter, 0.65, 0.65, 0.65);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
        PLAY_SE(fighter, Hash40::new("se_pickel_special_n_craft_end"));
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE, false, 0);

        let table = ArticleModule::get_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE);
        let table_id = smash::app::lua_bind::Article::get_battle_object_id(table) as u32;
        let table_boma = sv_battle_object::module_accessor(table_id);
        PostureModule::set_pos_2d(table_boma, &Vector2f {
            x: PostureModule::pos_x(boma),
            y: PostureModule::pos_y(boma) + 17.0
        });
        PostureModule::set_rot(table_boma, &Vector3f {x: 0.0, y: 45.0, z:0.0}, 0);
        PostureModule::set_scale(table_boma, 0.6, false);
        KineticModule::suspend_energy_all(table_boma);
        VarModule::set_int(boma.object(), TABLE_RESPAWN_TIMER, 24);
    }

    if ArticleModule::is_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE) {
        let table = ArticleModule::get_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE);
        let table_id = smash::app::lua_bind::Article::get_battle_object_id(table) as u32;
        let table_boma = sv_battle_object::module_accessor(table_id);
        // make sure steve cannot spawn the table if it already exists
        if VarModule::is_flag(boma.object(), CAN_RESPAWN_TABLE){
            VarModule::off_flag(boma.object(), CAN_RESPAWN_TABLE);
        }
        // sets timer for the respawn cooldown when table breaks
        if StatusModule::status_kind(table_boma) == *WEAPON_PICKEL_TABLE_STATUS_KIND_BREAK
        && cooldown_frame == 0 {
            VarModule::set_int(boma.object(), TABLE_RESPAWN_TIMER, 240);
        }
        // handles custom table effect
        if (2..24).contains(&cooldown_frame) {
            let y_offset = 17.0 + ((30.0 - cooldown_frame as f32) * 0.2); // base offset + distance it will rise each frame
            let table_rot_y = PostureModule::rot_y(table_boma, 0); 
            PostureModule::set_pos_2d(table_boma, &Vector2f {
                x: PostureModule::pos_x(boma),
                y: PostureModule::pos_y(boma) + y_offset
            });
            PostureModule::set_rot(table_boma, &Vector3f {x: 0.0, y: (table_rot_y + 1.0), z:0.0}, 0);
        } else if cooldown_frame == 1 { // remove effect
            KineticModule::resume_energy_all(table_boma);
            ArticleModule::remove_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE, app::ArticleOperationTarget(0));
            VarModule::set_int(boma.object(), TABLE_RESPAWN_TIMER, 0);
        }
    } else { // set the flag for table respawning when cooldown is over
        if !VarModule::is_flag(boma.object(), CAN_RESPAWN_TABLE)
        && cooldown_frame == 0 {
            VarModule::on_flag(boma.object(), CAN_RESPAWN_TABLE);
        }
    }
    // input for respawning table
    if VarModule::is_flag(boma.object(), CAN_RESPAWN_TABLE)
    && status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WAIT // if steve is in stationary mining status
    && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) { 
        StatusModule::change_status_force(boma, *FIGHTER_PICKEL_STATUS_KIND_RECREATE_TABLE, true); 
        VarModule::off_flag(boma.object(), CAN_RESPAWN_TABLE);
    }
}

unsafe fn build_ecb_shift(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if [*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_FALL,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_FALL_AERIAL,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP_AERIAL,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WAIT,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK_BACK].contains(&status_kind)
    && !VarModule::is_flag(boma.object(), vars::common::status::DISABLE_ECB_SHIFT)
    {
        VarModule::on_flag(boma.object(), vars::common::status::DISABLE_ECB_SHIFT);
    }
}

// allow steve to cancel into special fall by pressing shield
unsafe fn elytra_cancel(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_GLIDING) {
        VarModule::add_float(boma.object(), GLIDE_TIMER, 1.0);
        let glide_timer = VarModule::get_float(boma.object(), GLIDE_TIMER);
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)
        && (25.0..45.0).contains(&glide_timer) {
            if boma.is_situation(*SITUATION_KIND_AIR) {
                KineticModule::mul_speed(boma, &Vector3f {x: 0.4, y: 0.4, z:0.4}, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_FALL_SPECIAL, false);
        }
    }
}

// gets rid of the fishing rod when exiting the air catch but NOT if he is landing
// can be reimplemented if theres a way to blacklist air_lasso_landing from the end/exit status
unsafe fn rod_removal(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_prev_status(*FIGHTER_STATUS_KIND_AIR_LASSO)
    && !boma.is_status(*FIGHTER_STATUS_KIND_AIR_LASSO_LANDING)
    && ArticleModule::is_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD) {
        ArticleModule::remove_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_FISHINGROD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe fn shovel_position(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START, 
        *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD
    ])
    || (boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_HI4) && boma.motion_frame() < 42.0) {
        ModelModule::set_joint_translate(boma, Hash40::new("weaponr"), &Vector3f::new(0.0, 0.0, 2.0), false, false);
        ModelModule::set_joint_scale(boma, Hash40::new("weaponr"), &Vector3f::new(1.2, 1.2, 1.2));
        boma.set_int(*FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SHOVEL, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_JUMP,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_FAILED,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_FALL,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_FALL_SPECIAL
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
    material_handling(fighter, boma);
    hitstun_handling(fighter, boma, frame);
    table_recreate(fighter, boma, status_kind);
    build_ecb_shift(boma, status_kind);
    elytra_cancel(boma);
    rod_removal(boma);
    shovel_position(boma);
    fastfall_specials(fighter);
}

pub extern "C" fn pickel_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		pickel_frame(fighter)
    }
}

pub unsafe fn pickel_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, pickel_frame_wrapper);
}
