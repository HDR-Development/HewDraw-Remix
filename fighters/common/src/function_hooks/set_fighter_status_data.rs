use super::*;
use globals::*;

//=================================================================
//== FighterStatusModuleImpl::set_fighter_status_data
//=================================================================
// #[skyline::hook(replace=FighterStatusModuleImpl::set_fighter_status_data)]
// unsafe fn set_fighter_status_data_hook(boma: &mut BattleObjectModuleAccessor, arg2: bool, treaded_kind: i32, arg4: bool, arg5: bool, arg6: bool, log_mask_flag: u64, status_attr: u32, power_up_attack_bit: u32, arg10: u32) {
//     original!()(boma, arg2, treaded_kind, arg4, arg5, arg6, log_mask_flag, new_status_attr, power_up_attack_bit, arg10)
// }

pub fn install() {
    // skyline::install_hooks!(
    //     set_fighter_status_data_hook,
    // );
}