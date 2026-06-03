use super::*;

// vars since ParamModule does not work with articles
const FOLLOW_FRAME: i32 = 256;
//const TURN_INERTIA_FRAME: i32 = 50;
const ROT_SPEED: f32 = 22.0;
const ANGLE_X_BACK: f32 = -16.0;
const ACCEL: f32 = 0.047;
const SPEED_MIN: f32 = 0.3;
const TURN_DIST: f32 = 16.0;
const TURN_ANGLE: f32 = 0.75;
const SPEED_MAX: f32 = 3.0;
const SPEED_MUL: f32 = 1.0;
const TURN_FOLLOW_DIST: f32 = 19.0;

unsafe extern "C" fn move_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = weapon.get_param_float("param_stealthbomb", "life");
    weapon.set_float(life, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_LIFE);
    weapon.set_float(0.0, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_COUNT);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("move"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        move_substatus(weapon, false.into());
    }
    weapon.global_table[0x15].assign(&L2CValue::Ptr(move_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(move_main_loop as *const () as _))
}

unsafe extern "C" fn move_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::add_float(weapon.module_accessor, -1.0, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_LIFE);
        WorkModule::add_float(weapon.module_accessor, 1.0, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_COUNT);
        let count = weapon.get_param_float("param_stealthbomb", "count");
        if weapon.get_float(*WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_COUNT) > count {
            weapon.set_float(count, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_COUNT);
        }
    }

    return 0.into();
}

unsafe extern "C" fn move_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if weapon.get_float(*WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_LIFE) <= 0.0 {
        weapon.change_status(WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_KIND_TAME.into(), false.into());
    }

    return 0.into();
}

unsafe extern "C" fn move_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::req_on_joint(weapon.module_accessor, Hash40::new("sys_misfire"), Hash40::new("top"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
    let angle = weapon.get_float(*WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_ANGLE);
    VarModule::set_float(weapon.battle_object, vars::miigunner_stealthbomb::instance::ANGLE, angle);

    return 0.into();
}

unsafe extern "C" fn tame_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let bang_time = weapon.get_param_float("param_stealthbomb", "bang_time");
    weapon.set_float(bang_time, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_LIFE);
    let angle = VarModule::get_float(weapon.battle_object, vars::miigunner_stealthbomb::instance::ANGLE).to_degrees();
    //sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL, 1.0 * angle.cos(), 1.0 * angle.sin());
    
    return 0.into();
}

unsafe extern "C" fn tame_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn tame_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("sys_misfire"), false, false);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("tame"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        tame_substatus(weapon, false.into());
    }
    weapon.global_table[0x15].assign(&L2CValue::Ptr(tame_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(tame_main_loop as *const () as _))
}

unsafe extern "C" fn tame_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::add_float(weapon.module_accessor, -1.0, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_LIFE);
    }
    else {
        if weapon.get_float(*WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_LIFE) <= 0.0 {
            weapon.change_status(statuses::miigunner_stealthbomb::TURN.into(), false.into());
        }
    }

    return 0.into();
}

unsafe extern "C" fn tame_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn turn_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0,
    );

    return 0.into();
}

unsafe extern "C" fn turn_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);

    let angle = VarModule::get_float(weapon.battle_object, vars::miigunner_stealthbomb::instance::ANGLE);
    let new_angle = if angle <= 0.0 {
        angle + std::f32::consts::PI
    } else {
        angle - std::f32::consts::PI
    };

    VarModule::set_float(weapon.battle_object, vars::miigunner_stealthbomb::instance::ANGLE, new_angle);
    VarModule::set_int(weapon.battle_object, vars::miigunner_stealthbomb::status::FOLLOW_FRAME, FOLLOW_FRAME);
    WorkModule::set_float(weapon.module_accessor, FOLLOW_FRAME as f32, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_LIFE);

    let rot_x = PostureModule::rot_x(weapon.module_accessor, 0);
    let rot_y = PostureModule::rot_y(weapon.module_accessor, 0);
    let rot_z = PostureModule::rot_z(weapon.module_accessor, 0);

    VarModule::set_float(weapon.battle_object, vars::miigunner_stealthbomb::status::TURN_DIST, 0.0);

    let speed = WorkModule::get_float(weapon.module_accessor, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLOAT_SPEED);
    let speed_diff = speed - SPEED_MIN;
    let accel_diff = speed_diff / ACCEL;
    let floor = accel_diff.floor();
    let idkman = ANGLE_X_BACK - rot_y;
    let huh = idkman / floor;
    
    VarModule::set_int(weapon.battle_object, vars::miigunner_stealthbomb::status::BACK_ROT_FRAME, floor as i32);
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL, ROT_SPEED, huh, 0.0);

    return 0.into();
}

unsafe extern "C" fn turn_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("turn"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        if false {
            turn_substatus_inner(weapon);
        }
    }
    weapon.global_table[0x15].assign(&L2CValue::Ptr(turn_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(turn_fastshift as *const () as _))
}

unsafe extern "C" fn turn_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        turn_substatus_inner(weapon);
    }

    return 0.into();
}

unsafe extern "C" fn turn_substatus_inner(weapon: &mut L2CWeaponCommon) {
    if VarModule::countdown_int(weapon.battle_object, vars::miigunner_stealthbomb::status::FOLLOW_FRAME, 0) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
}

unsafe extern "C" fn turn_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_CMD_ARTICLE_GENERATE_ARTICLE_LINK_PARENTS, WEAPON_LINK_NO_CONSTRAINT, FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB_S);
        sv_module_access::article(weapon.lua_state_agent);
        weapon.pop_lua_stack(1);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_HIT) {
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_CMD_ARTICLE_GENERATE_ARTICLE_LINK_PARENTS, WEAPON_LINK_NO_CONSTRAINT, FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB_S);
        sv_module_access::article(weapon.lua_state_agent);
        weapon.pop_lua_stack(1);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x27936dbb96d));
    }
    if !StopModule::is_stop(weapon.module_accessor) {
        if turn_fastshift_inner(weapon).get_bool() {
            return 1.into();
        }
    }

    return 0.into();
}

unsafe extern "C" fn turn_fastshift_inner(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let correct = GroundModule::get_correct(weapon.module_accessor);
    if LinkModule::is_link(weapon.module_accessor, *LINK_NO_ARTICLE) {
        let parent_id = LinkModule::get_parent_id(weapon.module_accessor, *LINK_NO_ARTICLE, true);
        if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLAG_REFLECT) {
            let team_owner_id = TeamModule::team_owner_id(weapon.module_accessor);
            if parent_id == team_owner_id {
                weapon.clear_lua_stack();
                lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_X, LINK_NO_ARTICLE, Hash40::new("waist"), true);
                FL_sv_module_access::link(weapon.lua_state_agent);
                let x = weapon.pop_lua_stack(1).get_f32();
                weapon.clear_lua_stack();
                lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_Y, LINK_NO_ARTICLE, Hash40::new("waist"), true);
                FL_sv_module_access::link(weapon.lua_state_agent);
                let y = weapon.pop_lua_stack(1).get_f32();
                weapon.clear_lua_stack();
                lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_Z, LINK_NO_ARTICLE, Hash40::new("waist"), true);
                FL_sv_module_access::link(weapon.lua_state_agent);
                let z = weapon.pop_lua_stack(1).get_f32();

                let pos_x = PostureModule::pos_x(weapon.module_accessor);
                let pos_y = PostureModule::pos_y(weapon.module_accessor);
                let pos_z = PostureModule::pos_z(weapon.module_accessor);

                let length = sv_math::vec3_length(x - pos_x, y - pos_y, z - pos_z);
                if length <= 9.0 {
                    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
                    return 1.into();
                }
            }
        }

        //if correct != *GROUND_CORRECT_KIND_NONE {
        let parent_id = LinkModule::get_parent_id(weapon.module_accessor, *LINK_NO_ARTICLE, true);
        let parent_module_accessor = sv_battle_object::module_accessor(parent_id as u32);
        let parent_pos_y = PostureModule::pos_y(parent_module_accessor);
        let pos_y = PostureModule::pos_y(weapon.module_accessor);
        let diff = parent_pos_y - pos_y;
        let dist = 4.0;
        if diff.abs() >= dist * 10.0 {
            GroundModule::set_correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
        }
        else {
            GroundModule::set_correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        //}
    }

    let dist = VarModule::get_float(weapon.battle_object, vars::miigunner_stealthbomb::status::TURN_DIST);
    if TURN_DIST * 10.0 <= dist {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 1.into();
    }
    
    if AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_PARRY | *COLLISION_KIND_MASK_REFLECTOR) {
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLAG_REFLECT);
    }

    return 0.into();
}

unsafe extern "C" fn turn_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    turn_exec_inner(weapon)
}

unsafe extern "C" fn turn_exec_inner(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let mut angle = VarModule::get_float(weapon.battle_object, vars::miigunner_stealthbomb::instance::ANGLE);
    //let prev_angle = angle;
    if LinkModule::is_link(weapon.module_accessor, *LINK_NO_ARTICLE) {
        if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_WORK_FLAG_REFLECT)
        && VarModule::get_int(weapon.battle_object, vars::miigunner_stealthbomb::status::FOLLOW_FRAME) > 0 {
            weapon.clear_lua_stack();
            lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_X, LINK_NO_ARTICLE, Hash40::new("waist"), true);
            FL_sv_module_access::link(weapon.lua_state_agent);
            let x = weapon.pop_lua_stack(1).get_f32();
            weapon.clear_lua_stack();
            lua_args!(weapon, FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_Y, LINK_NO_ARTICLE, Hash40::new("waist"), true);
            FL_sv_module_access::link(weapon.lua_state_agent);
            let y = weapon.pop_lua_stack(1).get_f32();
            weapon.clear_lua_stack();

            let pos_x = PostureModule::pos_x(weapon.module_accessor);
            let pos_y = PostureModule::pos_y(weapon.module_accessor);
            
            let diff_x = x - pos_x;
            let diff_y = y - pos_y;
            
            let atan = diff_y.atan2(diff_x);
            
            let atan = if atan < -std::f32::consts::PI {
                atan + std::f32::consts::PI * 2.0
            }
            else {
                if std::f32::consts::PI < atan {
                    atan - std::f32::consts::PI * 2.0
                }
                else {
                    atan
                }
            };

            let atan = atan - angle;
            
            let atan = if atan < -std::f32::consts::PI {
                atan + std::f32::consts::PI * 2.0
            }
            else {
                if std::f32::consts::PI < atan {
                    atan - std::f32::consts::PI * 2.0
                }
                else {
                    atan
                }
            };

            let atan = if TURN_ANGLE < atan {
                TURN_ANGLE
            }
            else {
                if atan < -TURN_ANGLE {
                    -TURN_ANGLE
                }
                else {
                    atan
                }
            };

            angle += atan;

            // let frame = weapon.status_frame();
            // let angle_mul = if frame > TURN_INERTIA_FRAME { 1.0 } else { (1.0 + frame as f32) / TURN_INERTIA_FRAME as f32 };
            // let angle -= prev_angle - prev_angle * angle_mul;

            VarModule::set_float(weapon.battle_object, vars::miigunner_stealthbomb::instance::ANGLE, angle);
            VarModule::dec_int(weapon.battle_object, vars::miigunner_stealthbomb::status::FOLLOW_FRAME);
        }
    }

    weapon.clear_lua_stack();
    lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let mut length = sv_kinetic_energy::get_speed_length(weapon.lua_state_agent);
    length += ACCEL;
    let speed_max = SPEED_MAX * SPEED_MUL;
    if speed_max < length {
        length = speed_max;
    }
    let cos = angle.cos();
    let sin = angle.sin();
    let vel_x = cos * length;
    let vel_y = sin * length;
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, vel_x, vel_y);

    let turn_dist = VarModule::get_float(weapon.battle_object, vars::miigunner_stealthbomb::status::TURN_DIST);
    VarModule::set_float(weapon.battle_object, vars::miigunner_stealthbomb::status::TURN_DIST, turn_dist + length);

    if TURN_FOLLOW_DIST * 10.0 <= turn_dist + length {
        VarModule::set_int(weapon.battle_object, vars::miigunner_stealthbomb::status::FOLLOW_FRAME, 0);
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }

    let back_rot_frame = VarModule::get_int(weapon.battle_object, vars::miigunner_stealthbomb::status::BACK_ROT_FRAME);
    if back_rot_frame > 0 {
        if back_rot_frame - 1 == 0 {
            sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL, ROT_SPEED, 0.0, 0.0);
        }
        VarModule::dec_int(weapon.battle_object, vars::miigunner_stealthbomb::status::BACK_ROT_FRAME);
    }

    return 0.into();
}

unsafe extern "C" fn turn_end(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD) {
        weapon.clear_lua_stack();
        lua_args!(weapon, MA_MSC_CMD_ARTICLE_GENERATE_ARTICLE_LINK_PARENTS, WEAPON_LINK_NO_CONSTRAINT, FIGHTER_MIIGUNNER_GENERATE_ARTICLE_STEALTHBOMB_S);
        sv_module_access::article(weapon.lua_state_agent);
        weapon.pop_lua_stack(1);
    }

    return 0.into();
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_KIND_MOVE, move_main);
    agent.status(End, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_KIND_MOVE, move_end);

    agent.status(Init, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_KIND_TAME, tame_init);
    agent.status(Main, *WEAPON_MIIGUNNER_STEALTHBOMB_STATUS_KIND_TAME, tame_main);

    agent.status(Pre, statuses::miigunner_stealthbomb::TURN, turn_pre);
    agent.status(Init, statuses::miigunner_stealthbomb::TURN, turn_init);
    agent.status(Main, statuses::miigunner_stealthbomb::TURN, turn_main);
    agent.status(Exec, statuses::miigunner_stealthbomb::TURN, turn_exec);
    agent.status(End, statuses::miigunner_stealthbomb::TURN, turn_end);
}