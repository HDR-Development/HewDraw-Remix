use super::*;
use globals::*;
// status script import
 
mod run;
mod special_s;
mod bayonet_end;

/// Re-enables the ability to use aerial specials when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)){ 
        VarModule::on_flag(fighter.battle_object, vars::buddy::instance::FLUTTER_ENABLED);
    }
    return true.into();
}

//Called on rebirth and entry
unsafe fn on_rebirth(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor) {
    let entry = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    VarModule::off_flag(boma.object(), vars::buddy::instance::BEAKBOMB_ACTIVE);
    VarModule::off_flag(boma.object(), vars::buddy::instance::BAYONET_ACTIVE);
    
    VarModule::set_int(boma.object(), vars::buddy::instance::HUD_DISPLAY_TIME,60);
    VarModule::set_int(boma.object(), vars::buddy::instance::BEAKBOMB_FRAME,0);
    VarModule::set_int(boma.object(), vars::buddy::instance::BEAKBOMB_BOUNCE,0);
    VarModule::set_int(boma.object(), vars::buddy::instance::BAYONET_EGGS,0);
    BAYONET_EGGS[entry]=0;

    VarModule::set_float(boma.object(), vars::buddy::instance::FEATHERS_RED_COOLDOWN,0.0);
    VarModule::set_float(boma.object(), vars::buddy::instance::BEAKBOMB_ANGLE,0.0);
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _)); 

    let lua_state = fighter.lua_state_agent;    
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    on_rebirth(fighter, boma);
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    run::install(agent);
    special_s::install(agent);
    bayonet_end::install(agent);
}