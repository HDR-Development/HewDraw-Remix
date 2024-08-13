use std::mem::MaybeUninit;

#[repr(C)]
struct ShowKbdArg {
    config: [u8; 0x4D0],
    work_buffer: *mut u8,
    work_buffer_size: usize,
    text_buffer: *mut u8,
    text_buffer_size: usize,
    custom_dictionary_buffer: *mut u8,
    custom_dictionary_buffer_size: usize,
}

#[repr(C)]
struct String {
    data: *mut u16,
    len: usize,
}

extern "C" {
    #[link_name = "_ZN2nn5swkbd17MakePresetDefaultEPNS0_14KeyboardConfigE"]
    fn make_preset_default(x: *mut u8);

    #[link_name = "_ZN2nn5swkbd13SetHeaderTextEPNS0_14KeyboardConfigEPKDs"]
    fn set_header_text(x: *mut u8, text: *const u16);

    #[link_name = "_ZN2nn5swkbd14SetInitialTextEPNS0_15ShowKeyboardArgEPKDs"]
    fn set_initial_text(x: *mut u8, text: *const u16);

    #[link_name = "_ZN2nn5swkbd12ShowKeyboardEPNS0_6StringERKNS0_15ShowKeyboardArgE"]
    fn show_keyboard(string: &mut String, arg: *const ShowKbdArg) -> u32;
}

pub fn prompt_change_text(utf16: &[u16]) -> Vec<u16> {
    let mut initial = utf16.to_vec();
    initial.push(0);
    let mut arg: ShowKbdArg = unsafe { MaybeUninit::zeroed().assume_init() };
    unsafe {
        make_preset_default(arg.config.as_mut_ptr());
    }
    arg.config[0x3AC] = 10;
    arg.config[0x3B8] = 8;
    arg.config[0x3bc] = 0;

    let mut work_buffer = Box::new([0u8; 0xd000]);
    arg.work_buffer = work_buffer.as_mut_ptr();
    arg.work_buffer_size = 0xd000;

    let header: Vec<_> = "Edit Name".encode_utf16().chain([0]).collect();
    unsafe {
        set_header_text(arg.config.as_mut_ptr(), header.as_ptr());
    }
    unsafe {
        set_initial_text(arg.config.as_mut_ptr(), initial.as_ptr());
    }
    let mut output = Box::new([0u16; 1002]);

    let result = dbg!(unsafe {
        show_keyboard(
            &mut String {
                data: output.as_mut_ptr(),
                len: output.len(),
            },
            &arg,
        )
    });
    if result != 0x29f {
        let len = output.iter().position(|c| *c == 0).unwrap_or(output.len());
        output[..len].to_vec()
    } else {
        utf16.to_vec()
    }
}
