use super::*;
use globals::*;
 
// FIGHTER_STATUS_KIND_ATTACK_AIR_LW_START

unsafe extern "C" fn attack_air_lw_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if pickel_attack_que(fighter).get_bool() {
        return 0.into();
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_ATTACK_FRAME);
    let mut generate = false;
    if ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_ATTACK_AIR_LW_FORBID_FRAME) <= 0
        || !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE) {
            generate = true;
        }
    }
    if !FighterSpecializer_Pickel::check_material_attack_air_lw_generate(fighter.module_accessor) {
        generate = false;
    }
    let mot = if generate {
        KineticModule::clear_speed_attr(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        Hash40::new("attack_air_lw")
    }
    else {
        Hash40::new("attack_air_lw_fail")
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        mot,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    WorkModule::set_flag(fighter.module_accessor, generate, *FIGHTER_PICKEL_STATUS_ATTACK_FLAG_IS_GENERATE_FORGE);
    if generate {
        let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        WorkModule::set_int(fighter.module_accessor, jump_count, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_JUMP_COUNT_BACKUP);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_LAMDING_RECOVER);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_FORGE_LANDING);
        fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_lw_start_main_loop as *const () as _))
    }
    else {
        fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_lw_fail_main_status_loop as *const () as _))
    }
}

unsafe extern "C" fn attack_air_lw_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
            fighter.change_status(FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_LOOP.into(), false.into());
            return 0.into();
        }
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 0.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_ATTACK_FLAG_FORGE_GENERATE_ENABLE) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_ATTACK_FLAG_FORGE_GENERATE_ENABLE);
        if FighterSpecializer_Pickel::check_material_attack_air_lw_generate(fighter.module_accessor) {
            let attack_air_lw_interval = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("attack_air_lw_interval"));
            WorkModule::set_int(fighter.module_accessor, attack_air_lw_interval, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_ATTACK_AIR_LW_FORBID_FRAME);
            let anvil_iron_count = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), 0x188e0b0db2) + 2;
            FighterSpecializer_Pickel::sub_material_num(fighter.module_accessor, *FIGHTER_PICKEL_MATERIAL_KIND_IRON, anvil_iron_count);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE, false, -1);
            if LinkModule::is_link(fighter.module_accessor, *FIGHTER_PICKEL_LINK_NO_FORGE) {
                LinkModule::send_event_parents(fighter.module_accessor, *FIGHTER_PICKEL_LINK_NO_FORGE, Hash40::new_raw(0x11d608f91f));
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    pickel_attack_catch_item(fighter);
    attack_air_lw_dead_area(fighter);
    0.into()
}

pub unsafe extern "C" fn pickel_attack_que(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if !FighterSpecializer_Pickel::is_status_kind_attack(prev_status) {
        fighter.sub_GetLightItemImm(L2CValue::Void());
        if StatusModule::status_kind_que_from_script(fighter.module_accessor) as i32 != *STATUS_KIND_NONE {
            return true.into();
        }
    }
    false.into()
}

pub unsafe extern "C" fn pickel_attack_catch_item(fighter: &mut L2CFighterCommon) {
    let catch_frame_param = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        hash40("item_catch_frame_attack_3")
    } else {
        hash40("item_air_catch_frame")
    };
    let catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), catch_frame_param);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_ATTACK_FRAME) <= catch_frame {
        fighter.sub_GetLightItemImm(L2CValue::Void());
    }
}

unsafe extern "C" fn attack_air_lw_dead_area(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
        ModelModule::joint_global_position(
            fighter.module_accessor,
            Hash40::new("hip"),
            pos,
            true
        );
        let check_dead = GroundUtility::check_dead_area(pos);
        if check_dead != *GROUND_DEAD_AREA_CHECK_RESULT_OUTSIDE_UP {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        }
    }
}

pub unsafe extern "C" fn attack_air_lw_fail_main_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.status_AttackAir_Main_common().get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOPPING].get_bool() {
            fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
        }
        0.into()
    } else {
        1.into()
    }
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_START, attack_air_lw_start_main);
}