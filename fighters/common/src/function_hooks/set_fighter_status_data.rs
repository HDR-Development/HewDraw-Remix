use super::*;
use globals::*;

const MAX_SEED: u32 = 12_000;

//=================================================================
//== FighterStatusModuleImpl::set_fighter_status_data
//=================================================================
#[skyline::hook(replace=FighterStatusModuleImpl::set_fighter_status_data)]
unsafe fn set_fighter_status_data_hook(boma: &mut BattleObjectModuleAccessor, arg2: bool, treaded_kind: i32, arg4: bool, arg5: bool, arg6: bool, log_mask_flag: u64, status_attr: u32, power_up_attack_bit: u32, arg10: u32) {
    let bo = boma.object();
    if (VarModule::has_var_module(bo)) {
        let bit_last = VarModule::get_int(bo, vars::common::instance::POWER_UP_BIT_LAST) as u32;
        if (power_up_attack_bit != bit_last) {
            GLOBAL_SEED += 1;
            let new_seed = GLOBAL_SEED % MAX_SEED;
            VarModule::set_int(boma.object(), vars::common::instance::ATTACK_LOG_SEED, new_seed as i32);
            //println!("Seed Update {} | BIT: {} \n", new_seed, power_up_attack_bit);
        }
        VarModule::set_int(boma.object(), vars::common::instance::POWER_UP_BIT_LAST, power_up_attack_bit as i32);
    }
    return original!()(boma, arg2, treaded_kind, arg4, arg5, arg6, log_mask_flag, status_attr, power_up_attack_bit, arg10);
}

pub fn install() {
    skyline::install_hooks!(set_fighter_status_data_hook);
}
