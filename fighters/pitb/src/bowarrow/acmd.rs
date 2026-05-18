use super::*;

hitbox_templates!(
    pub PITB_BOWARROW_HITBOX = {
        extends: ENERGY_PROJECTILE_HITBOX,
        effect: "collision_attr_sting",
        sound_level: SoundLevel::S,
        hit_sound: CollisionSound::Cutup,
        region: AttackRegion::Palutena,
    };
);

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        hitbox!(agent, { extends: PITB_BOWARROW_HITBOX, id: 0, bone: "top", dmg: 1.0, angle: 45, kbg: 100, bkb: 4, size: 1.3, x: 0.0, y: 0.0, z: -1.5, });
        AttackModule::enable_safe_pos(boma);
    }
}

// unsafe extern "C" fn effect_fly(agent: &mut L2CAgentBase) {
//     let lua_state = agent.lua_state_agent;
//     let boma = agent.boma();
//     if is_excute(agent) {
//         EFFECT_FOLLOW(agent, Hash40::new("pitb_pa_fly_arrow"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, true);
//         EFFECT_FOLLOW(agent, Hash40::new("pitb_pa_fly_arrow2"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, true);
//     }
// }

// unsafe extern "C" fn sound_fly(agent: &mut L2CAgentBase) {
//     let lua_state = agent.lua_state_agent;
//     let boma = agent.boma();
//     if is_excute(agent) {
//         PLAY_STATUS(agent, Hash40::new("se_pitb_special_n03"));
//         SET_TAKEOUT_SE(agent, Hash40::new("se_pitb_special_n02"));
//     }
// }

pub fn install(agent: &mut Agent) {
    agent.acmd("game_fly", game_fly, Priority::Low);
    // agent.acmd("effect_fly", effect_fly, Priority::Low);
    // agent.acmd("sound_fly", sound_fly, Priority::Low);
}