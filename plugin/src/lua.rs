macro_rules! lua_gettop {
    ($state:ident) => {{
        let top = *($state as *const u64).add(2);
        let ci = *($state as *const u64).add(4);
        let func = *(ci as *const u64);
        (top - func) / 0x10
    }}
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
    }}
}

#[skyline::from_offset(0x38f7620)]
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

#[skyline::hook(offset = 0x38f49b0)]
unsafe fn lua_load(arg: u64, arg2: u64, arg3: u64, arg4: u64, mode: *const u8) -> u32 {
    let result = call_original!(arg, arg2, arg3, arg4, "bt\0".as_ptr());
    if result == 3 {
        let mut size = 0;
        let c_str = luaL_tolstring(arg, 1, &mut size);
        let slice = std::slice::from_raw_parts(c_str, size);
        println!("error reading lua file: {}", std::str::from_utf8(slice).unwrap());
    }
    result
}

pub fn install() {
    unsafe {
        skyline::patching::Patch::in_text(0x5291c70).data((lua_print_impl as *const ()));
        skyline::patching::Patch::in_text(0x372b4b0).data(0xD503201Fu32);
    }
    skyline::install_hook!(lua_load);
}