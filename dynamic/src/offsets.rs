extern "C" {
    #[link_name = "offsets_exec_command"]
    fn offsets_exec_command() -> usize;

    #[link_name = "offsets_get_command_flag_cat"]
    fn offsets_get_command_flag_cat() -> usize;

    #[link_name = "offsets_demon_on_link_capture_event"]
    fn offsets_demon_on_link_capture_event() -> usize;

    #[link_name = "offsets_dolly_super_special_check"]
    fn offsets_dolly_super_special_check() -> usize;

    #[link_name = "offsets_dolly_super_special_check_param"]
    fn offsets_dolly_super_special_check_param() -> usize;

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

    #[link_name = "offsets_p_p_game_state"]
    fn offsets_p_p_game_state() -> usize;

    #[link_name = "offsets_map_controls"]
    fn offsets_map_controls() -> usize;

    #[link_name = "offsets_once_per_game_frame"]
    fn offsets_once_per_game_frame() -> usize;

    #[link_name = "offsets_on_rule_select"]
    fn offsets_on_rule_select() -> usize;

    #[link_name = "offsets_global_frame_counter"]
    fn offsets_global_frame_counter() -> usize;

    #[link_name = "offsets_get_match_mode"]
    fn offsets_get_match_mode() -> usize;

    #[link_name = "offsets_kill_zoom_regular"]
    fn offsets_kill_zoom_regular() -> usize;

    #[link_name = "offsets_kill_zoom_throw"]
    fn offsets_kill_zoom_throw() -> usize;

    #[link_name = "offsets_analog_trigger_l"]
    fn offsets_analog_trigger_l() -> usize;

    #[link_name = "offsets_analog_trigger_r"]
    fn offsets_analog_trigger_r() -> usize;
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

pub fn dolly_super_special_check() -> usize {
    unsafe {
        offsets_dolly_super_special_check()
    }
}

pub fn dolly_super_special_check_param() -> usize {
    unsafe {
        offsets_dolly_super_special_check_param()
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

pub fn p_p_game_state() -> usize {
    unsafe {
        offsets_p_p_game_state()
    }
}

pub fn map_controls() -> usize {
    unsafe {
        offsets_map_controls()
    }
}

pub fn once_per_game_frame() -> usize {
    unsafe {
        offsets_once_per_game_frame()
    }
}

pub fn on_rule_select() -> usize {
    unsafe {
        offsets_on_rule_select()
    }
}

pub fn global_frame_counter() -> usize {
    unsafe {
        offsets_global_frame_counter()
    }
}

pub fn get_match_mode() -> usize {
    unsafe {
        offsets_get_match_mode()
    }
}

pub fn kill_zoom_regular() -> usize {
    unsafe {
        offsets_kill_zoom_regular()
    }
}

pub fn kill_zoom_throw() -> usize {
    unsafe {
        offsets_kill_zoom_throw()
    }
}

pub fn analog_trigger_l() -> usize {
    unsafe {
        offsets_analog_trigger_l()
    }
}

pub fn analog_trigger_r() -> usize {
    unsafe {
        offsets_analog_trigger_r()
    }
}