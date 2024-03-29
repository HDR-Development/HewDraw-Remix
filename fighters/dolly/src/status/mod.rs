use super::*;
use globals::*;
// status script import

mod wait;
mod turn_dash;
mod dash_back;
mod landing;
mod guard_off;

mod special_s;
mod special_hi;
mod super_special;
mod super_special2;

// Prevents sideB from being used again if it has already been used once in the current airtime
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Re-enables the ability to use sideB when connecting to ground or cliff
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::dolly::instance::DISABLE_SPECIAL_S);
    }
    
    // ORIGINAL
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_WAIT {
        FighterSpecializer_Dolly::update_opponent_lr_1on1(fighter.module_accessor, fighter.global_table[STATUS_KIND].get_i32());
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLAG_AUTO_TURN_END_STATUS);
    let lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if lr == 0.0
    || PostureModule::lr(fighter.module_accessor) == lr
    || StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_GROUND {
        return 0.into();
    }

    unsafe fn update_lr(fighter: &mut L2CFighterCommon, lr: f32) {
        PostureModule::set_lr(fighter.module_accessor, lr);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
    }

    if [
        *FIGHTER_STATUS_KIND_WALK,
        *FIGHTER_STATUS_KIND_SQUAT,
        *FIGHTER_STATUS_KIND_SQUAT_RV,
        *FIGHTER_STATUS_KIND_LANDING,
        *FIGHTER_STATUS_KIND_LANDING_LIGHT,
        *FIGHTER_STATUS_KIND_GUARD_ON,
        *FIGHTER_STATUS_KIND_ESCAPE,
        *FIGHTER_STATUS_KIND_ATTACK_HI3,
        *FIGHTER_STATUS_KIND_ATTACK_LW3,
        *FIGHTER_STATUS_KIND_ATTACK_HI4_START,
        *FIGHTER_STATUS_KIND_ATTACK_LW4_START,
        *FIGHTER_STATUS_KIND_CATCH,
        *FIGHTER_STATUS_KIND_ITEM_SWING,
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_FINAL,
        *FIGHTER_RYU_STATUS_KIND_WALK_BACK,
    ].contains(&fighter.global_table[globals::STATUS_KIND].get_i32())
    {
        update_lr(fighter, lr);
        return 0.into();
    }

    if fighter.global_table[globals::STATUS_KIND] == FIGHTER_STATUS_KIND_WAIT {
        if ![
            *FIGHTER_STATUS_KIND_DASH,
            *FIGHTER_RYU_STATUS_KIND_DASH_BACK,
            *FIGHTER_STATUS_KIND_RUN_BRAKE,
            *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE,
            *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
            *FIGHTER_STATUS_KIND_SQUAT_RV,
            *FIGHTER_STATUS_KIND_TREAD_DAMAGE_RV,
            *FIGHTER_STATUS_KIND_GUARD_OFF,
            *FIGHTER_STATUS_KIND_GUARD_DAMAGE,
            *FIGHTER_STATUS_KIND_ESCAPE,
            *FIGHTER_STATUS_KIND_ESCAPE_F,
            *FIGHTER_STATUS_KIND_ESCAPE_B,
            *FIGHTER_STATUS_KIND_ATTACK_DASH,
            *FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_CATCH,
            *FIGHTER_STATUS_KIND_CATCH_DASH,
            *FIGHTER_STATUS_KIND_CATCH_TURN,
            *FIGHTER_STATUS_KIND_CATCH_CUT,
            *FIGHTER_STATUS_KIND_THROW,
            *FIGHTER_STATUS_KIND_CAPTURE_CUT,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DOWN_STAND,
            *FIGHTER_STATUS_KIND_DOWN_STAND_FB,
            *FIGHTER_STATUS_KIND_PASSIVE,
            *FIGHTER_STATUS_KIND_PASSIVE_FB,
            *FIGHTER_STATUS_KIND_FURAFURA_END,
            *FIGHTER_STATUS_KIND_DAMAGE_SONG_END,
            *FIGHTER_STATUS_KIND_CLIFF_CLIMB,
            *FIGHTER_STATUS_KIND_CLIFF_ATTACK,
            *FIGHTER_STATUS_KIND_CLIFF_ESCAPE,
            *FIGHTER_STATUS_KIND_SLIP_STAND,
            *FIGHTER_STATUS_KIND_SLIP_STAND_ATTACK,
            *FIGHTER_STATUS_KIND_SLIP_STAND_F,
            *FIGHTER_STATUS_KIND_SLIP_STAND_B,
            *FIGHTER_STATUS_KIND_ITEM_LIGHT_PICKUP,
            *FIGHTER_STATUS_KIND_ITEM_THROW,
            *FIGHTER_STATUS_KIND_ITEM_THROW_DASH,
            *FIGHTER_STATUS_KIND_ITEM_THROW_HEAVY,
            *FIGHTER_STATUS_KIND_ITEM_SWING,
            *FIGHTER_STATUS_KIND_ITEM_SWING_S3,
            *FIGHTER_STATUS_KIND_ITEM_SWING_S4,
            *FIGHTER_STATUS_KIND_ITEM_SWING_DASH,
            *FIGHTER_STATUS_KIND_APPEAL,
            *FIGHTER_STATUS_KIND_SPECIAL_N
        ].contains(&fighter.global_table[globals::STATUS_KIND_INTERRUPT].get_i32())
        {
            update_lr(fighter, lr);
        }
        return 0.into();
    }

    if fighter.global_table[globals::STATUS_KIND] == FIGHTER_STATUS_KIND_JUMP_SQUAT {
        if fighter.global_table[globals::STATUS_KIND_INTERRUPT] != FIGHTER_STATUS_KIND_TURN_RUN {
            update_lr(fighter, lr);
        }
        return 0.into();
    }

    if fighter.global_table[globals::STATUS_KIND] == FIGHTER_STATUS_KIND_ATTACK {
        if fighter.global_table[globals::STATUS_KIND] != FIGHTER_STATUS_KIND_ATTACK || ComboModule::count(fighter.module_accessor) == 0 {
            update_lr(fighter, lr);
        }
        return 0.into();
    }

    if fighter.global_table[globals::STATUS_KIND] == FIGHTER_STATUS_KIND_ITEM_THROW
    && fighter.global_table[globals::SITUATION_KIND] == SITUATION_KIND_GROUND
    {
        let cat3 = fighter.global_table[globals::CMD_CAT3].get_i32();
        if cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_4 != 0 && cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_FB4 == 0 {
            update_lr(fighter, lr);
        } else if cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_HI != 0 {
            update_lr(fighter, lr);
        } else if cat3 & *FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_LW != 0 {
            update_lr(fighter, lr);
        }
    }

    0.into()
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    wait::install(agent);
    turn_dash::install(agent);
    dash_back::install(agent);
    landing::install(agent);
    guard_off::install(agent);

    special_s::install(agent);
    special_hi::install(agent);
    super_special::install(agent);
    super_special2::install(agent);
}