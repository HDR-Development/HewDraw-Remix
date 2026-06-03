use super::*;

// FIGHTER_STATUS_KIND_APPEAL

pub unsafe extern "C" fn appeal_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.boma();
    let frame = fighter.status_frame();
    let magic = fighter.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
    if frame == 0 {
        LEVIN_ON_TAUNT(boma);//show if you have levin
        let mut reflet_fighter = app::Fighter{battle_object: *(fighter.battle_object)};
        FighterSpecializer_Reflet::change_hud_kind(&mut reflet_fighter, magic);//show magic meter if you taunt
    }
    if frame > 4 && frame < 24 {
        KILL_RESOURCE(fighter, boma);
    } else if frame == 25 {
        VarModule::off_flag(fighter.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS);
    }
    return 0.into();
}

pub unsafe fn LEVIN_OFF(boma: &mut BattleObjectModuleAccessor) {
    if boma.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) <= 0 
    && boma.is_flag(*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON) {
        VarModule::off_flag(utils::util::get_battle_object_from_accessor(boma), vars::reflet::instance::DISCARD_SKIP_STATUS);
        VisibilityModule::set_int64(boma, Hash40::new("sword").hash as i64, Hash40::new("sword_normal").hash as i64);
        boma.off_flag( *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
    }
}

pub unsafe fn LEVIN_ON_TAUNT(boma: &mut BattleObjectModuleAccessor) {
    if boma.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0 {
        VisibilityModule::set_int64(boma, Hash40::new("sword").hash as i64, Hash40::new("sword_thunder").hash as i64);
        boma.on_flag( *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_THUNDER_SWORD_ON);
    }
}

pub unsafe fn KILL_RESOURCE(fighter: &mut L2CAgentBase, boma: &mut BattleObjectModuleAccessor) {
    if is_training_mode() && !VarModule::is_flag(fighter.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS) {
        let mut reflet_fighter = app::Fighter{battle_object: *(fighter.battle_object)};
        if boma.is_button_on(Buttons::Attack) {
            VarModule::on_flag(fighter.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS);
            app::FighterSpecializer_Reflet::change_hud_kind(&mut reflet_fighter, *FIGHTER_REFLET_MAGIC_KIND_SWORD);
            if boma.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT) > 0 {
                boma.set_int(0, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
                FighterSpecializer_Reflet::set_flag_to_table(fighter.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_SWORD, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
                LEVIN_OFF(boma);
                ItemModule::set_have_item_visibility(boma, true, 0);
                app::FighterSpecializer_Reflet::change_grimoire(fighter.module_accessor as *mut app::FighterModuleAccessor, -1);
            } else {
                let resource_cap = fighter.get_param_int("param_private", "thunder_sword_usage_count_max");
                boma.set_int(resource_cap, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THUNDER_SWORD_CURRENT_POINT);
                LEVIN_ON_TAUNT(boma);
                FighterSpecializer_Reflet::set_flag_to_table(fighter.module_accessor as *mut app::FighterModuleAccessor, *FIGHTER_REFLET_MAGIC_KIND_SWORD, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_RECOVER_TABLE);
                ItemModule::set_have_item_visibility(boma, false, 0);
            }
        } else if boma.is_button_on(Buttons::Special) {
            VarModule::on_flag(fighter.battle_object, vars::reflet::instance::DISCARD_SKIP_STATUS);
            let magic = boma.get_int(*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
            app::FighterSpecializer_Reflet::change_hud_kind(&mut reflet_fighter, magic);
            match magic {
                4 => {
                    // fire
                    let resource_cap = fighter.get_param_int("param_private", "grimoire_giga_fire_usage_count_max");
                    MAGIC_HANDLER(fighter, boma, magic, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_CURRENT_POINT, resource_cap);
                },
                5 => {
                    // wind
                    let resource_cap = fighter.get_param_int("param_private", "grimoire_el_window_usage_count_max");
                    MAGIC_HANDLER(fighter, boma, magic, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_HI_CURRENT_POINT, resource_cap);
                },
                6 => {
                    // nosferatu
                    let resource_cap = fighter.get_param_int("param_private", "grimoire_rizaia_usage_count_max");
                    MAGIC_HANDLER(fighter, boma, magic, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT, resource_cap);
                },
                _ => {
                    // thunder
                    let resource_cap = fighter.get_param_int("param_private", "grimoire_thunder_usage_count_max");
                    MAGIC_HANDLER(fighter, boma, magic, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_N_CURRENT_POINT, resource_cap);
                }
            }
        }
    }
}

pub unsafe fn MAGIC_HANDLER(fighter: &mut L2CAgentBase, boma: &mut BattleObjectModuleAccessor, last_magic_kind: i32, resource_kind: i32, resource_cap: i32) {
    if CHECK_MAGIC(fighter) {
        if last_magic_kind == *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE {boma.set_float(0.0, resource_kind); }
        else {boma.set_int(0, resource_kind); }
        if ItemModule::is_have_item(boma, 0) {
            ItemModule::throw_item(boma, 110.0, 1.62, 1.0, 0, true, fighter.get_float(*ITEM_FIGHTER_VAR_FLOAT_ITEM_THROW_POWER));
        }
        ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_BOOK), 0, 0, false, false);
        let item_id = ItemModule::get_have_item_id(boma, 0);
        let item_boma = sv_battle_object::module_accessor(item_id as u32);
        MotionModule::set_rate_material(item_boma, 0.0, MaterialAnimeKind{_address: 0});
        MotionModule::set_frame_material(item_boma, last_magic_kind as f32, MaterialAnimeKind{_address: 0});
        FighterSpecializer_Reflet::set_flag_to_table(fighter.module_accessor as *mut app::FighterModuleAccessor, last_magic_kind, false, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
        app::FighterSpecializer_Reflet::change_grimoire(fighter.module_accessor as *mut app::FighterModuleAccessor, -1);
        ItemModule::set_have_item_visibility(boma, true, 0);

    } else {
        if last_magic_kind == *FIGHTER_REFLET_MAGIC_KIND_GIGA_FIRE {boma.set_float(resource_cap as f32, resource_kind); }
        else {boma.set_int(resource_cap, resource_kind); }
        FighterSpecializer_Reflet::set_flag_to_table(fighter.module_accessor as *mut app::FighterModuleAccessor, last_magic_kind, true, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_RECOVER_TABLE);
        app::FighterSpecializer_Reflet::change_grimoire(fighter.module_accessor as *mut app::FighterModuleAccessor, last_magic_kind);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_STATUS_KIND_APPEAL, appeal_exec);
}