use crate::*;

mod impl_ {
    use crate::*;

    extern "C" {
        #[link_name = "_ZN3app9ai_weapon4flagEP9lua_StatePKNS_15FighterAIWeaponE"]
        pub(super) fn flag(state: *mut lua_State, weapon: *const app::FighterAIWeapon) -> app::FighterAIWeaponFlag;
    
        #[link_name = "_ZN3app9ai_weapon14have_item_kindEP9lua_State"]
        pub(super) fn have_item_kind(state: *mut lua_State) -> app::ItemKind;
    
        #[link_name = "_ZN3app9ai_weapon4kindEP9lua_StatePKNS_15FighterAIWeaponE"]
        pub(super) fn kind(state: *mut lua_State, weapon: *const app::FighterAIWeapon) -> app::WeaponKind;
    
        #[link_name = "_ZN3app9ai_weapon23most_earliest_hit_frameEP9lua_State"]
        pub(super) fn most_earliest_hit_frame(state: *mut lua_State) -> i32;
    
        #[link_name = "_ZN3app9ai_weapon20most_earliest_weaponEP9lua_State"]
        pub(super) fn most_earliest_weapon(state: *mut lua_State) -> *mut app::FighterAIWeapon;
    
        #[link_name = "_ZN3app9ai_weapon5pos_xEP9lua_StatePKNS_15FighterAIWeaponE"]
        pub(super) fn pos_x(state: *mut lua_State, weapon: *const app::FighterAIWeapon) -> f32;
    
        #[link_name = "_ZN3app9ai_weapon12speed_lengthEP9lua_StatePKNS_15FighterAIWeaponE"]
        pub(super) fn speed_length(state: *mut lua_State, weapon: *const app::FighterAIWeapon) -> cpp::simd::Vector2;
    
        #[link_name = "_ZN3app9ai_weapon7speed_xEP9lua_StatePKNS_15FighterAIWeaponE"]
        pub(super) fn speed_x(state: *mut lua_State, weapon: *const app::FighterAIWeapon) -> f32;
    
        #[link_name = "_ZN3app9ai_weapon4typeEP9lua_StatePKNS_15FighterAIWeaponE"]
        pub(super) fn type_(state: *mut lua_State, weapon: *const app::FighterAIWeapon) -> app::FighterAIWeaponType;
    
    }
}

pub fn flag(state: *mut lua_State, weapon: *const app::FighterAIWeapon) -> app::FighterAIWeaponFlag {
    unsafe {
        impl_::flag(state, weapon)
    }
}

pub fn have_item_kind(state: *mut lua_State) -> app::ItemKind {
    unsafe {
        impl_::have_item_kind(state)
    }
}

pub fn kind(state: *mut lua_State, weapon: *const app::FighterAIWeapon) -> app::WeaponKind {
    unsafe {
        impl_::kind(state, weapon)
    }
}

pub fn most_earliest_hit_frame(state: *mut lua_State) -> i32 {
    unsafe {
        impl_::most_earliest_hit_frame(state)
    }
}

pub fn most_earliest_weapon(state: *mut lua_State) -> *mut app::FighterAIWeapon {
    unsafe {
        impl_::most_earliest_weapon(state)
    }
}

pub fn pos_x(state: *mut lua_State, weapon: *const app::FighterAIWeapon) -> f32 {
    unsafe {
        impl_::pos_x(state, weapon)
    }
}

pub fn speed_length(state: *mut lua_State, weapon: *const app::FighterAIWeapon) -> phx::Vec2 {
    unsafe {
        impl_::speed_length(state, weapon).into()
    }
}

pub fn speed_x(state: *mut lua_State, weapon: *const app::FighterAIWeapon) -> f32 {
    unsafe {
        impl_::speed_x(state, weapon)
    }
}

pub fn type_(state: *mut lua_State, weapon: *const app::FighterAIWeapon) -> app::FighterAIWeaponType {
    unsafe {
        impl_::type_(state, weapon)
    }
}

