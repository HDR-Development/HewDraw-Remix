// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn levin_leniency(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    // move to attack_air
    if boma.is_motion_one_of(&[
        Hash40::new("attack_air_n"),
        Hash40::new("attack_air_f"),
        Hash40::new("attack_air_b"),
        Hash40::new("attack_air_hi"),
        Hash40::new("attack_air_lw"),
    ]) {
        if VarModule::get_int(fighter.battle_object, vars::reflet::instance::ATTACK_AIR_LEVIN_LENIENCY) > 0 {
            VarModule::dec_int(fighter.battle_object, vars::reflet::instance::ATTACK_AIR_LEVIN_LENIENCY);
            if !fighter.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) 
            && boma.is_button_on(Buttons::Smash | Buttons::SpecialRaw | Buttons::Catch) {
                let levin = *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT;
                if WorkModule::get_int(boma, levin) > 0 {
                    if WorkModule::get_int(boma, levin) == 1 {
                        app::FighterSpecializer_Reflet::set_flag_to_table(fighter.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_SWORD, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
                    }
                    fighter.on_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
                    VisibilityModule::set_int64(boma, Hash40::new("sword").hash as i64, Hash40::new("sword_thunder").hash as i64);
                    WorkModule::dec_int(boma, levin);
                }
            }
        }
    }
}

// Lengthen sword
unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_STATUS_KIND_ATTACK_HI4)
    && WorkModule::is_flag(boma, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        if boma.status_frame() <= 14 {
            ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("sword"), &Vector3f::new(1.0, 1.075, 1.0475));
        }
    }
    else {
        let long_sword_scale = Vector3f{x: 1.0, y: 1.175, z: 1.0475};
        ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("haver"), &long_sword_scale);
    }
}

// mess w book
unsafe fn resource_depleted(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    //enables discard 
    if StatusModule::is_changing(fighter.module_accessor)
    && !(prev_status == statuses::reflet::FLOAT && status == *FIGHTER_STATUS_KIND_ATTACK_AIR)
    && ![*FIGHTER_STATUS_KIND_ATTACK_100,
        *FIGHTER_STATUS_KIND_ATTACK_S4,
        *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_HI4,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD,
        *FIGHTER_STATUS_KIND_ATTACK_LW4,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD,
        *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_START,
        *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_HOLD,
        *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_TRON_END,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_CAPTURE,
        *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_REFLET_STATUS_KIND_SPECIAL_HI_2,
    ].contains(&status) {
        VarModule::off_flag(fighter.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS);
    }
    let magic = app::FighterSpecializer_Reflet::get_magickind(fighter.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
    let discard_type = VarModule::get_int(fighter.battle_object, vars::reflet::instance::DISCARD_TYPE);//not sure if needed
    //if discard queue'd in vanilla system, bypass to avoid hardcoded auto-toss-cancel mechanic
    if magic > -1 {
        if discard_type > 0 && magic != discard_type {//prevent buggy rapid-fire spawns (should already be fixed in statuses but)
            spawn_items(fighter);//prevent overriding discard queue (shouldnt be able to discard 2 resources same status but)
        }
        if magic < 4 {
            VarModule::set_int(fighter.battle_object, vars::reflet::instance::DISCARD_TYPE, fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND));
        } else {
            VarModule::set_int(fighter.battle_object, vars::reflet::instance::DISCARD_TYPE, magic);
        }
        FighterSpecializer_Reflet::set_flag_to_table(fighter.module_accessor as *mut app::FighterModuleAccessor, magic, false, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
        VarModule::on_flag(fighter.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS);
    }
    //if eligible to discard
    if discard_type > -1
    && !VarModule::is_flag(fighter.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS)//move that caused discard waits until x frame (generally when returning from attack pose)
    && (!StopModule::is_damage(fighter.module_accessor) || !ItemModule::is_have_item(fighter.module_accessor, 0))//if holding an item already dont toss until hitstun over
    {
        spawn_items(fighter);
    }
}

unsafe fn spawn_items(fighter: &mut L2CFighterCommon) {
    let discard_type = VarModule::get_int(fighter.battle_object, vars::reflet::instance::DISCARD_TYPE);
    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        ItemModule::throw_item(fighter.module_accessor, 110.0, 1.62, 1.0, 0, true, fighter.get_float(*ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_POWER));
    }
    if discard_type != *FIGHTER_REFLET_MAGIC_KIND_SWORD {
        ItemModule::have_item(fighter.module_accessor, app::ItemKind(*ITEM_KIND_BOOK), 0, 0, false, false);
        let item_id = ItemModule::get_have_item_id(fighter.module_accessor, 0);
        let item_boma = sv_battle_object::module_accessor(item_id as u32);
        MotionModule::set_rate_material(item_boma, 0.0, MaterialAnimeKind{_address: 0});
        MotionModule::set_frame_material(item_boma, discard_type as f32, MaterialAnimeKind{_address: 0});
    } else {
        ItemModule::have_item(fighter.module_accessor, app::ItemKind(*ITEM_KIND_THUNDERSWORD), 0, 0, false, false);
        app::FighterSpecializer_Reflet::change_grimoire(fighter.module_accessor as *mut app::FighterModuleAccessor, -1);//bookless
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
    }
    VarModule::set_int(fighter.battle_object, vars::reflet::instance::DISCARD_TYPE, -1);
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_REFLET_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_REFLET_STATUS_KIND_SPECIAL_LW_END
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    levin_leniency(fighter, boma);
    sword_length(boma);
    resource_depleted(fighter);
    fastfall_specials(fighter);
}

pub extern "C" fn reflet_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        reflet_frame(fighter)
    }
}

pub unsafe fn reflet_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, reflet_frame_wrapper);
}
