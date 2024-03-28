use super::*;
 
//implimented function for checking if an article is "constrained" to snake
extern "C" {
    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Snake21is_constraint_articleERNS_7FighterEiNS_22ArticleOperationTargetE"]
    pub fn is_constraint_article(
        arg1: *mut smash::app::Fighter,
        arg2: i32,
        arg3: smash::app::ArticleOperationTarget,
    ) -> bool;
}

// FIGHTER_STATUS_KIND_APPEAL
// added new up-taunt and side-taunt

unsafe extern "C" fn appeal_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    if ControlModule::get_command_flag_cat(fighter.module_accessor, 1) == *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_r"), 0.0, 1.0, false, 0.0, false, false);
    }
    else if ControlModule::get_command_flag_cat(fighter.module_accessor, 1) == *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L
    || ControlModule::get_command_flag_cat(fighter.module_accessor, 1) == *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_s_r"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_r"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(appeal_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn appeal_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    else if  MotionModule::is_end(fighter.module_accessor) {
        if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("appeal_lw_r") {
            fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return true.into()
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    else if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX)
    && MotionModule::motion_kind(fighter.module_accessor) != smash::hash40("appeal_lw_r") {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    return false.into()
}

unsafe extern "C" fn appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT {
        fighter.clear_lua_stack();
        let object = sv_system::battle_object(fighter.lua_state_agent);
        let fighta : *mut Fighter = std::mem::transmute(object);
        if is_constraint_article(fighta, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) {
            fighter.on_flag(*FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    return 0.into()
}

// FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT
// added down-taunt box walk and c4 place/explode

unsafe extern "C" fn appeal_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_wait"), 0.0, 1.0, false, 0.0, false, false);
    fighter.off_flag(*FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
    let appeal_wait_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("appeal_wait_frame"));
    fighter.set_int(appeal_wait_frame, *FIGHTER_SNAKE_STATUS_APPEAL_WORK_INT_WAIT_COUNTER);
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    VarModule::off_flag(fighter.object(), vars::snake::instance::DTAUNT_C4_EXLPODE);
    VarModule::set_int(fighter.object(), vars::snake::instance::DTAUNT_GRENADE_WAIT_COUNT, 0);
    fighter.sub_shift_status_main(L2CValue::Ptr(appeal_wait_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn appeal_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if VarModule::get_int(fighter.object(), vars::snake::instance::DTAUNT_GRENADE_WAIT_COUNT) > 0 {
        VarModule::dec_int(fighter.object(), vars::snake::instance::DTAUNT_GRENADE_WAIT_COUNT);
    }
    WorkModule::dec_int(fighter.module_accessor, *FIGHTER_SNAKE_STATUS_APPEAL_WORK_INT_WAIT_COUNTER);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
    || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
    || fighter.get_int(*FIGHTER_SNAKE_STATUS_APPEAL_WORK_INT_WAIT_COUNTER) <= 0 {
        fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_APPEAL_END.into(), false.into());
        return true.into()
    //place c4
    }
    else if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) == *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4) {
            VarModule::on_flag(fighter.object(), vars::snake::instance::DTAUNT_C4_EXLPODE);
            fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_APPEAL_END.into(), false.into());
        }
        else {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, false, 0);
            ArticleModule::have(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, Hash40::new("havel"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), 0, false);
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    //spawn grenade
    }
    else if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) == *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE)
    && VarModule::get_int(fighter.object(), vars::snake::instance::DTAUNT_GRENADE_WAIT_COUNT) <= 0
    {
        VarModule::set_int(fighter.object(), vars::snake::instance::DTAUNT_GRENADE_WAIT_COUNT, 30);

        ////adjusts first grenade position only
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, false, 0);
        PLAY_SE(fighter, Hash40::new("se_snake_special_n01"));
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ArticleModule::have(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, Hash40::new("havel"), ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), 0, false);
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
        // let pos_x = PostureModule::pos_x(fighter.module_accessor);
        // let pos_y = PostureModule::pos_y(fighter.module_accessor);
        // ArticleModule::set_pos(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, Vector3f{x:pos_x, y:pos_y+1.0, z:0.0});

        ////doesn't set player as owner of grenade
        // ItemModule::attach_item(fighter.module_accessor, ItemKind(*ITEM_KIND_SNAKEGRENADE), *ATTACH_ITEM_GROUP_BODY, true);
        // EFFECT_OFF_KIND(fighter, Hash40::new("sys_item_get"), true, true);
        // PLAY_SE(fighter, Hash40::new("se_snake_special_n01"));
        // ItemModule::drop_attach(fighter.module_accessor, ItemKind(*ITEM_KIND_SNAKEGRENADE), 0.0, 0.0);

        // DamageModule::add_damage(fighter.module_accessor, *LINK_NO_ARTICLE as f32, 0);

        ////adjusts first article only
        // ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_GRENADE, false, 0);
        // let article_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CONSTRAINT);
        // let article_boma = sv_battle_object::module_accessor(article_id as u32);
        // LinkModule::set_model_constraint_target_joint(article_boma, Hash40::new("kneer"));
    }
    else {
        let velocity_x :f32 = PostureModule::lr(fighter.module_accessor) * ControlModule::get_stick_x(fighter.module_accessor);
        SET_SPEED_EX(fighter, velocity_x*0.6, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    return false.into()
}

unsafe extern "C" fn appeal_wait_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb].get_i32() != *FIGHTER_SNAKE_STATUS_KIND_APPEAL_END {
        fighter.clear_lua_stack();
        let object = sv_system::battle_object(fighter.lua_state_agent);
        let fighta : *mut Fighter = std::mem::transmute(object);
        if is_constraint_article(fighta, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) {
            fighter.on_flag(*FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
            ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    return 0.into()
}

// FIGHTER_SNAKE_STATUS_KIND_APPEAL_END

unsafe extern "C" fn appeal_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if VarModule::is_flag(fighter.object(), vars::snake::instance::DTAUNT_C4_EXLPODE) {
        VarModule::off_flag(fighter.object(), vars::snake::instance::DTAUNT_C4_EXLPODE);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_end_explode"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.off_flag(*FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
    fighter.sub_shift_status_main(L2CValue::Ptr(appeal_end_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn appeal_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return true.into()
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return true.into()
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return true.into()
        }
    }
    return false.into()
}

unsafe extern "C" fn appeal_end_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    let object = sv_system::battle_object(fighter.lua_state_agent);
    let fighta : *mut Fighter = std::mem::transmute(object);
    if is_constraint_article(fighta, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) {
        fighter.on_flag(*FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_CBOX, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_APPEAL, appeal_main);
    agent.status(End, *FIGHTER_STATUS_KIND_APPEAL, appeal_end);

    agent.status(Main, *FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT, appeal_wait_main);
    agent.status(End, *FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT, appeal_wait_end);

    agent.status(Main, *FIGHTER_SNAKE_STATUS_KIND_APPEAL_END, appeal_end_main);
    agent.status(End, *FIGHTER_SNAKE_STATUS_KIND_APPEAL_END, appeal_end_end);
}