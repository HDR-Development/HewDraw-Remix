use super::*;

mod ground;
mod tilts;
mod smashes;
mod aerials;
mod specials;
mod throws;
mod other;

pub unsafe fn GET_COLOR_VEC(boma: &mut BattleObjectModuleAccessor) -> Vector3f {
    return match boma.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) {
        0 => if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP) { Vector3f::new(0.1, 0.01, 0.0) } else { Vector3f::new(0.196, 0.196, 0.216) },
        1 => if WorkModule::is_flag(boma, *FIGHTER_ROBOT_INSTANCE_WORK_ID_FLAG_REGION_JP) { Vector3f::new(0.196, 0.196, 0.216) } else { Vector3f::new(0.22, 0.059, 0.039) },
        2 => Vector3f::new(0.176, 0.137, 0.059),
        3 => Vector3f::new(0.235, 0.196, 0.255),
        4 => Vector3f::new(0.098, 0.157, 0.196),
        5 => Vector3f::new(0.098, 0.059, 0.0),
        6 => Vector3f::new(0.098, 0.098, 0.157),
        7 => Vector3f::new(0.118, 0.039, 0.051),
        _ => Vector3f::new(0.196, 0.196, 0.216)
    };
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