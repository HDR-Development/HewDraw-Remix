use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app25FighterSpecializer_Dedede28check_special_s_pickup_goldoERNS_7FighterE"]
        pub(super) fn check_special_s_pickup_goldo(fighter: *mut app::Fighter) -> bool;
    
        #[link_name = "_ZN3app25FighterSpecializer_Dedede29end_special_n_shot_object_hitERNS_7FighterE"]
        pub(super) fn end_special_n_shot_object_hit(fighter: *mut app::Fighter);
    
    }
}

pub fn check_special_s_pickup_goldo(fighter: &mut app::Fighter) -> bool {
    unsafe {
        impl_::check_special_s_pickup_goldo(fighter)
    }
}

pub fn end_special_n_shot_object_hit(fighter: &mut app::Fighter) {
    unsafe {
        impl_::end_special_n_shot_object_hit(fighter)
    }
}

