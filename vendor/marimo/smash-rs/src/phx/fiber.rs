use std::mem::MaybeUninit;

use skyline::libc::{
    c_void,
    c_char
};

extern "C" {
    #[link_name = "\u{1}_ZN3phx5FiberC1Ev"]
    fn fiber_ctor(fiber: *mut Fiber);

    #[link_name = "\u{1}_ZN3phx5Fiber17get_current_fiberEv"]
    fn get_current_fiber() -> *mut Fiber;

    #[link_name = "\u{1}_ZN3phx5Fiber8finalizeEv"]
    fn finalize(fiber: *mut Fiber);

    #[allow(dead_code)]
    #[link_name = "\u{1}_ZN3phx5Fiber5setupENSt3__18functionIFPS0_vEEEPKcm"]
    fn setup(fiber: *mut Fiber, setup_fn: *const c_void, name: *const c_char, stack_size: usize);

    #[link_name = "\u{1}_ZNK3phx5Fiber6statusEv"]
    fn status(fiber: *mut Fiber) -> u32;

    #[link_name = "\u{1}_ZN3phx5Fiber15switch_to_fiberEPS0_"]
    fn switch_to_fiber(fiber: *mut Fiber);

    #[link_name = "\u{1}_ZN3phx5FiberD1Ev"]
    fn fiber_dtor(fiber: *mut Fiber);
}

#[derive(Debug)]
#[repr(C)]
pub struct Fiber {
    os_fiber_type: *const c_void
}

impl Fiber {
    pub fn new() -> Fiber {
        unsafe {
            let mut fiber = MaybeUninit::<Self>::uninit();
            fiber_ctor(fiber.as_mut_ptr());
            fiber.assume_init()
        }
    }

    pub fn current() -> Option<&'static mut Fiber> {
        unsafe {
            get_current_fiber().as_mut()
        }
    }

    pub fn finalize(&mut self) {
        unsafe {
            finalize(self)
        }
    }

    pub fn setup(&mut self) -> ! {
        unimplemented!()
    }

    pub fn status(&mut self) -> u32 {
        unsafe {
            status(self)
        }
    }

    pub fn switch_to_fiber(&mut self) {
        unsafe {
            switch_to_fiber(self)
        }
    }
}

impl Drop for Fiber {
    fn drop(&mut self) {
        unsafe {
            fiber_dtor(self)
        }
    }
}