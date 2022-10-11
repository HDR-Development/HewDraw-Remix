use super::*;
use globals::*;

// This prevents projectiles from dying on platforms/ground
#[skyline::hook(replace=GroundModule::is_touch)]
unsafe fn is_touch_hook(boma: &mut BattleObjectModuleAccessor, ground_touch_flags: u32) -> bool {
    let mut ground_touch_flags = ground_touch_flags;
    let normal_y = GroundModule::get_touch_normal_y(boma, *GROUND_TOUCH_FLAG_DOWN as u32);

    if boma.is_weapon()
    && (normal_y >= 0.99 && normal_y <= 1.01)  // if touching a near-flat platform/ground
    && [*WEAPON_KIND_SAMUS_CSHOT,
        *WEAPON_KIND_RYU_HADOKEN,
        *WEAPON_KIND_LUCAS_PK_FIRE,
        *WEAPON_KIND_EDGE_FLARE1,
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
        *WEAPON_KIND_WIIFIT_SUNBULLET,
        *WEAPON_KIND_SAMUS_SUPERMISSILE,
        *WEAPON_KIND_SAMUSD_SUPERMISSILE,
        *WEAPON_KIND_MEWTWO_SHADOWBALL,
        *WEAPON_KIND_LUCARIO_AURABALL,
        *WEAPON_KIND_MIIGUNNER_SUPERMISSILE,
        *WEAPON_KIND_MIIGUNNER_GUNNERCHARGE,
        *WEAPON_KIND_GEKKOUGA_SHURIKEN,
        *WEAPON_KIND_PICHU_DENGEKIDAMA,
        *WEAPON_KIND_BRAVE_DEATHBALL,
        *WEAPON_KIND_KAMUI_RYUSENSYA,
        *WEAPON_KIND_EDGE_FIRE
    ].contains(&boma.kind())
    {
        if ground_touch_flags == *GROUND_TOUCH_FLAG_ALL as u32
        && !original!()(boma, *GROUND_TOUCH_FLAG_LEFT as u32 | *GROUND_TOUCH_FLAG_RIGHT as u32)  // if not touching a wall
        {
            // Ignore ground collision
            return false;
        }
        if ground_touch_flags & *GROUND_TOUCH_FLAG_DOWN as u32 != 0 {
            // Ignore ground collision
            ground_touch_flags -= *GROUND_TOUCH_FLAG_DOWN as u32;
        }
    }
    original!()(boma, ground_touch_flags)
}

#[skyline::hook(replace=GroundModule::is_floor_touch_line)]
unsafe fn is_floor_touch_line_hook(boma: &mut BattleObjectModuleAccessor, ground_touch_flags: u32) -> bool {
    let mut ground_touch_flags = ground_touch_flags;
    let normal_y = GroundModule::get_touch_normal_y(boma, *GROUND_TOUCH_FLAG_DOWN as u32);

    if boma.is_weapon()
    && (normal_y >= 0.99 && normal_y <= 1.01)  // if touching a near-flat platform/ground
    && ( ([*WEAPON_KIND_KROOL_IRONBALL, *WEAPON_KIND_KROOL_SPITBALL].contains(&boma.kind())
            && boma.is_status(*WEAPON_KROOL_IRONBALL_STATUS_KIND_SHOOT))
        || (boma.kind() == *WEAPON_KIND_KOOPAJR_CANNONBALL
            && boma.is_status(*WEAPON_KOOPAJR_CANNONBALL_STATUS_KIND_SHOOT)
            && !KineticModule::is_enable_energy(boma, *WEAPON_KOOPAJR_CANNONBALL_KINETIC_ENERGY_ID_GRAVITY)) )  // if Jr.'s cannonball is flying straight horizontally
    {
        if ground_touch_flags == *GROUND_TOUCH_FLAG_ALL as u32
        && !original!()(boma, *GROUND_TOUCH_FLAG_LEFT as u32 | *GROUND_TOUCH_FLAG_RIGHT as u32)  // if not touching a wall
        {
            // Ignore ground collision
            return false;
        }
        if ground_touch_flags & *GROUND_TOUCH_FLAG_DOWN as u32 != 0 {
            // Ignore ground collision
            ground_touch_flags -= *GROUND_TOUCH_FLAG_DOWN as u32;
        }
    }
    original!()(boma, ground_touch_flags)
}

#[skyline::hook(replace=GroundModule::get_touch_flag)]
unsafe fn get_touch_flag_hook(boma: &mut BattleObjectModuleAccessor) -> i32 {
    let normal_y = GroundModule::get_touch_normal_y(boma, *GROUND_TOUCH_FLAG_DOWN as u32);

    if boma.is_weapon()
    && (normal_y >= 0.99 && normal_y <= 1.01)  // if touching a near-flat platform/ground
    && boma.kind() == *WEAPON_KIND_DEDEDE_GORDO
    && boma.is_status(*WEAPON_DEDEDE_GORDO_STATUS_KIND_THROW)  // when Dedede first tosses Gordo
    && boma.status_frame() == 0  // on frame 1 of the toss
    {
        if original!()(boma) == *GROUND_TOUCH_FLAG_DOWN {
            // Ignore ground collision
            return *GROUND_TOUCH_FLAG_NONE;
        }
    }
    original!()(boma)
}

#[skyline::hook(offset = 0x6ca950)]
unsafe fn ground_module_update_hook(ground_module: u64) {
    call_original!(ground_module);
    let boma = *((ground_module + 0x20) as *mut *mut BattleObjectModuleAccessor);
    let line = *((ground_module + 0x28) as *mut *mut f32);
    let shift = VarModule::get_float((*boma).object(), vars::common::instance::ECB_Y_OFFSETS);
    if (*boma).is_fighter()
    && !(*boma).is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_THROWN])
    && (*boma).is_situation(*SITUATION_KIND_AIR)
    && shift != 0.0
    {
        *line.add(0x3D4 / 4) += shift;
    }
}

pub fn install() {
    skyline::install_hooks!(
        is_touch_hook,
        is_floor_touch_line_hook,
        get_touch_flag_hook,
        ground_module_update_hook
    );
}