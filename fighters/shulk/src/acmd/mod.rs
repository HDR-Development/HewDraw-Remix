use super::*;

mod ground;
mod tilts;
mod smashes;
mod aerials;
mod specials;
mod throws;
mod other;

pub unsafe fn shulk_get_trail(agent: &mut L2CAgentBase) -> String {
    let original_trail = "tex_shulk_sword_hdr";
    let boma = agent.boma();
        if !WorkModule::is_flag(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_FLAG_SPECIAL_N_ACTIVE) {
        return original_trail.to_string();
    }
    let active_art = WorkModule::get_int(boma, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE);
    let trail = match active_art {
        0 => format!("{}{}", original_trail, "_jump"),
        1 => format!("{}{}", original_trail, "_speed"),
        2 => format!("{}{}", original_trail, "_shield"),
        3 => format!("{}{}", original_trail, "_buster"),
        4 => format!("{}{}", original_trail, "_smash"),
        _ => original_trail.to_string()
    };

    return trail;
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