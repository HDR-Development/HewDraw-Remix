use utils::{
    *,
    ext::*,
    consts::*,
    consts::globals::*
};
use smash::app::BattleObjectModuleAccessor;
use smash::phx::{Vector2f, Vector3f};
use smash::lib::{lua_const::*, L2CValue, L2CAgent};
use smash::app::{self, lua_bind::*, sv_kinetic_energy, sv_animcmd};
use smash::lua2cpp::L2CFighterCommon;
use smash::hash40;
use smash::phx::Hash40;
use smash_script::{self, *, macros::*};


//=================================================================
//== GRAB JUMP REFRESH
//=================================================================

/// Gives fighters an additional jump if they are grabbed
unsafe fn grab_jump_refresh(boma: &mut BattleObjectModuleAccessor) {
    if boma.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_CAPTURE_CUT,
        *FIGHTER_STATUS_KIND_CAPTURE_JUMP,
        *FIGHTER_STATUS_KIND_SHOULDERED_DONKEY,
        *FIGHTER_STATUS_KIND_SWALLOWED,
        *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN,
        *FIGHTER_STATUS_KIND_KOOPA_DIVED,
        *FIGHTER_STATUS_KIND_CLUNG_GANON,
        *FIGHTER_STATUS_KIND_MEWTWO_THROWN,
        *FIGHTER_STATUS_KIND_BITTEN_WARIO_START,
        *FIGHTER_STATUS_KIND_CLUNG_DIDDY,
        *FIGHTER_STATUS_KIND_MIIFIGHTER_COUNTER_THROWN,
        *FIGHTER_STATUS_KIND_CATCHED_REFLET,
        *FIGHTER_STATUS_KIND_CATCHED_RIDLEY,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD,
        *FIGHTER_STATUS_KIND_SWING_GAOGAEN_CATCHED,
        *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE,
        *FIGHTER_STATUS_KIND_DEMON_DIVED
    ]) && WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) == WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX)
    {
        WorkModule::dec_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
}

unsafe fn plat_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR) {
        let hitlag_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME);
        let pass_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("pass_stick_y"));
        
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_ALL)
        && hitlag_frame <= 0
        && fighter.global_table[STICK_Y].get_f32() > pass_stick_y
        {
            GroundModule::clear_pass_floor(fighter.module_accessor);
        }
    }
}

pub unsafe fn run(fighter: &mut L2CFighterCommon, lua_state: u64, l2c_agent: &mut L2CAgent, boma: &mut BattleObjectModuleAccessor, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, fighter_kind: i32, stick_x: f32, stick_y: f32, facing: f32) {
    grab_jump_refresh(boma);
    plat_cancels(fighter);

    //WorkModule::unable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D); //Melee style spike knockdown (courtesey of zabimaru), leaving it commented here just to have it saved somewhere
}

