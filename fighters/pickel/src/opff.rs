// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

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
    fastfall_specials(fighter);
    material_handling(fighter, boma);
    hitstun_handling(fighter, boma, frame);
    table_recreate(fighter, boma, status_kind);
    build_ecb_shift(boma, status_kind);
    elytra_cancel(boma, status_kind);
    guardoff_shield(fighter, boma, frame);
    appeal_lw_loop(fighter, boma, frame);
    training_mode_resources(fighter, boma, status_kind, stick_x, stick_y);
    //buildwalk_crouch_disable(boma, status_kind);
    //logging_for_acmd(boma, status_kind);
}

unsafe fn material_handling(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    // wait 2 frames before letting the material table advance, preventing any jumps in entries
    if !VarModule::is_flag(boma.object(), vars::pickel::instance::SHOULD_CYCLE_MATERIAL) {
        if VarModule::get_int(boma.object(), vars::pickel::status::MINING_TIMER) == 0 {
            VarModule::on_flag(boma.object(), vars::pickel::instance::SHOULD_CYCLE_MATERIAL);
        } else {
            VarModule::dec_int(boma.object(), vars::pickel::status::MINING_TIMER);
        }
    }

    // check to see if steve (or anything) is currently hitting the table
    let mut tableIsDamage = false;
    if ArticleModule::is_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE) {
        let table_hp = ArticleModule::get_float(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE, *WEAPON_PICKEL_TABLE_INSTANCE_WORK_ID_FLOAT_HP);
        let prev_table_hp = VarModule::get_float(boma.object(), vars::pickel::instance::TABLE_HP_TRACKER);
        if table_hp < prev_table_hp {
            tableIsDamage = true;
            VarModule::set_float(boma.object(), vars::pickel::instance::TABLE_HP_TRACKER, table_hp);
        } else if table_hp != prev_table_hp { 
            VarModule::set_float(boma.object(), vars::pickel::instance::TABLE_HP_TRACKER, table_hp);
        }
    }

    // advance the index towards gold/diamond if steve lands an attack
    // additionally, if steve is about to mine a rare materal, his mining hand will faintly glow
    let index = VarModule::get_int(boma.object(), vars::pickel::instance::MATERIAL_INDEX) as i32;
    let hand_effect = VarModule::get_int(boma.object(), vars::pickel::instance::MATERIAL_EFFECT_HANDLER);
    if ![29, 49, 59, 89, 99].contains(&index) { // gold and diamond entries
        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_HIT) 
        && !tableIsDamage { // index will not advance if steve is hitting his crafting table
            VarModule::inc_int(boma.object(), vars::pickel::instance::MATERIAL_INDEX);
            // let next = VarModule::get_int(boma.object(), vars::pickel::instance::MATERIAL_INDEX) + 1;
            // println!("next entry: {}", next);
        }
        let hit_articles: [i32;4] = [ // articles that are allowed to advance the index on hit
            *FIGHTER_PICKEL_GENERATE_ARTICLE_FIRE,
            *FIGHTER_PICKEL_GENERATE_ARTICLE_MELT,
            *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE,
            *FIGHTER_PICKEL_GENERATE_ARTICLE_TROLLEY ];
        for weapon in hit_articles {
            if ArticleModule::is_exist(boma, weapon) 
            && VarModule::is_flag(boma.object(), vars::pickel::instance::SHOULD_CYCLE_MATERIAL) {
                let article = ArticleModule::get_article(boma, weapon);
                let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
                let article_boma = sv_battle_object::module_accessor(article_id);
                if AttackModule::is_infliction(article_boma, *COLLISION_KIND_MASK_HIT)
                && !tableIsDamage {
                    VarModule::inc_int(boma.object(), vars::pickel::instance::MATERIAL_INDEX);
                    VarModule::off_flag(boma.object(), vars::pickel::instance::SHOULD_CYCLE_MATERIAL);
                    VarModule::set_int(boma.object(), vars::pickel::status::MINING_TIMER, 5);
                    // let next = VarModule::get_int(boma.object(), vars::pickel::instance::MATERIAL_INDEX) + 1;
                    // println!("next entry: {}", next);
                }
            }
        }
        if hand_effect > 0 {
            VarModule::set_int(boma.object(), vars::pickel::instance::MATERIAL_EFFECT_HANDLER, 0);
            if EffectModule::is_exist_effect(boma, hand_effect as u32) {
                EffectModule::kill(boma, hand_effect as u32, false, false);
            }
        }
    } else if !fighter.is_status_one_of(&[*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WAIT, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WALK_BACK]) 
    && (hand_effect <= 0 || !EffectModule::is_exist_effect(boma, hand_effect as u32)) {
        let handle = EffectModule::req_follow(boma, Hash40::new("sys_status_all_up"), Hash40::new("haver"), &Vector3f::zero(), &Vector3f::zero(), 0.28, false, 0, 0, 0, 0, 0, false, false) as u32;
        EffectModule::set_rate(boma, handle, 0.5);
        if [49, 99].contains(&index) { // color effect blue if it will be a diamond
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.45, 0.94);
        }
        VarModule::set_int(boma.object(), vars::pickel::instance::MATERIAL_EFFECT_HANDLER, handle as i32);
    }
}

// hitstun-related effects
unsafe fn hitstun_handling(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    let timer = VarModule::get_int(boma.object(), vars::pickel::instance::HITSTUN_TIMER);
    if timer > 0 { VarModule::dec_int(boma.object(), vars::pickel::instance::HITSTUN_TIMER); }
    let current_damage = DamageModule::damage(boma, 0);
    let prev_damage = VarModule::get_float(boma.object(), vars::pickel::instance::DAMAGE_TRACKER);
    if current_damage > prev_damage { // steve will glow red when taking damage
        if current_damage > (prev_damage + 5.0) // 5.0% threshold for sfx to play
        && timer == 0 { // sound will play if it has been 55 frames since last damaged
            PLAY_SE(fighter, Hash40::new("se_pickel_landing_high_place"));
        }
        VarModule::set_float(boma.object(), vars::pickel::instance::DAMAGE_TRACKER, current_damage);
        let vec1 = Vector4f{ x: 0.85, y: 0.85, z: 0.85, w: 0.2};
        let vec2 = Vector4f{ x: 0.9907, y: 0.02, z: 0.0251, w: 0.8};
        ColorBlendModule::set_main_color(boma, &vec1, &vec2, 0.21, 2.2, 5, true);
        VarModule::set_int(boma.object(), vars::pickel::instance::HITSTUN_TIMER, 55);
    } else if current_damage < prev_damage { // in the case steve is healed, or respawns
        VarModule::set_float(boma.object(), vars::pickel::instance::DAMAGE_TRACKER, current_damage);
    }
    if timer == 30 { // red tint clears after 25 frames
        ColorBlendModule::cancel_main_color(boma, 0);
    }

    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_DAMAGE_FLY,
                                  *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
                                  *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR]) {
        // posing while being launched
        fighter.set_joint_rotate("neck", Vector3f {x: 35.0, y: 25.0, z: -50.0});
        fighter.set_joint_rotate("hip", Vector3f {x: -105.0, y: 0.0, z: 0.0});
        fighter.set_joint_rotate("shoulderl", Vector3f {x: 0.0, y: 35.0, z: 0.0});
        fighter.set_joint_rotate("shoulderr", Vector3f {x: 0.0, y: -35.0, z: 0.0});
        VarModule::on_flag(boma.object(), vars::pickel::instance::SHOULD_RESET_ROT);
    } else if fighter.is_status(*FIGHTER_STATUS_KIND_DAMAGE_FALL) {
        // rotation during tumble
        let rot_y = PostureModule::rot_y(boma, 0); 
        PostureModule::set_rot(boma, &Vector3f {x: 0.0, y: (rot_y + 6.0), z:0.0}, 0);
        fighter.set_joint_rotate("neck", Vector3f {x: 0.0, y: 0.0, z: -40.0});
        fighter.set_joint_rotate("shoulderl", Vector3f {x: 0.0, y: 15.0, z: 0.0});
        fighter.set_joint_rotate("shoulderr", Vector3f {x: 0.0, y: -15.0, z: 0.0});
        VarModule::on_flag(boma.object(), vars::pickel::instance::SHOULD_RESET_ROT);
    } else if PostureModule::rot_y(boma, 0) != 0.0
    && VarModule::is_flag(boma.object(), vars::pickel::instance::SHOULD_RESET_ROT) {
        PostureModule::set_rot(boma, &Vector3f {x: 0.0, y: 0.0, z:0.0}, 0);
        VarModule::off_flag(boma.object(), vars::pickel::instance::SHOULD_RESET_ROT);
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

// steve table respawn
unsafe fn table_recreate(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    let flash_timer = VarModule::get_int(boma.object(), vars::common::instance::GIMMICK_TIMER);
    if flash_timer > 0 { VarModule::dec_int(boma.object(), vars::common::instance::GIMMICK_TIMER); }
    
    // initiate effects for the cooldown indication
    if flash_timer == 25 {
        gimmick_flash(boma);
        LAST_EFFECT_SET_SCALE_W(fighter, 0.65, 0.65, 0.65);
        LAST_EFFECT_SET_RATE(fighter, 0.6);
        PLAY_SE(fighter, Hash40::new("se_pickel_special_n_craft_end"));
        ArticleModule::generate_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE, false, 0);

        let table = ArticleModule::get_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE);
        let table_id = smash::app::lua_bind::Article::get_battle_object_id(table) as u32;
        let table_boma = sv_battle_object::module_accessor(table_id);
        let pickel_pos_x = PostureModule::pos_x(boma);
        let pickel_pos_y = PostureModule::pos_y(boma);
        PostureModule::set_pos_2d(table_boma, &Vector2f {x: pickel_pos_x, y: (pickel_pos_y + 17.0)});
        PostureModule::set_rot(table_boma, &Vector3f {x: 0.0, y: 45.0, z:0.0}, 0);
        PostureModule::set_scale(table_boma, 0.6, false);
        KineticModule::suspend_energy_all(table_boma);
        VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 24);
    }

    if ArticleModule::is_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE) {
        let table = ArticleModule::get_article(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE);
        let table_id = smash::app::lua_bind::Article::get_battle_object_id(table) as u32;
        let table_boma = sv_battle_object::module_accessor(table_id);
        // make sure steve cannot spawn the table if it already exists
        if VarModule::is_flag(boma.object(), vars::pickel::instance::CAN_RESPAWN_TABLE){
            VarModule::off_flag(boma.object(), vars::pickel::instance::CAN_RESPAWN_TABLE);
        }
        // sets timer for the respawn cooldown when table breaks
        if StatusModule::status_kind(table_boma) == *WEAPON_PICKEL_TABLE_STATUS_KIND_BREAK
        && flash_timer == 0 {
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 240);
        }
        // handles custom table effect
        if (2..24).contains(&flash_timer) {
            let pickel_pos_x = PostureModule::pos_x(boma);
            let pickel_pos_y = PostureModule::pos_y(boma);
            let y_offset = 17.0 + ((30.0 - flash_timer as f32) * 0.2); // base offset + distance it will rise each frame
            let table_rot_y = PostureModule::rot_y(table_boma, 0); 
            PostureModule::set_pos_2d(table_boma, &Vector2f {x: pickel_pos_x, y: (pickel_pos_y + y_offset)});
            PostureModule::set_rot(table_boma, &Vector3f {x: 0.0, y: (table_rot_y + 1.0), z:0.0}, 0);
        } else if flash_timer == 1 { // remove effect
            KineticModule::resume_energy_all(table_boma);
            ArticleModule::remove_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_TABLE, app::ArticleOperationTarget(0));
            VarModule::set_int(boma.object(), vars::common::instance::GIMMICK_TIMER, 0);
        }
    } else { // set the flag for table respawning when cooldown is over
        if !VarModule::is_flag(boma.object(), vars::pickel::instance::CAN_RESPAWN_TABLE)
        && flash_timer == 0 {
            VarModule::on_flag(boma.object(), vars::pickel::instance::CAN_RESPAWN_TABLE);
        }
    }
    // input for respawning table
    if VarModule::is_flag(boma.object(), vars::pickel::instance::CAN_RESPAWN_TABLE)
    && status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WAIT // if steve is in stationary mining status
    && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD) { 
        StatusModule::change_status_force(boma, *FIGHTER_PICKEL_STATUS_KIND_RECREATE_TABLE, true); 
        VarModule::off_flag(boma.object(), vars::pickel::instance::CAN_RESPAWN_TABLE);
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

// Increment glide timer during elytra
unsafe fn elytra_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if (status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_GLIDING) {
        VarModule::add_float(boma.object(), vars::pickel::status::GLIDE_TIMER, 1.0);
        let glide_timer = VarModule::get_float(boma.object(), vars::pickel::status::GLIDE_TIMER);
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)
        && (25.0..45.0).contains(&glide_timer) {
            //VarModule::on_flag(boma.object(), vars::common::instance::UP_SPECIAL_CANCEL);
            if boma.is_situation(*SITUATION_KIND_AIR) {
                KineticModule::mul_speed(boma, &Vector3f {x: 0.4, y: 0.4, z:0.4}, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_HI_FALL_SPECIAL,false);
        }
    }
}

// remove the shield article from an attempted parry 
unsafe fn guardoff_shield(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_GUARD_OFF)
    && frame >= 3.5 
    && ArticleModule::is_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF) {
        ArticleModule::remove_exist(boma, *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

// looping/accelerating down taunt
unsafe fn appeal_lw_loop(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    if fighter.is_motion_one_of(&[Hash40::new("appeal_lw_l"), Hash40::new("appeal_lw_r")])
    && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_APPEAL_LW)
    && (39.0..62.0).contains(&frame) {
        MotionModule::set_frame(boma, 28.0, true);
        let rate =  MotionModule::rate(boma);
        if rate == 1.0 {
            MotionModule::set_rate(boma, 0.001); // slow down initially
        }
        if rate < 4.0 { // max rate of 4x speed
            MotionModule::set_rate(boma, (rate * 1.01)); // gradually speed up
        }
    }
}

// input for giving steve materials/upgrades in training mode
unsafe fn training_mode_resources(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_x: f32, stick_y: f32) {
    if is_training_mode() {
        if status_kind == *FIGHTER_STATUS_KIND_APPEAL 
        && ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_GUARD) {
            let dirt_num = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_GRADE_1);
            let wood_num = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_WOOD);
            let stone_num = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_STONE);
            let iron_num =WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_IRON);
            let gold_num = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_GOLD);
            let redstone_num = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_RED_STONE);
            let diamond_num = WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_DIAMOND);
            let dirt = *FIGHTER_PICKEL_MATERIAL_KIND_GRADE_1;
            let wood = *FIGHTER_PICKEL_MATERIAL_KIND_WOOD;
            let stone = *FIGHTER_PICKEL_MATERIAL_KIND_STONE;
            let iron = *FIGHTER_PICKEL_MATERIAL_KIND_IRON;
            let gold = *FIGHTER_PICKEL_MATERIAL_KIND_GOLD;
            let redstone = *FIGHTER_PICKEL_MATERIAL_KIND_RED_STONE;
            let diamond = *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND;
            if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) { // fill materials to defined amount
                FighterSpecializer_Pickel::add_material_num(boma, dirt, 40 - dirt_num);
                FighterSpecializer_Pickel::add_material_num(boma, wood, 20 - wood_num);
                FighterSpecializer_Pickel::add_material_num(boma, stone, 20 - stone_num);
                FighterSpecializer_Pickel::add_material_num(boma, iron, 20 - iron_num);
                FighterSpecializer_Pickel::add_material_num(boma, gold, 12 - gold_num);
                FighterSpecializer_Pickel::add_material_num(boma, redstone, 18 - redstone_num);
                FighterSpecializer_Pickel::add_material_num(boma, diamond, 5 - diamond_num);
            } else { // remove all materials if special is held
                FighterSpecializer_Pickel::sub_material_num(boma, dirt, dirt_num);
                FighterSpecializer_Pickel::sub_material_num(boma, wood, wood_num);
                FighterSpecializer_Pickel::sub_material_num(boma, stone, stone_num);
                FighterSpecializer_Pickel::sub_material_num(boma, iron, iron_num);
                FighterSpecializer_Pickel::sub_material_num(boma, gold, gold_num);
                FighterSpecializer_Pickel::sub_material_num(boma, redstone, redstone_num);
                FighterSpecializer_Pickel::sub_material_num(boma, diamond, diamond_num);
            }
            let mut material = wood;
            let mut effect = "pickel_craft_icon_wood";
            // checks the position of the control stick to determine what upgrade to give steve, if any
            if (-0.25..0.25).contains(&stick_y) { // stick is moved mostly horizontally
                if stick_x >= 0.5 { // holding right
                    material = stone;
                    effect = "pickel_craft_icon_stone";
                } else if stick_x <= -0.5 { // holding left
                    material = gold;
                    effect = "pickel_craft_icon_gold";
                }
            }
            if (-0.25..0.25).contains(&stick_x) { // stick is moved mostly vertically
                if stick_y >= 0.5 { // holding up
                    material = diamond;
                    effect = "pickel_craft_icon_diamond";
                } else if stick_y <= -0.5 { // holding down
                    material = iron; 
                    effect = "pickel_craft_icon_iron";
                }
            }
            if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                let lr = PostureModule::lr(boma) as i32;
                EFFECT_FOLLOW(fighter, Hash40::new(effect), Hash40::new("top"), 0, 20, -5 * lr, 0, 0, 0, 1, true);
                PLAY_SE(fighter, Hash40::new("se_pickel_special_n05")); // ding
            } else { // if special is held, his tools will instead break
                material = *FIGHTER_PICKEL_MATERIAL_KIND_NONE;
                PLAY_SE(fighter, Hash40::new("se_pickel_special_n_item_break")); // break sfx
            }
            let weapons: [i32;4] = [*FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, 
                                    *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, 
                                    *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_AXE, 
                                    *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SHOVEL];
            // assign the appropriate material
            let pickel = fighter.global_table[0x4].get_ptr() as *mut Fighter;
            for kind in weapons {
                FighterSpecializer_Pickel::set_craft_weapon_param(pickel, 
                    FighterPickelCraftWeaponKind(kind), 
                    FighterPickelMaterialKind(material), 
                    100.0) // weapon durability
            }
        }
    }
}

// this makes it easier to place blocks diagonally down during build-walk
unsafe fn buildwalk_crouch_disable(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if [*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK, *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_WALK_BACK].contains(&status_kind) {
        WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
    }
}

// Logging for deciphering ACMD scripts
unsafe fn logging_for_acmd(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3 || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        // println!("craft_weapon_kind: {}", WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_KIND));
        // println!("request_have_craft_weapon_kind: {}", WorkModule::get_int(boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND));
        // println!("craft_sword: {}", *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD);
        // println!("craft_axe: {}", *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_AXE);
        // println!("craft_pick: {}", *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK);
        // println!("craft_shovel: {}", *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SHOVEL);
    }
}

#[utils::macros::opff(FIGHTER_KIND_PICKEL )]
pub fn pickel_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
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

///               ///
/// WEAPON FRAMES ///
///               ///

// minecart
#[smashline::weapon_frame(agent = WEAPON_KIND_PICKEL_TROLLEY, main)]
pub fn pickel_trolley_frame(weapon: &mut smash::lua2cpp::L2CFighterBase) {
    unsafe {
        let boma = weapon.boma();
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        // Ensure the boma's owner is Steve
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_PICKEL {
            let pickel = utils::util::get_battle_object_from_id(owner_id);
            let pickel_boma = &mut *(*pickel).module_accessor;
            // Burn double jump when jumping out of Minecart
            if boma.is_situation(*SITUATION_KIND_AIR)
            && pickel_boma.is_status(*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_JUMP) {
                if MotionModule::frame(pickel_boma) <= 1.0
                && pickel_boma.get_num_used_jumps() < pickel_boma.get_jump_count_max() {
                    WorkModule::inc_int(pickel_boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                }
            }
            // Restore double jump when landing with Minecart
            if boma.is_situation(*SITUATION_KIND_GROUND)
            && pickel_boma.is_status(*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_DRIVE) {
                if pickel_boma.get_num_used_jumps() >= pickel_boma.get_jump_count_max() {
                    WorkModule::dec_int(pickel_boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
                }
            }
        }
    }
}

// anvil
#[smashline::weapon_frame(agent = WEAPON_KIND_PICKEL_FORGE, main)]
pub fn pickel_forge_frame(weapon: &mut smash::lua2cpp::L2CFighterBase){
    unsafe{
        let boma = weapon.boma();
        let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        if sv_battle_object::kind(owner_id) == *FIGHTER_KIND_PICKEL {
            let pickel = utils::util::get_battle_object_from_id(owner_id);
            let pickel_boma = &mut *(*pickel).module_accessor;
            if pickel_boma.is_motion_one_of(&[Hash40::new("attack_air_lw"),
            Hash40::new("attack_air_lw_2"),
            Hash40::new("attack_air_lw_fall"),]) && !boma.is_situation(*SITUATION_KIND_GROUND) 
            //&& !pickel_boma.is_status(*FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_START)
            && WorkModule::is_flag(boma, *WEAPON_PICKEL_FORGE_INSTANCE_WORK_ID_FLAG_UPDATE_ATTACK){
                MotionAnimcmdModule::call_script_single(boma, *FIGHTER_ANIMCMD_GAME, Hash40::new_raw(0x1397d77a71), -1);
            }
        }
    }
}