extern "C" {
    #[link_name = "offsets_exec_command"]
    fn offsets_exec_command() -> usize;

    #[link_name = "offsets_get_command_flag_cat"]
    fn offsets_get_command_flag_cat() -> usize;

    #[link_name = "offsets_demon_on_link_capture_event"]
    fn offsets_demon_on_link_capture_event() -> usize;

    #[link_name = "offsets_force_linear_histun"]
    fn offsets_force_linear_histun() -> usize;

    #[link_name = "offsets_get_param_int_impl"]
    fn offsets_get_param_int_impl() -> usize;

    #[link_name = "offsets_get_param_float_impl"]
    fn offsets_get_param_float_impl() -> usize;

    #[link_name = "offsets_set_fighter_vtable"]
    fn offsets_set_fighter_vtable() -> usize;

    #[link_name = "offsets_set_weapon_vtable"]
    fn offsets_set_weapon_vtable() -> usize;

    #[link_name = "offsets_set_item_vtable"]
    fn offsets_set_item_vtable() -> usize;

    #[link_name = "offsets_get_battle_object_from_id"]
    fn offsets_get_battle_object_from_id() -> usize;

    #[link_name = "offsets_fighter_handle_damage"]
    fn offsets_fighter_handle_damage() -> usize;
}


pub fn exec_command() -> usize {
    unsafe {
        offsets_exec_command()
    }
}

pub fn get_command_flag_cat() -> usize {
    unsafe {
        offsets_get_command_flag_cat()
    }
}

pub fn demon_on_link_capture_event() -> usize {
    unsafe {
        offsets_demon_on_link_capture_event()
    }
}

pub fn force_linear_histun() -> usize {
    unsafe {
        offsets_force_linear_histun()
    }
}

pub fn get_param_int_impl() -> usize {
    unsafe {
        offsets_get_param_int_impl()
    }
}

pub fn get_param_float_impl() -> usize {
    unsafe {
        offsets_get_param_float_impl()
    }
}

pub fn set_fighter_vtable() -> usize {
    unsafe {
        offsets_set_fighter_vtable()
    }
}

pub fn set_weapon_vtable() -> usize {
    unsafe {
        offsets_set_weapon_vtable()
    }
}

pub fn set_item_vtable() -> usize {
    unsafe {
        offsets_set_item_vtable()
    }
}

pub fn get_battle_object_from_id() -> usize {
    unsafe {
        offsets_get_battle_object_from_id()
    }
}

pub fn fighter_handle_damage() -> usize {
    unsafe {
        offsets_fighter_handle_damage()
    }
}