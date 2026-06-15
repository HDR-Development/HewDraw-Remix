use super::*;
use globals::*;

//===================================================================
//== MOMENTUM TRANSFER
//== The chonky meat of the code; includes some status script hooks
//===================================================================

pub fn install() {
    skyline::nro::add_hook(nro_main).unwrap();
}

fn nro_main(nro: &skyline::nro::NroInfo) {
    match nro.name {
        "common" => {
            skyline::install_hooks!(
                // lua2cpp_common.nro hooks here
                status_pre_AttackAir,
                status_pre_ItemThrow
            );
        }
        _ => (),
    }
}

//Aerials (runs once at the beginning of the status)
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_AttackAir)]
pub unsafe fn status_pre_AttackAir(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    original!()(fighter)
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_pre_ItemThrow)]
unsafe fn status_pre_ItemThrow(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_JUMP
    && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT_ONCE);
    }
    
    original!()(fighter)
}