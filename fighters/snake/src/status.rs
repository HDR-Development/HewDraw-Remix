use super::*;
use globals::*;
use smashline::*;
 

pub fn install() {
    install_status_scripts!(
        // snake_c4_status_pre,
        // snake_c4_fall_status_main,
        // snake_down_special_start_status_main,
        // snake_down_special_start_status_end,
        // snake_down_special_ground_status_main,
        // snake_down_special_air_ground_status_main,
        // snake_down_special_ground_status_end,
        // snake_down_special_air_ground_status_end,
        // snake_down_special_status_main,
        // snake_landing_attack_status_main,
        snake_side_smash_status_main,
        snake_side_smash_status_end,
        snake_side_special_status_main,
        // snake_side_special_status_end
        snake_grab_pull_status_main,
        snake_grab_dash_pull_status_main,
        snake_grab_attack_status_main,
        // snake_grab_throw_status_main,
        snake_grab_wait_status_main,
        snake_grab_wait_status_end,
        snake_taunt_status_main,
        snake_taunt_status_end,
        snake_down_taunt_wait_status_main,
        snake_down_taunt_wait_status_end,
        snake_down_taunt_end_status_main,
        snake_down_taunt_end_status_end
    );
}

//implimented function for checking if an article is "constrained" to snake
extern "C" {
    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Snake21is_constraint_articleERNS_7FighterEiNS_22ArticleOperationTargetE"]
    pub fn is_constraint_article(
        arg1: *mut smash::app::Fighter,
        arg2: i32,
        arg3: smash::app::ArticleOperationTarget,
    ) -> bool;
}

#[status_script(agent = "snake_c4", status = WEAPON_SNAKE_C4_STATUS_KIND_ESTABLISH_TARGET, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn snake_c4_status_pre(weapon: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_SNAKE_C4_DROP_FALL_FLAG, // <-- wrong flag make it check ground collision?
        *WEAPON_STATUS_WORK_KEEP_FLAG_SNAKE_C4_ESTABLISH_TARGET_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_SNAKE_C4_ESTABLISH_TARGET_FLOAT,
        0,
    );
    0.into()
}


#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_PRODUCE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_down_special_start_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT) {
        fighter.set_int64(hash40("special_lw_squat_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
    }
    else {
        fighter.set_int64(hash40("special_lw_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
    }
    fighter.set_int64(hash40("special_air_lw_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);

    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        let motion = fighter.get_int64(*FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        let motion = fighter.get_int64(*FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(motion), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_lw_produce_main_loop as *const () as _))
}
pub unsafe fn special_lw_produce_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //println!("prev status is {}, {}", StatusModule::prev_status_kind(fighter.module_accessor, 0), StatusModule::prev_status_kind(fighter.module_accessor, 1));
    if fighter.is_flag(*FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT)
    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.off_flag(*FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT);
        fighter.set_int64(hash40("special_lw_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH)
        || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_DROP.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_ESTABLISH_GROUND.into(), false.into());
            }
            VarModule::on_flag(fighter.object(), vars::snake::instance::SELF_STICK);
        }
        else {
            VarModule::off_flag(fighter.object(), vars::snake::instance::SELF_STICK);
            fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_ESTABLISH_JUDGE.into(), false.into());
        }
        false.into()
    }
    else {
        change_motion_by_situation(fighter, false);
        true.into()
    }
}
pub unsafe fn change_motion_by_situation(fighter: &mut L2CFighterCommon, skip_check: bool) {
    if skip_check == false {
        if StatusModule::situation_kind(fighter.module_accessor) == StatusModule::prev_situation_kind(fighter.module_accessor) {
            return
        }
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        GroundModule::set_rhombus_offset(fighter.module_accessor, &Vector2f::zero());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        let motion = fighter.get_int64(*FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let motion = fighter.get_int64(*FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(motion), -1.0, 1.0, 0.0, false, false);
    }
}

#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_PRODUCE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_down_special_start_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_ESTABLISH_GROUND
    && fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_DROP
    && fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_ESTABLISH_JUDGE {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(0));
    }
    0.into()
}

#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_ESTABLISH_GROUND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_down_special_ground_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if VarModule::is_flag(fighter.object(), vars::snake::instance::SELF_STICK) {
        if fighter.is_flag(*FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_squat_self_stick"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_self_stick"), 0.0, 1.0, false, 0.0, false, false);
        }
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_START, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    ret
}
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_DROP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_down_special_air_ground_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if VarModule::is_flag(fighter.object(), vars::snake::instance::SELF_STICK) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_self_stick"), 0.0, 1.0, false, 0.0, false, false);
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_START, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    ret
}

//deletes c4 if not "shot"
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_ESTABLISH_GROUND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_down_special_ground_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    let object = sv_system::battle_object(fighter.lua_state_agent);
    let fighta : *mut Fighter = std::mem::transmute(object);
    if is_constraint_article(fighta, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
    && ArticleModule::motion_kind(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) != hash40("stick_target") {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_DROP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_down_special_air_ground_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    let object = sv_system::battle_object(fighter.lua_state_agent);
    let fighta : *mut Fighter = std::mem::transmute(object);
    if is_constraint_article(fighta, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL))
    && ArticleModule::motion_kind(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) != hash40("stick_target") {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    0.into()
}

// landing back-air ends in squat position
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_landing_attack_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.off_flag(*FIGHTER_SNAKE_INSTANCE_WORK_FLAG_CYPHER_FALL);
    let motion = fighter.get_int64(*FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    if motion == hash40("attack_air_b") {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_air_b"), 0.0, 1.0, false, 0.0, false, false);
        fighter.on_flag(*FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT);
    }
    else if motion == hash40("attack_air_f") {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_air_f"), 0.0, 1.0, false, 0.0, false, false);
    }
    else if motion == hash40("attack_air_lw") {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_air_lw"), 0.0, 1.0, false, 0.0, false, false);
    }
    else if motion == hash40("attack_air_n") {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_air_n"), 0.0, 1.0, false, 0.0, false, false);
    }
    else if motion == hash40("attack_air_hi") {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_air_hi"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_heavy"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_landing_attack_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_landing_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_flag(*FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT) { // added check for back air
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.is_flag(*FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT) { // added check for back air
                fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            // true.into()
        }
        else if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                // true.into()
            }
        }
        else {
            return false.into()
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        // true.into()
    }
    true.into()
}

// added self-stick-squat and back-air to down-special check
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_down_special_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SQUAT_WAIT
    || fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SQUAT_F
    || fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_SQUAT_B
    || fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_DASH
    || fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_LW3
    || MotionModule::motion_kind(fighter.module_accessor) ==  hash40("special_lw_squat_ground")
    || MotionModule::motion_kind(fighter.module_accessor) ==  hash40("special_lw_squat_blast")
    || MotionModule::motion_kind(fighter.module_accessor) ==  hash40("special_lw_squat_self_stick") // added exception for sticking yourself
    || MotionModule::motion_kind(fighter.module_accessor) ==  hash40("landing_air_b") { // added exception for landing back air
        fighter.on_flag(*FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT);
    }
    else {
        fighter.off_flag(*FIGHTER_SNAKE_STATUS_SPECIAL_LW_FLAG_SQUAT);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_down_special_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_down_special_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4) {
        fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_EXPLODING.into(), false.into());
    }
    else  {
        fighter.change_status(FIGHTER_SNAKE_STATUS_KIND_SPECIAL_LW_PRODUCE.into(), false.into());
    }
    1.into()
}

////changed rpg7 side-smash to multi-hit knife
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_side_smash_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    original!(fighter)
}
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_side_smash_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    VarModule::off_flag(fighter.object(), vars::snake::instance::KNIFE_COMBO_ENABLE); 
    VarModule::off_flag(fighter.object(), vars::snake::instance::KNIFE_COMBO_IS_BUFFERED); 
    VarModule::set_int(fighter.object(), vars::snake::instance::KNIFE_COMBO_COUNT, 0); 
    original!(fighter)
}

////side-special tranq gun
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_side_special_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    fighter.set_int64(hash40("special_s_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_KIND);
    fighter.set_int64(hash40("special_air_s_start") as i64, *FIGHTER_SNAKE_STATUS_WORK_INT_MOT_AIR_KIND);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(special_side_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn special_side_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        return true.into()
    }
    else {
        change_motion_by_situation(fighter, false);
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool()
            || fighter.sub_air_check_fall_common().get_bool() {
                return true.into()
            }
        }
    }
    false.into()
}

////fixed unwanted buffered throws and walking
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_CATCH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_grab_pull_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_pull"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_grab_pull_main_loop as *const () as _))
    // 0.into()
}
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_CATCH_DASH_PULL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_grab_dash_pull_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_pull"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_grab_pull_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_grab_pull_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        return true.into()
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.1
        || PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.1{
            VarModule::on_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
        }
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
        return true.into()
    }
    else if MotionModule::frame(fighter.module_accessor) >= 1.0 {
        if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW != 0 {
            fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }
        else if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI != 0 {
            fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        //check stick directly for easier instant f-throw
        }
        else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.7 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }
        }
        else if fighter.global_table[CMD_CAT2].get_i32() & *FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B != 0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }
        }
        else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_ATTACK.into(), false.into());
            return true.into()
        }
    }
    return false.into()
}
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_CATCH_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_grab_attack_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_attack"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_grab_attack_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_grab_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        return true.into()
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
        return true.into()
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if ControlModule::get_stick_y(fighter.module_accessor) < -0.7 {
            fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }
        else if ControlModule::get_stick_y(fighter.module_accessor) > 0.7 {
            fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()

        }
        else if PostureModule::lr(fighter.module_accessor) * ControlModule::get_stick_x(fighter.module_accessor) > 0.7 {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }
            else {
                VarModule::on_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
                return true.into()
            }
        }
        else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.7 {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
                fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
                return true.into()
            }
            else{
                VarModule::on_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_WAIT.into(), false.into());
                return true.into()
            }
        }
        else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_attack"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    return false.into()
}

////added grab walk
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_grab_wait_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    ControlModule::reset_trigger(fighter.module_accessor);
    if VarModule::is_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK) {
        if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.1 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_f"), 0.0, 1.0, false, 0.0, false, false);
        }
        else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.1 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_b"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            VarModule::off_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_wait"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_grab_wait_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_grab_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
        return true.into()
    }
    else if fighter.is_cat_flag(Cat2::ThrowLw) {
        fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
        fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
        return true.into()
    }
    else if fighter.is_cat_flag(Cat2::ThrowHi) {
        fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
        fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
        return true.into()
    }
    else if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.7 {
            fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }
        else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.7 {
            fighter.set_int(*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
            fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
            return true.into()
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_ATTACK.into(), false.into());
            return true.into()
        }
    }
    else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) < -0.1 {
        if !VarModule::is_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK) {
            VarModule::on_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_b"), 0.0, 1.0, false, 0.0, false, false);
        }
        let walk_speed:f32 = 1.6*(PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor)*-1.0);
        MotionModule::set_rate(fighter.module_accessor, walk_speed);
    }
    else if PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor) > 0.1 {
        if !VarModule::is_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK) {
            VarModule::on_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_walk_f"), 0.0, 1.0, false, 0.0, false, false);
        }
        let walk_speed:f32 = 1.4*(PostureModule::lr(fighter.module_accessor)*ControlModule::get_stick_x(fighter.module_accessor));
        MotionModule::set_rate(fighter.module_accessor, walk_speed);
    }
    else if VarModule::is_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK) {
        VarModule::off_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_wait"), 0.0, 1.0, false, 0.0, false, false);
    }
    return false.into()
}
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_CATCH_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_grab_wait_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    VarModule::off_flag(fighter.object(), vars::snake::instance::IS_GRAB_WALK);
    original!(fighter)
}
////
 
////added new up-taunt and side-taunt
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_APPEAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_taunt_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_taunt_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_taunt_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
#[status_script(agent = "snake", status = FIGHTER_STATUS_KIND_APPEAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_taunt_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
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

//added down-taunt box walk and c4 place/explode
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_down_taunt_wait_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::reset_trigger(fighter.module_accessor);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_wait"), 0.0, 1.0, false, 0.0, false, false);
    fighter.off_flag(*FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
    let appeal_wait_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("appeal_wait_frame"));
    fighter.set_int(appeal_wait_frame, *FIGHTER_SNAKE_STATUS_APPEAL_WORK_INT_WAIT_COUNTER);
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    VarModule::off_flag(fighter.object(), vars::snake::instance::DTAUNT_C4_EXLPODE);
    VarModule::set_int(fighter.object(), vars::snake::instance::DTAUNT_GRENADE_WAIT_COUNT, 0);
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_down_taunt_wait_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_down_taunt_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_APPEAL_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_down_taunt_wait_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
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
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_APPEAL_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn snake_down_taunt_end_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if VarModule::is_flag(fighter.object(), vars::snake::instance::DTAUNT_C4_EXLPODE) {
        VarModule::off_flag(fighter.object(), vars::snake::instance::DTAUNT_C4_EXLPODE);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_end_explode"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_end"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.off_flag(*FIGHTER_SNAKE_STATUS_APPEAL_FLAG_EXIT);
    fighter.sub_shift_status_main(L2CValue::Ptr(snake_down_taunt_end_main_loop as *const () as _))
    // 0.into()
}
pub unsafe fn snake_down_taunt_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
#[status_script(agent = "snake", status = FIGHTER_SNAKE_STATUS_KIND_APPEAL_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn snake_down_taunt_end_status_end(fighter: &mut L2CFighterCommon) -> L2CValue {
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
////