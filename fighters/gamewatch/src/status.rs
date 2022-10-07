use super::*;
use globals::*;

mod landing_attack_air;

// #[smashline::fighter_init]
// fn gamewatch_init(fighter: &mut L2CFighterCommon) {
//     unsafe {
//         // set the callbacks on fighter init
//         if fighter.kind() == *FIGHTER_KIND_GANON {
//             fighter.global_table[globals::USE_SPECIAL_N_CALLBACK].assign(&L2CValue::Ptr(should_use_special_n_callback as *const () as _));
//             fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
//         }
//     }
// }

#[status_script(agent = "gamewatch", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn landing_attack_air_pre(fighter: L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_LANDING_ATTACK_AIR_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        true,
        *FIGHTER_TREADED_KIND_ENABLE,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK as u64,
        *FIGHTER_STATUS_ATTR_INTO_DOOR as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_AIR as u32,
        0
    );

    0.into()
}

pub fn install() {
    // smashline::install_agent_init_callbacks!(gamewatch_init);
    landing_attack_air::install();
    install_status_scripts!(
        landing_attack_air_pre,
    );
}