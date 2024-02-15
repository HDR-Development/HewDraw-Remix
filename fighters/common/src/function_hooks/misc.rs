use super::*;

/// This is an arbitrary type but it is used as a LinkEvent and is passed to LinkEvent listeners on ID 0x58
#[repr(C)]
struct LinkEventRebelGaugeUpdate {
    vtable: *const u64,
    id: u32,
    unk_id: u32,
    percentage: f32
}

unsafe fn handle_max_rebel_gauge(boma: &mut app::BattleObjectModuleAccessor) {
    boma.set_float(100.0, 0x4D); // FIGHTER_JACK_INSTANCE_WORK_ID_FLOAT_REBEL_GAUGE
    if boma.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_STATUS) {
        return;
    }

    if boma.is_flag(*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST) {
        return;
    }

    // boma.on_work_flag(0x200000e3); // FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_SUMMON

    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_WAIT,
        *FIGHTER_STATUS_KIND_FALL,
        *FIGHTER_STATUS_KIND_FALL_AERIAL
    ]) {
        // let status_to_change_to = FighterSpecializer_Jack::check_doyle_summon_dispatch(boma, true, false);
        // StatusModule::change_status_request(boma, status_to_change_to, false);
    } else {
        // let wait_frame = boma.work_param_int("param_private", "doyle_summon_wait_frame");
        // boma.set_work_int(wait_frame, 0x100000be); // FIGHTER_JACK_INSTANCE_WORK_ID_INT_DOYLE_SUMMON_DISPATCH_WAIT_FRAME
    }
}

// TODO: Get this from an offset search
#[skyline::from_offset(0x37addc0)]
fn kill_dead_event_listeners(arg: *mut u32);

unsafe fn send_rebel_gauge_event(entry_id: i32, gauge: f32) {
    let fighter_manager = utils::singletons::FighterManager();
    let fighter_entry = app::lua_bind::FighterManager::get_fighter_entry(fighter_manager, app::FighterEntryID(entry_id));

    if !*(fighter_entry as *const bool).add(0x41E8) {
        return;
    }

    let event = LinkEventRebelGaugeUpdate {
        vtable: 0 as _, // praying this doesn't actually get referenced
        id: 0x58,
        unk_id: *(fighter_entry as *const u32).add(0x41E4 / 0x4),
        percentage: gauge / 100.0
    };

    let event_listener_info = *(fighter_manager as *mut *mut u32);
    let event_listener_count = *event_listener_info;
    if event_listener_count <= 0x58 {
        return;
    }

    *event_listener_info.add(6) += 1;

    let linked_list_start = *(event_listener_info as *const u64).add(1) + 0x848;
    let mut linked_list_next = *(linked_list_start as *const u64);
    while linked_list_next != linked_list_start {
        let listener = *(linked_list_next as *const u64).add(2);
        if listener != 0 {
            let callable: extern "C" fn(u64, *const LinkEventRebelGaugeUpdate) = std::mem::transmute(*((*(listener as *const u64) + 0x18) as *const u64));
            callable(listener, &event);
        }
        linked_list_next = *(linked_list_next as *const u64);
    }
    *event_listener_info.add(6) -= 1;
    if *event_listener_info.add(6) == 0 && *event_listener_info.add(7) != 0 {
        kill_dead_event_listeners(event_listener_info);
    }
}

/// Replaces add_rebel_gauge by a function which still adds to the rebel gauge but doesn't trigger arsene
#[skyline::hook(replace = app::FighterSpecializer_Jack::add_rebel_gauge)]
pub unsafe fn add_rebel_gauge(boma: &mut app::BattleObjectModuleAccessor, entry_id: i32, amount: f32) {
    if !boma.is_flag(0x200000e9) { // FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_ADD_REBEL_GAUGE
        return;
    }

    if boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_STANDBY]) {
        return;
    }

    if boma.is_flag(*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST) {
        let customize = boma.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_CUSTOMIZE_SPECIAL_N_NO);
        if customize != 0 {
            return;
        }
    }

    if boma.is_flag(0x200000e7) { // FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_SUSPEND
        return
    }

    if boma.is_flag(0x200000e3) { // FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_SUMMON
        return;
    }

    if boma.is_flag(*FIGHTER_INSTANCE_WORK_ID_FLAG_KNOCKOUT) {
        return;
    }

    let current_gauge = boma.get_float(0x4D); // FIGHTER_JACK_INSTANCE_WORK_ID_FLOAT_REBEL_GAUGE
    let new_gauge = current_gauge + amount;
    if new_gauge >= 100.0 {
        handle_max_rebel_gauge(boma);
    } else {
        boma.set_float(new_gauge.max(0.0), 0x4D); // FIGHTER_JACK_INSTANCE_WORK_ID_FLOAT_REBEL_GAUGE
    }

    send_rebel_gauge_event(entry_id, new_gauge.min(100.0).max(0.0));
}

pub fn install() {
    skyline::install_hook!(add_rebel_gauge);
}