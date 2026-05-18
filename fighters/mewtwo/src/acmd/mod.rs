use super::*;

mod ground;
mod tilts;
mod smashes;
mod aerials;
mod specials;
mod throws;
mod other;

pub unsafe fn GET_COLOR_EFF_NAME(boma: &mut BattleObjectModuleAccessor) -> &str {
    return match boma.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
        0 => "mewtwo_tail_attack_a_01",
        1 => "mewtwo_tail_attack_a_02",
        2 => "mewtwo_tail_attack_a_03",
        3 => "mewtwo_tail_attack_a_04",
        4 => "mewtwo_tail_attack_a_05",
        5 => "mewtwo_tail_attack_a_06",
        6 => "mewtwo_tail_attack_a_07",
        7 => "mewtwo_tail_attack_a_08",
        _ => "mewtwo_tail_attack_a_01",
    }; //matches glow color
}

//pub unsafe fn TOGGLE_TAIL(agent: &mut L2CAgentBase, toggle: bool) {
//    if toggle {
//        //HIT_NO(agent, 12, *HIT_STATUS_NORMAL);
//        HIT_NO(agent, 13, *HIT_STATUS_NORMAL);
//    } else {
//        //HIT_NO(agent, 12, *HIT_STATUS_NORMAL);
//        HIT_NO(agent, 13, *HIT_STATUS_OFF);
//    }
//}

pub fn install(agent: &mut Agent) {
    ground::install(agent);
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    specials::install(agent);
    throws::install(agent);
    other::install(agent);
}