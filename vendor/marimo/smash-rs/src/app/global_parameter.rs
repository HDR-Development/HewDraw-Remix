mod impl_ {
    extern "C" {
        #[link_name = "_ZN3app16global_parameter14is_boss_battleEv"]
        pub(super) fn is_boss_battle() -> bool;

        #[link_name = "_ZN3app16global_parameter14is_team_battleEv"]
        pub(super) fn is_team_battle() -> bool;

        #[link_name = "_ZN3app16global_parameter17is_spirits_battleEv"]
        pub(super) fn is_spirits_battle() -> bool;

        #[link_name = "_ZN3app16global_parameter30is_team_battle_and_team_attackEv"]
        pub(super) fn is_team_battle_and_team_attack() -> bool;
    }
}

pub fn is_boss_battle() -> bool {
    unsafe {
        impl_::is_boss_battle()
    }
}

pub fn is_team_battle() -> bool {
    unsafe {
        impl_::is_team_battle()
    }
}

pub fn is_spirits_battle() -> bool {
    unsafe {
        impl_::is_spirits_battle()
    }
}

pub fn is_team_battle_and_team_attack() -> bool {
    unsafe {
        impl_::is_team_battle_and_team_attack()
    }
}
