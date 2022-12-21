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
                status_pre_AttackAir
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

/*      FALL CHANGED KINETIC TYPE NEUTRAL SPECIAL MOMENTUM   */
/* called in hooks/function_hooks/djcancel.rs in the change_kinetic hook */
/* special cased cus putting these in momentum_transfer_helper didnt work... momentum seemed to be reset every frame */
pub unsafe fn change_kinetic_momentum_related(boma: &mut smash::app::BattleObjectModuleAccessor) -> Option<i32> { //spacie laser momentum conservation
    let status_kind = StatusModule::status_kind(boma);
    let prev_status_kind = StatusModule::prev_status_kind(boma, 0);
    let situation_kind = StatusModule::situation_kind(boma);
    let fighter_kind = boma.kind();
    if (([*FIGHTER_KIND_CAPTAIN, *FIGHTER_KIND_FALCO, *FIGHTER_KIND_FOX, *FIGHTER_KIND_GAMEWATCH, *FIGHTER_KIND_WOLF].contains(&fighter_kind) && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N))
        && situation_kind == *SITUATION_KIND_AIR && [*FIGHTER_STATUS_KIND_JUMP, *FIGHTER_STATUS_KIND_JUMP_SQUAT].contains(&prev_status_kind) {
        return Some(-1);
    }
    None
}