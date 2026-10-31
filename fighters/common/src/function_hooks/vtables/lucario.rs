use super::*;
use utils::ext::*;

extern "C" {
    #[link_name = "lucario_get_aura"]
    fn get_aura(object: *mut BattleObject) -> f32;
}

#[skyline::hook(offset = 0xc5c010)]
pub unsafe extern "C" fn lucario_check_aura(module_accessor: *mut BattleObjectModuleAccessor) -> f32 {
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let object = utils::util::get_battle_object_from_id((*module_accessor).battle_object_id);
    get_aura(object)
}

#[skyline::hook(offset = 0xc5be40)]
pub unsafe extern "C" fn lucario_check_aura2(module: u64) -> f32 {
    let module_accessor = &mut *(*((module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let object = utils::util::get_battle_object_from_id(module_accessor.battle_object_id);
    get_aura(object)
}

// conditionally disable the lucario aurapower multiplier for collision_attr_aura, so it only affects the knockback of his specials
#[skyline::from_offset(0x646180)]
extern "C" fn FUN_7100646180(module_accessor: *mut BattleObjectModuleAccessor, data: &mut smash_rs::app::AttackData, param_3: u32) -> f32;
#[skyline::hook(offset = 0xc5bac0)]
pub unsafe extern "C" fn disable_collision_attr_aura_power_mul(module_accessor: *mut BattleObjectModuleAccessor, data: &mut smash_rs::app::AttackData, param_3: u32) -> f32 {
    let boma = &mut *module_accessor;
    let aurapower_mul = if data.attr == smash_rs::phx::Hash40::new("collision_attr_aura") 
    || WorkModule::is_flag(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLAG_FORCE_AURAPOWER_ATTACK_POWER_MUL) {
        WorkModule::get_float(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER)
    } else {
        1.0
    };

    let other_mul = FUN_7100646180(module_accessor, data, param_3);
    if boma.is_status_one_of(&[
        // these statuses still use the old aurapower multiplier
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_STATUS_KIND_SPECIAL_HI,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_APPEAR,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_END,
        *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_SPLIT,
    ]) {
        return other_mul * aurapower_mul;
    }

    return other_mul;
}

pub fn install() {
    skyline::install_hooks!(
        lucario_check_aura,
        lucario_check_aura2,
        disable_collision_attr_aura_power_mul,
    );
}
