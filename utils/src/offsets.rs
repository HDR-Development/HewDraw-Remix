#[cfg(feature = "no-offset-search")]
mod offsets_impl {
    // These offsets are hardcoded to increase runtime speed but will are only viable for smash version 13.0.1
    pub const fn exec_command() -> usize {
        0x6babf0
    }

    pub const fn get_command_flag_cat() -> usize {
        0x6ba980
    }

    pub const fn demon_on_link_capture_event() -> usize {
        0x9337e0
    }

    pub const fn force_linear_histun() -> usize {
        0x62ba54
    }

    pub const fn get_param_int_impl() -> usize {
        0x4e5380
    }

    pub const fn get_param_float_impl() -> usize {
        0x4e53C0
    }

    pub const fn set_fighter_vtable() -> usize {
        0x14f4784
    }

    pub const fn set_weapon_vtable() -> usize {
        0x14f4c9c
    }

    pub const fn set_item_vtable() -> usize {
        0x14f4f34
    }

    pub const fn get_battle_object_from_id() -> usize {
        0x3ac540
    }
}

#[cfg(not(feature = "no-offset-search"))]
mod offsets_impl {
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
                get_battle_object_from_id: 0
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
            offsets
        };
    }

    pub fn exec_command() -> usize {
        CORE_OFFSETS.exec_command
    }

    pub fn get_command_flag_cat() -> usize {
        CORE_OFFSETS.get_command_flag_cat
    }

    pub fn demon_on_link_capture_event() -> usize {
        CORE_OFFSETS.demon_on_link_capture_event
    }

    pub fn force_linear_histun() -> usize {
        CORE_OFFSETS.force_linear_histun
    }

    pub fn get_param_int_impl() -> usize {
        CORE_OFFSETS.get_param_int_impl
    }

    pub fn get_param_float_impl() -> usize {
        CORE_OFFSETS.get_param_float_impl
    }

    pub fn set_fighter_vtable() -> usize {
        CORE_OFFSETS.set_fighter_vtable
    }

    pub fn set_weapon_vtable() -> usize {
        CORE_OFFSETS.set_weapon_vtable
    }

    pub fn set_item_vtable() -> usize {
        CORE_OFFSETS.set_item_vtable
    }

    pub fn get_battle_object_from_id() -> usize {
        CORE_OFFSETS.get_battle_object_from_id
    }
}

pub const BATTLE_OBJECT_VTABLE_DESTRUCTOR_OFFSET: usize = 0x340;
pub const BATTLE_OBJECT_VTABLE_DELETER_OFFSET: usize    = 0x348;

pub use offsets_impl::*;