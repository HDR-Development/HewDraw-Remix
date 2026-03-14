mod impl_ {
    extern "C" {
        #[link_name = "_ZN3app24FighterSpecializer_Cloud20display_final_windowEb"]
        pub(super) fn display_final_window(arg: bool);
    
        #[link_name = "_ZN3app24FighterSpecializer_Cloud27display_final_window_final2Eb"]
        pub(super) fn display_final_window_final2(arg: bool);
    
        #[link_name = "_ZN3app24FighterSpecializer_Cloud20is_camera_off_final2Ev"]
        pub(super) fn is_camera_off_final2() -> bool;
    
    }
}

pub fn display_final_window(arg: bool) {
    unsafe {
        impl_::display_final_window(arg)
    }
}

pub fn display_final_window_final2(arg: bool) {
    unsafe {
        impl_::display_final_window_final2(arg)
    }
}

pub fn is_camera_off_final2() -> bool {
    unsafe {
        impl_::is_camera_off_final2()
    }
}

