use super::*;

pub unsafe extern "C" fn special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let kinetic = KineticModule::get_kinetic_type(fighter.module_accessor);

    let original_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_INKLING_SPECIAL_N} else {*FIGHTER_STATUS_KIND_SPECIAL_N};
    let to_return = smashline::original_status(Main, fighter, original_status)(fighter);

    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        let lr = PostureModule::lr(fighter.module_accessor);
        KineticModule::change_kinetic(fighter.module_accessor, kinetic);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST_NO_CAP,
            speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            speed_y,
            0.0,
            0.0,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }

    to_return
}

pub unsafe extern "C" fn special_n_shoot_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_kirby = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY;
    let original_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_INKLING_SPECIAL_N_SHOOT} else {*FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_SHOOT};
    let toreturn = smashline::original_status(Main, fighter, original_status)(fighter);
    let current_count = WorkModule::get_int(fighter.module_accessor,*FIGHTER_INKLING_STATUS_SPECIAL_N_WORK_INT_BULLET_NUM);
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
    && current_count == 0 {
        WorkModule::set_int(fighter.module_accessor, -1,*FIGHTER_INKLING_STATUS_SPECIAL_N_WORK_INT_BULLET_NUM);
        let fighter_ptr = fighter.global_table[0x4].get_ptr() as *mut Fighter;
        let ink_cost = FighterSpecializer_Inkling::get_sub_ink_special_n(fighter_ptr);
        let can_max_shot = super::spend_ink(fighter,ink_cost);
        if can_max_shot {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_INKBULLET, false, -1);

            let new_status = if is_kirby {*FIGHTER_KIRBY_STATUS_KIND_INKLING_SPECIAL_N_END} else {*FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_END};
            fighter.change_status(new_status.into(),false.into());
            return 1.into()
        }
    }

    toreturn
}

pub fn install(agent: &mut Agent)  {
    agent.status(Main, *FIGHTER_INKLING_STATUS_KIND_SPECIAL_N_SHOOT, special_n_shoot_main);
}
