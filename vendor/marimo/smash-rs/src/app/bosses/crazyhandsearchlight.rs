mod impl_ {
    extern "C" {
        #[link_name = "_ZN3app20crazyhandsearchlight17add_flower_damageEiff"]
        pub(super) fn add_flower_damage(battle_object_id: u32, frames: f32, damage: f32);
    
    }
}

pub fn add_flower_damage(battle_object_id: u32, frames: f32, damage: f32) {
    unsafe {
        impl_::add_flower_damage(battle_object_id, frames, damage)
    }
}

