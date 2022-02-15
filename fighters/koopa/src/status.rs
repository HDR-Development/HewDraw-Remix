use super::*;
use globals::*;
// status script import
 
pub fn install() {
    install_status_scripts!(
        exec_special_hi_a
    );
}

// FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A

#[status_script(agent = "koopa", status = FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_A, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
pub unsafe fn exec_special_hi_a(fighter: &mut L2CFighterCommon) -> L2CValue {
    if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL && fighter.global_table[PREV_STATUS_KIND] == FIGHTER_KOOPA_STATUS_KIND_SPECIAL_HI_G {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        GroundModule::set_cliff_check(fighter.module_accessor, app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
    }
    if GroundModule::can_entry_cliff(fighter.module_accessor) == 1 && KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_FALL && fighter.global_table[STICK_Y].get_f32() > -0.66 {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE),
            L2CValue::Bool(false)
        );
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