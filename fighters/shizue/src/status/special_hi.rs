use super::*;

unsafe extern "C" fn shizue_special_hi_detach_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boost = VarModule::is_flag(fighter.object(), vars::shizue::status::IS_DETACH_BOOST);
    let ret = smashline::original_status(Pre, fighter, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH)(fighter);
    VarModule::set_flag(fighter.object(), vars::shizue::status::IS_DETACH_BOOST, boost);
    ret
}

pub fn install() {
    smashline::Agent::new("shizue")
        .status(
            Pre,
            *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_HI_DETACH,
            shizue_special_hi_detach_pre,
        )
        .install();
}
