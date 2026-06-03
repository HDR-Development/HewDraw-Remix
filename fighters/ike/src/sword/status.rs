use super::*;

unsafe extern "C" fn special_hi_2_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // let correct = if PostureModule::scale(weapon.module_accessor) > 1.0 {
    //     *GROUND_CORRECT_KIND_NONE
    // }
    // else {
    //     *GROUND_CORRECT_KIND_AIR
    // };
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *WEAPON_KINETIC_TYPE_IKE_SWORD_SPECIAL_HI,
        // correct as u32,
        *GROUND_CORRECT_KIND_NONE as u32,
        GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *WEAPON_IKE_SWORD_STATUS_KIND_SPECIAL_HI_2, special_hi_2_pre);
}