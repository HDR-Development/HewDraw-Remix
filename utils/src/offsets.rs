#[cfg(feature = "no-offset-search")]
mod offsets_impl {
    // These offsets are hardcoded to increase runtime speed but are only viable for smash version 13.0.1
    #[export_name = "offsets_exec_command"]
    pub const fn exec_command() -> usize {
        0x6babf0
    }

    #[export_name = "offsets_get_command_flag_cat"]
    pub const fn get_command_flag_cat() -> usize {
        0x6ba980
    }

    #[export_name = "offsets_demon_on_link_capture_event"]
    pub const fn demon_on_link_capture_event() -> usize {
        0x9337e0
    }

    #[export_name = "offsets_dolly_super_special_check"]
    pub const fn dolly_super_special_check() -> usize {
        0x970fd0
    }

    #[export_name = "offsets_dolly_super_special_check_param"]
    pub const fn dolly_super_special_check_param() -> usize {
        0x971230
    }

    #[export_name = "offsets_force_linear_histun"]
    pub const fn force_linear_histun() -> usize {
        0x62ba54
    }

    #[export_name = "offsets_get_param_int_impl"]
    pub const fn get_param_int_impl() -> usize {
        0x4e5380
    }

    #[export_name = "offsets_get_param_float_impl"]
    pub const fn get_param_float_impl() -> usize {
        0x4e53C0
    }

    #[export_name = "offsets_set_fighter_vtable"]
    pub const fn set_fighter_vtable() -> usize {
        0x14f4784
    }

    #[export_name = "offsets_set_weapon_vtable"]
    pub const fn set_weapon_vtable() -> usize {
        0x14f4c9c
    }

    #[export_name = "offsets_set_item_vtable"]
    pub const fn set_item_vtable() -> usize {
        0x14f4f34
    }

    #[export_name = "offsets_get_battle_object_from_id"]
    pub const fn get_battle_object_from_id() -> usize {
        0x3ac540
    }

    #[export_name = "offsets_fighter_handle_damage"]
    pub const fn fighter_handle_damage() -> usize {
        0x6310a0
    }

    #[export_name = "offsets_p_p_game_state"]
    pub const fn p_p_game_state() -> usize {
        0x52c1760
    }

    #[export_name = "offsets_map_controls"]
    pub const fn map_controls() -> usize {
        0x17504a0
    }

    #[export_name = "offsets_once_per_game_frame"]
    pub const fn once_per_game_frame() -> usize {
        0x135b7f0
    }

    #[export_name = "offsets_on_rule_select"]
    pub const fn on_rule_select() -> usize {
        0x1792190
    }

    #[export_name = "offsets_global_frame_counter"]
    pub const fn global_frame_counter() -> usize {
        0x52e6b44
    }

    #[export_name = "offsets_get_match_mode"]
    pub const fn get_match_mode() -> usize {
        0x1742da0
    }
  
    #[export_name = "offsets_kill_zoom_regular"]
    pub const fn kill_zoom_regular() -> usize {
        0x633dc0
    }

    #[export_name = "offsets_kill_zoom_throw"]
    pub const fn kill_zoom_throw() -> usize {
        0x637384
    }

    #[export_name = "offsets_analog_trigger_l"]
    pub const fn analog_trigger_l() -> usize {
        0x3665e60
    }

    #[export_name = "offsets_analog_trigger_r"]
    pub const fn analog_trigger_r() -> usize {
        0x3665e74
    }
}

#[cfg(not(feature = "no-offset-search"))]
mod offsets_impl {
    use utils_dyn::util::offset_to_addr;

    use crate::util::byte_search;

    struct CoreOffsets {
        pub exec_command: usize,
        pub get_command_flag_cat: usize,
        pub demon_on_link_capture_event: usize,
        pub force_linear_histun: usize,
        pub get_param_int_impl: usize,
        pub get_param_float_impl: usize,
        pub set_fighter_vtable: usize,
        pub set_weapon_vtable: usize,
        pub set_item_vtable: usize,
        pub get_battle_object_from_id: usize,
        pub fighter_handle_damage: usize,
        pub p_p_game_state: usize,
        pub map_controls: usize,
        pub once_per_game_frame: usize,
        pub on_rule_select: usize,
        pub global_frame_counter: usize,
        pub get_match_mode: usize,
        pub kill_zoom_regular: usize,
        pub kill_zoom_throw: usize,
        pub analog_trigger_l: usize,
        pub analog_trigger_r: usize,
    }

    static EXEC_COMMAND_SEARCH_CODE: &[u8] = &[
        0x08, 0x88, 0x40, 0xf9, // ldr  x8, [x0, #0x110]
        0x13, 0x51, 0x51, 0x29, // ldp  w19, w20, [x8, #0x88]
        0x09, 0x7d, 0x40, 0xb9, // ldr  w9, [x8, #0x7c]
        0x00, 0x05, 0xc0, 0x3d, // ldr  q0, [x8, #0x10]
        0x1a, 0xd1, 0x42, 0x39, // ldrb w26, [x8, #0xb4]
    ];

    const EXEC_COMMAND_OFFSET_FROM_START: usize = 0x30;

    static GET_COMMAND_FLAG_CAT_SEARCH_CODE: &[u8] = &[
        0x09, 0x05, 0x80, 0x52, // mov    w9, #0x28
        0x29, 0x00, 0x29, 0x9b, // smaddl x9, w1, w9, x0
        0x29, 0xb9, 0x42, 0xf9, // ldr    [x9, #0x570]
    ];

    static DEMON_ON_LINK_CAPTURE_EVENT_SEARCH_CODE: &[u8] = &[
        0xc9, 0x50, 0x8c, 0xd2, // mov  x9, #0x6286
        0x49, 0x9c, 0xb0, 0xf2, // movk x9, #0x84e2, LSL #16
        0x49, 0x01, 0xc0, 0xf2, // movk x9, #0xa, LSL #32
        0x55, 0x23, 0x40, 0xf9, // ldr  x21, [x26, #0x40]
    ];

    const DEMON_ON_LINK_CAPTURE_EVENT_OFFSET_FROM_START: usize = 0x24;

    static FORCE_LINEAR_HISTUN_SEARCH_CODE: &[u8] = &[
        0x88, 0x02, 0x40, 0xf9, // ldr  x8, [x20]
        0x08, 0x85, 0x40, 0xf9, // ldr  x8, [x8, #0x108]
        0x35, 0x11, 0x80, 0x52, // mov  w21, #0x89
        0x15, 0x00, 0xa4, 0x72, // movk w21, #0x2000, LSL #16
    ];

    const FORCE_LINEAR_HISTUN_OFFSET_TO_START: usize = 0x28;

    static GET_PARAM_INT_IMPL_SEARCH_CODE: &[u8] = &[
        // WorkModule::GetParamIntImpl
        0x00, 0x1c, 0x40, 0xf9, // ldr x0, [x0, #0x38]
        0x08, 0x00, 0x40, 0xf9, // ldr x8, [x0]
        0x03, 0x11, 0x40, 0xf9, // ldr x3, [x8, #0x20]
        0x60, 0x00, 0x1f, 0xd6, // br  x3
        // WorkModule::GetParamInt64
        0x08, 0x00, 0x40, 0xf9, // ldr x8, [x0]
        0x03, 0x31, 0x41, 0xf9, // ldr x3, [x8, #0x260]
        0x60, 0x00, 0x1f, 0xd6, // br  x3
    ];

    static GET_PARAM_FLOAT_IMPL_SEARCH_CODE: &[u8] = &[
        // WorkModule::GetParamFloatImpl
        0x00, 0x1c, 0x40, 0xf9, // ldr x0, [x0, #0x38]
        0x08, 0x00, 0x40, 0xf9, // ldr x8, [x0]
        0x03, 0x19, 0x40, 0xf9, // ldr x3, [x8, #0x30]
        0x60, 0x00, 0x1f, 0xd6, // br  x3
        // WorkModule::SetCustomizeNo
        0x00, 0x1c, 0x40, 0xf9, // ldr x0, [x0, #0x38]
        0x08, 0x00, 0x40, 0xf9, // ldr x8, [x0]
        0x03, 0x1d, 0x40, 0xf9, // ldr x3, [x8, #0x38]
        0x60, 0x00, 0x1f, 0xd6, // br  x3
    ];

    static SET_FIGHTER_VTABLE_SEARCH_CODE: &[u8] = &[
        0xe0, 0x03, 0x14, 0xaa, // mov  x0, x20
        0x3f, 0xc3, 0x00, 0x79, // strh wzr, [x25, #0x60]
        0x3f, 0x8b, 0x01, 0x39, // strb wzr, [x25, #0x62]
    ];

    const SET_FIGHTER_VTABLE_OFFSET_TO_START: usize = 0xC;

    static SET_WEAPON_VTABLE_SEARCH_CODE: &[u8] = &[
        0xe0, 0x03, 0x17, 0xaa, // mov x0, x23
        0xc8, 0x7a, 0x00, 0xb9, // str x8, [x22, #0x78]
        0xc8, 0x7a, 0x04, 0x91, // add x8, x22, #0x11e
        0xdf, 0xc2, 0x00, 0x79, // strh wzr, [x22, #0x60]
        0xdf, 0x8a, 0x01, 0x39, // strb x22, [x22, #0x62]
    ];

    const SET_WEAPON_VTABLE_OFFSET_TO_START: usize = 0x18;

    static SET_ITEM_VTABLE_SEARCH_CODE: &[u8] = &[
        0xe8, 0x3f, 0x10, 0x32, // orr  w8, wzr, #0xffff0000
        0xe0, 0x03, 0x14, 0xaa, // mov  x0, x20
        0x9f, 0xc3, 0x00, 0x79, // strh wzr, [x28, #0x60]
        0x9f, 0x8b, 0x01, 0x39, // strb wzr, [x28, #0x62]
        0x88, 0xa3, 0x03, 0xb8, // stur w8, [x29, #0x3a]
    ];

    const SET_ITEM_VTABLE_OFFSET_TO_START: usize = 0x14;

    static GET_BATTLE_OBJECT_FROM_ID_SEARCH_CODE: &[u8] = &[
        0x1f, 0x60, 0x02, 0x39, // strb wzr, [x0, #0x98]
        0xc0, 0x03, 0x5f, 0xd6, // ret
        0x00, 0x00, 0x00, 0x00, // ??
        0x08, 0x7c, 0x1c, 0x53, // lsr w9, x0, #0x1C
        0x1f, 0x11, 0x00, 0x71, // cmp w8, #0x4
    ];

    const GET_BATTLE_OBJECT_FROM_ID_OFFSET_TO_START: usize = 0xC;

    static FIGHTER_HANDLE_DAMAGE_SEARCH_CODE: &[u8] = &[
        0xff, 0xc3, 0x06, 0xd1, // sub sp, sp, #0x1b0
        0xed, 0x33, 0x12, 0x6d, // stp d13, d12, [sp, #0x120]
        0xeb, 0x2b, 0x13, 0x6d, // stp d11, d10, [sp, #0x130]
        0xe9, 0x23, 0x14, 0x6d, // stp d9, d8, [sp, #0x140]
        0xfc, 0x6f, 0x15, 0xa9, // stp x28, x27, [sp, #0x150]
        0xfa, 0x67, 0x16, 0xa9, // stp x26, x25, [sp, #0x160]
        0xf8, 0x5f, 0x17, 0xa9, // stp x24, x23, [sp, #0x170]
    ];

    static GET_GAME_STATE_SEARCH_CODE: &[u8] = &[
        0xf5, 0x0f, 0x1d, 0xf8, // str x21, [sp, #-0x30]
        0xf4, 0x4f, 0x01, 0xa9, // stp x20, x19, [sp, #0x10]
        0xfd, 0x7b, 0x02, 0xa9, // stp x29, x30, [sp, #0x20]
        0xfd, 0x83, 0x00, 0x91, // add x29, sp, #0x20
        0x08, 0x14, 0x40, 0xb9, // ldr w8, [x0, #0x14]
        0xf4, 0x03, 0x01, 0xaa, // mov x20, x1
        0xf3, 0x03, 0x00, 0xaa, // mov x19, x0
        0x1f, 0x05, 0x00, 0x71, // cmp w8, #0x1
    ];

    const GET_GAME_STATE_OFFSET_FROM_START: usize = 0x28;

    static MAP_CONTROLS_SEARCH_CODE: &[u8] = &[
        0xff, 0x03, 0x02, 0xd1, // sub sp, sp, #0x80
        0xf7, 0x23, 0x00, 0xf9, // str x23, [sp, #local_40]
        0xf6, 0x57, 0x05, 0xa9, // stp x22, x21, [sp, #local_30]
        0xf4, 0x4f, 0x06, 0xa9, // stp x20, x19, [sp, #local_20]
        0xfd, 0x7b, 0x07, 0xa9, // stp x29, x30, [sp, #local_10]
        0xfd, 0xc3, 0x01, 0x91, // add x29, sp, #0x70
        0x3f, 0x04, 0x00, 0x31, // cmn w1, #0x1
    ];

    static ONCE_PER_GAME_FRAME_SEARCH_CODE: &[u8] = &[
        0xff, 0xc3, 0x01, 0xd1, // sub sp, sp, #0x70
        0xfb, 0x0b, 0x00, 0xf9, // str x27, [sp, #0x10]
        0xfa, 0x67, 0x02, 0xa9, // stp x26, x25, [sp, #0x20]
        0xf8, 0x5f, 0x03, 0xa9, // stp x24, x23, [sp, #0x30]
        0xf6, 0x57, 0x04, 0xa9, // stp x22, x21, [sp, #0x40]
        0xf4, 0x4f, 0x05, 0xa9, // stp x20, x19, [sp, #0x50]
        0xfd, 0x7b, 0x06, 0xa9, // stp x29, x30, [sp, #0x60]
        0xfd, 0x83, 0x01, 0x91, // add x29, sp, #0x60
        0x0b, 0x90, 0x40, 0xf9, // ldr x11, [x0, #0x120]
    ];

    static ON_RULE_SELECT_SEARCH_CODE: &[u8] = &[
        0xee, 0xf1, 0x40, 0x78, // ldurh w14, [x15, #0xf]
        0xee, 0x5b, 0x00, 0x79, // strh w14, [sp, #0x2c]
        0xee, 0xb1, 0x40, 0xb8, // ldur w14, [x15, #0xb]
        0x11, 0x40, 0x80, 0x52, // mov w17, #0x200
        0x71, 0x40, 0xa0, 0x72, // movk w17, #0x203, lsl #16
        0x7f, 0x03, 0x11, 0x6b, // cmp w27, w17
    ];

    // TODO: Define search code to find global frame counter, current offset is hard coded for 13.0.1: 0x52e6b44

    static GET_MATCH_MODE_SEARCH_CODE: &[u8] = &[
        0xa8, 0x03, 0x51, 0xf8, // ldur x8, [x29, #-0xf0]
        0x08, 0x01, 0x40, 0xf9, // ldr x8, [x8]
        0xa2, 0x83, 0x94, 0xb8, // ldursw x2, [x29, #-0xb8]
        0x00, 0x01, 0x14, 0x8b, // add x0, x8, x20
    ];
    
    const GET_MATCH_MODE_OFFSET_TO_START: usize = 0x4;
    static KILL_ZOOM_REGULAR_SEARCH_CODE: &[u8] = &[
        0xd4, 0x62, 0x42, 0xf9, // ldr x20, [x22, #0x4c0]
        0x88, 0x02, 0x40, 0xf9, // ldr x8, [x20]
        0x08, 0x3d, 0x40, 0xf9, // ldr x8, [x8, #0x78]
        0xe1, 0x03, 0x1e, 0x32, // orr w1, wzr, #4
        0xe0, 0x03, 0x14, 0xaa, // mov x0, x20
        0x00, 0x01, 0x3f, 0xd6, // blr x8
    ];

    const KILL_ZOOM_REGULAR_OFFSET_TO_START: usize = 0x4;

    static KILL_ZOOM_THROW_SEARCH_CODE: &[u8] = &[
        0xe4, 0x03, 0x1f, 0x2a, // mov w4, wzr
        0xff, 0x03, 0x00, 0x39, // strb wzr, [sp]
        0x00, 0x01, 0x3f, 0xd6, // blr x8
        0xe2, 0x03, 0x00, 0x32, // orr w2, wzr, #1
        0xe0, 0x03, 0x13, 0xaa, // mov x0, x19
        0xe1, 0x03, 0x1f, 0x2a, // mov w1, wzr
        0xe3, 0x03, 0x14, 0x2a, // mov w3, w20
        0xe4, 0x03, 0x1f, 0xaa, // mov x4, xzr
    ];

    const KILL_ZOOM_THROW_OFFSET_FROM_START: usize = 0x20;

    static ANALOG_TRIGGER_L_SEARCH_CODE: &[u8] = &[
        0x29, 0x01, 0x7a, 0xb2, // orr x9, x9, #0x40
        0x1f, 0x01, 0x0b, 0x6b, // cmp w8, w11
        0x28, 0xc1, 0x8a, 0x9a, // csel x8, x9, x10, gt
        0xe9, 0x2b, 0x40, 0xb9, // ldr w9, [sp, #0x28]
        0x0a, 0xf9, 0x78, 0x92, // and x10, x8, #0xffffffffffffff7f
        0x08, 0x01, 0x79, 0xb2, // orr x8, x8, #0x80
        0x3f, 0x01, 0x0b, 0x6b, // cmp w9, w11
    ];

    const ANALOG_TRIGGER_R_OFFSET_FROM_L: usize = 0x14;

    fn offset_from_adrp(adrp_offset: usize) -> usize {
        unsafe {
            let adrp = *offset_to_addr::<u32>(adrp_offset);
            let immhi = (adrp & 0b0_00_00000_1111111111111111111_00000) >> 3;
            let immlo = (adrp & 0b0_11_00000_0000000000000000000_00000) >> 29;
            let imm = ((immhi | immlo) << 12) as i32 as usize;
            let base = adrp_offset & 0xFFFF_FFFF_FFFF_F000;
            base + imm
        }
    }

    fn offset_from_ldr(ldr_offset: usize) -> usize {
        unsafe {
            let ldr = *offset_to_addr::<u32>(ldr_offset);
            let size = (ldr & 0b11_000_0_00_00_000000000000_00000_00000) >> 30;
            let imm = (ldr & 0b00_000_0_00_00_111111111111_00000_00000) >> 10;
            (imm as usize) << size
        }
    }

    fn offset_from_bl(bl_offset: usize) -> usize {
        unsafe {
            let bl = *offset_to_addr::<u32>(bl_offset);
            let imm = bl & 0b0_00000_11111111111111111111111111;
            (imm * 4) as usize
        }
    }

    lazy_static! {
        static ref CORE_OFFSETS: CoreOffsets = {
            let mut offsets = CoreOffsets {
                exec_command: 0,
                get_command_flag_cat: 0,
                demon_on_link_capture_event: 0,
                force_linear_histun: 0,
                get_param_int_impl: 0,
                get_param_float_impl: 0,
                set_fighter_vtable: 0,
                set_weapon_vtable: 0,
                set_item_vtable: 0,
                get_battle_object_from_id: 0,
                fighter_handle_damage: 0,
                p_p_game_state: 0,
                map_controls: 0,
                once_per_game_frame: 0,
                on_rule_select: 0,
                global_frame_counter: 0,
                get_match_mode: 0,
                kill_zoom_regular: 0,
                kill_zoom_throw: 0,
                analog_trigger_l: 0,
                analog_trigger_r: 0
            };

            offsets.exec_command = byte_search(EXEC_COMMAND_SEARCH_CODE).expect("Unable to find exec command hook!") - EXEC_COMMAND_OFFSET_FROM_START;
            offsets.get_command_flag_cat = byte_search(GET_COMMAND_FLAG_CAT_SEARCH_CODE).expect("Unable to find get command flag cat hook!");
            offsets.demon_on_link_capture_event = byte_search(DEMON_ON_LINK_CAPTURE_EVENT_SEARCH_CODE).expect("Unable to find Kazuya OnLinkCaptureEvent hook!") - DEMON_ON_LINK_CAPTURE_EVENT_OFFSET_FROM_START;
            offsets.force_linear_histun = byte_search(FORCE_LINEAR_HISTUN_SEARCH_CODE).expect("Unable to find force linear histun hook!") + FORCE_LINEAR_HISTUN_OFFSET_TO_START;
            offsets.get_param_int_impl = byte_search(GET_PARAM_INT_IMPL_SEARCH_CODE).expect("Unable to find WorkModule::GetParamIntImpl hook!");
            offsets.get_param_float_impl = byte_search(GET_PARAM_FLOAT_IMPL_SEARCH_CODE).expect("Unable to find WorkModule::GetParamFloatImpl hook!");
            offsets.set_fighter_vtable = byte_search(SET_FIGHTER_VTABLE_SEARCH_CODE).expect("Unable to find Fighter class constructor hook!") + SET_FIGHTER_VTABLE_OFFSET_TO_START;
            offsets.set_weapon_vtable = byte_search(SET_WEAPON_VTABLE_SEARCH_CODE).expect("Unable to find Weapon class constructor hook!") + SET_WEAPON_VTABLE_OFFSET_TO_START;
            offsets.set_item_vtable = byte_search(SET_ITEM_VTABLE_SEARCH_CODE).expect("Unable to find Item class constructor hook!") + SET_ITEM_VTABLE_OFFSET_TO_START;
            offsets.get_battle_object_from_id = byte_search(GET_BATTLE_OBJECT_FROM_ID_SEARCH_CODE).expect("Unable to find Item class constructor hook!") + GET_BATTLE_OBJECT_FROM_ID_OFFSET_TO_START;
            offsets.fighter_handle_damage = byte_search(FIGHTER_HANDLE_DAMAGE_SEARCH_CODE).expect("Unable to find Fighter::HandleDamage!");
            offsets.p_p_game_state = {
                let offset = byte_search(GET_GAME_STATE_SEARCH_CODE).expect("Unable to find ppGameState!") + GET_GAME_STATE_OFFSET_FROM_START;
                let adrp_offset = offset_from_adrp(offset);
                let ldr_offset = offset_from_ldr(offset + 4);
                adrp_offset + ldr_offset
            };
            offsets.map_controls = byte_search(MAP_CONTROLS_SEARCH_CODE).expect("Unable to find control mapping function!");
            offsets.once_per_game_frame = byte_search(ONCE_PER_GAME_FRAME_SEARCH_CODE).expect("Unable to find once-per-game-frame function!");
            offsets.on_rule_select = byte_search(ON_RULE_SELECT_SEARCH_CODE).expect("Unable to find on-rule-select instructions!");
            offsets.global_frame_counter = 0x52e6b44;
            offsets.get_match_mode = {
                let offset = byte_search(GET_MATCH_MODE_SEARCH_CODE).expect("Unable to find get_match_mode!") - GET_MATCH_MODE_OFFSET_TO_START;
                let bl_offset = offset_from_bl(offset);
                offset + bl_offset
            };
            offsets.kill_zoom_regular = byte_search(KILL_ZOOM_REGULAR_SEARCH_CODE).expect("Unable to find the regular kill zoom function!") - KILL_ZOOM_REGULAR_OFFSET_TO_START;
            offsets.kill_zoom_throw = byte_search(KILL_ZOOM_THROW_SEARCH_CODE).expect("Unable to find the throw kill zoom function!") + KILL_ZOOM_THROW_OFFSET_FROM_START;
            offsets.analog_trigger_l = byte_search(ANALOG_TRIGGER_L_SEARCH_CODE).expect("Unable to find the analog trigger l");
            offsets.analog_trigger_r = offsets.analog_trigger_l + ANALOG_TRIGGER_R_OFFSET_FROM_L;
            offsets
        };
    }

    #[export_name = "offsets_exec_command"]
    pub fn exec_command() -> usize {
        CORE_OFFSETS.exec_command
    }

    #[export_name = "offsets_get_command_flag_cat"]
    pub fn get_command_flag_cat() -> usize {
        CORE_OFFSETS.get_command_flag_cat
    }

    #[export_name = "offsets_demon_on_link_capture_event"]
    pub fn demon_on_link_capture_event() -> usize {
        CORE_OFFSETS.demon_on_link_capture_event
    }

    #[export_name = "offsets_force_linear_histun"]
    pub fn force_linear_histun() -> usize {
        CORE_OFFSETS.force_linear_histun
    }

    #[export_name = "offsets_get_param_int_impl"]
    pub fn get_param_int_impl() -> usize {
        CORE_OFFSETS.get_param_int_impl
    }

    #[export_name = "offsets_get_param_float_impl"]
    pub fn get_param_float_impl() -> usize {
        CORE_OFFSETS.get_param_float_impl
    }

    #[export_name = "offsets_set_fighter_vtable"]
    pub fn set_fighter_vtable() -> usize {
        CORE_OFFSETS.set_fighter_vtable
    }

    #[export_name = "offsets_set_weapon_vtable"]
    pub fn set_weapon_vtable() -> usize {
        CORE_OFFSETS.set_weapon_vtable
    }

    #[export_name = "offsets_set_item_vtable"]
    pub fn set_item_vtable() -> usize {
        CORE_OFFSETS.set_item_vtable
    }

    #[export_name = "offsets_get_battle_object_from_id"]
    pub fn get_battle_object_from_id() -> usize {
        CORE_OFFSETS.get_battle_object_from_id
    }

    #[export_name = "offsets_fighter_handle_damage"]
    pub fn fighter_handle_damage() -> usize {
        CORE_OFFSETS.fighter_handle_damage
    }

    #[export_name = "offsets_p_p_game_state"]
    pub fn p_p_game_state() -> usize {
        CORE_OFFSETS.p_p_game_state
    }

    #[export_name = "offsets_map_controls"]
    pub fn map_controls() -> usize {
        CORE_OFFSETS.map_controls
    }

    #[export_name = "offsets_once_per_game_frame"]
    pub fn once_per_game_frame() -> usize {
        CORE_OFFSETS.once_per_game_frame
    }

    #[export_name = "offsets_on_rule_select"]
    pub fn on_rule_select() -> usize {
        CORE_OFFSETS.on_rule_select
    }

    #[export_name = "offsets_global_frame_counter"]
    pub fn global_frame_counter() -> usize {
        CORE_OFFSETS.global_frame_counter
    }

    #[export_name = "offsets_get_match_mode"]
    pub fn get_match_mode() -> usize {
        CORE_OFFSETS.get_match_mode
    }

    #[export_name = "offsets_kill_zoom_regular"]
    pub fn kill_zoom_regular() -> usize {
        CORE_OFFSETS.kill_zoom_regular
    }

    #[export_name = "offsets_kill_zoom_throw"]
    pub fn kill_zoom_throw() -> usize {
        CORE_OFFSETS.kill_zoom_throw
    }

    #[export_name = "offsets_analog_trigger_l"]
    pub fn analog_trigger_l() -> usize {
        CORE_OFFSETS.analog_trigger_l
    }

    #[export_name = "offsets_analog_trigger_r"]
    pub fn analog_trigger_r() -> usize {
        CORE_OFFSETS.analog_trigger_r
    }
}

pub const BATTLE_OBJECT_VTABLE_DESTRUCTOR_OFFSET: usize = 0x340;
pub const BATTLE_OBJECT_VTABLE_DELETER_OFFSET: usize    = 0x348;

pub use offsets_impl::*;