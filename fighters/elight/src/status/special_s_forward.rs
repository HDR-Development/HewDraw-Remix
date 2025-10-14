use super::*;
use globals::*;

unsafe extern "C" fn special_s_forward_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_CHECK_CLIFF);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_NEAR_CLIFF);
    VarModule::set_float(fighter.battle_object, vars::elight::status::SPECIAL_S_ANGLE, 0.0);

    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    if situation == *SITUATION_KIND_GROUND {
        if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            let normal_x = GroundModule::get_touch_normal_x_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
            let normal_y = GroundModule::get_touch_normal_y_consider_gravity(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
            let length = sv_math::vec2_length(normal_x, normal_y);
            if 1e-05_f32 < length {
                let deg = normal_x.atan2(normal_y).to_degrees().abs();
                let angle_check = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), 0x11e7fad1ad);
                if angle_check < deg {
                    let lr = PostureModule::lr(fighter.module_accessor);
                    if 0.0 < normal_x * lr {
                        fighter.set_situation(SITUATION_KIND_AIR.into());
                        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ELIGHT_STATUS_SPECIAL_S_FLAG_IS_TILT);
                    }
                }
            }
        }
    }

    let situation = fighter.global_table[SITUATION_KIND].get_i32();
    fighter.sub_change_motion_by_situation(Hash40::new("special_s").into(), Hash40::new("special_air_s").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(false.into());

    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let dir = if stick_y < -0.5 && situation != *SITUATION_KIND_GROUND {
        vars::elight::SPECIAL_S_ANGLE_LW
    }
    else if stick_y > 0.5 {
        vars::elight::SPECIAL_S_ANGLE_HI
    }
    else {
        vars::elight::SPECIAL_S_ANGLE_NONE
    };
    WorkModule::set_int(fighter.module_accessor, dir, *FIGHTER_ELIGHT_STATUS_SPECIAL_S_WORK_INT_BUNSHIN_NUM);

    let kinetic = if situation == *SITUATION_KIND_GROUND {
        if dir == vars::elight::SPECIAL_S_ANGLE_HI {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE
        }
        else {
            *FIGHTER_KINETIC_TYPE_MOTION
        }
    }
    else {
        *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE
    };
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);

    let speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("speed_x_mul"));
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        speed_x_mul
    );

    if dir != vars::elight::SPECIAL_S_ANGLE_NONE {
        let lr = PostureModule::lr(fighter.module_accessor);
        let mut angle = 15.0;
        if dir == vars::elight::SPECIAL_S_ANGLE_LW {
            angle *= -1.0;
        }
        VarModule::set_float(fighter.battle_object, vars::elight::status::SPECIAL_S_ANGLE, angle);
        angle *= lr;
        sv_kinetic_energy!(
            set_angle,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            angle.to_radians()
        );

    }

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE_DAMAGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CURSOR);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);

    GroundModule::set_shape_flag(fighter.module_accessor, *GROUND_CORRECT_SHAPE_RHOMBUS_MODIFY_FLAG_FRONT_FIX as u16, true);

    fighter.main_shift(special_s_forward_main_loop)
}

unsafe extern "C" fn special_s_forward_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    // cancel module check we won't use

    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 0.into();
    }

    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END.into(), false.into());
            return 0.into();
        }
        else {
            let frame = MotionModule::frame(fighter.module_accessor);
            let rate = MotionModule::rate(fighter.module_accessor);
            // MotionModule::change_motion_inherit_frame(
            //     fighter.module_accessor,
            //     Hash40::new("special_air_s"),
            //     frame,
            //     rate,
            //     0.0,
            //     true,
            //     false
            // );
            // KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
    }

    0.into()
}

unsafe extern "C" fn special_s_forward_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rot = VarModule::get_float(fighter.battle_object, vars::elight::status::SPECIAL_S_ANGLE);
    fighter.set_joint_rotate("rot", Vector3f::new(-rot, 0.0, 0.0));

    0.into()
}

unsafe extern "C" fn special_s_forward_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END {
        VisibilityModule::set_whole(fighter.module_accessor, true);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_BUNSHIN, ArticleOperationTarget(0));
    }

    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_LOUPE_DAMAGE);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CURSOR);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NAME_CURSOR);

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD, special_s_forward_main);
    agent.status(Exec, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD, special_s_forward_exec);
    agent.status(ExecStop, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD, special_s_forward_exec);
    agent.status(End, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_FORWARD, special_s_forward_end);
}