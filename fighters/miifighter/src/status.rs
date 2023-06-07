use super::*;
use globals::*;

//Forces Grounded Earthquake punch on the ground
#[status_script(agent = "miifighter", status = FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_GROUND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn special_lw1_ground_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = original!(fighter);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        StatusModule::set_situation_kind(fighter.module_accessor, app::SituationKind(*SITUATION_KIND_GROUND), false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    ret
}
pub fn install() {
    smashline::install_status_scripts!(
        special_lw1_ground_main
    );
}