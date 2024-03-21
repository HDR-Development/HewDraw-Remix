use super::*;

mod special_hi;

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    VarModule::off_flag(fighter.object(), vars::shizue::instance::LLOID_ASYNC);
    VarModule::set_int(fighter.object(), vars::shizue::instance::LLOID_TIMER, 0);
}

pub fn install() {
    agent.on_start(on_start);

    special_hi::install();
}
