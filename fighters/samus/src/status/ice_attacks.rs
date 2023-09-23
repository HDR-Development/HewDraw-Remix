use super::*;


#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attacklw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter){
        return original!(fighter);
    }
    fighter.status_AttackLw3_common();
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw32"), 0.0, 1.0, false, 0.0, false, false);
    let groundcheck = *GROUND_TOUCH_FLAG_DOWN as u32;
    let lr = PostureModule::lr(fighter.module_accessor);
    let nearGround = GroundModule::ray_check(
        fighter.module_accessor, 
        &smash::phx::Vector2f{ x: PostureModule::pos_x(fighter.module_accessor) + (16.0 * lr), y: PostureModule::pos_y(fighter.module_accessor) + 2.0}, 
        &Vector2f{ x: 0.0, y: -4.0}, true
    ) == 1;
    VarModule::set_flag(fighter.battle_object, vars::samus::status::ATTACK_LW3_ICE_PILLAR, nearGround);
    fighter.sub_shift_status_main(L2CValue::Ptr(attacklw3_main_loop as *const () as _))
}

unsafe extern "C" fn attacklw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackLw3_Main();
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn attacklw3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::samus::status::ATTACK_LW3_ICE_PILLAR) {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_attacklw32end"), -1);
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_attacklw32end"), -1);
    }
    0.into()
}


#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter){
        return original!(fighter);
    }

    let toReturn = original!(fighter);
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    let targetMotion = Hash40::new("attack_air_f");
    let newMotion = Hash40::new("attack_air_f2");

    if motion == targetMotion.hash {
        MotionModule::change_motion(fighter.module_accessor, newMotion, 0.0, 1.0, false, 0.0, false, false);
        VarModule::on_flag(fighter.battle_object, vars::samus::status::ATTACK_HAS_ICE);
    }
    toReturn
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn attackair_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if is_ice(fighter) && ice_crash(fighter) {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_ice_punch_break"), -1);
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_GAME, Hash40::new("game_attackairf2_break"), -1);
    }
    fighter.sub_attack_air_uniq_process_exec()
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn attackair_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter,Hash40::new("sys_ice"),false,false);
    0.into()
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_S4_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attacks4_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter){
        return original!(fighter);
    }
    fighter.status_AttackS4Start_Common();
    attacks4_anim(fighter,false);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(attacks4_start_loop as *const () as _))
}
unsafe extern "C" fn attacks4_start_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackS4Start_Main()
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attacks4_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter){
        return original!(fighter);
    }
    fighter.status_AttackS4Hold();
    attacks4_anim(fighter,true);
    fighter.sub_shift_status_main(L2CValue::Ptr(attacks4_hold_loop as *const () as _))
}
unsafe extern "C" fn attacks4_hold_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackS4Hold_main()
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attacks4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter){
        return original!(fighter);
    }
    fighter.status_AttackS4();
    attacks4_anim(fighter,false);
    fighter.sub_shift_status_main(L2CValue::Ptr(attacks4_main_loop as *const () as _))
}
unsafe extern "C" fn attacks4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackLw4_Main() //Using Lw4 prevents angling
}

unsafe extern "C" fn attacks4_anim(fighter: &mut L2CFighterCommon, charge: bool) {
    let targetMotion = if charge {Hash40::new("attack_s4_hold2")} else {Hash40::new("attack_s4_s2")};
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    if motion != targetMotion.hash{
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, targetMotion, -1.0, 1.0, 0.0);
    }
    //LinkModule::send_event_nodes(fighter.module_accessor, *LINK_NO_ARTICLE, Hash40::new_raw(0x1c5609e30f), 0);
}

unsafe extern "C" fn attacks4_end_eff(fighter: &mut L2CFighterCommon) {
    COL_NORMAL(fighter);
    if is_ice(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_ice"),false,false);
    }
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn attacks4_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter) {
        return original!(fighter);
    }
    fighter.status_pre_AttackS4();
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLOAT,
        *FS_SUCCEEDS_KEEP_EFFECT,
    );

    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("handr"),&Vector3f{ x: 1.0, y: 1.0, z: 1.0});
    VarModule::on_flag(fighter.battle_object, vars::samus::status::ATTACK_HAS_ICE);
    0.into()
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn attacks4_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter) {
        return 0.into();
    }
    if ice_crash(fighter) {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_ice_lance_break"), -1);
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_GAME, Hash40::new("game_attacks42_break"), -1);
    }
    0.into()
}


#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_HI4_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attackhi4_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter){
        return original!(fighter);
    }
    fighter.status_AttackHi4Start();
    //fighter.status_AttackHi4Start_common(0.into());
    attackhi4_anim(fighter,false);
    
    fighter.sub_shift_status_main(L2CValue::Ptr(attackhi4_start_loop as *const () as _))
}
unsafe extern "C" fn attackhi4_start_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackHi4Start_Main()
}

#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attackhi4_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter){
        return original!(fighter);
    }
    fighter.status_AttackHi4Hold();
    attackhi4_anim(fighter,true);
    fighter.sub_shift_status_main(L2CValue::Ptr(attackhi4_hold_loop as *const () as _))
}
unsafe extern "C" fn attackhi4_hold_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackHi4Hold_main()
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_HI4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attackhi4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter){
        return original!(fighter);
    }
    fighter.status_AttackHi4();
    attackhi4_anim(fighter,false);
    fighter.sub_shift_status_main(L2CValue::Ptr(attackhi4_main_loop as *const () as _))
}
unsafe extern "C" fn attackhi4_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackHi4_Main()
}

unsafe extern "C" fn attackhi4_anim(fighter: &mut L2CFighterCommon, charge: bool) {
    let targetMotion = if charge {Hash40::new("attack_hi4_hold2")} else {Hash40::new("attack_hi42")};
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    if motion != targetMotion.hash{
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, targetMotion, -1.0, 1.0, 0.0);
    }
    //LinkModule::send_event_nodes(fighter.module_accessor, *LINK_NO_ARTICLE, Hash40::new_raw(0x1c5609e30f), 0);
}

unsafe extern "C" fn attackhi4_end_eff(fighter: &mut L2CFighterCommon) {
    COL_NORMAL(fighter);
    if is_ice(fighter) {
        EFFECT_OFF_KIND(fighter,Hash40::new("sys_ice"),false,false);
    }
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_HI4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn attackhi4_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter) {
        return original!(fighter);
    }
    fighter.status_pre_AttackHi4();
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_4_FLOAT,
        *FS_SUCCEEDS_KEEP_EFFECT,
    );

    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("handr"),&Vector3f{ x: 1.0, y: 1.0, z: 1.0});
    VarModule::on_flag(fighter.battle_object, vars::samus::status::ATTACK_HAS_ICE);
    0.into()
}
#[status_script(agent = "samus", status = FIGHTER_STATUS_KIND_ATTACK_HI4, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn attackhi4_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !is_ice(fighter) {
        return 0.into();
    }
    if ice_crash(fighter) {
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_ice_punch_break"), -1);
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_GAME, Hash40::new("game_attackhi42_break"), -1);
    }
    0.into()
}

unsafe extern "C" fn ice_crash(fighter: &mut L2CFighterCommon) -> bool{
    if VarModule::is_flag(fighter.battle_object, vars::samus::status::ATTACK_HAS_ICE) {
        if AttackModule::is_infliction_status(fighter.module_accessor,  
            //*COLLISION_KIND_MASK_HIT  |
            *COLLISION_KIND_MASK_SHIELD
        ){
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)
            {
                MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_SOUND, Hash40::new("sound_ice_break"), -1);
            }
            VarModule::off_flag(fighter.battle_object, vars::samus::status::ATTACK_HAS_ICE);
            return true;
        }
    }
    return false;
}

pub fn install() {
    install_status_scripts!(
        attacklw3_main,
        attacklw3_end,

        attackair_main,
        attackair_exec,
        attackair_end,

        attacks4_start_main,
        attacks4_hold_main,
        attacks4_main,
        attacks4_pre,
        attacks4_exec,

        attackhi4_start_main,
        attackhi4_hold_main,
        attackhi4_main,
        attackhi4_pre,
        attackhi4_exec,
    );
}