use super::*;
use globals::*;

//===================================================================
//== DIRECTIONAL INFLUENCE
//== DI during nontumble knockback
//===================================================================

// ftStatusUniqProcessDamage_exec_common
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon::ftStatusUniqProcessDamage_exec_common)]
pub unsafe fn ftStatusUniqProcessDamage_exec_common_hook(fighter: &mut L2CFighterCommon){
    let ret = original!()(fighter);

    smash::lua2cpp::L2CFighterCommon::FighterStatusDamage__correctDamageVector(fighter);

    ret

}

pub fn install() {
    skyline::install_hooks!(
        ftStatusUniqProcessDamage_exec_common_hook
    );
}
