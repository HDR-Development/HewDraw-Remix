use super::*;

// FIGHTER_STATUS_KIND_SPECIAL_HI

#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
pub unsafe fn exec_special_hi(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_HI_FLAG_YACL_DEFAULT);
    }
    if GroundModule::can_entry_cliff(fighter.module_accessor) == 1 && KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_FALL && fighter.global_table[STICK_Y].get_f32() > -0.66 {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE),
            L2CValue::Bool(false)
        );
        return 1.into()
    }
    return 0.into()
}

pub fn install() {
    install_status_scripts!(
        exec_special_hi
    );
}