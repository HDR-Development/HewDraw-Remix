use super::*;
use globals::*;

mod special_s;
mod special_hi;

pub fn install() {
    special_s::install();
    special_hi::install();
    smashline::install_agent_init_callbacks!(
        wario_init
    );
    install_status_scripts!(
        catch_attack_exec,
        catch_attack_end,

        wario_throwk_pre,
        wario_throwk_init,
        wario_throwk_main,
        wario_throwk_exit,
        wario_throwk_end,
        wario_throwk_exec,

        wario_landing_attack_end,
        wario_attack_air_exec
    );
}

pub const THROW_HI_STATUS_KIND: i32 = 0x47;
#[smashline::fighter_init]
fn wario_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() == *FIGHTER_KIND_WARIO {
            fighter.global_table[THROW_HI_STATUS_KIND].assign(&FIGHTER_STATUS_KIND_THROW_KIRBY.into());
            println!("Wario throw replaced");
        }
    }
}

//Force opponent rotation
#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_CATCH_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn catch_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue {

    let boma = fighter.boma();

    let mut vec =Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let offset = ModelModule::joint_global_rotation(fighter.module_accessor,Hash40::new("throw"),&mut vec,false);
    let rot = Vector3f{x: vec.x, y: 0.0, z: 0.0};
    PostureModule::set_rot(
        boma.get_grabbed_opponent_boma(),
        &rot,
        0
    );
    return false.into();
}
//Reset opponent rotation
#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_CATCH_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn catch_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.boma();

    PostureModule::set_rot(
        boma.get_grabbed_opponent_boma(),
        &Vector3f::zero(),
        0
    );
    return original!(fighter);
}

#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn wario_throwk_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.boma();

    return fighter.status_pre_ThrowKirby();
}
#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn wario_throwk_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_uniq_process_ThrowKirby_initStatus();
    
    let hitStop = 8;
    WorkModule::set_int(fighter.module_accessor, hitStop, *FIGHTER_STATUS_THROW_WORK_INT_STOP_FRAME);

    return false.into();
}
#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn wario_throwk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_ThrowKirby();
}
#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn wario_throwk_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("sys_merikomi"),false,true);
    return fighter.sub_status_uniq_process_ThrowKirby_exitStatus();
}
#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn wario_throwk_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("sys_merikomi"),false,true);
    return fighter.status_end_ThrowKirby();
}

#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn wario_throwk_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
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
                let speed = smash::phx::Vector3f { x: 0.0, y: -0.1, z: 0.0 };
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
        let speed = smash::phx::Vector3f { x: 0.0, y: -0.425, z: 0.0 };
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


#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn wario_landing_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    EFFECT_OFF_KIND(fighter, Hash40::new("sys_merikomi"),false,true);
    return false.into();
}

#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn wario_attack_air_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dairAnim = Hash40::new("attack_air_lw");
    let dairRiseAnim = Hash40::new("attack_air_lw2");
    
    if MotionModule::motion_kind(fighter.module_accessor) != dairAnim.hash{
        return false.into();
    }
    if (AttackModule::is_infliction_status(fighter.module_accessor,  *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)){
        MotionModule::change_motion(fighter.module_accessor, dairRiseAnim, 18.0, 1.0, false, 0.0, false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_machstamp"),false,true);
        AttackModule::clear_all(fighter.module_accessor);
    }
    
    return false.into();
}