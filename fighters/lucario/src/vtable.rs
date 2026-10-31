use super::*;
use interpolation::Lerp;

/// Exported for the non-vtable aura hooks that remain in `common`.
#[no_mangle]
pub unsafe extern "C" fn lucario_get_aura(object: *mut BattleObject) -> f32 {
    let module_accessor = (*object).module_accessor;
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        return 1.0;
    }

    let aura_override = VarModule::get_float(object, vars::lucario::status::AURA_OVERRIDE);
    if aura_override > 0.0 {
        // println!("aura_override: {}", aura_override);
        return aura_override;
    }

    if VarModule::is_flag(object, vars::lucario::instance::METER_BURNOUT) {
        let penalty_aurapower = ParamModule::get_float(object, ParamType::Agent, "aura.penalty_aurapower");
        // println!("penalty_aurapower: {}", penalty_aurapower);
        return penalty_aurapower;
    }

    let min_aurapower = ParamModule::get_float(object, ParamType::Agent, "aura.min_aurapower");
    let max_aurapower = ParamModule::get_float(object, ParamType::Agent, "aura.max_aurapower");
    let meter = MeterModule::meter(object) as f32;
    let meter_max = MeterModule::meter_cap(object) as f32 * MeterModule::meter_per_level(object);
    let aurapower = min_aurapower.lerp(&max_aurapower, &(meter / meter_max));

    let rounded_aurapower = (aurapower * 100.0).round() / 100.0; // 2 decimal places
    // println!("aurapower: {}", rounded_aurapower);
    return rounded_aurapower;
}

// FighterVtableSlot::OnDeathReset1
unsafe extern "C" fn lucario_on_death_reset_1(_vtable: u64, fighter: &mut Fighter) {
    let object = &mut fighter.battle_object;
    let module_accessor = object.module_accessor;
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let aura = lucario_get_aura(object);
    WorkModule::set_float(module_accessor, aura, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
}

// FighterVtableSlot::OnFrame9
unsafe extern "C" fn lucario_on_frame_9(_vtable: u64, fighter: &mut Fighter) {
    let object = &mut fighter.battle_object;
    let object_id = object.battle_object_id;
    let module_accessor = sv_battle_object::module_accessor(object_id);
    if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) > 7 {
        std::process::abort();
    }
    let aura = lucario_get_aura(object);
    WorkModule::set_float(module_accessor, aura, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
}

// FighterVtableSlot::OnFrame7
// Stubbed: this is where the game scales Lucario's aura effects.
unsafe extern "C" fn lucario_on_frame_7(_vtable: u64, _fighter: &mut Fighter) {}

// FighterVtableSlot::OnLinkEvent
// unsafe extern "C" fn lucario_on_grab(_vtable: u64, fighter: &mut Fighter, event: &mut LinkEvent) -> u64 {
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

pub fn install() {
    replace_fighter_vtable(
        *FIGHTER_KIND_LUCARIO,
        FighterVtableSlot::OnDeathReset1,
        lucario_on_death_reset_1 as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_LUCARIO,
        FighterVtableSlot::OnFrame9,
        lucario_on_frame_9 as *const () as _
    );
    replace_fighter_vtable(
        *FIGHTER_KIND_LUCARIO,
        FighterVtableSlot::OnFrame7,
        lucario_on_frame_7 as *const () as _
    );
    // replace_fighter_vtable(
    //     *FIGHTER_KIND_LUCARIO,
    //     FighterVtableSlot::OnLinkEvent,
    //     lucario_on_grab as *const () as _
    // );
}
