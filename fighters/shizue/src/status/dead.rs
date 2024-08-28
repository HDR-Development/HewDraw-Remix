use super::*;

// FIGHTER_STATUS_KIND_DEAD

unsafe extern "C" fn dead_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_DEAD)(fighter);
    
    if fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_DEAD_MODE) == *FIGHTER_DEAD_MODE_LOTTERY_NORMAL {
        let handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_shizue_damage_twinkle"), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, handle as i32, 0.9, 0);
    } else {
        let rng = sv_math::rand(hash40("fighter"), 2);
        let sfx_id = match rng {
            0 => "vc_shizue_missfoot01",
            _ => "vc_shizue_missfoot02"
        };
        let handle = SoundModule::play_se(fighter.module_accessor, Hash40::new(sfx_id), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, handle as i32, 1.1, 0);
    }

    ret
}


pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_DEAD, dead_main);
}