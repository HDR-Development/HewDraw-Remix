use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app10ai_utility8get_kindEj"]
        pub(super) fn get_kind(battle_object_id: u32) -> app::FighterKind;
    
    }
}

pub fn get_kind(battle_object_id: u32) -> app::FighterKind {
    unsafe {
        impl_::get_kind(battle_object_id)
    }
}