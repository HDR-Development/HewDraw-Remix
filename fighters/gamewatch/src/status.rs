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



pub fn install() {
    // smashline::install_agent_init_callbacks!(gamewatch_init);
    landing_attack_air::install();
    install_status_scripts!(
        
    );
}