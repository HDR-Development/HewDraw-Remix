use super::*;
use globals::*;
utils::import_noreturn!(common::opff::fighter_common_opff);

use vars::daisy::instance::*;

unsafe fn wall_bounce(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status(*FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_JUMP) {
        let lr = PostureModule::lr(boma);
        let frame = MotionModule::frame(boma) as i32;
        let mut touch_wall = false;
        if lr > 0.0 {
            touch_wall = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT as u32);
        } else {
            touch_wall = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT as u32);
        };
        if touch_wall && (1..25).contains(&frame) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_S_HIT_END, true);
        }
    }
}

// transition daisy into unique animations for her third jump
unsafe fn triple_jump_motion(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    if fighter.is_motion_one_of(&[Hash40::new("jump_aerial_f"), Hash40::new("jump_aerial_b")])
    && boma.get_num_used_jumps() == boma.get_jump_count_max() {
        if fighter.is_motion(Hash40::new("jump_aerial_f")) {
            MotionModule::change_motion(boma, Hash40::new("jump_aerial_f2"), 0.0, 1.0, false, 0.0, false, false);
        } else {
            MotionModule::change_motion(boma, Hash40::new("jump_aerial_b2"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
}

unsafe fn crystal_handling(boma: &mut BattleObjectModuleAccessor) {
    if ArticleModule::is_exist(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR) {
        let article = ArticleModule::get_article(boma, *FIGHTER_DAISY_GENERATE_ARTICLE_KASSAR);
        let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
        let article_boma = sv_battle_object::module_accessor(article_id);
        // toggle the correct fist mesh depending on the direction daisy is facing
        if PostureModule::lr(boma) == 1.0 {
            ModelModule::set_mesh_visibility(article_boma, Hash40::new("daisy_glove"), false);
            ModelModule::set_mesh_visibility(article_boma, Hash40::new("daisy_gloveleft"), true);
        } else {
            ModelModule::set_mesh_visibility(article_boma, Hash40::new("daisy_glove"), true);
            ModelModule::set_mesh_visibility(article_boma, Hash40::new("daisy_gloveleft"), false);
        }
    }
}

// i wonder what you taste like
unsafe fn appeal_special(boma: &mut BattleObjectModuleAccessor) {
    // transitions to the special taunt if the button is released within 2 frames
    if boma.is_motion_one_of(&[Hash40::new("appeal_s_l"), Hash40::new("appeal_s_r")])
    && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L)
    && ControlModule::check_button_off(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R)
    && boma.motion_frame() as i32 == 3 {
        MotionModule::change_motion_inherit_frame(boma, Hash40::new("appeal_s_special"), -1.0, 1.0, 0.0, false, false);
    }
}

// unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
//     if !fighter.is_in_hitlag()
//     && !StatusModule::is_changing(fighter.module_accessor)
//     && fighter.is_status_one_of(&[
//         *FIGHTER_STATUS_KIND_SPECIAL_N,
//         *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT,
//         ]) 
//     && fighter.is_situation(*SITUATION_KIND_AIR) {
//         fighter.sub_air_check_dive();
//         if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
//             if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
//                 fighter.clear_lua_stack();
//                 lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
//                 let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

//                 fighter.clear_lua_stack();
//                 lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
//                 app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                
//                 fighter.clear_lua_stack();
//                 lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
//                 app::sv_kinetic_energy::enable(fighter.lua_state_agent);

//                 KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
//             }
//         }
//     }
// }

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    wall_bounce(boma);
    triple_jump_motion(fighter, boma);
    crystal_handling(boma);
    appeal_special(boma);
    //fastfall_specials(fighter);

    // FYI there's process_daisydaikon_knockback in function_hooks/killscreen.rs
}

pub unsafe extern "C" fn daisy_frame_wrapper(fighter: &mut L2CFighterCommon) {
    common::opff::fighter_common_opff(fighter);
    daisy_frame(fighter)
}

pub unsafe fn daisy_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, daisy_frame_wrapper);
}