// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if (fighter.is_motion(Hash40::new("special_air_hi3")) && fighter.motion_frame() > 49.0)
    && !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor) {
        fighter.sub_air_check_dive();
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor) {
    fastfall_specials(fighter);
}

pub extern "C" fn miiswordsman_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		miiswordsman_frame(fighter)
    }
}

pub unsafe fn miiswordsman_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma);
    }
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, miiswordsman_frame_wrapper);
}