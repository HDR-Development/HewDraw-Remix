use crate::*;

mod impl_ {
    extern "C" {
        #[link_name = "_ZN3app20DamageTransactorImpl19calc_reaction_frameEf"]
        pub(super) fn calc_reaction_frame(damage: f32) -> f32;
    }
}

#[allow(non_snake_case)]
pub mod DamageTransactorImpl {
    use super::impl_;
    
    pub fn calc_reaction_frame(damage: f32) -> f32 {
        unsafe {
            impl_::calc_reaction_frame(damage)
        }
    }
}

#[repr(C)]
pub struct DamageInfo {
    pub damage: f32,
    pub damage_add: f32,
    pub damage_add_reaction: f32,
    pub power_max: f32,
    pub reaction_max: f32,
    unknown: [u8; 0x14C],
    pub damage_lr: f32,
    padding: [f32; 3],
    pub attack_pos: phx::Vector3f,
    padding2: f32,
    pub attack_speed: phx::Vector2f,
    padding3: u64,
    pub damage_add_2nd: f32,
    pub owner_object_id: u32,
    pub shake_scale: f32,
    pub catch_absolute: bool,
    pub no_dead_damage_fly_effect: bool,
    padding4: [u8; 0x22]
}

#[cfg(feature = "type_assert")]
impl DamageInfo {
    pub fn assert() {
        assert_eq!(size_of!(DamageInfo), 0x1C0);
        assert_eq!(offset_of!(DamageInfo, damage), 0x0);
        assert_eq!(offset_of!(DamageInfo, damage_add), 0x4);
        assert_eq!(offset_of!(DamageInfo, damage_add_reaction), 0x8);
        assert_eq!(offset_of!(DamageInfo, power_max), 0xC);
        assert_eq!(offset_of!(DamageInfo, reaction_max), 0x10);
        assert_eq!(offset_of!(DamageInfo, damage_lr), 0x160);
        assert_eq!(offset_of!(DamageInfo, attack_pos), 0x170);
        assert_eq!(offset_of!(DamageInfo, attack_speed), 0x180);
        assert_eq!(offset_of!(DamageInfo, damage_add_2nd), 0x190);
        assert_eq!(offset_of!(DamageInfo, owner_object_id), 0x194);
        assert_eq!(offset_of!(DamageInfo, shake_scale), 0x198);
        assert_eq!(offset_of!(DamageInfo, catch_absolute), 0x19C);
        assert_eq!(offset_of!(DamageInfo, no_dead_damage_fly_effect), 0x19D);
    }
}