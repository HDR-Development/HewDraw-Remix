use super::*;

pub fn install() {
    smashline::install_status_scripts!(
        peach_jump_aerial_pre,
        peach_jump_aerial_main
    );
}

// TAGS: DJC, Double Jump Cancel, Peach
// Reimplements peach's double jump to use FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION instead of FIGHTER_KINETIC_TYPE_JUMP_AERIAL
#[status_script(agent = "peach", status = FIGHTER_STATUS_KIND_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn peach_jump_aerial_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.status_pre_JumpAerial_sub().get_bool() {
        StatusModule::init_settings(
            fighter.module_accessor,
            app::SituationKind(*SITUATION_KIND_AIR),
            *FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION,
            *GROUND_CORRECT_KIND_AIR as u32,
            app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
            true,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
            *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
            0
        );
        FighterStatusModuleImpl::set_fighter_status_data(
            fighter.module_accessor,
            false,
            *FIGHTER_TREADED_KIND_ENABLE,
            true,
            false,
            true,
            0,
            *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
            0,
            0
        );
        false.into()
    } else {
        true.into()
    }
}

// TAGS: DJC, Double Jump Cancel, Peach
// Reimplemented to use the double jump animation as movement
#[status_script(agent = "peach", status = FIGHTER_STATUS_KIND_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn peach_jump_aerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.sub_jump_aerial_item_rocketbelt();
    fighter.status_JumpAerialSub(false.into(), false.into());
    fighter.main_shift(peach_jump_aerial_main_loop)
}

unsafe extern "C" fn peach_jump_aerial_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0 {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    } else {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
    }
    fighter.status_JumpAerial_Main()
}