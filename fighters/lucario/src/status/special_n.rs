use super::*;
use globals::*;
// status script import

pub fn install() {
    install_status_scripts!(
        special_n_shoot_pre,
        auraball_shoot_pre,
        special_n_main,
        lucario_special_n_hold_main,
    );
}

#[status_script(agent = "lucario", status = FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn special_n_shoot_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.battle_object, vars::lucario::instance::IS_POWERED_UP);
    original!(fighter)
}

#[status_script(agent = "lucario_auraball", status = WEAPON_LUCARIO_AURABALL_STATUS_KIND_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn auraball_shoot_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    VarModule::set_flag(fighter.battle_object, vars::lucario::instance::IS_POWERED_UP, VarModule::is_flag(owner_module_accessor.object(), vars::lucario::instance::IS_POWERED_UP));
    println!("lucario_auraball is_powered_up: {}", VarModule::is_flag(fighter.battle_object, vars::lucario::instance::IS_POWERED_UP));
    original!(fighter)
}

#[status_script(agent = "lucario", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_start") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_start") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    special_n_set_kinetic(fighter);
    let max_charge_frame = fighter.get_param_float("param_special_n", "max_charge_frame");
    let curr_charge_frame = fighter.get_int(*FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AURABALL_CHARGE_FRAME) as f32;
    if curr_charge_frame >= max_charge_frame {
        fighter.on_flag(*FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_FLAG_CHARGE_MAX);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_HOLD);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_SHOOT);
    lucario_special_n_joint_translate(fighter);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_main_loop as *const () as _))
}

unsafe extern "C" fn special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if !StopModule::is_stop(fighter.module_accessor)
        && fighter.is_button_trigger(Buttons::Special) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_SHOOT);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_HOLD);
        }
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            special_n_set_kinetic(fighter);
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        // ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, false, -1);
        ControlModule::clear_command(fighter.module_accessor, true);
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_HOLD) {
            fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD.into(), false.into());
            return 0.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LUCARIO_AURABALL_TRANSITION_TERM_ID_START_SHOOT) {
            fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
            return 0.into();
        }
        return 0.into();
    }
    0.into()
}

unsafe extern "C" fn special_n_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
            MotionModule::change_motion_inherit_frame_keep_rate(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                -1.0,
                1.0,
                0.0
            );
        }
        else {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
            MotionModule::change_motion_inherit_frame_keep_rate(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                -1.0,
                1.0,
                0.0
            );
        }
        else {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
}

pub unsafe extern "C" fn lucario_special_n_joint_translate(fighter: &mut L2CFighterCommon) {
    let havel = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let haver = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_position(
        fighter.module_accessor,
        Hash40::new("havel"),
        havel,
        true
    );
    ModelModule::joint_global_position(
        fighter.module_accessor,
        Hash40::new("haver"),
        haver,
        true
    );
    let pos = Vector3f{x: havel.x + haver.x, y: havel.y + haver.y, z: havel.z + haver.z};
    let new_pos = Vector3f{x: pos.x * 0.5, y: pos.y * 0.5, z: pos.z * 0.5};
    ModelModule::set_joint_translate(
        fighter.module_accessor,
        Hash40::new("throw"),
        &new_pos,
        true,
        false
    );
}

#[status_script(agent = "lucario", status = FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucario_special_n_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_hold") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_hold") as i64, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
    ArticleModule::change_status(
        fighter.module_accessor,
        *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL,
        *WEAPON_LUCARIO_AURABALL_STATUS_KIND_CHARGE,
        ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)
    );
    lucario_special_n_hold_set_kinetic(fighter);
    ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_special_n_hold_main_loop as *const () as _))
}

unsafe extern "C" fn lucario_special_n_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        lucario_special_n_hold_set_kinetic(fighter);
        return 0.into();
    }
    
    if fighter.is_button_trigger(Buttons::Special) {
        fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        return 0.into();
    }

    if fighter.check_jump_cancel(false) {
        return 0.into();
    }

    if lucario_special_n_hold_check_cancel(fighter).get_bool() {
        fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX.into(), false.into());
        return 0.into();
    }

    0.into()
}

unsafe extern "C" fn lucario_special_n_hold_check_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        if fighter.is_button_on(Buttons::Guard) {
            fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            return true.into();
        }
    }
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        if fighter.is_cat_flag(Cat2::StickEscape) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE) {
                fighter.set_int(*FIGHTER_STATUS_KIND_ESCAPE, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            } else {
                fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
            return true.into();
        }
        if fighter.is_cat_flag(Cat2::StickEscapeF) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F) {
                fighter.set_int(*FIGHTER_STATUS_KIND_ESCAPE_F, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            } else {
                fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
            return true.into();
        }
        if fighter.is_cat_flag(Cat2::StickEscapeB) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B) {
                fighter.set_int(*FIGHTER_STATUS_KIND_ESCAPE_B, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            } else {
                fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
            return true.into();
        }
        if fighter.sub_check_command_guard().get_bool() {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON) {
                fighter.set_int(*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            } else {
                fighter.set_int(*STATUS_KIND_NONE, *FIGHTER_LUCARIO_SPECIAL_N_STATUS_WORK_ID_INT_CANCEL_STATUS);
            }
            return true.into();
        }
    }
    return false.into();
}

unsafe extern "C" fn lucario_special_n_hold_set_kinetic(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_AIR_MOT);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
            MotionModule::change_motion_inherit_frame_keep_rate(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                -1.0,
                1.0,
                0.0
            );
        }
        else {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    }
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_GROUND_MOT);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT) {
            MotionModule::change_motion_inherit_frame_keep_rate(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                -1.0,
                1.0,
                0.0
            );
        }
        else {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_MOT_INHERIT);
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
}