// subroutines that are called by various status scripts
// status imports
use super::*;
use globals::*;

use super::fighter_status_guard;

#[skyline::hook(replace = L2CFighterCommon_sub_guard_cont_pre)]
pub unsafe fn sub_guard_cont_pre(fighter: &mut L2CFighterCommon) {
    WorkModule::enable_transition_term(
        fighter.module_accessor,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD
    );
    if
        fighter.global_table[STATUS_KIND_INTERRUPT] == FIGHTER_STATUS_KIND_GUARD_ON &&
        fighter.global_table[PREV_STATUS_KIND] == FIGHTER_STATUS_KIND_RUN
    {
        WorkModule::enable_transition_term(
            fighter.module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN
        );
        WorkModule::enable_transition_term(
            fighter.module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH
        );
        let catch_dash_frame = WorkModule::get_param_int(
            fighter.module_accessor,
            hash40("common"),
            hash40("catch_dash_frame")
        );
        WorkModule::set_int(
            fighter.module_accessor,
            catch_dash_frame,
            *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME
        );
    }
    let enables = [
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F,
        *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B,
    ];
    for x in enables.iter() {
        WorkModule::enable_transition_term(fighter.module_accessor, *x);
    }
    WorkModule::enable_transition_term_group(
        fighter.module_accessor,
        *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP
    );
}

#[skyline::hook(replace = L2CFighterCommon_sub_guard_on_uniq)]
pub unsafe fn sub_guard_on_uniq(fighter: &mut L2CFighterCommon, arg: L2CValue) -> L2CValue {
    if !arg.get_bool() {
        fighter_status_guard::landing_effect_control(fighter);
    } else {
        if
            !WorkModule::is_flag(
                fighter.module_accessor,
                *FIGHTER_INSTANCE_WORK_ID_FLAG_SHIELD_LOCK
            )
        {
            let shield_dec1 = WorkModule::get_param_float(
                fighter.module_accessor,
                hash40("common"),
                hash40("shield_dec1")
            );
            // let analog = InputModule::get_analog_for_guard(fighter.battle_object);
            // let dec = if analog > 0.0 && analog < 1.0 {
            //     let variable = 1.9 * analog;
            //     (variable + 0.1) * shield_dec1 / 2.0
            // } else {
            let dec = shield_dec1;
            // };

            let shield_frame = WorkModule::get_param_float(
                fighter.module_accessor,
                hash40("shield_frame"),
                0
            );
            let shield_dec_rate = dec / shield_frame;
            WorkModule::sub_float(
                fighter.module_accessor,
                shield_dec_rate,
                *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD
            );
        }
        let shield = WorkModule::get_float(
            fighter.module_accessor,
            *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD
        );
        let minimum_shield = WorkModule::get_float(
            fighter.module_accessor,
            *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MIN
        );
        if shield < minimum_shield {
            WorkModule::set_float(
                fighter.module_accessor,
                minimum_shield,
                *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MIN
            );
        }
        let min_frame = WorkModule::get_int(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME
        );
        if 0 < min_frame {
            WorkModule::dec_int(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME
            );
        }
        let catch_frame = WorkModule::get_int(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME
        );
        if 0 < catch_frame {
            WorkModule::dec_int(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_INT_CATCH_FRAME
            );
        } else {
            WorkModule::unable_transition_term(
                fighter.module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN
            );
            WorkModule::unable_transition_term(
                fighter.module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH
            );
        }
    }
    L2CValue::I32(0)
}

#[skyline::hook(replace = L2CFighterCommon_check_guard_hold)]
pub unsafe fn check_guard_hold(fighter: &mut L2CFighterCommon) -> L2CValue {
    let guard_hold = ControlModule::check_button_on(
        fighter.module_accessor,
        *CONTROL_PAD_BUTTON_GUARD_HOLD
    );
    L2CValue::Bool(guard_hold)
}

#[skyline::hook(replace = L2CFighterCommon_check_guard_attack_special_hi)]
pub unsafe fn check_guard_attack_special_hi(
    fighter: &mut L2CFighterCommon,
    arg: L2CValue
) -> L2CValue {
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if
        WorkModule::is_enable_transition_term(
            fighter.module_accessor,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START
        ) &&
        (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 &&
        fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
    {
        // if we were about to USmash, check that we shouldn't item toss instead
        if ItemModule::is_have_item(fighter.module_accessor, 0) {
            if check_item_oos(fighter).get_bool() {
                return true.into();
            }
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI4_START.into(), true.into());
            return true.into();
        }
    }
    let special_stick_y = WorkModule::get_param_float(
        fighter.module_accessor,
        hash40("common"),
        hash40("special_stick_y")
    );
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    if
        WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_SPECIAL_HI
        ) ||
        special_stick_y <= stick_y
    {
        if
            WorkModule::is_enable_transition_term(
                fighter.module_accessor,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI
            ) &&
            (cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI) != 0
        {
            let should_cancel = if fighter.global_table[0x3a].get_bool() {
                let callable: extern "C" fn(
                    &mut L2CFighterCommon
                ) -> L2CValue = std::mem::transmute(fighter.global_table[0x3a].get_ptr());
                callable(fighter).get_bool()
            } else {
                true
            };
            if should_cancel {
                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
                return true.into();
            }
        }
        WorkModule::off_flag(
            fighter.module_accessor,
            *FIGHTER_STATUS_GUARD_ON_WORK_FLAG_SPECIAL_HI
        );
    }
    false.into()
}

pub unsafe fn check_escape_oos(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;

    if fighter.check_guard_hold().get_bool() {
        return false.into();
    }

    let c_stick_override = fighter.is_button_on(Buttons::CStickOverride);
    let c_stick_on = dbg!(
        !VarModule::is_flag(
            fighter.battle_object,
            vars::common::instance::DISABLE_CSTICK_BUFFER_ROLL_OOS
        ) && (fighter.is_button_on(Buttons::CStickOn) || c_stick_override)
    );
    let sub_stick_x = if c_stick_override {
        ControlModule::get_stick_x(boma) * PostureModule::lr(boma)
    } else {
        ControlModule::get_sub_stick_x(boma) * PostureModule::lr(boma)
    };
    let sub_stick_y = if c_stick_override {
        ControlModule::get_stick_y(boma)
    } else {
        ControlModule::get_sub_stick_y(boma)
    };
    let stick_vertical = sub_stick_y.abs() >= sub_stick_x.abs() && sub_stick_y < 0.0;

    let escapes = [
        (
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE,
            // checks if Cat2::StickEscape, or CStickOn and stick is vertical and below zero
            fighter.is_cat_flag(Cat2::StickEscape) || (c_stick_on && stick_vertical),
            *FIGHTER_STATUS_KIND_ESCAPE,
        ),
        (
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F,
            // checks if Cat2::StickEscape, or CStickOn and stick is horizontal and above zero
            fighter.is_cat_flag(Cat2::StickEscapeF) ||
                (c_stick_on && !stick_vertical && sub_stick_x >= 0.0),
            *FIGHTER_STATUS_KIND_ESCAPE_F,
        ),
        (
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B,
            // checks if Cat2::StickEscape, or CStickOn and stick is horizontal and above zero
            fighter.is_cat_flag(Cat2::StickEscapeB) ||
                (c_stick_on && !stick_vertical && sub_stick_x < 0.0),
            *FIGHTER_STATUS_KIND_ESCAPE_B,
        ),
    ];

    for (term, condition, status) in escapes.iter() {
        if WorkModule::is_enable_transition_term(boma, *term) && *condition {
            // NOTE: DO NOT TOUCH
            // We must pass `false` to `change_status` so that the game does not clear our buffer/pad flag.
            // When it is done via `change_status`, the game will regenerate them the next time `sub_shift_status_main` is called.
            fighter.change_status((*status).into(), false.into());
            // We then must pass `true` to `clear_command` so that game "forgets" that we cleared our buffer
            // and will not regenerate our pad flags
            ControlModule::clear_command(fighter.module_accessor, true);
            return true.into();
        }
    }

    return false.into();
}

pub unsafe fn check_item_oos(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let pad_flag = fighter.global_table[PAD_FLAG].get_i32();
    let cat3 = fighter.global_table[CMD_CAT3].get_i32();

    let item_throwable =
        ItemModule::is_have_item(boma, 0) &&
        fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND &&
        WorkModule::is_enable_transition_term(
            boma,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD
        ) &&
        // wizard shit
        ({
            fighter.clear_lua_stack();
            lua_args!(fighter, MA_MSC_ITEM_CHECK_HAVE_ITEM_TRAIT, ITEM_TRAIT_FLAG_NO_THROW);
            app::sv_module_access::item(fighter.lua_state_agent);
            !fighter.pop_lua_stack(1).get_bool()
        });

    if
        item_throwable &&
        ((pad_flag & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) != 0 ||
            (cat3 &
                (*FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI |
                    *FIGHTER_PAD_CMD_CAT3_ITEM_LIGHT_THROW_HI4)) != 0)
    {
        fighter.change_status(FIGHTER_STATUS_KIND_ITEM_THROW.into(), false.into());
        return true.into();
    }

    return false.into();
}

pub unsafe fn check_grab_oos(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    if
        // check grab
        fighter.get_int(*FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CATCH_FRAME) == 0 &&
        WorkModule::is_enable_transition_term(
            boma,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH
        ) &&
        fighter.is_cat_flag(Cat1::Catch) &&
        fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
    {
        fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), true.into());
        return true.into();
    }
    return false.into();
}

pub unsafe fn check_plat_drop_oos(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let cat2 = fighter.global_table[CMD_CAT2].get_i32();
    let guard_hold = fighter.check_guard_hold().get_bool();
    if guard_hold {
        // If we are in shield lock, shield drop input only requires a downwards flick (or taunt input)
        if
            GroundModule::is_passable_ground(boma) &&
            (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS) != 0
        {
            fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
            return true.into();
        }
    } else {
        // If your left stick is near the rim and you haven't triggered a roll
        let escape_fb_stick_x = WorkModule::get_param_float(
            boma,
            hash40("common"),
            hash40("escape_fb_stick_x")
        );
        if
            fighter.global_table[STICK_X].get_f32().abs() > escape_fb_stick_x &&
            (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_F) == 0 &&
            (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_B) == 0 &&
            !VarModule::is_flag(fighter.battle_object, vars::common::status::ENABLE_UCF)
        {
            // Enable UCF shielddrop thresholds
            // change spotdodge y req from -0.75 to -0.8
            // change platdrop y req from -0.71 to -0.675
            VarModule::on_flag(fighter.battle_object, vars::common::status::ENABLE_UCF);
        }
        // Shielddrop with either traditional shielddrop input, or with taunt buttons
        if GroundModule::is_passable_ground(boma) && fighter.is_cat_flag(CatHdr::ShieldDrop) {
            fighter.change_status(FIGHTER_STATUS_KIND_PASS.into(), true.into());
            return true.into();
        }
    }
    return false.into();
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_guard_cont)]
pub unsafe fn sub_guard_cont(fighter: &mut L2CFighterCommon) -> L2CValue {
    // TODO: document this
    if fighter.global_table[0x34].get_bool() {
        let callable: extern "C" fn(&mut L2CFighterCommon) -> L2CValue = std::mem::transmute(
            fighter.global_table[0x34].get_ptr()
        );
        if callable(fighter).get_bool() {
            return true.into();
        }
    }

    // check shorthop aerial
    if fighter.sub_transition_group_check_ground_jump_mini_attack().get_bool() {
        return true.into();
    }

    // check USpecial/USmash
    let guard_hold = fighter.check_guard_hold().get_bool();
    if fighter.check_guard_attack_special_hi(guard_hold.into()).get_bool() {
        return true.into();
    }

    // check jump
    if
        fighter.sub_check_button_jump().get_bool() ||
        (!guard_hold && fighter.sub_check_button_frick().get_bool())
    {
        fighter.change_status(FIGHTER_STATUS_KIND_JUMP_SQUAT.into(), true.into());
        return true.into();
    }

    if check_escape_oos(fighter).get_bool() {
        return true.into();
    }

    if ItemModule::is_have_item(fighter.module_accessor, 0) {
        if check_item_oos(fighter).get_bool() {
            return true.into();
        }
    } else if check_grab_oos(fighter).get_bool() {
        return true.into();
    }

    if check_plat_drop_oos(fighter).get_bool() {
        return true.into();
    }

    // check parry
    if fighter.is_parry_input() {
        fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), true.into());
        VarModule::on_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF);
        return true.into();
    }

    return false.into();
}

// enables (if disabled) cstick buffered rolls, if we let go of the cstick since starting shield
// returns true if previously disabled, but now enabled
pub unsafe fn check_enable_cstick_buffer_rolls(fighter: &mut L2CFighterCommon) -> L2CValue {
    if
        VarModule::is_flag(
            fighter.battle_object,
            vars::common::instance::DISABLE_CSTICK_BUFFER_ROLL_OOS
        ) &&
        !fighter.is_button_on(Buttons::CStickOn)
    {
        VarModule::off_flag(
            fighter.battle_object,
            vars::common::instance::DISABLE_CSTICK_BUFFER_ROLL_OOS
        );
        return true.into();
    }
    return false.into();
}

#[skyline::hook(replace = L2CFighterCommon_status_guard_main_common)]
pub unsafe fn status_guard_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let shield = WorkModule::get_float(
        fighter.module_accessor,
        *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD
    );

    let handle = VarModule::get_int(
        fighter.object(),
        vars::common::instance::SHIELD_EFFECT_HANDLE
    ) as u32;

    // let analog = InputModule::get_analog_for_guard(fighter.object());
    // if analog > 0.0 && analog < 1.0 {
    //     EffectModule::set_alpha(fighter.module_accessor, handle as _, analog);
    // } else {
    //     EffectModule::set_alpha(fighter.module_accessor, handle as _, 1.0);
    // }

    if shield < 0.0 {
        fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY.into(), false.into());
        true.into()
    } else {
        if
            ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) &&
            WorkModule::get_int(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME
            ) <= 0 &&
            fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND
        {
            fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), true.into());
            VarModule::off_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF);
            true.into()
        } else {
            false.into()
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardFunc_updateShield)]
pub unsafe fn sub_ftStatusUniqProcessGuardFunc_updateShield(
    fighter: &mut L2CFighterCommon,
    arg: L2CValue
) {
    if
        !WorkModule::is_flag(
            fighter.module_accessor,
            *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION
        )
    {
        let stick_x = ControlModule::get_stick_x_no_clamp(fighter.module_accessor);
        let stick_y = ControlModule::get_stick_y_no_clamp(fighter.module_accessor);
        let (delta_x, delta_y) = if arg.get_bool() {
            (stick_x, stick_y)
        } else {
            let reach_mul = WorkModule::get_param_float(
                fighter.module_accessor,
                hash40("common"),
                hash40("shield_comp_reach_mul")
            );
            let prev_x = WorkModule::get_float(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_REACH_PREV_X
            );
            let prev_y = WorkModule::get_float(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_REACH_PREV_Y
            );
            ((stick_x - prev_x) * reach_mul + prev_x, (stick_y - prev_y) * reach_mul + prev_y)
        };

        let (stick_x, stick_y) = if !arg.get_bool() {
            let comp_mul = WorkModule::get_param_float(
                fighter.module_accessor,
                hash40("common"),
                hash40("shield_comp_mul")
            );
            let prev_x = WorkModule::get_float(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_X
            );
            let prev_y = WorkModule::get_float(
                fighter.module_accessor,
                *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_Y
            );

            ((stick_x - prev_x) * comp_mul + prev_x, (stick_y - prev_y) * comp_mul + prev_y)
        } else {
            (stick_x, stick_y)
        };

        super::fighter_status_guard::set_guard_blend_motion(
            fighter,
            delta_x.into(),
            delta_y.into(),
            stick_x.into(),
            stick_y.into(),
            arg
        );
        WorkModule::set_float(
            fighter.module_accessor,
            stick_x,
            *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_X
        );
        WorkModule::set_float(
            fighter.module_accessor,
            stick_y,
            *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_PREV_Y
        );
        WorkModule::set_float(
            fighter.module_accessor,
            delta_x,
            *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_REACH_PREV_X
        );
        WorkModule::set_float(
            fighter.module_accessor,
            delta_y,
            *FIGHTER_STATUS_GUARD_ON_WORK_FLOAT_REACH_PREV_Y
        );
    }

    let shield = WorkModule::get_float(
        fighter.module_accessor,
        *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD
    );
    let scale = super::fighter_status_guard::calc_shield_scale(fighter, shield.into()).get_f32();
    ModelModule::set_joint_scale(
        fighter.module_accessor,
        Hash40::new("throw"),
        &(Vector3f {
            x: scale,
            y: scale,
            z: scale,
        })
    );
}

pub fn install() {
    skyline::install_hooks!(
        sub_guard_cont_pre,
        sub_guard_on_uniq,
        check_guard_hold,
        check_guard_attack_special_hi,
        status_guard_main_common,
        sub_ftStatusUniqProcessGuardFunc_updateShield,
        sub_guard_cont
    );
}
