use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app26FighterSpecializer_Inkling10change_inkERNS_7FighterEf"]
        pub(super) fn change_ink(fighter: *mut app::Fighter, amount: f32);
    
        #[link_name = "_ZN3app26FighterSpecializer_Inkling19check_roller_groundERNS_7FighterERKN3phx8Vector2fES6_RS4_S7_b"]
        pub(super) fn check_roller_ground(fighter: *mut app::Fighter, joint_pos: *const phx::Vector2f, y_velocity: *const phx::Vector2f, arg: *mut phx::Vector2f, arg2: *mut phx::Vector2f, arg3: bool) -> bool;
    
        #[link_name = "_ZN3app26FighterSpecializer_Inkling18generate_rollerinkERNS_7FighterE"]
        pub(super) fn generate_rollerink(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app26FighterSpecializer_Inkling11get_ink_maxERNS_7FighterE"]
        pub(super) fn get_ink_max(fighter: *mut app::Fighter) -> f32;
    
        #[link_name = "_ZN3app26FighterSpecializer_Inkling15get_ink_work_idEj"]
        pub(super) fn get_ink_work_id(fighter_kind: app::FighterKind) -> i32;
    
        #[link_name = "_ZN3app26FighterSpecializer_Inkling23get_roller_check_velo_yERNS_7FighterE"]
        pub(super) fn get_roller_check_velo_y(fighter: *mut app::Fighter) -> cpp::simd::Vector2;
    
        #[link_name = "_ZN3app26FighterSpecializer_Inkling22get_sub_ink_special_lwERNS_7FighterE"]
        pub(super) fn get_sub_ink_special_lw(fighter: *mut app::Fighter) -> f32;
    
        #[link_name = "_ZN3app26FighterSpecializer_Inkling21get_sub_ink_special_nERNS_7FighterE"]
        pub(super) fn get_sub_ink_special_n(fighter: *mut app::Fighter) -> f32;
    
        #[link_name = "_ZN3app26FighterSpecializer_Inkling15is_body_visibleERNS_7FighterE"]
        pub(super) fn is_body_visible(fighter: *mut app::Fighter) -> bool;
    
        #[link_name = "_ZN3app26FighterSpecializer_Inkling22is_paintable_rollerinkERNS_7FighterE"]
        pub(super) fn is_paintable_rollerink(fighter: *mut app::Fighter) -> bool;
    
        #[link_name = "_ZN3app26FighterSpecializer_Inkling8lack_inkERNS_7FighterE"]
        pub(super) fn lack_ink(fighter: *mut app::Fighter);
    
        #[link_name = "_ZN3app26FighterSpecializer_Inkling13request_paintERNS_7FighterEN3phx6Hash40ERNS3_8Vector3fERNS3_8Vector2fEf"]
        pub(super) fn request_paint(fighter: *mut app::Fighter, joint: phx::Hash40, joint_offset: *mut phx::Vector3f, arg: *mut phx::Vector2f, arg2: f32);
    
    }
}

pub fn change_ink(fighter: &mut app::Fighter, amount: f32) {
    unsafe {
        impl_::change_ink(fighter, amount)
    }
}

pub fn check_roller_ground(fighter: &mut app::Fighter, joint_pos: &phx::Vector2f, y_velocity: &phx::Vector2f, arg: &mut phx::Vector2f, arg2: &mut phx::Vector2f, arg3: bool) -> bool {
    unsafe {
        impl_::check_roller_ground(fighter, joint_pos, y_velocity, arg, arg2, arg3)
    }
}

pub fn generate_rollerink(fighter: &mut app::Fighter) {
    unsafe {
        impl_::generate_rollerink(fighter)
    }
}

pub fn get_ink_max(fighter: &mut app::Fighter) -> f32 {
    unsafe {
        impl_::get_ink_max(fighter)
    }
}

pub fn get_ink_work_id(fighter_kind: app::FighterKind) -> i32 {
    unsafe {
        impl_::get_ink_work_id(fighter_kind)
    }
}

pub fn get_roller_check_velo_y(fighter: &mut app::Fighter) -> phx::Vec2 {
    unsafe {
        impl_::get_roller_check_velo_y(fighter).into()
    }
}

pub fn get_sub_ink_special_lw(fighter: &mut app::Fighter) -> f32 {
    unsafe {
        impl_::get_sub_ink_special_lw(fighter)
    }
}

pub fn get_sub_ink_special_n(fighter: &mut app::Fighter) -> f32 {
    unsafe {
        impl_::get_sub_ink_special_n(fighter)
    }
}

pub fn is_body_visible(fighter: &mut app::Fighter) -> bool {
    unsafe {
        impl_::is_body_visible(fighter)
    }
}

pub fn is_paintable_rollerink(fighter: &mut app::Fighter) -> bool {
    unsafe {
        impl_::is_paintable_rollerink(fighter)
    }
}

pub fn lack_ink(fighter: &mut app::Fighter) {
    unsafe {
        impl_::lack_ink(fighter)
    }
}

pub fn request_paint(fighter: &mut app::Fighter, joint: phx::Hash40, joint_offset: &mut phx::Vector3f, arg: &mut phx::Vector2f, arg2: f32) {
    unsafe {
        impl_::request_paint(fighter, joint, joint_offset, arg, arg2)
    }
}

