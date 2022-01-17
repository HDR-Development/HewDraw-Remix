use super::*;
use globals::*;

//=================================================================
//== sv_animcmd::ATTACK
//== Note: Lua stack is 1-indexed, and "pop" means "get"
//=================================================================
#[skyline::hook(replace=smash::app::sv_animcmd::ATTACK)]
unsafe fn ATTACK_hook(lua_state: u64) {
    if crate::vars::DEBUG {
        // visualizer::handle_attack(lua_state);
    }

    // No longer requiring reverse hits
    return original!()(lua_state);

    /*
    let mut l2c_agent: L2CAgent = L2CAgent::new(lua_state);

    let mut hitbox_params: [L2CValue ; 36] = [L2CValue::new_void() ; 36];

    for i in 0..36 {
        hitbox_params[i as usize] = l2c_agent.pop_lua_stack(i + 1);
    }

    l2c_agent.clear_lua_stack();

    for i in 0..36 {
        // Index of FacingRestrict
        if i == 18 {
            let mut aux: L2CValue = L2CValue::new_int(*ATTACK_LR_CHECK_POS as u64);
            l2c_agent.push_lua_stack(&mut aux);
        } else {
            l2c_agent.push_lua_stack(&mut hitbox_params[i as usize]);
        }
    }

    original!()(lua_state)
    */
}

pub fn install() {
    // skyline::install_hook!(ATTACK_hook);
}
