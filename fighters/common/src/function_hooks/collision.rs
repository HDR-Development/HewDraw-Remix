use super::*;
use globals::*;

#[skyline::hook(replace=GroundModule::is_touch)]
unsafe fn is_touch_hook(boma: &mut BattleObjectModuleAccessor, ground_touch_flags: u32) -> bool {
    let mut ground_touch_flags = ground_touch_flags;
    if boma.is_weapon()
    && [*WEAPON_KIND_SAMUS_CSHOT,
        *WEAPON_KIND_RYU_HADOKEN,
        *WEAPON_KIND_LUCAS_PK_FIRE,
        *WEAPON_KIND_EDGE_FLARE1,
        *WEAPON_KIND_EDGE_FLARE2,
        *WEAPON_KIND_WOLF_BLASTER_BULLET,
        *WEAPON_KIND_MASTER_ARROW1,
        *WEAPON_KIND_MASTER_ARROW2,
        *WEAPON_KIND_LUIGI_FIREBALL,
        *WEAPON_KIND_FOX_BLASTER_BULLET,
        *WEAPON_KIND_SAMUSD_CSHOT,
        *WEAPON_KIND_BRAVE_FIREBALL,
        *WEAPON_KIND_FALCO_BLASTER_BULLET,
        *WEAPON_KIND_KEN_HADOKEN,
        *WEAPON_KIND_REFLET_THUNDER,
        *WEAPON_KIND_SZEROSUIT_PARALYZER_BULLET,
        *WEAPON_KIND_MIIGUNNER_ATTACKAIRF_BULLET,
    ].contains(&boma.kind())
    {
        let normal_y = GroundModule::get_touch_normal_y(boma, *GROUND_TOUCH_FLAG_DOWN as u32);

        if ground_touch_flags == *GROUND_TOUCH_FLAG_ALL as u32
        && (0.99 <= normal_y && normal_y <= 1.01) {
            return false;
        }
        if ground_touch_flags & *GROUND_TOUCH_FLAG_DOWN as u32 != 0
        && (0.99 <= normal_y && normal_y <= 1.01) {
            ground_touch_flags -= *GROUND_TOUCH_FLAG_DOWN as u32;
        }
    }
    original!()(boma, ground_touch_flags)
}

pub fn install() {
    skyline::install_hooks!(
        is_touch_hook
    );
}