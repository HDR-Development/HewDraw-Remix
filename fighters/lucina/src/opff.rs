// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

// Fixes weird vanilla behavior where touching ground during upB puts you into special fall for 1f before landing
unsafe fn up_special_proper_landing(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI)
    && fighter.is_situation(*SITUATION_KIND_GROUND)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
    }
}

// Up Special Reverse
unsafe fn up_special_reverse(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    //Lucina frame 6
    let mut target_frame = 6.0;
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        if frame == target_frame {
            if stick_x * facing < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
            }
        }
    }
}

// lets lucina toggle her mask on/off with down taunt
unsafe fn mask_toggle(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, frame: f32) {
    let mask_is_equipped = VarModule::is_flag(boma.object(), vars::lucina::instance::APPEAL_EQUIP_MASK);
    let mask_is_exist = ArticleModule::is_exist(boma, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK);
    
    if fighter.is_motion_one_of(&[Hash40::new("appeal_lw_l"), Hash40::new("appeal_lw_r")])
    && frame as i32 == 12 {
        if mask_is_equipped { // take off mask
            VarModule::off_flag(boma.object(), vars::lucina::instance::APPEAL_EQUIP_MASK);
        } else { // put on mask
            VarModule::on_flag(boma.object(), vars::lucina::instance::APPEAL_EQUIP_MASK);
        }
    } else {
        if mask_is_equipped && !mask_is_exist {
            ArticleModule::generate_article(boma, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, false, -1);

            let article = ArticleModule::get_article(boma, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK);
            let article_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
            let article_boma = sv_battle_object::module_accessor(article_id);
            MotionModule::change_motion(article_boma, Hash40::new("appeal_lw"), 0.0, 1.0, false, 0.0, false, false);
            MotionModule::set_frame(article_boma, 50.0, true);
            MotionModule::set_rate(article_boma, 0.0);
        } 
        else if !mask_is_equipped && mask_is_exist
        && !fighter.is_motion_one_of(&[Hash40::new("entry_l"), Hash40::new("entry_r")]) { // ignore during entry animation
            ArticleModule::remove_exist(boma, *FIGHTER_LUCINA_GENERATE_ARTICLE_MASK, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }

    // remove mask on entry and death
    if fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_ENTRY, *FIGHTER_STATUS_KIND_DEAD])
    && mask_is_equipped {
        VarModule::off_flag(boma.object(), vars::lucina::instance::APPEAL_EQUIP_MASK);
    }
}

// symbol-based call for the fe characters' common opff
extern "Rust" {
    fn fe_common(fighter: &mut smash::lua2cpp::L2CFighterCommon);
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP,
        *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX
        ]) 
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
    }
}

unsafe fn sword_length(boma: &mut BattleObjectModuleAccessor) {
	let long_sword_scale = Vector3f{x: 1.0, y: 1.0, z: 1.05};
	ModelModule::set_joint_scale(boma, smash::phx::Hash40::new("sword1"), &long_sword_scale);
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    // Magic Series
    //side_special_cancels(boma, status_kind, situation_kind, cat[0], motion_kind);
    up_special_proper_landing(fighter);
    mask_toggle(fighter, boma, frame);
    fastfall_specials(fighter);
    up_special_reverse(boma, status_kind, stick_x, facing, frame);
    sword_length(boma);
}

pub extern "C" fn lucina_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		lucina_frame(fighter);
    }
}

pub unsafe fn lucina_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, lucina_frame_wrapper);
}
