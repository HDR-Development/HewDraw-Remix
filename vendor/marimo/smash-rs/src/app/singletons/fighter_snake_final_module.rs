use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app23FighterSnakeFinalModule9end_finalERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn end_final(module_accessor: *mut app::BattleObjectModuleAccessor);
    
        #[link_name = "_ZN3app23FighterSnakeFinalModule22get_guide_point_offsetEii"]
        pub(super) fn get_guide_point_offset(missile_num: i32, point_num: i32) -> cpp::simd::Vector3;
    
        #[link_name = "_ZN3app23FighterSnakeFinalModule21get_lock_on_directionERKN3phx8Vector3fES4_fRS2_"]
        pub(super) fn get_lock_on_direction(arg1: *const phx::Vector3f, arg2: *const phx::Vector3f, arg3: f32, arg4: *mut phx::Vector3f) -> f32;
    
        #[link_name = "_ZN3app23FighterSnakeFinalModule20get_lock_on_positionEi"]
        pub(super) fn get_lock_on_position(missile_num: i32) -> cpp::simd::Vector3;
    
        #[link_name = "_ZN3app23FighterSnakeFinalModule20get_lock_on_rotationERKN3phx8Vector3fES4_"]
        pub(super) fn get_lock_on_rotation(arg1: *const phx::Vector3f, arg2: *const phx::Vector3f) -> cpp::simd::Vector3;
    
        #[link_name = "_ZN3app23FighterSnakeFinalModule20get_missile_positionEi"]
        pub(super) fn get_missile_position(missile_num: i32) -> cpp::simd::Vector3;
    
        #[link_name = "_ZN3app23FighterSnakeFinalModule18initialize_reticleERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn initialize_reticle(module_accessor: *mut app::BattleObjectModuleAccessor);
    
        #[link_name = "_ZN3app23FighterSnakeFinalModule7lock_onERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn lock_on(module_accessor: *mut app::BattleObjectModuleAccessor);
    
        #[link_name = "_ZN3app23FighterSnakeFinalModule13lock_on_readyERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn lock_on_ready(module_accessor: *mut app::BattleObjectModuleAccessor);
    
        #[link_name = "_ZN3app23FighterSnakeFinalModule5startERNS_26BattleObjectModuleAccessorE"]
        pub(super) fn start(module_accessor: *mut app::BattleObjectModuleAccessor);
    
        #[link_name = "_ZN3app23FighterSnakeFinalModule14start_zoom_outERNS_26BattleObjectModuleAccessorEi"]
        pub(super) fn start_zoom_out(module_accessor: *mut app::BattleObjectModuleAccessor, arg: i32);
    
        #[link_name = "_ZN3app23FighterSnakeFinalModule14update_reticleERNS_26BattleObjectModuleAccessorEff"]
        pub(super) fn update_reticle(module_accessor: *mut app::BattleObjectModuleAccessor, x: f32, y: f32);
    
    }
}

pub fn end_final(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::end_final(module_accessor)
    }
}

pub fn get_guide_point_offset(missile_num: i32, point_num: i32) -> phx::Vector3f {
    unsafe {
        impl_::get_guide_point_offset(missile_num, point_num).into()
    }
}

pub fn get_lock_on_direction(arg1: &phx::Vector3f, arg2: &phx::Vector3f, arg3: f32, arg4: &mut phx::Vector3f) -> f32 {
    unsafe {
        impl_::get_lock_on_direction(arg1, arg2, arg3, arg4)
    }
}

pub fn get_lock_on_position(missile_num: i32) -> phx::Vector3f {
    unsafe {
        impl_::get_lock_on_position(missile_num).into()
    }
}

pub fn get_lock_on_rotation(arg1: &phx::Vector3f, arg2: &phx::Vector3f) -> phx::Vector3f {
    unsafe {
        impl_::get_lock_on_rotation(arg1, arg2).into()
    }
}

pub fn get_missile_position(missile_num: i32) -> phx::Vector3f {
    unsafe {
        impl_::get_missile_position(missile_num).into()
    }
}

pub fn initialize_reticle(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::initialize_reticle(module_accessor)
    }
}

pub fn lock_on(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::lock_on(module_accessor)
    }
}

pub fn lock_on_ready(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::lock_on_ready(module_accessor)
    }
}

pub fn start(module_accessor: &mut app::BattleObjectModuleAccessor) {
    unsafe {
        impl_::start(module_accessor)
    }
}

pub fn start_zoom_out(module_accessor: &mut app::BattleObjectModuleAccessor, arg: i32) {
    unsafe {
        impl_::start_zoom_out(module_accessor, arg)
    }
}

pub fn update_reticle(module_accessor: &mut app::BattleObjectModuleAccessor, x: f32, y: f32) {
    unsafe {
        impl_::update_reticle(module_accessor, x, y)
    }
}

