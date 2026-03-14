use std::mem::MaybeUninit;

use skyline::libc::{c_void, c_char};

extern "C" {
    #[link_name = "\u{1}_ZN3lib7utility8VariadicC1Ev"]
    fn ctor(variadic: *mut Variadic);

    #[link_name = "\u{1}_ZN3lib7utility8VariadicD1Ev"]
    fn dtor(variadic: *mut Variadic);

    #[link_name = "\u{1}_ZNK3lib7utility8Variadic10get_formatEv"]
    fn get_format(variadic: *const Variadic) -> *const c_char;
}

#[repr(C)]
pub struct Variadic {
    variadic_type: *const c_void
}

impl Variadic {
    pub fn new() -> Self {
        unsafe {
            let mut value = MaybeUninit::uninit();
            ctor(value.as_mut_ptr());
            value.assume_init()
        }
    }

    pub fn get_format(&self) -> Option<String> {
        unsafe {
            let format = get_format(self);
            if format.is_null() {
                None
            } else {
                Some(skyline::from_c_str(format))
            }
        }
    }

    pub fn get_format_cstr(&self) -> *const c_char {
        unsafe {
            get_format(self)
        }
    }
}

impl Drop for Variadic {
    fn drop(&mut self) {
        unsafe {
            dtor(self);
        }
    }
}