use super::*;
use globals::*;


// FIGHTER_STATUS_KIND_SPECIAL_S

#[status_script(agent = "duckhunt", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_s"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_s"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_INT_SPECIAL_S_SHOOT_TIMER);
    let disable_shoot_can_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("disable_shoot_can_frame"));
    WorkModule::set_int(fighter.module_accessor, disable_shoot_can_frame, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_INT_SPECIAL_S_DISABLE_SHOOT_CAN_FRAME);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_INT_SPECIAL_S_FRAME);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_RETICLE, Hash40::new("special_s"), true, -1.0);
    if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CLAY) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DUCKHUNT_INSTANCE_WORK_ID_FLAG_RELEASE_CLAY);
        //ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CLAY, false, -1);
    }
    // <HDR>
    if fighter.global_table[CMD_CAT3].get_i32() & *FIGHTER_PAD_CMD_CAT3_FLAG_SPECIAL_S_SMASH_DASH != 0 {
        VarModule::on_flag(fighter.battle_object, vars::duckhunt::status::CLAY_SMASH_INPUT);
    }
    // </HDR>
    fighter.main_shift(special_s_main_loop)
}

unsafe extern "C" fn special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    // <HDR>
    if fighter.global_table[CURRENT_FRAME].get_i32() == 6 {
        WorkModule::enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
        WorkModule::enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CLAY) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_CLAY, false, -1);
        }
    }
    // </HDR>
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_GROUND {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, false);
        }
        else {
            fighter.change_status_req(*FIGHTER_STATUS_KIND_WAIT, false);
        }
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
            }
            else {
                GroundModule::correct(fighter.module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    0.into()
}

#[status_script(agent = "duckhunt_clay", status = WEAPON_DUCKHUNT_CLAY_STATUS_KIND_FLY, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
pub unsafe fn clay_fly_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let duckhunt = utils::util::get_battle_object_from_id(owner_id);
    if VarModule::is_flag(duckhunt, vars::duckhunt::status::CLAY_SMASH_INPUT) {
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_DUCKHUNT_CLAY_INSTANCE_WORK_ID_FLAG_BY_SMASH);
    }
    original!(weapon)
}

pub fn install() {
    install_status_scripts!(
        special_s_main,
        clay_fly_init
    );
}