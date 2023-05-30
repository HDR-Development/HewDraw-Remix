use super::*;
use utils::ext::*;
use std::arch::asm;

#[skyline::hook(offset = 0xc5bff0)]
pub unsafe extern "C" fn lucario_check_aura(module_accessor: *mut BattleObjectModuleAccessor) -> f32 {
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let object = utils::util::get_battle_object_from_id((*module_accessor).battle_object_id);
    get_aura(object)
}

#[skyline::hook(offset = 0xc5be20)]
pub unsafe extern "C" fn lucario_check_aura2(module: u64) -> f32 {
    let module_accessor = &mut *(*((module as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let object = utils::util::get_battle_object_from_id(module_accessor.battle_object_id);
    get_aura(object)
}

#[skyline::hook(offset = 0xc5e530)]
pub unsafe extern "C" fn lucario_handle_aura(_vtable: u64, fighter: &mut Fighter) {
    let object = &mut fighter.battle_object;
    let module_accessor = object.module_accessor;
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let aura = get_aura(object);
    WorkModule::set_float(module_accessor, aura, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
}

#[skyline::hook(offset = 0xc5e6d0)]
pub unsafe extern "C" fn lucario_handle_aura2(_vtable: u64, fighter: &mut Fighter) {
    let object = &mut fighter.battle_object;
    let object_id = object.battle_object_id;
    let module_accessor = sv_battle_object::module_accessor(object_id);
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let aura = get_aura(object);
    WorkModule::set_float(module_accessor, aura, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
}

// #[skyline::hook(offset = 0xc5d580)]
// pub unsafe extern "C" fn lucario_on_grab(_vtable: u64, fighter: &mut Fighter, event: &mut LinkEvent) -> u64 {
//     // param_3 + 0x10
//     if event.link_event_kind.0 == hash40("capture") {
//         let capture_event : &mut LinkEventCapture = std::mem::transmute(event);
//         let module_accessor = fighter.battle_object.module_accessor;
//         if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S {
//             // param_3 + 0x30
//             capture_event.node = Hash40::new("throw");
//             // param_3[0x28]
//             capture_event.result = true;
//             let offset = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET);
//             // param_3 + 0x44
//             capture_event.motion_offset = offset;
//             let offset_lw = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CATCH_MOTION_OFFSET_LW);
//             // param_3 + 0x48
//             capture_event.motion_offset_lw = offset_lw;
//             StatusModule::change_status_request(module_accessor, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_S_THROW, false);
//             return 0;
//         }
//     }
//     1
// }

unsafe extern "C" fn get_aura(object: *mut BattleObject) -> f32 {
    let module_accessor = (*object).module_accessor;
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        return 1.0;
    }
    if VarModule::is_flag(object, vars::lucario::instance::METER_IS_BURNOUT) {
        return ParamModule::get_float(object, ParamType::Agent, "aura.penalty_aurapower");
    }

    let min_aurapower = ParamModule::get_float(object, ParamType::Agent, "aura.min_aurapower");
    let max_aurapower = ParamModule::get_float(object, ParamType::Agent, "aura.max_aurapower");

    let charge = MeterModule::level(object) as f32;
    let max_charge = (ParamModule::get_float(object, ParamType::Common, "meter_max_damage") / MeterModule::meter_per_level(object)) as f32;

    let diff = max_aurapower - min_aurapower;
    let aura_power = min_aurapower + (diff * charge.clamp(0.0, max_charge) / max_charge);
    return aura_power;
}

#[skyline::hook(offset = 0xc5ce20)]
pub unsafe extern "C" fn lucario_set_effect_scale(vtable: u64, fighter: &mut Fighter) {
    original!()(vtable, fighter);
    let object = &mut fighter.battle_object;
    let module_accessor = object.module_accessor;
    let effect = WorkModule::get_int64(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_KIND);
    if effect != hash40("null") && VarModule::is_flag(object, vars::lucario::instance::METER_IS_BURNOUT) {
        let left = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_LHADOU) as u32;
        let right = WorkModule::get_int(module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_INT_EF_ID_RHADOU) as u32;
        let scale = 0.0;
        if left != 0 {
            EffectModule::set_scale(module_accessor, left, &smash::phx::Vector3f{x: scale, y: scale, z: scale});
        }
        if right != 0 {
            EffectModule::set_scale(module_accessor, right, &smash::phx::Vector3f{x: scale, y: scale, z: scale});
        }
    }
}

pub fn install() {
    skyline::install_hooks!(
        lucario_check_aura,
        lucario_check_aura2,
        lucario_handle_aura,
        lucario_handle_aura2,
        // lucario_on_grab,
        lucario_set_effect_scale
    );
}