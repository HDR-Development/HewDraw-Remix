use super::*;

mod ground;
mod tilts;
mod smashes;
mod aerials;
mod specials;
mod throws;
mod other;

pub unsafe fn GET_COLOR_VEC(boma: &mut BattleObjectModuleAccessor) -> Vector3f {
    return match WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
        0 => Vector3f::new(0.1, 0.7, 3.0),//nor
        1 => Vector3f::new(0.55, 0.88, 0.0004),//g
        2 => Vector3f::new(1.25, 0.55, 1.5),//pur
        3 => Vector3f::new(0.84, 0.7, 0.03),//r
        4 => Vector3f::new(0.1, 1.0, 2.0),//y
        5 => Vector3f::new(0.9, 0.03, 0.03),//w
        6 => Vector3f::new(1.15, 0.65, 0.03),//blac
        7 => Vector3f::new(0.78, 0.5, 2.5),//pi
        _ => Vector3f::new(0.1, 0.7, 3.0)
    }; //matches glow color
}

pub unsafe fn EFFECT_AURA(agent: &mut L2CAgentBase) {
    EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5, true);
    EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("clavicler"), 2, 0, 0.5, 0, 0, 0, 2, true);
    EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
    EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1, true);
    EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
    EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
    EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("claviclel"), 2, 0, -0.5, 0, 0, 0, 2, true);
    EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, 0.5, 0, 0, 0, 1.7, true);
    EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1, true);
    EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
    EFFECT_FOLLOW(agent, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
}

pub fn install(agent: &mut Agent) {
    ground::install(agent);
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);
    throws::install(agent);
    other::install(agent);
}