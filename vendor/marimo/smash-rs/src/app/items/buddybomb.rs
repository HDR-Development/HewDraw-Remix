use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app9buddybomb19bound_se_speed_lessEv"]
        pub(super) fn bound_se_speed_less() -> f32;
    
        #[link_name = "_ZN3app9buddybomb16calc_bound_speedERNS_26BattleObjectModuleAccessorEijN3phx6Hash40E"]
        pub(super) fn calc_bound_speed(module_accessor: *mut app::BattleObjectModuleAccessor, work_int_id: i32, battle_object_id: u32, param_struct_name: phx::Hash40);
    
        #[link_name = "_ZN3app9buddybomb31flashing_frame_before_life_overEv"]
        pub(super) fn flashing_frame_before_life_over() -> f32;
    
        #[link_name = "_ZN3app9buddybomb16init_bound_frameEv"]
        pub(super) fn init_bound_frame() -> i32;
    
        #[link_name = "_ZN3app9buddybomb23length_angle_x_velocityEv"]
        pub(super) fn length_angle_x_velocity() -> f32;
    
        #[link_name = "_ZN3app9buddybomb23length_angle_y_velocityEv"]
        pub(super) fn length_angle_y_velocity() -> f32;
    
        #[link_name = "_ZN3app9buddybomb23length_angle_z_velocityEv"]
        pub(super) fn length_angle_z_velocity() -> f32;
    
        #[link_name = "_ZN3app9buddybomb14length_gravityEv"]
        pub(super) fn length_gravity() -> f32;
    
        #[link_name = "_ZN3app9buddybomb18length_speed_y_maxEv"]
        pub(super) fn length_speed_y_max() -> f32;
    
        #[link_name = "_ZN3app9buddybomb4lifeEv"]
        pub(super) fn life() -> f32;
    
        #[link_name = "_ZN3app9buddybomb19rebound_speed_x_addEv"]
        pub(super) fn rebound_speed_x_add() -> f32;
    
        #[link_name = "_ZN3app9buddybomb19rebound_speed_y_addEv"]
        pub(super) fn rebound_speed_y_add() -> f32;
    
        #[link_name = "_ZN3app9buddybomb20request_bound_effectEP9lua_StateRKN3phx8Vector2fE"]
        pub(super) fn request_bound_effect(state: *mut lua_State, pos: *const phx::Vector2f);
    
        #[link_name = "_ZN3app9buddybomb21side_angle_x_velocityEv"]
        pub(super) fn side_angle_x_velocity() -> f32;
    
        #[link_name = "_ZN3app9buddybomb21side_angle_y_velocityEv"]
        pub(super) fn side_angle_y_velocity() -> f32;
    
        #[link_name = "_ZN3app9buddybomb21side_angle_z_velocityEv"]
        pub(super) fn side_angle_z_velocity() -> f32;
    
        #[link_name = "_ZN3app9buddybomb12side_gravityEv"]
        pub(super) fn side_gravity() -> f32;
    
        #[link_name = "_ZN3app9buddybomb16side_speed_y_maxEv"]
        pub(super) fn side_speed_y_max() -> f32;
    
        #[link_name = "_ZN3app9buddybomb18special_lw_gravityEv"]
        pub(super) fn special_lw_gravity() -> f32;
    
        #[link_name = "_ZN3app9buddybomb22special_lw_speed_y_maxEv"]
        pub(super) fn special_lw_speed_y_max() -> f32;
    
    }
}

pub fn bound_se_speed_less() -> f32 {
    unsafe {
        impl_::bound_se_speed_less()
    }
}

pub fn calc_bound_speed(module_accessor: &mut app::BattleObjectModuleAccessor, work_int_id: i32, battle_object_id: u32, param_struct_name: phx::Hash40) {
    unsafe {
        impl_::calc_bound_speed(module_accessor, work_int_id, battle_object_id, param_struct_name)
    }
}

pub fn flashing_frame_before_life_over() -> f32 {
    unsafe {
        impl_::flashing_frame_before_life_over()
    }
}

pub fn init_bound_frame() -> i32 {
    unsafe {
        impl_::init_bound_frame()
    }
}

pub fn length_angle_x_velocity() -> f32 {
    unsafe {
        impl_::length_angle_x_velocity()
    }
}

pub fn length_angle_y_velocity() -> f32 {
    unsafe {
        impl_::length_angle_y_velocity()
    }
}

pub fn length_angle_z_velocity() -> f32 {
    unsafe {
        impl_::length_angle_z_velocity()
    }
}

pub fn length_gravity() -> f32 {
    unsafe {
        impl_::length_gravity()
    }
}

pub fn length_speed_y_max() -> f32 {
    unsafe {
        impl_::length_speed_y_max()
    }
}

pub fn life() -> f32 {
    unsafe {
        impl_::life()
    }
}

pub fn rebound_speed_x_add() -> f32 {
    unsafe {
        impl_::rebound_speed_x_add()
    }
}

pub fn rebound_speed_y_add() -> f32 {
    unsafe {
        impl_::rebound_speed_y_add()
    }
}

pub fn request_bound_effect(state: *mut lua_State, pos: &phx::Vector2f) {
    unsafe {
        impl_::request_bound_effect(state, pos)
    }
}

pub fn side_angle_x_velocity() -> f32 {
    unsafe {
        impl_::side_angle_x_velocity()
    }
}

pub fn side_angle_y_velocity() -> f32 {
    unsafe {
        impl_::side_angle_y_velocity()
    }
}

pub fn side_angle_z_velocity() -> f32 {
    unsafe {
        impl_::side_angle_z_velocity()
    }
}

pub fn side_gravity() -> f32 {
    unsafe {
        impl_::side_gravity()
    }
}

pub fn side_speed_y_max() -> f32 {
    unsafe {
        impl_::side_speed_y_max()
    }
}

pub fn special_lw_gravity() -> f32 {
    unsafe {
        impl_::special_lw_gravity()
    }
}

pub fn special_lw_speed_y_max() -> f32 {
    unsafe {
        impl_::special_lw_speed_y_max()
    }
}

