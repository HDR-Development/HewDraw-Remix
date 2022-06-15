
#[repr(C)]
pub struct HashedString {
    pub hash: smash::phx::Hash40,
    pub contents: [u8; 0x100]
}

#[repr(C)]
pub struct CppVector<T> {
    start: *mut T,
    end: *mut T,
    eos: *mut T
}

impl<T> CppVector<T> {
    pub fn len(&self) -> usize {
        unsafe {
            self.end.offset_from(self.start) as usize
        }
    }
}

impl<T: Copy> CppVector<T> {
    pub fn push(&mut self, value: T) {
        unsafe {
            let length = self.end.offset_from(self.start) as usize;
            let cap = self.eos.offset_from(self.start) as usize;
            if length == cap {
                let new_ptr = skyline::libc::malloc(std::mem::size_of::<T>() * cap * 2);
                skyline::libc::memcpy(new_ptr, self.start as _, std::mem::size_of::<T>() * length);
                let old = self.start;
                self.start = new_ptr as _;
                self.end = self.start.add(length as usize);
                self.eos = self.start.add((cap * 2) as usize);
    
                skyline::libc::free(old as _);
            }
    
            *self.end = value;
            self.end = self.end.add(1);
        }
    }
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
        let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);


        if input_list_vector.len() < 6 {
            input_list_vector.push(0x12);
        }
    }
}

#[skyline::hook(offset = 0x1d326f8, inline)]
unsafe fn add_footstool_to_fk(ctx: &skyline::hooks::InlineCtx) {
    let button = *ctx.registers[25].w.as_ref();
    if [0x4, 0x5, 0x6, 0x9].contains(&button) {
        return;
    }
    let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);


    if input_list_vector.len() < 6 {
        input_list_vector.push(0x12);
    }
}

#[skyline::hook(offset = 0x1d3395c, inline)]
unsafe fn add_footstool_to_jc(ctx: &skyline::hooks::InlineCtx) {
    let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);
    
    if input_list_vector.len() < 6 {
        input_list_vector.push(0x12);
    }
}

#[skyline::hook(offset = 0x1d34e4c, inline)]
unsafe fn add_more_buttons(ctx: &mut skyline::hooks::InlineCtx) {
    let input_list_vector = &mut *((*ctx.registers[24].x.as_ref() + 0x148) as *mut CppVector<u8>);
    // panic!("{}", input_list_vector.len());
    *ctx.registers[25].x.as_mut() = input_list_vector.len() as u64;
}

pub fn install() {
    unsafe {
        skyline::patching::nop_data(0x1d34e4c);
    }
    skyline::install_hooks!(
        get_button_label_by_operation_kind,
        add_footstool_to_gc,
        add_footstool_to_fk,
        add_footstool_to_jc,
        add_more_buttons
    );

}