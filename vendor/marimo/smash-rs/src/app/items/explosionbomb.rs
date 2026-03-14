use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app13explosionbomb23EXPLOSIONBOMB_BOUND_NUMEv"]
        pub(super) fn EXPLOSIONBOMB_BOUND_NUM() -> i32;
    
        #[link_name = "_ZN3app13explosionbomb31EXPLOSIONBOMB_BOUND_SPEED_MUL_XEv"]
        pub(super) fn EXPLOSIONBOMB_BOUND_SPEED_MUL_X() -> f32;
    
        #[link_name = "_ZN3app13explosionbomb31EXPLOSIONBOMB_BOUND_SPEED_MUL_YEv"]
        pub(super) fn EXPLOSIONBOMB_BOUND_SPEED_MUL_Y() -> f32;
    
        #[link_name = "_ZN3app13explosionbomb26EXPLOSIONBOMB_BURST_DAMAGEEv"]
        pub(super) fn EXPLOSIONBOMB_BURST_DAMAGE() -> f32;
    
        #[link_name = "_ZN3app13explosionbomb25EXPLOSIONBOMB_BURST_FRAMEEv"]
        pub(super) fn EXPLOSIONBOMB_BURST_FRAME() -> i32;
    
        #[link_name = "_ZN3app13explosionbomb21EXPLOSIONBOMB_GRAVITYEv"]
        pub(super) fn EXPLOSIONBOMB_GRAVITY() -> f32;
    
        #[link_name = "_ZN3app13explosionbomb29EXPLOSIONBOMB_HOP_SPEED_MUL_XEv"]
        pub(super) fn EXPLOSIONBOMB_HOP_SPEED_MUL_X() -> f32;
    
        #[link_name = "_ZN3app13explosionbomb25EXPLOSIONBOMB_HOP_SPEED_YEv"]
        pub(super) fn EXPLOSIONBOMB_HOP_SPEED_Y() -> f32;
    
        #[link_name = "_ZN3app13explosionbomb28EXPLOSIONBOMB_IGNITION_FRAMEEv"]
        pub(super) fn EXPLOSIONBOMB_IGNITION_FRAME() -> i32;
    
        #[link_name = "_ZN3app13explosionbomb26EXPLOSIONBOMB_LIFE_MISFIREEv"]
        pub(super) fn EXPLOSIONBOMB_LIFE_MISFIRE() -> i32;
    
        #[link_name = "_ZN3app13explosionbomb27EXPLOSIONBOMB_LIMIT_GRAVITYEv"]
        pub(super) fn EXPLOSIONBOMB_LIMIT_GRAVITY() -> f32;
    
        #[link_name = "_ZN3app13explosionbomb30EXPLOSIONBOMB_WIRE_BOUND_MUL_YEv"]
        pub(super) fn EXPLOSIONBOMB_WIRE_BOUND_MUL_Y() -> f32;
    
        #[link_name = "_ZN3app13explosionbomb26EXPLOSIONBOMB_WIRE_GRAVITYEv"]
        pub(super) fn EXPLOSIONBOMB_WIRE_GRAVITY() -> f32;
    
        #[link_name = "_ZN3app13explosionbomb28EXPLOSIONBOMB_WIRE_ROT_SPEEDEv"]
        pub(super) fn EXPLOSIONBOMB_WIRE_ROT_SPEED() -> i32;
    
        #[link_name = "_ZN3app13explosionbomb26EXPLOSIONBOMB_WIRE_SPEED_XEv"]
        pub(super) fn EXPLOSIONBOMB_WIRE_SPEED_X() -> f32;
    
        #[link_name = "_ZN3app13explosionbomb26EXPLOSIONBOMB_WIRE_SPEED_YEv"]
        pub(super) fn EXPLOSIONBOMB_WIRE_SPEED_Y() -> f32;
    
        #[link_name = "_ZN3app13explosionbomb30set_notify_article_event_ejectERNS_18ItemModuleAccessorE"]
        pub(super) fn set_notify_article_event_eject(module_accessor: *mut app::ItemModuleAccessor);
    
    }
}

pub fn EXPLOSIONBOMB_BOUND_NUM() -> i32 {
    unsafe {
        impl_::EXPLOSIONBOMB_BOUND_NUM()
    }
}

pub fn EXPLOSIONBOMB_BOUND_SPEED_MUL_X() -> f32 {
    unsafe {
        impl_::EXPLOSIONBOMB_BOUND_SPEED_MUL_X()
    }
}

pub fn EXPLOSIONBOMB_BOUND_SPEED_MUL_Y() -> f32 {
    unsafe {
        impl_::EXPLOSIONBOMB_BOUND_SPEED_MUL_Y()
    }
}

pub fn EXPLOSIONBOMB_BURST_DAMAGE() -> f32 {
    unsafe {
        impl_::EXPLOSIONBOMB_BURST_DAMAGE()
    }
}

pub fn EXPLOSIONBOMB_BURST_FRAME() -> i32 {
    unsafe {
        impl_::EXPLOSIONBOMB_BURST_FRAME()
    }
}

pub fn EXPLOSIONBOMB_GRAVITY() -> f32 {
    unsafe {
        impl_::EXPLOSIONBOMB_GRAVITY()
    }
}

pub fn EXPLOSIONBOMB_HOP_SPEED_MUL_X() -> f32 {
    unsafe {
        impl_::EXPLOSIONBOMB_HOP_SPEED_MUL_X()
    }
}

pub fn EXPLOSIONBOMB_HOP_SPEED_Y() -> f32 {
    unsafe {
        impl_::EXPLOSIONBOMB_HOP_SPEED_Y()
    }
}

pub fn EXPLOSIONBOMB_IGNITION_FRAME() -> i32 {
    unsafe {
        impl_::EXPLOSIONBOMB_IGNITION_FRAME()
    }
}

pub fn EXPLOSIONBOMB_LIFE_MISFIRE() -> i32 {
    unsafe {
        impl_::EXPLOSIONBOMB_LIFE_MISFIRE()
    }
}

pub fn EXPLOSIONBOMB_LIMIT_GRAVITY() -> f32 {
    unsafe {
        impl_::EXPLOSIONBOMB_LIMIT_GRAVITY()
    }
}

pub fn EXPLOSIONBOMB_WIRE_BOUND_MUL_Y() -> f32 {
    unsafe {
        impl_::EXPLOSIONBOMB_WIRE_BOUND_MUL_Y()
    }
}

pub fn EXPLOSIONBOMB_WIRE_GRAVITY() -> f32 {
    unsafe {
        impl_::EXPLOSIONBOMB_WIRE_GRAVITY()
    }
}

pub fn EXPLOSIONBOMB_WIRE_ROT_SPEED() -> i32 {
    unsafe {
        impl_::EXPLOSIONBOMB_WIRE_ROT_SPEED()
    }
}

pub fn EXPLOSIONBOMB_WIRE_SPEED_X() -> f32 {
    unsafe {
        impl_::EXPLOSIONBOMB_WIRE_SPEED_X()
    }
}

pub fn EXPLOSIONBOMB_WIRE_SPEED_Y() -> f32 {
    unsafe {
        impl_::EXPLOSIONBOMB_WIRE_SPEED_Y()
    }
}

pub fn set_notify_article_event_eject(module_accessor: &mut app::ItemModuleAccessor) {
    unsafe {
        impl_::set_notify_article_event_eject(module_accessor)
    }
}

