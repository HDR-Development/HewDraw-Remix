
#[repr(C)]
pub struct HashedString {
    pub hash: smash::phx::Hash40,
    pub contents: [u8; 0x100]
}

#[skyline::hook(offset = 0x1d39500)]
unsafe fn get_button_label_by_operation_kind(hashed_string: &mut HashedString, operation: u8, arg: bool) {
    if operation == 0x12 {
        for (index, byte) in "mnu_opt_btn_key_footstool\0".as_bytes().iter().enumerate() {
            hashed_string.contents[index] = *byte;
        }

        hashed_string.hash = smash::phx::Hash40::new("mnu_opt_btn_key_footstool");
    } else {
        return call_original!(hashed_string, operation, arg)
    }
}

#[skyline::hook(offset = 0x1d329e8, inline)]
unsafe fn add_footstool_to_gc(ctx: &skyline::hooks::InlineCtx) {
    let button = *ctx.registers[25].w.as_ref();
    if ![0x3, 0x4, 0x5, 0x8].contains(&button) {
        let input_list_vector = (*ctx.registers[24].x.as_ref() + 0x148) as *mut u64;
        let input_list_vector = std::slice::from_raw_parts_mut(input_list_vector, 3);
        let len = (input_list_vector[1] - input_list_vector[0]) as usize;
        let cap = (input_list_vector[2] - input_list_vector[0]) as usize;
        if len >= cap {
            let new_memory = skyline::libc::malloc(cap + 1) as *mut u8;
            for x in 0..len {
                *new_memory.add(x) = *(input_list_vector[0] as *const u8).add(x);
            }
            input_list_vector[0] = new_memory as u64;
            input_list_vector[1] = new_memory.add(len) as u64;
            input_list_vector[2] = input_list_vector[1];
        }

        if len < 6 {
            *(input_list_vector[1] as *mut u8) = 0x12;
            input_list_vector[1] += 1;
        }
    
    }

}

#[skyline::hook(offset = 0x1d326f8, inline)]
unsafe fn add_footstool_to_fk(ctx: &skyline::hooks::InlineCtx) {
    let button = dbg!(*ctx.registers[25].w.as_ref());
    if [0x4, 0x5, 0x6, 0x9].contains(&button) {
        return;
    }

    let input_list_vector = (*ctx.registers[24].x.as_ref() + 0x148) as *mut u64;
    let input_list_vector = std::slice::from_raw_parts_mut(input_list_vector, 3);
    let len = (input_list_vector[1] - input_list_vector[0]) as usize;
    let cap = (input_list_vector[2] - input_list_vector[0]) as usize;
    if len >= cap {
        let new_memory = skyline::libc::malloc(cap + 1) as *mut u8;
        for x in 0..len {
            *new_memory.add(x) = *(input_list_vector[0] as *const u8).add(x);
        }
        input_list_vector[0] = new_memory as u64;
        input_list_vector[1] = new_memory.add(len) as u64;
        input_list_vector[2] = input_list_vector[1];
    }

    *(input_list_vector[1] as *mut u8) = 0x12;
    input_list_vector[1] += 1;
}

#[skyline::hook(offset = 0x1d3395c, inline)]
unsafe fn add_footstool_to_jc(ctx: &skyline::hooks::InlineCtx) {
    let input_list_vector = (*ctx.registers[24].x.as_ref() + 0x148) as *mut u64;
    let input_list_vector = std::slice::from_raw_parts_mut(input_list_vector, 3);
    let len = (input_list_vector[1] - input_list_vector[0]) as usize;
    let cap = (input_list_vector[2] - input_list_vector[0]) as usize;
    if len >= cap {
        let new_memory = skyline::libc::malloc(cap + 1) as *mut u8;
        for x in 0..len {
            *new_memory.add(x) = *(input_list_vector[0] as *const u8).add(x);
        }
        input_list_vector[0] = new_memory as u64;
        input_list_vector[1] = new_memory.add(len) as u64;
        input_list_vector[2] = input_list_vector[1];
    }

    *(input_list_vector[1] as *mut u8) = 0x12;
    input_list_vector[1] += 1;
}

pub fn install() {
    skyline::install_hooks!(
        get_button_label_by_operation_kind,
        add_footstool_to_gc,
        add_footstool_to_fk,
        add_footstool_to_jc
    );
}