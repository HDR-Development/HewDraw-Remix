mod impl_ {
    extern "C" {
        #[link_name = "_ZN3app9ai_camera8dead_topEv"]
        pub(super) fn dead_top() -> f32;
    
        #[link_name = "_ZN3app9ai_camera11dead_bottomEv"]
        pub(super) fn dead_bottom() -> f32;
    
        #[link_name = "_ZN3app9ai_camera9dead_leftEv"]
        pub(super) fn dead_left() -> f32;
    
        #[link_name = "_ZN3app9ai_camera10dead_rightEv"]
        pub(super) fn dead_right() -> f32;
    }    
}

pub fn dead_top() -> f32 {
    unsafe {
        impl_::dead_top()
    }
}

pub fn dead_bottom() -> f32 {
    unsafe {
        impl_::dead_bottom()
    }
}

pub fn dead_left() -> f32 {
    unsafe {
        impl_::dead_left()
    }
}

pub fn dead_right() -> f32 {
    unsafe {
        impl_::dead_right()
    }
}