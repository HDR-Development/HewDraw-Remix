use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3lib9SingletonIN3app17BattleObjectWorldEE9instance_E"]
        pub(super) static INSTANCE: *mut app::BattleObjectWorld;

        #[link_name = "_ZN3app8lua_bind35BattleObjectWorld__gravity_pos_implEPNS_17BattleObjectWorldE"]
        pub(super) fn gravity_pos(battle_object_world: *const app::BattleObjectWorld) -> cpp::simd::Vector3;

        #[link_name = "_ZN3app8lua_bind49BattleObjectWorld__gravity_speed_coefficient_implEPNS_17BattleObjectWorldE"]
        pub(super) fn gravity_speed_coefficient(battle_object_world: *const app::BattleObjectWorld) -> f32;

        #[link_name = "_ZN3app8lua_bind42BattleObjectWorld__is_disable_reverse_implEPNS_17BattleObjectWorldE"]
        pub(super) fn is_disable_reverse(battle_object_world: *const app::BattleObjectWorld) -> bool;

        #[link_name = "_ZN3app8lua_bind41BattleObjectWorld__is_gravity_normal_implEPNS_17BattleObjectWorldE"]
        pub(super) fn is_gravity_normal(battle_object_world: *const app::BattleObjectWorld) -> bool;

        #[link_name = "_ZN3app8lua_bind31BattleObjectWorld__is_move_implEPNS_17BattleObjectWorldE"]
        pub(super) fn is_move(battle_object_world: *const app::BattleObjectWorld) -> bool;

        #[link_name = "_ZN3app8lua_bind34BattleObjectWorld__move_speed_implEPNS_17BattleObjectWorldE"]
        pub(super) fn move_speed(battle_object_world: *const app::BattleObjectWorld) -> *const cpp::simd::Vector3;

        #[link_name = "_ZN3app8lua_bind31BattleObjectWorld__scale_z_implEPNS_17BattleObjectWorldE"]
        pub(super) fn scale_z(battle_object_world: *const app::BattleObjectWorld) -> f32;
    }
}

#[repr(C)]
pub struct BattleObjectWorld {
    pub gravity_speed: f32,
    pub gravity_speed_coefficient: f32,
    scale_z: f32,
    unk1: bool,
    pub gravity_pos: cpp::simd::Vector3,
    move_speed: cpp::simd::Vector3,
    unk2: u64,
    unk3: u64,
    unk4: *const u64,
    unk5: *const u64,
    unk6: u64,
    is_stop_battle_objects: bool,
    pub is_gravity_normal: bool,
    is_move: bool,
    unk7: bool,
    unk8: bool,
    is_disable_reverse: bool,
    unk9: bool,
    unk10: bool,
    unk11: *const u64,
}

impl BattleObjectWorld {
    pub fn instance() -> Option<&'static Self> {
        unsafe {
            impl_::INSTANCE.as_ref()
        }
    }

    pub fn instance_mut() -> Option<&'static mut Self> {
        unsafe {
            impl_::INSTANCE.as_mut()
        }
    }

    pub fn gravity_pos(&self) -> phx::Vector3f {
        unsafe {
            impl_::gravity_pos(self).into()
        }
    }

    pub fn gravity_speed_coefficient(&self) -> f32 {
        unsafe {
            impl_::gravity_speed_coefficient(self)
        }
    }

    pub fn is_disable_reverse(&self) -> bool {
        unsafe {
            impl_::is_disable_reverse(self)
        }
    }

    pub fn is_gravity_normal(&self) -> bool {
        unsafe {
            impl_::is_gravity_normal(self)
        }
    }

    pub fn is_move(&self) -> bool {
        unsafe {
            impl_::is_move(self)
        }
    }

    pub fn move_speed(&self) -> &phx::Vector3f {
        unsafe {
            &*(impl_::move_speed(self) as *const phx::Vector3f)
        }
    }

    pub fn scale_z(&self) -> f32 {
        unsafe {
            impl_::scale_z(self)
        }
    }
}
