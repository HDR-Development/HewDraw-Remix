use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app8ai_param8air_highEP9lua_State"]
        pub(super) fn air_high(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param10air_lengthEP9lua_State"]
        pub(super) fn air_length(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param25catch_attack_cancel_frameEP9lua_State"]
        pub(super) fn catch_attack_cancel_frame(state: *mut lua_State) -> u8;
    
        #[link_name = "_ZN3app8ai_param21challenger_guard_rateEP9lua_State"]
        pub(super) fn challenger_guard_rate(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param10closed_mulEP9lua_State"]
        pub(super) fn closed_mul(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param16dive_speed_y_maxEP9lua_State"]
        pub(super) fn dive_speed_y_max(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param23escape_air_cancel_frameEP9lua_State"]
        pub(super) fn escape_air_cancel_frame(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param30escape_air_slide_move_distanceEP9lua_State"]
        pub(super) fn escape_air_slide_move_distance(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param16fall_speed_y_maxEP9lua_State"]
        pub(super) fn fall_speed_y_max(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param14has_attack_100EP9lua_State"]
        pub(super) fn has_attack_100(state: *mut lua_State) -> bool;
    
        #[link_name = "_ZN3app8ai_param6jump_gEP9lua_State"]
        pub(super) fn jump_g(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param10jump_g_mulEP9lua_State"]
        pub(super) fn jump_g_mul(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param9jump_highEP9lua_State"]
        pub(super) fn jump_high(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param25meteor_action_probabilityEP9lua_State"]
        pub(super) fn meteor_action_probability(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param12my_range_mulEP9lua_State"]
        pub(super) fn my_range_mul(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param17num_of_attack_123EP9lua_State"]
        pub(super) fn num_of_attack_123(state: *mut lua_State) -> i32;
    
        #[link_name = "_ZN3app8ai_param10opened_mulEP9lua_State"]
        pub(super) fn opened_mul(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param11param_floatEP9lua_StateN3phx6Hash40E"]
        pub(super) fn param_float(state: *mut lua_State, param_name: phx::Hash40) -> f32;
    
        #[link_name = "_ZN3app8ai_param9param_intEP9lua_StateN3phx6Hash40E"]
        pub(super) fn param_int(state: *mut lua_State, param_name: phx::Hash40) -> i32;
    
        #[link_name = "_ZN3app8ai_param10range_longEP9lua_State"]
        pub(super) fn range_long(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param12range_middleEP9lua_State"]
        pub(super) fn range_middle(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param24return_goal_pad_x_strictEP9lua_State"]
        pub(super) fn return_goal_pad_x_strict(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param13return_goal_xEP9lua_State"]
        pub(super) fn return_goal_x(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param20return_goal_x_strictEP9lua_State"]
        pub(super) fn return_goal_x_strict(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param19return_sp_u_cliff_xEP9lua_State"]
        pub(super) fn return_sp_u_cliff_x(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param19return_sp_u_cliff_yEP9lua_State"]
        pub(super) fn return_sp_u_cliff_y(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param14safe_range_mulEP9lua_State"]
        pub(super) fn safe_range_mul(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param9sp_u_highEP9lua_State"]
        pub(super) fn sp_u_high(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param11sp_u_lengthEP9lua_State"]
        pub(super) fn sp_u_length(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param17target_range_longEP9lua_State"]
        pub(super) fn target_range_long(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param19target_range_middleEP9lua_State"]
        pub(super) fn target_range_middle(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param23training_smash_intervalEP9lua_State"]
        pub(super) fn training_smash_interval(state: *mut lua_State) -> i32;
    
        #[link_name = "_ZN3app8ai_param25training_special_intervalEP9lua_State"]
        pub(super) fn training_special_interval(state: *mut lua_State) -> i32;
    
        #[link_name = "_ZN3app8ai_param5widthEP9lua_State"]
        pub(super) fn width(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param15wire_length_maxEP9lua_State"]
        pub(super) fn wire_length_max(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param15wire_length_minEP9lua_State"]
        pub(super) fn wire_length_min(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param10wire_rangeEP9lua_State"]
        pub(super) fn wire_range(state: *mut lua_State) -> cpp::simd::Vector2;
    
        #[link_name = "_ZN3app8ai_param17wire_range_offsetEP9lua_State"]
        pub(super) fn wire_range_offset(state: *mut lua_State) -> cpp::simd::Vector2;
    
        #[link_name = "_ZN3app8ai_param16wire_reach_frameEP9lua_State"]
        pub(super) fn wire_reach_frame(state: *mut lua_State) -> f32;
    
        #[link_name = "_ZN3app8ai_param18zako2_search_rangeEP9lua_State"]
        pub(super) fn zako2_search_range(state: *mut lua_State) -> f32;
    
    }
}

pub fn air_high(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::air_high(state)
    }
}

pub fn air_length(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::air_length(state)
    }
}

pub fn catch_attack_cancel_frame(state: *mut lua_State) -> u8 {
    unsafe {
        impl_::catch_attack_cancel_frame(state)
    }
}

pub fn challenger_guard_rate(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::challenger_guard_rate(state)
    }
}

pub fn closed_mul(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::closed_mul(state)
    }
}

pub fn dive_speed_y_max(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::dive_speed_y_max(state)
    }
}

pub fn escape_air_cancel_frame(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::escape_air_cancel_frame(state)
    }
}

pub fn escape_air_slide_move_distance(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::escape_air_slide_move_distance(state)
    }
}

pub fn fall_speed_y_max(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::fall_speed_y_max(state)
    }
}

pub fn has_attack_100(state: *mut lua_State) -> bool {
    unsafe {
        impl_::has_attack_100(state)
    }
}

pub fn jump_g(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::jump_g(state)
    }
}

pub fn jump_g_mul(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::jump_g_mul(state)
    }
}

pub fn jump_high(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::jump_high(state)
    }
}

pub fn meteor_action_probability(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::meteor_action_probability(state)
    }
}

pub fn my_range_mul(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::my_range_mul(state)
    }
}

pub fn num_of_attack_123(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::num_of_attack_123(state)
    }
}

pub fn opened_mul(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::opened_mul(state)
    }
}

pub fn param_float(state: *mut lua_State, param_name: phx::Hash40) -> f32 {
    unsafe {
        impl_::param_float(state, param_name)
    }
}

pub fn param_int(state: *mut lua_State, param_name: phx::Hash40) -> i32 {
    unsafe {
        impl_::param_int(state, param_name)
    }
}

pub fn range_long(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::range_long(state)
    }
}

pub fn range_middle(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::range_middle(state)
    }
}

pub fn return_goal_pad_x_strict(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::return_goal_pad_x_strict(state)
    }
}

pub fn return_goal_x(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::return_goal_x(state)
    }
}

pub fn return_goal_x_strict(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::return_goal_x_strict(state)
    }
}

pub fn return_sp_u_cliff_x(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::return_sp_u_cliff_x(state)
    }
}

pub fn return_sp_u_cliff_y(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::return_sp_u_cliff_y(state)
    }
}

pub fn safe_range_mul(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::safe_range_mul(state)
    }
}

pub fn sp_u_high(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::sp_u_high(state)
    }
}

pub fn sp_u_length(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::sp_u_length(state)
    }
}

pub fn target_range_long(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_range_long(state)
    }
}

pub fn target_range_middle(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::target_range_middle(state)
    }
}

pub fn training_smash_interval(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::training_smash_interval(state)
    }
}

pub fn training_special_interval(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::training_special_interval(state)
    }
}

pub fn width(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::width(state)
    }
}

pub fn wire_length_max(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::wire_length_max(state)
    }
}

pub fn wire_length_min(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::wire_length_min(state)
    }
}

pub fn wire_range(state: *mut lua_State) -> phx::Vec2 {
    unsafe {
        impl_::wire_range(state).into()
    }
}

pub fn wire_range_offset(state: *mut lua_State) -> phx::Vec2 {
    unsafe {
        impl_::wire_range_offset(state).into()
    }
}

pub fn wire_reach_frame(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::wire_reach_frame(state)
    }
}

pub fn zako2_search_range(state: *mut lua_State) -> f32 {
    unsafe {
        impl_::zako2_search_range(state)
    }
}

