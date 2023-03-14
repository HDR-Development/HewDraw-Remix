use smash::app::sv_battle_object::module_accessor;

use super::*;

mod dein;
mod phantom;

// Prevents side special from being used if a Din's Fire is present
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dein_object_id = VarModule::get_int(fighter.battle_object, vars::zelda::instance::DEIN_OBJECT_ID) as u32;
    if sv_battle_object::is_active(dein_object_id)
    && sv_battle_object::category(dein_object_id) == *BATTLE_OBJECT_CATEGORY_WEAPON
    && sv_battle_object::kind(dein_object_id) == *WEAPON_KIND_ZELDA_DEIN {
        let article_boma = sv_battle_object::module_accessor(dein_object_id);

        // Fire
        let handle1 = EffectModule::req_on_joint(article_boma, Hash40::new("zelda_appeal_s_fire"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.5, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        EffectModule::set_rate_last(article_boma, 2.5);
        EffectModule::set_rgb(article_boma, handle1 as u32, 0.65, 0.3, 0.3);
        
        // Smoke Dark
        let handle = EffectModule::req_on_joint(article_boma, Hash40::new("sys_steam"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.5, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        EffectModule::set_rgb(article_boma, handle as u32, 0.0, 0.0, 0.0);
        EffectModule::set_alpha(article_boma, handle as u32, 3.0);
        
        // Smoke Light
        let handle2 = EffectModule::req_on_joint(article_boma, Hash40::new("sys_steam"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.5, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
        EffectModule::set_rgb(article_boma, handle2 as u32, 0.1, 0.1, 0.1);
        EffectModule::set_alpha(article_boma, handle2 as u32, 3.0);

        sv_battle_object::end_inhaled(dein_object_id, true);
        true.into()
    } else {
        true.into()
    }
}

#[smashline::fighter_init]
fn zelda_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_ZELDA {
            fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
        }
    }
}

pub fn install() {
    install_status_scripts!();
    smashline::install_agent_init_callbacks!(zelda_init);
    dein::install();
    phantom::install();
}