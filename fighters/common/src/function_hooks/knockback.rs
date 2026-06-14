use super::*;
use super::knockback_util::*;
use utils::ext::*;
use utils::game_modes::*;

pub fn install() {
    skyline::install_hooks!(
        process_knockback,
        calculate_knockback,
        set_thrown_lr,
        set_damage_lr,
        get_attack_lr_check
    );
}

extern "C" {
    #[link_name = "_ZN3app10sv_animcmd25EFFECT_GLOBAL_BACK_GROUNDEP9lua_State"]
    fn EFFECT_GLOBAL_BACK_GROUND(lua_state: u64);
}

/// Knockback log
/// 0x8a -> the opponent was grounded (bool)
/// 0x90 -> backslash (bool)
/// 0x60 -> stop delay (f32) 
/// 0x50 -> collision attr (Hash40)
/// 0x40 -> launch angle in rad (f32)
/// 0x4 -> level (?)
/// 0x4c -> hitstop frame

static mut IS_CALCULATING: Option<(u32, u32)> = None;

#[skyline::hook(offset = 0x402f00, inline)]
unsafe fn calculate_knockback(ctx: &skyline::hooks::InlineCtx) {
    let damage_module = ctx.registers[19].x();
    let our_boma = *((damage_module + 0x8) as *mut *mut smash::app::BattleObjectModuleAccessor);
    let ptr = ctx.registers[20].x() as *mut u8;
    let id = *(ptr.add(0x24) as *const u32);
    IS_CALCULATING = Some(((*our_boma).battle_object_id, id));

    if let Some((defender, attacker)) = IS_CALCULATING {
        // stores the attacker's team color here, since this runs before the spark effects are called
        // process_knockback occurs after the sparks, so we can't put it there
        set_attacker_team_color(attacker);

        // Track last attacker entry ID on the defender for War Mode stock steal
        if check_custom_mode(CustomMode::WarMode) {
            set_last_attacker_entry_id(&mut *our_boma, attacker);
        }
    }
}

#[skyline::hook(offset = 0x403950, inline)]
unsafe fn process_knockback(ctx: &skyline::hooks::InlineCtx) {
    if let Some((defender, attacker)) = IS_CALCULATING {
        let boma = ctx.registers[20].x() as *mut smash::app::BattleObjectModuleAccessor;
        if (*boma).battle_object_id == defender {
            process_item_on_collision(defender, attacker, ctx.registers[19].x() as *const f32);
            calculate_finishing_hit(defender, attacker, ctx.registers[19].x() as *const f32);
        }
    }
}

pub unsafe extern "C" fn set_attacker_team_color(attacker:u32) {
    let attacker_boma = &mut *(*util::get_battle_object_from_id(attacker)).module_accessor;
    if attacker_boma.is_fighter() {
        LAST_ATTACK_TEAM_COLOR = FighterUtil::get_team_color(attacker_boma) as i32;
        // println!("team_color set to {} for team {}", LAST_ATTACK_TEAM_COLOR, FighterUtil::get_team_color(attacker_boma) as i32);
    }
    else if attacker_boma.is_weapon() {
        let owner_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let owner = util::get_battle_object_from_id(owner_id);
        let mut owner_boma = &mut *(*owner).module_accessor;
        // weapons can sometimes be owned by another weapon, so we make an additional check
        if owner_boma.is_weapon() {
            let owner_id = WorkModule::get_int(owner_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let owner = util::get_battle_object_from_id(owner_id);
            owner_boma = &mut *(*owner).module_accessor;
        }
        // ensure that we 100% have a fighter now, since if we dont the fn below will cause a crash
        if !owner_boma.is_fighter(){ return };
        LAST_ATTACK_TEAM_COLOR = FighterUtil::get_team_color(owner_boma) as i32;
        // println!("team_color set to {} for team {}", LAST_ATTACK_TEAM_COLOR, FighterUtil::get_team_color(owner_boma) as i32);
    }

    // optional replacement logic for cycling teams, good for testing colors
    // LAST_ATTACK_TEAM_COLOR += 1;
    // if LAST_ATTACK_TEAM_COLOR == 9 { LAST_ATTACK_TEAM_COLOR = 0 };
}

/// When a defending fighter is hit by an attack, get the attacker's entry ID and store it in the defender's var module
/// This code is currently only used for tracking the kill reward for War Mode
unsafe fn set_last_attacker_entry_id(defender_boma: &mut BattleObjectModuleAccessor, attacker: u32) {
    if !defender_boma.is_fighter() || !VarModule::has_var_module(defender_boma.object()) {
        return;
    }

    let attacker_obj = util::get_battle_object_from_id(attacker);
    if attacker_obj.is_null() { return; }
    let mut attacker_boma = &mut *(*attacker_obj).module_accessor;

    // Check weapons
    if attacker_boma.is_weapon() {
        let owner_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let owner = util::get_battle_object_from_id(owner_id);
        if owner.is_null() { return; }
        attacker_boma = &mut *(*owner).module_accessor;
        if attacker_boma.is_weapon() {
            let owner_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
            let owner = util::get_battle_object_from_id(owner_id);
            if owner.is_null() { return; }
            attacker_boma = &mut *(*owner).module_accessor;
        }
    }

    // Check items
    if attacker_boma.is_item() {
        let owner_id = if LinkModule::is_link(attacker_boma, *ITEM_LINK_NO_HAVE) {
            LinkModule::get_parent_id(attacker_boma, *ITEM_LINK_NO_HAVE, true) as u32
        } else if LinkModule::is_link(attacker_boma, *ITEM_LINK_NO_CREATEOWNER) {
            LinkModule::get_parent_id(attacker_boma, *ITEM_LINK_NO_CREATEOWNER, true) as u32
        } else if LinkModule::is_link(attacker_boma, *ITEM_LINK_NO_TEAMOWNER) {
            LinkModule::get_parent_id(attacker_boma, *ITEM_LINK_NO_TEAMOWNER, true) as u32
        } else {
            return;
        };
        let owner = utils::util::get_battle_object_from_id(owner_id);
        if owner.is_null() { return; }
        attacker_boma = &mut *(*owner).module_accessor;
    }

    if !attacker_boma.is_fighter() { return; }

    let attacker_entry_id = WorkModule::get_int(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    VarModule::set_int(defender_boma.object(), vars::common::instance::LAST_ATTACKER_ENTRY_ID, attacker_entry_id);
}

pub unsafe extern "C" fn process_item_on_collision(defender: u32, attacker: u32, knockback_info: *const f32) {
    let defender_boma = &mut *(*util::get_battle_object_from_id(defender)).module_accessor;
    let attacker_boma = &mut *(*util::get_battle_object_from_id(attacker)).module_accessor;
    if defender_boma.is_item() {
        if defender_boma.kind() == *ITEM_KIND_DAISYDAIKON {
            let damage = *knockback_info.add(22);
            let carrot_dmg = WorkModule::get_int64(defender_boma, *ITEM_DAISYDAIKON_INSTANCE_WORK_INT_ATTACK_POWER) as f32;
            let new_damage = (carrot_dmg + damage * 0.5).max(14.0);
            WorkModule::set_int64(defender_boma, new_damage as i64, *ITEM_DAISYDAIKON_INSTANCE_WORK_INT_ATTACK_POWER);
            if attacker_boma.is_fighter() {
                let attacker_team_no = TeamModule::hit_team_no(attacker_boma) as i32;
                TeamModule::set_team(defender_boma, attacker_team_no, false);
            } else {
                HitModule::set_xlu_frame_global(defender_boma, 15, 0);
            }
            StatusModule::change_status_force(defender_boma, *ITEM_STATUS_KIND_THROW, true);
        }
        if defender_boma.kind() == *ITEM_KIND_BARREL {
            if attacker_boma.is_fighter() {
                let attacker_team_no = TeamModule::hit_team_no(attacker_boma) as i32;
                let owner_id = attacker_boma.battle_object_id;
                //println!("swapping barrel team to {} and owner id to {}", attacker_team_no, owner_id);
                TeamModule::set_team_owner_id(defender_boma, owner_id);
                TeamModule::set_hit_team(defender_boma, attacker_team_no);
            }
            else if attacker_boma.is_weapon() {
                let mut owner_id = WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
                let owner = util::get_battle_object_from_id(owner_id);
                let mut owner_boma = &mut *(*owner).module_accessor;
                if owner_boma.is_weapon() {
                    owner_id = WorkModule::get_int(owner_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
                    let owner = util::get_battle_object_from_id(owner_id);
                    owner_boma = &mut *(*owner).module_accessor;
                }
                // failsafe in case this somehow isn't a fighter
                if !owner_boma.is_fighter(){ return };
                let attacker_team_no = TeamModule::hit_team_no(owner_boma) as i32;
                //println!("swapping barrel team to {} and owner id to {}", attacker_team_no, owner_id);
                TeamModule::set_team_owner_id(defender_boma, owner_id);
                TeamModule::set_hit_team(defender_boma, attacker_team_no);
            }
            else if attacker_boma.is_item() {
                let owner_id;
                if LinkModule::is_link(attacker_boma, *ITEM_LINK_NO_CREATEOWNER) {
                    owner_id = LinkModule::get_parent_id(attacker_boma, *ITEM_LINK_NO_CREATEOWNER, true) as u32;
                } else if !LinkModule::is_link(attacker_boma, *ITEM_LINK_NO_TEAMOWNER) {
                    owner_id = LinkModule::get_parent_id(attacker_boma, *ITEM_LINK_NO_TEAMOWNER, true) as u32;
                } else {
                    return; // this item somehow isn't able to have its team found, get out of here
                }
                let owner_boma = &mut *(*utils::util::get_battle_object_from_id(owner_id));
                // failsafe in case this somehow isn't a fighter
                if !owner_boma.is_fighter() { return };
                let attacker_team_no = TeamModule::hit_team_no(owner_boma) as i32;
                //println!("swapping barrel team to {} and owner id to {}", attacker_team_no, owner_id);
                TeamModule::set_team_owner_id(defender_boma, owner_id);
                TeamModule::set_hit_team(defender_boma, attacker_team_no);
            }
        }
    }
}

pub unsafe extern "C" fn calculate_finishing_hit(defender: u32, attacker: u32, knockback_info: *const f32) {
    *(knockback_info.add(0x4c / 4) as *mut u32) = 60;
    let defender_boma = &mut *(*util::get_battle_object_from_id(defender)).module_accessor;
    let attacker_boma = &mut *(*util::get_battle_object_from_id(attacker)).module_accessor;

    if VarModule::has_var_module(defender_boma.object()) {VarModule::off_flag(defender_boma.object(), vars::common::instance::IS_KILLING_BLOW);}
    if !is_potential_finishing_hit(defender_boma, attacker_boma) { 
        return; 
    }
    if !is_valid_finishing_hit(knockback_info, defender_boma, attacker_boma) { 
        return; 
    }
    if VarModule::has_var_module(defender_boma.object()) {VarModule::on_flag(defender_boma.object(), vars::common::instance::IS_KILLING_BLOW);}
    
    call_finishing_hit_effects(defender_boma, attacker_boma);
}

unsafe extern "C" fn is_potential_finishing_hit(defender_boma: &mut BattleObjectModuleAccessor, attacker_boma: &mut BattleObjectModuleAccessor) -> bool {
    if !defender_boma.is_fighter() || defender_boma.kind() == *FIGHTER_KIND_NANA {
        // println!("kill screen defender is not fighter"); 
        return false;
    }

    if !attacker_boma.is_fighter() && !attacker_boma.is_weapon() { 
        // println!("kill screen attacker is not fighter or weapon"); 
        return false; 
    }

    if VarModule::get_int(defender_boma.object(), COUNTER) > 0 {
        // println!("kill screen is on cooldown"); 
        return false; 
    }

    if attacker_boma.is_fighter() && util::is_no_finishing_hit(attacker_boma) { 
        // println!("kill screen incoming attack is_no_finishing_hit"); 
        return false; 
    }

    return true;
}

pub unsafe extern "C" fn is_teammate_alive(defender_boma: &mut BattleObjectModuleAccessor) -> bool {
    for object_id in util::get_all_active_battle_object_ids() {
        let object = util::get_battle_object_from_id(object_id);
        if object.is_null() { continue; } // skip null

        let other_boma = &mut *(*object).module_accessor;
        if other_boma.kind() == *FIGHTER_KIND_NANA { continue; } // skip nana

        // skip this boma if it belongs to the same player
        let defender_entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        let other_entry_id = WorkModule::get_int(other_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        if defender_entry_id == other_entry_id { continue; }

        // check if another player on the same team has at least one stock
        let fighter_info = app::lua_bind::FighterManager::get_fighter_information(
            crate::singletons::FighterManager(), 
            app::FighterEntryID(other_entry_id)
        );
        if FighterUtil::get_team_color(defender_boma) == FighterUtil::get_team_color(other_boma)
        && app::lua_bind::FighterInformation::stock_count(fighter_info) > 0 { 
            return true;
        }
    }

    return false;
}

pub unsafe extern "C" fn is_final_killing_hit(defender_boma: &mut BattleObjectModuleAccessor, attacker_boma: &mut BattleObjectModuleAccessor) -> bool {
    if util::is_training_mode() {
        return false;
    }

    if is_teammate_alive(defender_boma) { 
        // println!("kill screen teammate stock exists"); 
        return false; 
    }

    // check if the defender is on their last stock
    let defender_entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
    let fighter_info = app::lua_bind::FighterManager::get_fighter_information(
        crate::singletons::FighterManager(), 
        app::FighterEntryID(defender_entry_id)
    );
    if app::lua_bind::FighterInformation::stock_count(fighter_info) != 1 {
        return false;
    }

    return true;
}

unsafe extern "C" fn is_valid_finishing_hit(knockback_info: *const f32, defender_boma: &mut BattleObjectModuleAccessor, attacker_boma: &mut BattleObjectModuleAccessor) -> bool {
    let knockback = *knockback_info;
    let hitstun = *knockback_info.add(0x48 / 4);
    let damage = *knockback_info.add(22);
    let sdi_mul = *knockback_info.add(24);
    let launch_radians = *knockback_info.add(0x10);
    let launch_speed = Vector2f::new(*knockback_info.add(4), *knockback_info.add(5));
    let is_tumble = *(knockback_info.add(1) as *const u32) >= 3;
    let mut context = KnockbackCalcContext::new(
        defender_boma,
        knockback,
        hitstun,
        damage,
        sdi_mul,
        launch_radians,
        launch_speed,
        is_tumble,
    );

    let is_final = is_final_killing_hit(defender_boma, attacker_boma);
    return context.is_finishing_hit(is_final);
}

const HANDLE: i32 = 0x01FF;
const COUNTER: i32 = 0x01FE;

pub unsafe extern "C" fn call_finishing_hit_effects(defender_boma: &mut BattleObjectModuleAccessor, attacker_boma: &mut BattleObjectModuleAccessor) {    
    if is_final_killing_hit(defender_boma, attacker_boma) {
        let handle = EffectModule::req_screen(defender_boma, Hash40::new("bg_finishhit"), false, true, true);
        EffectModule::set_billboard(defender_boma, handle as u32, true);
        EffectModule::set_disable_render_offset_last(defender_boma);
        VarModule::set_int(defender_boma.object(), HANDLE, handle as i32);
        VarModule::set_int(defender_boma.object(), COUNTER, 20);
        SoundModule::play_se(defender_boma, Hash40::new("se_common_boss_down"), false, false, false, false, enSEType(0));
        SlowModule::set_whole(defender_boma, 8, 25);
        let common = util::get_fighter_common_from_accessor(defender_boma);
        EFFECT_GLOBAL_BACK_GROUND(common.lua_state_agent);
    } else {
        let mut pos = Vector3f::new(
            PostureModule::pos_x(defender_boma),
            PostureModule::pos_y(defender_boma) + 10.0,     
            PostureModule::pos_z(defender_boma) + 10.0
        );
        let handle = EffectModule::req(defender_boma, Hash40::new("sys_dead_ripple"), &pos, &Vector3f::new(0.0, 0.0, 0.0), 0.5625, 0, 0, false, 0);
        EffectModule::set_alpha(defender_boma, handle as u32, 0.9);
        EffectModule::set_billboard(defender_boma, handle as u32, true);
        EffectModule::set_disable_render_offset_last(defender_boma);
        EffectModule::set_rate_last(defender_boma, 2.5);
    }
}

// This runs immediately before PostureModule::set_lr is called on throw release
// which determines the facing direction of the receiver when thrown
// 
// We override this to allow throw receiver direction to always be determined by
// which side the attacker was on
#[skyline::hook(offset = 0x6c59cc, inline)]
unsafe fn set_thrown_lr(ctx: &mut skyline::hooks::InlineCtx) {
    let opponent_battle_object_id = *(ctx.registers[20].x() as *const u32).add(0x44 / 4);
    let opponent_battle_object = utils::util::get_battle_object_from_id(opponent_battle_object_id);
    let opponent_boma = (&mut *(*opponent_battle_object).module_accessor);

    if !opponent_boma.is_fighter() {
        return;
    }

    let boma = ctx.registers[19].x() as *mut smash::app::BattleObjectModuleAccessor;
    let fighter = util::get_fighter_common_from_accessor(&mut *boma);

    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("speed_vec_x") as u64);
    sv_information::damage_log_value(fighter.lua_state_agent);
    let damage_speed_x = fighter.pop_lua_stack(1).get_f32();

    let lr: f32 = if damage_speed_x < 0.0 {
        1.0
    } else if damage_speed_x > 0.0 {
        -1.0
    } else {
        PostureModule::lr(boma)
    };

    ctx.registers_f[0].set_s(lr)
}

// This runs immediately before FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR is set
// which determines whether or not to turn the receiver around on hit
// 
// We override this to allow receiver turnaround to always be determined by
// which side the attacker was on
#[skyline::hook(offset = 0x6c5980, inline)]
unsafe fn set_damage_lr(ctx: &mut skyline::hooks::InlineCtx) {
    let opponent_battle_object_id = *(ctx.registers[20].x() as *const u32).add(0x44 / 4);
    let opponent_battle_object = utils::util::get_battle_object_from_id(opponent_battle_object_id);
    let opponent_boma = (&mut *(*opponent_battle_object).module_accessor);

    if !opponent_boma.is_fighter() {
        return;
    }
    
    let opponent_pos_x = PostureModule::pos_x(opponent_boma);

    let boma = ctx.registers[19].x() as *mut smash::app::BattleObjectModuleAccessor;
    let pos_x = PostureModule::pos_x(boma);
    let lr = PostureModule::lr(boma);
    // The ATTACK_LR_CHECK value used by the connecting hitbox
    let attack_lr_check = VarModule::get_int((*opponent_boma).object(), vars::common::instance::ATTACK_LR_CHECK);

    let default_lr = if opponent_pos_x >= pos_x { 1.0 } else { -1.0 };
    let damage_lr: f32 = {
        if attack_lr_check != *ATTACK_LR_CHECK_F && attack_lr_check != *ATTACK_LR_CHECK_B {
            // If reverse hits are disabled for a hitbox,
            // always determine facing direction solely based on the attacker's base position
            // in relation to your base position
            default_lr
        } else {
            // Get the position of your ECB's forward point
            let ecb_front = if opponent_pos_x >= pos_x {
                let ecb_right = *GroundModule::get_rhombus(boma, true).add(3);
                ecb_right.x
            } else {
                let ecb_left = *GroundModule::get_rhombus(boma, true).add(2);
                ecb_left.x
            };

            let attacker_lr = PostureModule::lr(opponent_boma);

            // Determine whether you are functionally "behind" the attacker
            let is_behind_attacker = if opponent_pos_x >= pos_x {
                (attack_lr_check == *ATTACK_LR_CHECK_F && attacker_lr > 0.0)
                    || (attack_lr_check == *ATTACK_LR_CHECK_B && attacker_lr < 0.0)
            } else {
                (attack_lr_check == *ATTACK_LR_CHECK_F && attacker_lr < 0.0)
                    || (attack_lr_check == *ATTACK_LR_CHECK_B && attacker_lr > 0.0)
            };

            // Determine whether your ECB crosses the attacker's base position
            let ecb_crosses_attacker = if opponent_pos_x >= pos_x {
                ecb_front >= opponent_pos_x
            } else {
                ecb_front <= opponent_pos_x
            };

            // If hit behind the attacker,
            // only turn to face them if you are far enough behind them
            if is_behind_attacker && ecb_crosses_attacker {
                -default_lr
            } else {
                default_lr
            }
        }
    };

    ctx.registers_f[0].set_s(damage_lr)
}

#[skyline::hook(offset = 0x3ff1b8, inline)]
unsafe fn get_attack_lr_check(ctx: &mut skyline::hooks::InlineCtx) {
    let attack_module = ctx.registers[1].x();
    let boma = *(attack_module as *mut *mut BattleObjectModuleAccessor).add(1);

    if !(*boma).is_fighter() {
        return;
    }

    let attack_lr_check = ctx.registers[8].w() as i32;
    VarModule::set_int((*boma).object(), vars::common::instance::ATTACK_LR_CHECK, attack_lr_check);
}