use super::*;
use globals::*;

// FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A

pub unsafe extern "C" fn special_hi_a_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL && fighter.global_table[PREV_STATUS_KIND] == FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1SET) {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_WORK_INT_F);
            return 0.into()
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_FLAG1SET);
        }
    }
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_KOOPA_STATUS_SPECIAL_HI_WORK_INT_F);
    return 0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Exec, *FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, special_hi_a_exec);
}