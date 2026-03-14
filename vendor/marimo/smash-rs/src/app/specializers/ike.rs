use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app22FighterSpecializer_Ike25get_final_target_pos_baseERNS_21FighterModuleAccessorE"]
        pub(super) fn get_final_target_pos_base(module_accessor: *mut app::FighterModuleAccessor) -> cpp::simd::Vector2;
    
        #[link_name = "_ZN3app22FighterSpecializer_Ike30get_final_target_pos_base_workERNS_21FighterModuleAccessorE"]
        pub(super) fn get_final_target_pos_base_work(module_accessor: *mut app::FighterModuleAccessor) -> cpp::simd::Vector2;
    
    }
}

pub fn get_final_target_pos_base(module_accessor: &mut app::FighterModuleAccessor) -> phx::Vec2 {
    unsafe {
        impl_::get_final_target_pos_base(module_accessor).into()
    }
}

pub fn get_final_target_pos_base_work(module_accessor: &mut app::FighterModuleAccessor) -> phx::Vec2 {
    unsafe {
        impl_::get_final_target_pos_base_work(module_accessor).into()
    }
}

