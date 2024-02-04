use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_HI


pub unsafe extern "C" fn exec_special_hi(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_YACL_DEFAULT);
        fighter.select_cliff_hangdata_from_name("special_hi_slipoff");
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    return 0.into()
}


pub fn install() {
    smashline::Agent::new("donkey")
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_HI, exec_special_hi)
        .install();
}
