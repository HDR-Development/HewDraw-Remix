use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app22FighterSpecializer_Fox14final_end_execERNS_7FighterE"]
        pub(super) fn final_end_exec(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app22FighterSpecializer_Fox14final_end_exitERNS_7FighterE"]
        pub(super) fn final_end_exit(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app22FighterSpecializer_Fox14final_end_initERNS_7FighterE"]
        pub(super) fn final_end_init(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app22FighterSpecializer_Fox16final_ready_execERNS_7FighterE"]
        pub(super) fn final_ready_exec(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app22FighterSpecializer_Fox16final_ready_exitERNS_7FighterE"]
        pub(super) fn final_ready_exit(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app22FighterSpecializer_Fox20final_ready_exit_preERNS_7FighterE"]
        pub(super) fn final_ready_exit_pre(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app22FighterSpecializer_Fox16final_ready_initERNS_7FighterE"]
        pub(super) fn final_ready_init(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app22FighterSpecializer_Fox26final_remove_screen_effectEv"]
        pub(super) fn final_remove_screen_effect();
    
        #[link_name = "_ZN3app22FighterSpecializer_Fox26final_scene01_exec_fix_posERNS_7FighterE"]
        pub(super) fn final_scene01_exec_fix_pos(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app22FighterSpecializer_Fox18final_scene01_exitERNS_7FighterE"]
        pub(super) fn final_scene01_exit(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app22FighterSpecializer_Fox18final_scene01_initERNS_7FighterE"]
        pub(super) fn final_scene01_init(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app22FighterSpecializer_Fox16final_start_exitERNS_7FighterE"]
        pub(super) fn final_start_exit(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app22FighterSpecializer_Fox16final_start_initERNS_7FighterE"]
        pub(super) fn final_start_init(fighter: *mut app::Fighter);
    
    }
}

pub fn final_end_exec(fighter: &mut app::Fighter) {
    unsafe {
        impl_::final_end_exec(fighter)
    }
}

pub fn final_end_exit(fighter: &mut app::Fighter) {
    unsafe {
        impl_::final_end_exit(fighter)
    }
}

pub fn final_end_init(fighter: &mut app::Fighter) {
    unsafe {
        impl_::final_end_init(fighter)
    }
}

pub fn final_ready_exec(fighter: &mut app::Fighter) {
    unsafe {
        impl_::final_ready_exec(fighter)
    }
}

pub fn final_ready_exit(fighter: &mut app::Fighter) {
    unsafe {
        impl_::final_ready_exit(fighter)
    }
}

pub fn final_ready_exit_pre(fighter: &mut app::Fighter) {
    unsafe {
        impl_::final_ready_exit_pre(fighter)
    }
}

pub fn final_ready_init(fighter: &mut app::Fighter) {
    unsafe {
        impl_::final_ready_init(fighter)
    }
}

pub fn final_remove_screen_effect() {
    unsafe {
        impl_::final_remove_screen_effect()
    }
}

pub fn final_scene01_exec_fix_pos(fighter: &mut app::Fighter) {
    unsafe {
        impl_::final_scene01_exec_fix_pos(fighter)
    }
}

pub fn final_scene01_exit(fighter: &mut app::Fighter) {
    unsafe {
        impl_::final_scene01_exit(fighter)
    }
}

pub fn final_scene01_init(fighter: &mut app::Fighter) {
    unsafe {
        impl_::final_scene01_init(fighter)
    }
}

pub fn final_start_exit(fighter: &mut app::Fighter) {
    unsafe {
        impl_::final_start_exit(fighter)
    }
}

pub fn final_start_init(fighter: &mut app::Fighter) {
    unsafe {
        impl_::final_start_init(fighter)
    }
}

