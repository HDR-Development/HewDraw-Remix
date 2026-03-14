
#[allow(non_snake_case)]
pub mod FighterItemModuleImpl {
    use crate::*;
    mod impl_ {
        use crate::*;
    
        extern "C" {
            #[link_name = "_ZN3app21FighterItemModuleImpl30get_fighter_throw_param_memberEN3phx6Hash40ES2_"]
            pub(super) fn get_fighter_throw_param_member(struct_name: phx::Hash40, field_name: phx::Hash40) -> f32;
        
        }
    }
    
    pub fn get_fighter_throw_param_member(struct_name: phx::Hash40, field_name: phx::Hash40) -> f32 {
        unsafe {
            impl_::get_fighter_throw_param_member(struct_name, field_name)
        }
    }
}
