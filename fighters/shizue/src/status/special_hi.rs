use super::*;

#[status_script(agent = "shizue", status = FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn shizue_special_hi_detach_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boost = VarModule::is_flag(fighter.object(), vars::shizue::status::IS_DETACH_BOOST);
    let ret = original!(fighter);
    VarModule::set_flag(fighter.object(), vars::shizue::status::IS_DETACH_BOOST, boost);
    ret
}

pub fn install() {
    smashline::install_status_scripts!(
        shizue_special_hi_detach_pre
    );
}