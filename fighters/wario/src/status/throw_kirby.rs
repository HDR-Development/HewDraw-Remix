use super::*;

// FIGHTER_STATUS_KIND_THROW_KIRBY

unsafe extern "C" fn throw_kirby_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.boma();

    return fighter.status_pre_ThrowKirby();
}

unsafe extern "C" fn throw_kirby_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_uniq_process_ThrowKirby_initStatus();
    
    let hitStop = 8;
    WorkModule::set_int(fighter.module_accessor, hitStop, *FIGHTER_STATUS_THROW_WORK_INT_STOP_FRAME);

    return false.into();
}

unsafe extern "C" fn throw_kirby_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_ThrowKirby();
}

unsafe extern "C" fn throw_kirby_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("sys_merikomi"),false,true);
    return fighter.sub_status_uniq_process_ThrowKirby_exitStatus();
}

unsafe extern "C" fn throw_kirby_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("sys_merikomi"),false,true);
    return fighter.status_end_ThrowKirby();
}

unsafe extern "C" fn throw_kirby_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let FRAME_FALL = 48.0;
    let FRAME_FALLLOOP = FRAME_FALL+2.0;
    let FRAME_LAND = 55.0;

    let currentFrame = MotionModule::frame(fighter.module_accessor);
    if currentFrame >= FRAME_LAND {
        let grounded = GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let status = if grounded {FIGHTER_STATUS_KIND_WAIT} else {FIGHTER_STATUS_KIND_FALL};
        let lastFrame = if grounded {MotionModule::end_frame(fighter.module_accessor)-3.0} else {MotionModule::end_frame(fighter.module_accessor)-9.0};
        
        if currentFrame >= lastFrame { 
            if !grounded{
                let speed = Vector3f { x: 0.0, y: -0.1, z: 0.0 };
                KineticModule::add_speed(fighter.module_accessor, &speed);
            }
            fighter.change_status(status.into(), false.into());
        }
        return false.into();
    }

    //If we go past a certain frame, then freeze animation and accel downwards
    if (currentFrame >= FRAME_FALLLOOP && currentFrame < FRAME_LAND)
    {
        MotionModule::set_rate(fighter.module_accessor, 0.0);
        let speed = Vector3f { x: 0.0, y: -0.425, z: 0.0 };
        KineticModule::add_speed(fighter.module_accessor, &speed);
    }
    //Groundcast to see if we touched the ground (only after falling), then cut to the landing frame
    if currentFrame >= FRAME_FALL
    {
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32)
        {
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, FRAME_LAND, true,true,false);
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            KineticModule::resume_energy(fighter.module_accessor,*FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            
            //Spawn effects//
            let boma = fighter.boma();
            let opponent = boma.get_grabbed_opponent_boma();
            let opponentScale = PostureModule::scale(opponent);
            //Bury effect
            //EFFECT_FOLLOW(fighter, Hash40::new("sys_merikomi"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, (opponentScale+0.4), true);
            
            LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            EFFECT(fighter, Hash40::new("sys_quake"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
            EFFECT(fighter, Hash40::new("sys_crown_collision"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
    }

    return false.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, *FIGHTER_STATUS_KIND_THROW_KIRBY, throw_kirby_pre);
    agent.status(Init, *FIGHTER_STATUS_KIND_THROW_KIRBY, throw_kirby_init);
    agent.status(Main, *FIGHTER_STATUS_KIND_THROW_KIRBY, throw_kirby_main);
    agent.status(Exit, *FIGHTER_STATUS_KIND_THROW_KIRBY, throw_kirby_exit);
    agent.status(End, *FIGHTER_STATUS_KIND_THROW_KIRBY, throw_kirby_end);
    agent.status(Exec, *FIGHTER_STATUS_KIND_THROW_KIRBY, throw_kirby_exec);
}
