use super::*;
use globals::*;
use utils::*;

#[skyline::hook(offset = 0x4cf6a0)]
unsafe fn soundmodule__play_se_hook(sound_module: u64, se: smash::phx::Hash40, arg2: bool, arg3: bool, arg4: bool, arg5: bool, se_type: smash::app::enSEType) -> u64 {
    let handle = original!()(sound_module, se, arg2, arg3, arg4, arg5, se_type);
    let boma = *(sound_module as *mut *mut BattleObjectModuleAccessor).add(1);
    if se_type.0 == 0
    && !utils::se::SE_LIST.contains(&se.hash) {
        // Increase volume of most of the game's SFX (excluding voice clips)
        SoundModule::set_se_vol(boma, handle as i32, 1.25, 0);
    }
    handle
}

#[skyline::hook(replace=smash::app::sv_animcmd::PLAY_SEQUENCE)]
unsafe fn PLAY_SEQUENCE_hook(lua_state: u64) {
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_ATTACK,
        *FIGHTER_STATUS_KIND_ATTACK_DASH,
        *FIGHTER_STATUS_KIND_ATTACK_S3,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_AIR,
        ])
    {
        // Reduce frequency of voice clips during normals
        if app::sv_math::rand(hash40("fighter"), 5) != 0 {
            return;
        }
    }

    original!()(lua_state);
}

pub fn install() {
    skyline::install_hooks!(
        soundmodule__play_se_hook,
        PLAY_SEQUENCE_hook,
    );
}