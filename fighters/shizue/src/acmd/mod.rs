use super::*;

mod ground;
mod tilts;
mod smashes;
mod aerials;
mod specials;
mod throws;
mod other;

pub fn install(agent: &mut Agent) {
    ground::install(agent);
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);
    throws::install(agent);
    other::install(agent);
}

pub unsafe extern "C" fn SHIZUE_VC_SEQUENCE_ATTACK(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let rng = sv_math::rand(hash40("fighter"), 5);
    let should_vocalize = sv_math::rand(hash40("fighter"), 4) == 1;
    if should_vocalize {
        let sfx_id = match rng {
            0 => "vc_shizue_attack01",
            1 => "vc_shizue_attack02",
            2 => "vc_shizue_attack03",
            3 => "vc_shizue_attack04",
            _ => "vc_shizue_attack05"
        };
        let handle = SoundModule::play_se(boma, Hash40::new(sfx_id), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, handle as i32, 0.75, 0);
    }
}

pub unsafe extern "C" fn SHIZUE_VC_SEQUENCE_ATTACK_HEAVY(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let rng = sv_math::rand(hash40("fighter"), 4);
    let should_vocalize = sv_math::rand(hash40("fighter"), 3) >= 1;
    if should_vocalize {
        let sfx_id = match rng {
            0 => "vc_shizue_attack_heavy01",
            1 => "vc_shizue_attack_heavy02",
            2 => "vc_shizue_attack_heavy03",
            _ => "vc_shizue_attack_heavy04"
        };
        let handle = SoundModule::play_se(boma, Hash40::new(sfx_id), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, handle as i32, 0.75, 0);
    }
}

pub unsafe extern "C" fn SHIZUE_VC_SEQUENCE_DAMAGE(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let rng = sv_math::rand(hash40("fighter"), 4);
    let should_vocalize = sv_math::rand(hash40("fighter"), 4) == 1;
    if should_vocalize {
        let sfx_id = match rng {
            0 => "vc_shizue_damage01",
            1 => "vc_shizue_damage02",
            _ => "vc_shizue_damage03"
        };
        let handle = SoundModule::play_se(boma, Hash40::new(sfx_id), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, handle as i32, 0.75, 0);
    }
}

pub unsafe extern "C" fn SHIZUE_VC_SEQUENCE_DAMAGEFLY(agent: &mut L2CAgentBase) {
    let boma = agent.boma();
    let rng = sv_math::rand(hash40("fighter"), 3);
    let should_vocalize = sv_math::rand(hash40("fighter"), 3) == 1;
    if should_vocalize {
        let sfx_id = match rng {
            0 => "vc_shizue_damagefly01",
            _ => "vc_shizue_damagefly02"
        };
        let handle = SoundModule::play_se(boma, Hash40::new(sfx_id), true, false, false, false, app::enSEType(0));
        SoundModule::set_se_vol(boma, handle as i32, 0.75, 0);
    }
}