macro_rules! lua_gettop {
    ($state:ident) => {{
        let top = *($state as *const u64).add(2);
        let ci = *($state as *const u64).add(4);
        let func = *(ci as *const u64);
        (top - func) / 0x10
    }};
}

macro_rules! lua_settop {
    ($state:ident, $index:expr) => {{
        if $index < 0 {
            let ci = *($state as *const u64).add(4);
            let func = *(ci as *const i64);
            let new_top = (-$index * 0x10) + func;
            let top = ($state as *mut i64).add(2);
            while *top < new_top {
                *((*top) as *mut u32) = 0x0;
                *top = *top + 0x10;
            }
        } else {
            let top = ($state as *mut i64).add(2);
            *top = *top - (($index) * 0x10);
        }
    }};
}

#[skyline::from_offset(0x38f82a0)]
unsafe extern "C" fn luaL_tolstring(lua_state: u64, index: i32, size: *mut usize) -> *const u8;

unsafe extern "C" fn lua_print_impl(lua_state: u64) -> i32 {
    let num_args = lua_gettop!(lua_state) - 1;
    for x in 1..=num_args {
        let mut size = 0;
        let c_str = luaL_tolstring(lua_state, x as i32, &mut size);
        let slice = std::slice::from_raw_parts(c_str, size);
        if x > 1 {
            print!("\t");
        }
        print!("{}", std::str::from_utf8(slice).unwrap());
        lua_settop!(lua_state, -2i64);
    }
    println!();
    return 0;
}

#[skyline::hook(offset = 0x38f5630)]
unsafe fn lua_load(arg: u64, arg2: u64, arg3: u64, arg4: u64, mode: *const u8) -> u32 {
    let result = call_original!(arg, arg2, arg3, arg4, "bt\0".as_ptr());
    if result == 3 {
        let mut size = 0;
        let c_str = luaL_tolstring(arg, 1, &mut size);
        let slice = std::slice::from_raw_parts(c_str, size);
        println!(
            "error reading lua file: {}",
            std::str::from_utf8(slice).unwrap()
        );
    }
    result
}

#[skyline::from_offset(0x3770e20)]
unsafe fn register_button(arg: u64, id: i32, string: *const u8);

#[skyline::hook(offset = 0x1d33440, inline)]
unsafe fn add_buttons_to_subwindow(ctx: &mut skyline::hooks::InlineCtx) {
    let ptr = *ctx.registers[0].x.as_ref();
    register_button(ptr, 4, "set_parts_category_04\0".as_ptr());
    register_button(ptr, 6, "set_parts_category_05\0".as_ptr());
    // Can't use 5 here since that's for the "OK" button and
    // changing that will break the ability to save changes :weary:
    // *ctx.registers[1].x.as_mut() = 6;
    // *ctx.registers[20].x.as_mut() = 6;
    IS_IN_UI = true;
}

#[skyline::from_offset(0x37713b0)]
unsafe fn layout_get(arg: u64, arg2: u64, id: u64);

#[skyline::from_offset(0x37710d0)]
unsafe fn set_something(arg: u64, val: u64, val2: u64);

#[skyline::hook(offset = 0x1d33664, inline)]
unsafe fn setup_buttons(ctx: &skyline::hooks::InlineCtx) {
    let ptr = *ctx.registers[0].x.as_ref();
    let ptr2 = *ctx.registers[1].x.as_ref();

    layout_get(ptr, ptr2, 4);

    let p_vtable = **(ptr as *const *const u64).add(1);
    let func: extern "C" fn(u64, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x208) as *const u64));

    func(p_vtable, 0);

    let func: extern "C" fn(u64) -> bool =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x200) as *const u64));

    let var = func(p_vtable);
    let var = if !var {
        true
    } else {
        let func: extern "C" fn(u64) -> bool =
            std::mem::transmute(*((*(p_vtable as *const u64) + 0x210) as *const u64));

        !func(p_vtable)
    };

    let func: extern "C" fn(u64, bool, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x388) as *const u64));

    func(p_vtable, var, 0);

    *(ptr as *mut u64).add(1) = 0;

    layout_get(ptr, ptr2, 6);

    let p_vtable = **(ptr as *const *const u64).add(1);
    let func: extern "C" fn(u64, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x208) as *const u64));

    func(p_vtable, 0);

    let func: extern "C" fn(u64) -> bool =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x200) as *const u64));

    let var = func(p_vtable);
    let var = if !var {
        true
    } else {
        let func: extern "C" fn(u64) -> bool =
            std::mem::transmute(*((*(p_vtable as *const u64) + 0x210) as *const u64));

        !func(p_vtable)
    };

    let func: extern "C" fn(u64, bool, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x388) as *const u64));

    func(p_vtable, var, 0);

    *(ptr as *mut u64).add(1) = 0;
}

#[skyline::hook(offset = 0x1d33e7c, inline)]
unsafe fn hijack_animation_get(ctx: &skyline::hooks::InlineCtx) {
    // this memleaks but I DON'T GIVE A FUCK (Askew: doesn't actually memleak you schmuck)
    let ptr = *ctx.registers[0].x.as_ref();
    let our_ptr = SHARED_PTR1[0];
    layout_get(ptr, our_ptr, 0);

    let p_vtable = **(ptr as *const *const u64).add(1);
    let func: extern "C" fn(u64, bool, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x2f0) as *const u64));
    func(p_vtable, !CURRENT_UI_PARRY_TOGGLE, 0);

    *(ptr as *mut u64).add(1) = 0;

    layout_get(ptr, our_ptr, 1);

    let p_vtable = **(ptr as *const *const u64).add(1);
    let func: extern "C" fn(u64, bool, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x2f0) as *const u64));
    func(p_vtable, CURRENT_UI_PARRY_TOGGLE, 0);

    *(ptr as *mut u64).add(1) = 0;

    set_something(SHARED_PTR1.as_ptr() as _, CURRENT_UI_PARRY_TOGGLE as _, 1);

    let our_ptr = SHARED_PTR2[0];
    layout_get(ptr, our_ptr, 0);

    let p_vtable = **(ptr as *const *const u64).add(1);
    let func: extern "C" fn(u64, bool, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x2f0) as *const u64));
    func(p_vtable, !CURRENT_UI_RIVALS_JUMP, 0);

    *(ptr as *mut u64).add(1) = 0;

    layout_get(ptr, our_ptr, 1);

    let p_vtable = **(ptr as *const *const u64).add(1);
    let func: extern "C" fn(u64, bool, u64) =
        std::mem::transmute(*((*(p_vtable as *const u64) + 0x2f0) as *const u64));
    func(p_vtable, CURRENT_UI_RIVALS_JUMP, 0);

    *(ptr as *mut u64).add(1) = 0;

    set_something(SHARED_PTR2.as_ptr() as _, CURRENT_UI_RIVALS_JUMP as _, 1);
}

// #[skyline::hook(offset = 0x1d2b470)]
// unsafe fn get_smash_option(arg: u64, controller_id: i32) -> u8 {
//     let id = controller_id & 3;
//     let extra = (controller_id & 0xC) >> 2;
//     let flag = (((controller_id & 0x10) >> 4) & 1) as u8;

//     match extra {
//         0 => match id {
//             0 => *(arg as *const u8).add(0xd4),
//             1 => *(arg as *const u8).add(0xe0),
//             2 => *(arg as *const u8).add(0xc4),
//             _ => unimplemented!(),
//         },
//         other => {
//             match id {
//                 0 => *(arg as *mut u8).add(0xd5) |= flag << other,
//                 1 => *(arg as *mut u8).add(0xe1) |= flag << other,
//                 2 => *(arg as *mut u8).add(0xc5) |= flag << other,
//                 _ => {}
//             }
//             0
//         }
//     }
// }

// #[skyline::hook(offset = 0x1d2b4a0)]
// unsafe fn get_flick_stick_option(arg: u64, controller_id: i32) -> u8 {
//     let id = controller_id & 3;
//     let extra = (controller_id & 0xC) >> 2;

//     let value = match id {
//         0 => *(arg as *const u8).add(0xd5),
//         1 => *(arg as *const u8).add(0xe1),
//         2 => *(arg as *const u8).add(0xc5),
//         _ => unimplemented!(),
//     };

//     match extra {
//         0 => value & 1,
//         1 => (value >> 1) & 1,
//         2 => (value >> 2) & 1,
//         3 => (value >> 3) & 1,
//         _ => unimplemented!(),
//     }
// }

static mut SHARED_PTR1: [u64; 2] = [0, 0];
static mut SHARED_PTR2: [u64; 2] = [0, 0];

#[skyline::hook(offset = 0x1d338cc, inline)]
unsafe fn frank_talk_think_tankk(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[22].x.as_ref() == 4 {
        SHARED_PTR1[0] = 0;
        SHARED_PTR1[1] = 0;
        *ctx.registers[19].x.as_mut() = SHARED_PTR1.as_ptr() as u64;
    }

    if *ctx.registers[22].x.as_ref() == 5 {
        SHARED_PTR2[0] = 0;
        SHARED_PTR2[1] = 0;
        *ctx.registers[19].x.as_mut() = SHARED_PTR2.as_ptr() as u64;
    }
}

static mut CURRENT_UI_PARRY_TOGGLE: bool = false;
static mut CURRENT_UI_RIVALS_JUMP: bool = false;

#[skyline::hook(offset = 0x1d309f8, inline)]
unsafe fn get_on_value_for_custom(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[1].x.as_ref() == 4 {
        *ctx.registers[19].x.as_mut() = CURRENT_UI_PARRY_TOGGLE as u64;
    } else if *ctx.registers[1].x.as_ref() == 6 {
        *ctx.registers[19].x.as_mut() = CURRENT_UI_RIVALS_JUMP as u64;
    }
}

#[skyline::hook(offset = 0x1d30a14, inline)]
unsafe fn get_shared_ptr_for_custom(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[8].x.as_ref() == 4 {
        *ctx.registers[20].x.as_mut() = SHARED_PTR1.as_ptr() as u64;
    } else if *ctx.registers[8].x.as_ref() == 6 {
        *ctx.registers[20].x.as_mut() = SHARED_PTR2.as_ptr() as u64;
    }
}

#[skyline::hook(offset = 0x1d2fd9c, inline)]
unsafe fn init_buttons_in_main_loop(ctx: &skyline::hooks::InlineCtx) {
    let flag = *ctx.registers[1].x.as_ref() != 0;
    let ptr = SHARED_PTR1[0];
    let func: extern "C" fn(u64, bool) =
        std::mem::transmute(*((*(ptr as *const u64) + 0x60) as *const u64));

    func(ptr, flag);

    let ptr = SHARED_PTR2[0];
    let func: extern "C" fn(u64, bool) =
        std::mem::transmute(*((*(ptr as *const u64) + 0x60) as *const u64));

    func(ptr, flag);
}

#[skyline::hook(offset = 0x1d2fe38, inline)]
unsafe fn init_buttons_in_main_loop_again(ctx: &skyline::hooks::InlineCtx) {
    init_buttons_in_main_loop(ctx);
}

#[skyline::hook(offset = 0x1d30038, inline)]
unsafe fn get_index_for_a_press(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[22].x.as_ref() == 4 {
        *ctx.registers[10].x.as_mut() = CURRENT_UI_PARRY_TOGGLE as u64;
    } else if *ctx.registers[22].x.as_ref() == 6 {
        *ctx.registers[10].x.as_mut() = CURRENT_UI_RIVALS_JUMP as u64;
    }
}

#[skyline::hook(offset = 0x1d30114, inline)]
unsafe fn update_index_for_a_press(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[22].x.as_ref() == 4 {
        CURRENT_UI_PARRY_TOGGLE = *ctx.registers[11].x.as_ref() != 0;
    } else if *ctx.registers[22].x.as_ref() == 6 {
        CURRENT_UI_RIVALS_JUMP = *ctx.registers[11].x.as_ref() != 0;
    }
}

static mut IS_IN_UI: bool = false;

#[skyline::hook(offset = 0x376cfe0)]
unsafe fn set_next_button(arg: u64, button: i32, other: u64) {
    if !IS_IN_UI {
        call_original!(arg, button, other);
        return;
    }

    let current_button = *(arg as *const u32).add(0x250 / 0x4);

    if current_button == 3 && button == 5 {
        call_original!(arg, 4, other);
    } else if current_button == 6 && button == 0 {
        call_original!(arg, 5, other);
    } else {
        call_original!(arg, button, other);
    }
}

#[skyline::hook(offset = 0x1d309a4, inline)]
unsafe fn skip_set_setting_for_ok(ctx: &mut skyline::hooks::InlineCtx) {
    if *ctx.registers[1].x.as_ref() == 5 {
        *ctx.registers[1].x.as_mut() = 300;
    }
}

#[skyline::hook(offset = 0x1d311e0, inline)]
unsafe fn init_ui_state(ctx: &mut skyline::hooks::InlineCtx) {
    let ptr = (*ctx.registers[8].x.as_ref() as *mut u8).add(1);
    CURRENT_UI_PARRY_TOGGLE = (*ptr >> 1) & 1 != 0;
    CURRENT_UI_RIVALS_JUMP = (*ptr >> 2) & 1 != 0;
    *ptr &= 1;
}

#[skyline::hook(offset = 0x1d2f3c4, inline)]
unsafe fn exit_gc(ctx: &mut skyline::hooks::InlineCtx) {
    let ptr = (*ctx.registers[20].x.as_ref() as *mut u8).add(0xC4);
    *ptr |= (CURRENT_UI_PARRY_TOGGLE as u8) << 1;
    *ptr |= (CURRENT_UI_RIVALS_JUMP as u8) << 2;
    IS_IN_UI = false;
}

#[skyline::hook(offset = 0x1d2f390, inline)]
unsafe fn exit_fk(ctx: &mut skyline::hooks::InlineCtx) {
    let ptr = (*ctx.registers[20].x.as_ref() as *mut u8).add(0xE0);
    *ptr |= (CURRENT_UI_PARRY_TOGGLE as u8) << 1;
    *ptr |= (CURRENT_UI_RIVALS_JUMP as u8) << 2;
    IS_IN_UI = false;
}

#[skyline::hook(offset = 0x1d2f344, inline)]
unsafe fn exit_jc(ctx: &mut skyline::hooks::InlineCtx) {
    let ptr = (*ctx.registers[20].x.as_ref() as *mut u8).add(0xD4);
    *ptr |= (CURRENT_UI_PARRY_TOGGLE as u8) << 1;
    *ptr |= (CURRENT_UI_RIVALS_JUMP as u8) << 2;
    IS_IN_UI = false;
}

unsafe fn get_parts(arg: u64, arg2: *const u8) -> [u64; 4] {
    let func_addr =
        (skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *mut u8).add(0x3775C90);
    let callable: extern "C" fn(u64, *const u8, ...) -> [u64; 4] = std::mem::transmute(func_addr);
    callable(arg, arg2)
}

#[skyline::hook(offset = 0x1d33254, inline)]
unsafe fn set_pane_text_values(ctx: &skyline::hooks::InlineCtx) {
    let layout_view = *ctx.registers[0].x.as_ref();

    let mut parts = get_parts(
        [0, layout_view].as_ptr() as _,
        b"set_parts_category_04\0".as_ptr(),
    );

    let pane = crate::online::get_pane_by_name(parts.as_mut_ptr() as _, b"txt_name_00\0".as_ptr());

    crate::online::set_text_string(*((pane[1] + 0x10) as *const u64), "Parry Input\0".as_ptr());

    let mut parts = get_parts(
        [0, layout_view].as_ptr() as _,
        b"set_parts_category_05\0".as_ptr(),
    );

    let pane = crate::online::get_pane_by_name(parts.as_mut_ptr() as _, b"txt_name_00\0".as_ptr());

    crate::online::set_text_string(
        *((pane[1] + 0x10) as *const u64),
        "Wall Jump Input\0".as_ptr(),
    );
}

#[skyline::hook(offset = 0x1d33b40, inline)]
unsafe fn set_parry_button_shield_text(ctx: &skyline::hooks::InlineCtx) {
    let sp = (ctx as *const _ as *const u8).add(0x100);
    let ptr = *(sp.add(0xa8) as *const u64);

    if *ctx.registers[22].x.as_ref() == 4 {
        crate::online::set_text_string(*((ptr + 0x10) as *const u64), "Special\0".as_ptr());
    }
    if *ctx.registers[22].x.as_ref() == 5 {
        crate::online::set_text_string(*((ptr + 0x10) as *const u64), "Flick\0".as_ptr());
    }
}

#[skyline::hook(offset = 0x1d33b80, inline)]
unsafe fn set_parry_button_taunt_text(ctx: &skyline::hooks::InlineCtx) {
    let sp = (ctx as *const _ as *const u8).add(0x100);
    let ptr = *(sp.add(0xa8) as *const u64);

    if *ctx.registers[22].x.as_ref() == 4 {
        crate::online::set_text_string(*((ptr + 0x10) as *const u64), "Taunt\0".as_ptr());
    }
    if *ctx.registers[22].x.as_ref() == 5 {
        crate::online::set_text_string(*((ptr + 0x10) as *const u64), "Button\0".as_ptr());
    }
}

pub fn install() {
    unsafe {
        skyline::patching::Patch::in_text(0x5293c70).data((lua_print_impl as *const ()));
        skyline::patching::Patch::in_text(0x372c130).data(0xD503201Fu32);
        skyline::patching::Patch::in_text(0x1d33cfc).data(0xF1001ADFu32);
        skyline::patching::Patch::in_text(0x1d309a4).data(0x7100183Fu32);
        // skyline::patching::Patch::in_text(0x1d2fec8).nop();
        skyline::patching::Patch::in_text(0x1d30a28).data(0xAA1403F8u32);
    }
    skyline::install_hooks!(
        lua_load,
        add_buttons_to_subwindow,
        frank_talk_think_tankk,
        hijack_animation_get,
        setup_buttons,
        get_on_value_for_custom,
        get_shared_ptr_for_custom,
        init_buttons_in_main_loop,
        init_buttons_in_main_loop_again,
        get_index_for_a_press,
        update_index_for_a_press,
        set_next_button,
        skip_set_setting_for_ok,
        init_ui_state,
        exit_gc,
        exit_fk,
        exit_jc,
        set_pane_text_values,
        set_parry_button_shield_text,
        set_parry_button_taunt_text
    );
}
