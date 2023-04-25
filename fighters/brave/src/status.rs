use super::*;
use smash2::app::{FighterManager, BraveSetMenuCommand, BraveEnableMenuCommand, BraveShowMenu, BraveSetMenuSelectedCommand};

extern "C" {
    #[link_name = "_ZN3lib9SingletonIN3app21FighterParamAccessor2EE9instance_E"]
    static PARAM_INSTANCE: u64;
}

extern "C" {
    #[link_name = "_ZN3app24FighterSpecializer_Brave30get_special_lw_command_sp_costERKNS_26BattleObjectModuleAccessorENS_28FighterBraveSpecialLwCommandEb"]
    fn get_special_lw_command_sp_cost(boma: *mut BattleObjectModuleAccessor, command: i32, pass_false: bool) -> f32;
}

unsafe fn set_command_for_slot(fighter: &mut BattleObject, slot: usize, id: i32) {
    let hero_mana = fighter.get_float(0x53);
    let mana = get_special_lw_command_sp_cost(fighter.module_accessor, id, false);

    FighterManager::instance().unwrap().send_event(BraveSetMenuCommand::new(
        fighter.get_int(0x1000000) as u32, // ENTRY_ID
        (slot + 1) as u32,
        (id + 1) as i32,
        mana
    ));
    FighterManager::instance().unwrap().send_event(BraveEnableMenuCommand::new(
        fighter.get_int(0x1000000) as u32,
        (slot + 1) as u32,
        hero_mana >= mana
    ));

    if hero_mana >= mana {
        fighter.set_int(id, 0x10000d4 + slot as i32);
    }
    else {
        fighter.set_int(-1, 0x10000d4 + slot as i32);
    }

    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_1 + slot as i32, id);
    *(*(fighter as *mut _ as *const u64).add(0xd8 / 8) as *mut u8).add(slot) = id as u8;
}

// [Command ID List]
// 0x0 - Heal
// 0x1 - Sizz
// 0x2 - Sizzle
// 0x3 - Bang
// 0x4 - Kaboom
// 0x5 - Whack
// 0x6 - Thwack
// 0x7 - Magic Burst
// 0x8 - Kamikazee
// 0x9 - Kaclang
// 0xA - Acceleratle
// 0xB - Oomph
// 0xC - Bounce
// 0xD - Snooze
// 0xE - Hocus Pocus
// 0xF - Zoom (unused)
// 0x10 - Flame Slash
// 0x11 - Kacrackle Slash
// 0x12 - Metal Slash
// 0x13 - Hatchet Man
// 0x14 - Psyche Up
pub unsafe fn roll_spells(fighter: &mut BattleObject, vals: &mut Vec<i32>) {
    loop {
        // get history of last two rolls
        let mut used_vals = vec![];
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_1));
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_2));
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_3));
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_4));
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_1));
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_2));
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_3));
        used_vals.push(VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_4));
        let roll = smash::app::sv_math::rand(smash::hash40("fighter"), 100);

        // rarer rolls
        let mut val = match roll {
            0 => 0x8,
            1..=3 => 0x7,
            4..=6 => 0x5,
            7..=9 => 0x6,
            10..=12 => 0xE,
            13..=22 => 0x9,
            23..=32 => 0x12,
            _ => 0xF
        };

        // other spells (and not zoom)
        if val == 0xF {
            loop {
                val = smash::app::sv_math::rand(smash::hash40("fighter"), 0x15);
                if vals.contains(&val) || used_vals.contains(&val) || (val >= 0x5 && val <= 0x9) || val == 0xE || val == 0xF || val == 0x12 {
                    continue;
                }
                else {
                    break;
                }
            }
        }
        else {
            // remove duplicates
            if vals.contains(&val) || used_vals.contains(&val) {
                continue;
            }
        }
        
        vals.push(val);
        if vals.len() >= 4 {
            break;
        }
    }

    // store new roll history
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_1, vars::brave::instance::SPELL_SLOT_USED_1_1);
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_2, vars::brave::instance::SPELL_SLOT_USED_1_2);
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_3, vars::brave::instance::SPELL_SLOT_USED_1_3);
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_2_4, vars::brave::instance::SPELL_SLOT_USED_1_4);
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_1, vals[0]);
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_2, vals[1]);
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_3, vals[2]);
    VarModule::set_int(fighter, vars::brave::instance::SPELL_SLOT_USED_1_4, vals[3]);
}

#[no_mangle]
pub unsafe extern "C" fn hero_rng_hook_impl(fighter: &mut BattleObject) {
    // println!("b8: {:#x}", *(fighter as *mut _ as *const u64).add(0xb8 / 8));
    // println!("c8: {:#x}", *(fighter as *mut _ as *const u32).add(0xc8 / 4));
    // println!("d0: {:#x}", *(fighter as *mut _ as *const u64).add(0xd0 / 8));
    // println!("c0: {:#x}", *(fighter as *mut _ as *const u64).add(0xc0 / 8));
    // println!("d8: {:#x}", *(fighter as *mut _ as *const u64).add(0xd8 / 8));
    // 0x15 = *FIGHTER_BRAVE_SPECIAL_LW_COMMAND_NUM
    // *(*(fighter as *mut _ as *const u64).add(0xd8 / 8) as *mut u32) = 0x0E0E0E0E;

    *(fighter as *mut _ as *mut u64).add(0xc0 / 8) = 4; // controls how many commands are active
    *(fighter as *mut _ as *mut u64).add(0xd0 / 8) = 4;
    fighter.set_int(3, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_WINDOW_STATE);
    fighter.set_int(0, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_WINDOW_OVERWRITE_FRAME);
    fighter.off_flag(*FIGHTER_BRAVE_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_ENABLE_COMMAND_WINDOW_OVERWRITE);
    let mut index = VarModule::get_int(fighter, vars::brave::instance::CURSOR_SLOT);

    if !VarModule::is_flag(fighter, vars::brave::instance::PERSIST_RNG) {
        VarModule::on_flag(fighter, vars::brave::instance::PERSIST_RNG);
        index = 0;
        let we_ball = smash::app::sv_math::rand(smash::hash40("fighter"), 100);
        if we_ball == 1 {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_level_up"), Hash40::new("top"), &Vector3f::new(0.0, 10.0, 0.0), &Vector3f::new(0.0, 0.0, 0.0), 0.8, false, 0, 0, 0, 0, 0, false, false);
            let mut rand: i32;
            loop {
                rand = smash::app::sv_math::rand(smash::hash40("fighter"), 0x15);
                if rand == 0xF {
                    continue;
                }
                else {
                    break;
                }
            }
            
            set_command_for_slot(fighter, 0, rand);/*0x14);*/
            set_command_for_slot(fighter, 1, rand);/*0xB);*/
            set_command_for_slot(fighter, 2, rand);/*0xA);*/
            set_command_for_slot(fighter, 3, rand);/*0x7);*/
        }
        else {
            let mut vals = vec![];
            roll_spells(fighter, &mut vals);

            set_command_for_slot(fighter, 0, vals[0]);
            set_command_for_slot(fighter, 1, vals[1]);
            set_command_for_slot(fighter, 2, vals[2]);
            set_command_for_slot(fighter, 3, vals[3]);
        }
    }
    else {
        let slot_1 = VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_1);
        let slot_2 = VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_2);
        let slot_3 = VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_3);
        let slot_4 = VarModule::get_int(fighter, vars::brave::instance::SPELL_SLOT_4);
        set_command_for_slot(fighter, 0, slot_1);
        set_command_for_slot(fighter, 1, slot_2);
        set_command_for_slot(fighter, 2, slot_3);
        set_command_for_slot(fighter, 3, slot_4);
    }

    fighter.set_int(index, *FIGHTER_BRAVE_INSTANCE_WORK_ID_INT_SPECIAL_LW_SELECT_INDEX);
    FighterManager::instance().unwrap().send_event(BraveShowMenu::new(
        fighter.get_int(0x1000000) as u32
    ));
    FighterManager::instance().unwrap().send_event(BraveSetMenuSelectedCommand::new(
        fighter.get_int(0x1000000) as u32,
        (index as u32) + 1
    ));
}