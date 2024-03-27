use super::*;
use globals::*;

pub unsafe fn spend_ink(fighter: &mut L2CFighterCommon, ink_cost: f32) -> bool {
    let ink_current = WorkModule::get_float(fighter.module_accessor,*FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK);
    if (ink_current-ink_cost) < 0.0 {
        return false;
    }
    let fighter_ptr = fighter.global_table[0x4].get_ptr() as *mut Fighter;
    let ink_max = FighterSpecializer_Inkling::get_ink_max(fighter_ptr);
    let new_ink = (0.0 as f32).max(ink_current-ink_cost);
    WorkModule::set_float(fighter.module_accessor, new_ink, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS);
    
    if ink_max <= 0.0 {
        FighterSpecializer_Inkling::lack_ink(fighter_ptr);
    }
    if new_ink <= 0.0 {
        VisibilityModule::set_status_default(fighter.module_accessor, Hash40::new_raw(0x4ad12b739), Hash40::new_raw(0xa48dd021e));
        MotionModule::add_motion_partial(fighter.module_accessor, *FIGHTER_INKLING_MOTION_PART_SET_KIND_TANK, Hash40::new_raw(0xa48dd021e), 0.0, 1.0, true, false,0.0,true,true,false);
    }
    FighterSpecializer_Inkling::change_ink(fighter_ptr, new_ink);

    return true;
}

pub unsafe extern "C" fn special_n_shoot_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let original_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_INKLING_SPECIAL_N_SHOOT} else {*FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_SHOOT};
    let toreturn = smashline::original_status(Main, fighter, original_status)(fighter);
    let current_count = WorkModule::get_int(fighter.module_accessor,*FIGHTER_INKLING_STATUS_SPECIAL_N_WORK_INT_BULLET_NUM);
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && current_count == 0 {
        println!("Cancel!");
        WorkModule::set_int(fighter.module_accessor, MAX_BULLET_ID,*FIGHTER_INKLING_STATUS_SPECIAL_N_WORK_INT_BULLET_NUM);
        let fighter_ptr = fighter.global_table[0x4].get_ptr() as *mut Fighter;
        let ink_cost = FighterSpecializer_Inkling::get_sub_ink_special_n(fighter_ptr);
        let can_max_shot = spend_ink(fighter,ink_cost);
        if can_max_shot {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_INKBULLET, false, -1);

            let new_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_INKLING_SPECIAL_N_END} else {*FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_END};
            fighter.change_status(new_status.into(),false.into());
            return 1.into()
        }
    }

    toreturn
}

pub fn install() {
    Agent::new("kirby")
        .status(Main, *FIGHTER_KIRBY_STATUS_KIND_INKLING_SPECIAL_N_SHOOT, special_n_shoot_exec)
    .install();
}
