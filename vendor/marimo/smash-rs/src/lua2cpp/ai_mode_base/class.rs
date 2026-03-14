use std::ops::{Deref, DerefMut};

use crate::*;

use super::cpp::*;

#[repr(C)]
pub struct L2CFighterAIModeBase {
    ai_base: lua2cpp::L2CFighterAIBase, // 0x0
    pub private_table: lib::L2CValue, // 0x158
    pub zero_globals1: lib::L2CValue, // 0x168
    pub one_globals1: lib::L2CValue, // 0x178
    pub two_globals1: lib::L2CValue, // 0x188
    pub three_globals1: lib::L2CValue, // 0x198
    pub one_globals2: lib::L2CValue, // 0x1A8
    pub two_globals2: lib::L2CValue, // 0x1B8
    pub three_globals2: lib::L2CValue, // 0x1C8
    pub four_globals2: lib::L2CValue, // 0x1D8
    pub global_table: lib::L2CValue, // 0x1e8
    pub action_mode_34: lib::L2CValue, // 0x1f8
    pub action_mode_28: lib::L2CValue, // 0x208
    pub action_mode_3: lib::L2CValue, // 0x218
    pub action_mode_14: lib::L2CValue, // 0x228
    pub action_mode_15: lib::L2CValue, // 0x238
    pub action_mode_32: lib::L2CValue, // 0x248
    pub action_mode_23: lib::L2CValue, // 0x258
    pub action_mode_9: lib::L2CValue, // 0x268
    pub action_mode_7: lib::L2CValue, // 0x278
    pub action_mode_8: lib::L2CValue, // 0x288
    pub action_mode_1: lib::L2CValue, // 0x298
    pub action_mode_52: lib::L2CValue, // 0x2a8
    pub action_mode_53: lib::L2CValue, // 0x2b8
    pub action_mode_16: lib::L2CValue, // 0x2c8
    pub action_mode_17: lib::L2CValue, // 0x2d8
    pub action_mode_10: lib::L2CValue, // 0x2e8
    pub action_mode_24: lib::L2CValue, // 0x2f8
    pub action_mode_25: lib::L2CValue, // 0x308
    pub func_table: lib::L2CValue, // 0x318
    pub action_mode_44: lib::L2CValue, // 0x328
    pub action_mode_2: lib::L2CValue, // 0x338
    pub action_mode_31: lib::L2CValue, // 0x348
    pub action_mode_27: lib::L2CValue, // 0x358
    pub action_mode_57: lib::L2CValue, // 0x368
    pub action_mode_58: lib::L2CValue, // 0x378
    pub action_mode_5: lib::L2CValue, // 0x388
    pub action_mode_6: lib::L2CValue, // 0x398
    pub action_mode_0: lib::L2CValue, // 0x3a8
    pub action_mode_41: lib::L2CValue, // 0x3b8
    pub action_mode_46: lib::L2CValue, // 0x3c8
    pub action_mode_45: lib::L2CValue, // 0x3d8
    pub action_mode_40: lib::L2CValue, // 0x3E8
    pub action_mode_42: lib::L2CValue, // 0x3f8
    pub action_mode_51: lib::L2CValue, // 0x408
    pub action_mode_55: lib::L2CValue, // 0x418
    pub action_mode_56: lib::L2CValue, // 0x428
    pub action_mode_36: lib::L2CValue, // 0x438
    pub action_mode_39: lib::L2CValue, // 0x448
    pub action_mode_54: lib::L2CValue, // 0x458
    pub action_mode_33: lib::L2CValue, // 0x468
    pub action_mode_37: lib::L2CValue, // 0x478
    pub action_mode_35: lib::L2CValue, // 0x488
    pub action_mode_38: lib::L2CValue, // 0x498
    pub action_mode_13: lib::L2CValue, // 0x4a8
    pub action_mode_49: lib::L2CValue, // 0x4b8
    pub action_mode_47: lib::L2CValue, // 0x4c8
    pub action_mode_48: lib::L2CValue, // 0x4d8
    pub action_mode_43: lib::L2CValue, // 0x4e8
    pub unk_val: lib::L2CValue, // 0x4f8
    pub action_mode_59: lib::L2CValue, // 0x508
    pub action_mode_50: lib::L2CValue, // 0x518
    pub action_mode_12: lib::L2CValue, // 0x528
    pub action_mode_19: lib::L2CValue, // 0x538
    pub unk_val2: lib::L2CValue, // 0x548
    pub action_mode_4: lib::L2CValue, // 0x558
    pub action_mode_11: lib::L2CValue, // 0x568
    pub action_mode_18: lib::L2CValue, // 0x578
    pub action_mode_20: lib::L2CValue, // 0x588
    pub action_mode_21: lib::L2CValue, // 0x598
    pub action_mode_22: lib::L2CValue, // 0x5a8
    pub action_mode_26: lib::L2CValue, // 0x5b8
    pub action_mode_29: lib::L2CValue, // 0x5c8
    pub action_mode_30: lib::L2CValue, // 0x5d8
    pub action_run: lib::L2CValue, // 0x5E8
    pub attack_air_attack: lib::L2CValue, // 0x5F8
    pub action_guard: lib::L2CValue, // 0x608
    pub action_escape: lib::L2CValue, // 0x618
    pub action_escape_near: lib::L2CValue, // 0x628
    pub action_escape_far: lib::L2CValue, // 0x638
    pub action_escape_air: lib::L2CValue, // 0x648
    pub action_escape_air_move: lib::L2CValue, // 0x658
    pub action_roll_f: lib::L2CValue, // 0x668
    pub action_roll_b: lib::L2CValue, // 0x678
    pub action_walk: lib::L2CValue, // 0x688
    pub action_dash_attack: lib::L2CValue, // 0x698
    pub action_dash_guard: lib::L2CValue, // 0x6a8
    pub action_dash_roll_f: lib::L2CValue, // 0x6b8
    pub action_dash_roll_b: lib::L2CValue, // 0x6c8
    pub action_dash_b: lib::L2CValue, // 0x6d8
    pub action_dash_b_dash_f: lib::L2CValue, // 0x6e8
    pub action_dash_f_dash_b: lib::L2CValue, // 0x6f8
    pub action_dash_f_jump_b: lib::L2CValue, // 0x708
    pub action_dash_b_jump_s: lib::L2CValue, // 0x718
    pub action_dash_f_wait: lib::L2CValue, // 0x728
    pub action_turn_turn: lib::L2CValue, // 0x738
    pub action_drill: lib::L2CValue, // 0x748
    pub action_damaged: lib::L2CValue, // 0x758
    pub action_fall: lib::L2CValue, // 0x768
    pub action_stroll_s: lib::L2CValue, // 0x778
    pub action_stroll_l: lib::L2CValue, // 0x788
    pub action_stroll_squat: lib::L2CValue, // 0x798
    pub action_wait_s: lib::L2CValue, // 0x7a8
    pub action_wait_l: lib::L2CValue, // 0x7b8
    pub action_wait_squat: lib::L2CValue, // 0x7c8
    pub action_pursue: lib::L2CValue, // 0x7d8
    pub action_return: lib::L2CValue, // 0x7e8
    pub action_jump_attack: lib::L2CValue, // 0x7f8
    pub action_jump_attack_f: lib::L2CValue, // 0x808
    pub action_jump_s_attack_f: lib::L2CValue, // 0x818
    pub action_jump_s_b: lib::L2CValue, // 0x828
    pub action_jump_s_n_attack: lib::L2CValue, // 0x838
    pub action_rebirth: lib::L2CValue, // 0x848
    pub action_down: lib::L2CValue, // 0x858
    pub action_cliff: lib::L2CValue, // 0x868
    pub action_dragoon: lib::L2CValue, // 0x878
    pub action_combo: lib::L2CValue, // 0x888
    pub action_entry_l: lib::L2CValue, // 0x898
    pub action_appeal: lib::L2CValue, // 0x8a8
    pub action_buildup: lib::L2CValue, // 0x8b8
    pub action_meteor: lib::L2CValue, // 0x8c8
    pub action_attack_123: lib::L2CValue, // 0x8d8
    pub action_pass_floor: lib::L2CValue, // 0x8e8
    pub action_run_attack: lib::L2CValue, // 0x8f8
    pub action_final_end_jump: lib::L2CValue, // 0x908
    pub action_machstamp: lib::L2CValue, // 0x918
    pub action_high_speed_dash: lib::L2CValue, // 0x928
    pub action_avoid_donkey_special_lw: lib::L2CValue, // 0x938
    pub action_avoid_koopa_special_s: lib::L2CValue, // 0x948
    pub action_avoid_ganon_special_air_s: lib::L2CValue, // 0x958
    pub action_deadarea_air_attack: lib::L2CValue, // 0x968
    pub action_deadarea_dash_attack: lib::L2CValue, // 0x978
    pub action_count: lib::L2CValue, // 0x988
}

impl Deref for L2CFighterAIModeBase {
    type Target = lua2cpp::L2CFighterAIBase;

    fn deref(&self) -> &Self::Target {
        &self.ai_base
    }
}

impl DerefMut for L2CFighterAIModeBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ai_base
    }
}

impl L2CFighterAIModeBase {
    pub fn main_func__mode_global_variables(&mut self) {
        unsafe {
            main_func__mode_global_variables(self)
        }
    }

    pub fn main_func__mode_private_variables(&mut self) {
        unsafe {
            main_func__mode_private_variables(self)
        }
    }

    pub fn main_func__action_air_attack(&mut self) {
        unsafe {
            main_func__action_air_attack(self)
        }
    }

    pub fn main_func__action_appeal(&mut self) {
        unsafe {
            main_func__action_appeal(self)
        }
    }

    pub fn main_func__action_attack_123(&mut self) {
        unsafe {
            main_func__action_attack_123(self)
        }
    }

    pub fn main_func__action_avoid_donkey_special_lw(&mut self) {
        unsafe {
            main_func__action_avoid_donkey_special_lw(self)
        }
    }

    pub fn main_func__action_avoid_ganon_special_air_s(&mut self) {
        unsafe {
            main_func__action_avoid_ganon_special_air_s(self)
        }
    }

    pub fn main_func__action_avoid_koopa_special_s(&mut self) {
        unsafe {
            main_func__action_avoid_koopa_special_s(self)
        }
    }

    pub fn main_func__action_buildup(&mut self) {
        unsafe {
            main_func__action_buildup(self)
        }
    }

    pub fn main_func__action_cliff(&mut self) {
        unsafe {
            main_func__action_cliff(self)
        }
    }

    pub fn main_func__action_combo(&mut self) {
        unsafe {
            main_func__action_combo(self)
        }
    }

    pub fn main_func__action_damaged(&mut self) {
        unsafe {
            main_func__action_damaged(self)
        }
    }

    pub fn main_func__action_dash_attack(&mut self) {
        unsafe {
            main_func__action_dash_attack(self)
        }
    }

    pub fn main_func__action_dash_b(&mut self) {
        unsafe {
            main_func__action_dash_b(self)
        }
    }

    pub fn main_func__action_dash_b_dash_f(&mut self) {
        unsafe {
            main_func__action_dash_b_dash_f(self)
        }
    }

    pub fn main_func__action_dash_b_jump_s(&mut self) {
        unsafe {
            main_func__action_dash_b_jump_s(self)
        }
    }

    pub fn main_func__action_dash_f_dash_b(&mut self) {
        unsafe {
            main_func__action_dash_f_dash_b(self)
        }
    }

    pub fn main_func__action_dash_f_jump_b(&mut self) {
        unsafe {
            main_func__action_dash_f_jump_b(self)
        }
    }

    pub fn main_func__action_dash_f_wait(&mut self) {
        unsafe {
            main_func__action_dash_f_wait(self)
        }
    }

    pub fn main_func__action_dash_guard(&mut self) {
        unsafe {
            main_func__action_dash_guard(self)
        }
    }

    pub fn main_func__action_dash_roll_b(&mut self) {
        unsafe {
            main_func__action_dash_roll_b(self)
        }
    }

    pub fn main_func__action_dash_roll_f(&mut self) {
        unsafe {
            main_func__action_dash_roll_f(self)
        }
    }

    pub fn main_func__action_deadarea_air_attack(&mut self) {
        unsafe {
            main_func__action_deadarea_air_attack(self)
        }
    }

    pub fn main_func__action_deadarea_dash_attack(&mut self) {
        unsafe {
            main_func__action_deadarea_dash_attack(self)
        }
    }

    pub fn main_func__action_down(&mut self) {
        unsafe {
            main_func__action_down(self)
        }
    }

    pub fn main_func__action_dragoon(&mut self) {
        unsafe {
            main_func__action_dragoon(self)
        }
    }

    pub fn main_func__action_drill(&mut self) {
        unsafe {
            main_func__action_drill(self)
        }
    }

    pub fn main_func__action_entry_1(&mut self) {
        unsafe {
            main_func__action_entry_1(self)
        }
    }

    pub fn main_func__action_escape(&mut self) {
        unsafe {
            main_func__action_escape(self)
        }
    }

    pub fn main_func__action_escape_air(&mut self) {
        unsafe {
            main_func__action_escape_air(self)
        }
    }

    pub fn main_func__action_escape_air_move(&mut self) {
        unsafe {
            main_func__action_escape_air_move(self)
        }
    }

    pub fn main_func__action_escape_far(&mut self) {
        unsafe {
            main_func__action_escape_far(self)
        }
    }

    pub fn main_func__action_escape_near(&mut self) {
        unsafe {
            main_func__action_escape_near(self)
        }
    }

    pub fn main_func__action_fall(&mut self) {
        unsafe {
            main_func__action_fall(self)
        }
    }

    pub fn main_func__action_final_end_jump(&mut self) {
        unsafe {
            main_func__action_final_end_jump(self)
        }
    }

    pub fn main_func__action_guard(&mut self) {
        unsafe {
            main_func__action_guard(self)
        }
    }

    pub fn main_func__action_high_speed_dash(&mut self) {
        unsafe {
            main_func__action_high_speed_dash(self)
        }
    }

    pub fn main_func__action_jump_attack(&mut self) {
        unsafe {
            main_func__action_jump_attack(self)
        }
    }

    pub fn main_func__action_jump_attack_f(&mut self) {
        unsafe {
            main_func__action_jump_attack_f(self)
        }
    }

    pub fn main_func__action_jump_s_attack(&mut self) {
        unsafe {
            main_func__action_jump_s_attack(self)
        }
    }

    pub fn main_func__action_jump_s_b(&mut self) {
        unsafe {
            main_func__action_jump_s_b(self)
        }
    }

    pub fn main_func__action_jump_s_n_attack(&mut self) {
        unsafe {
            main_func__action_jump_s_n_attack(self)
        }
    }

    pub fn main_func__action_machstamp(&mut self) {
        unsafe {
            main_func__action_machstamp(self)
        }
    }

    pub fn main_func__action_meteor(&mut self) {
        unsafe {
            main_func__action_meteor(self)
        }
    }

    pub fn main_func__action_pass_floor(&mut self) {
        unsafe {
            main_func__action_pass_floor(self)
        }
    }

    pub fn main_func__action_pursue(&mut self) {
        unsafe {
            main_func__action_pursue(self)
        }
    }

    pub fn main_func__action_rebirth(&mut self) {
        unsafe {
            main_func__action_rebirth(self)
        }
    }

    pub fn main_func__action_return(&mut self) {
        unsafe {
            main_func__action_return(self)
        }
    }

    pub fn main_func__action_roll_b(&mut self) {
        unsafe {
            main_func__action_roll_b(self)
        }
    }

    pub fn main_func__action_roll_f(&mut self) {
        unsafe {
            main_func__action_roll_f(self)
        }
    }

    pub fn main_func__action_run(&mut self) {
        unsafe {
            main_func__action_run(self)
        }
    }

    pub fn main_func__action_run_attack(&mut self) {
        unsafe {
            main_func__action_run_attack(self)
        }
    }

    pub fn main_func__action_stroll_l(&mut self) {
        unsafe {
            main_func__action_stroll_l(self)
        }
    }

    pub fn main_func__action_stroll_s(&mut self) {
        unsafe {
            main_func__action_stroll_s(self)
        }
    }

    pub fn main_func__action_stroll_squat(&mut self) {
        unsafe {
            main_func__action_stroll_squat(self)
        }
    }

    pub fn main_func__action_turn_turn(&mut self) {
        unsafe {
            main_func__action_turn_turn(self)
        }
    }

    pub fn main_func__action_wait_l(&mut self) {
        unsafe {
            main_func__action_wait_l(self)
        }
    }

    pub fn main_func__action_wait_s(&mut self) {
        unsafe {
            main_func__action_wait_s(self)
        }
    }

    pub fn main_func__action_wait_squat(&mut self) {
        unsafe {
            main_func__action_wait_squat(self)
        }
    }

    pub fn main_func__action_walk(&mut self) {
        unsafe {
            main_func__action_walk(self)
        }
    }

    pub fn call_event_on_dead(&mut self) -> lib::L2CValue {
        unsafe {
            call_event_on_dead(self).into()
        }
    }

    pub fn change_mode_action(&mut self, arg1: lib::L2CValue) {
        unsafe {
            change_mode_action(self, arg1.into())
        }
    }

    pub fn call_event_on_attack_shield(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            call_event_on_attack_shield(self, arg1.into()).into()
        }
    }

    pub fn notify_on_attack_shield(&mut self, arg1: lib::L2CValue) {
        unsafe {
            notify_on_attack_shield(self, arg1.into())
        }
    }

    pub fn global__tactics___notify_on_attack_shield(&mut self) -> lib::L2CValue {
        unsafe {
            global__tactics___notify_on_attack_shield(self).into()
        }
    }

    pub fn call_event_on_attack_hit(&mut self) -> lib::L2CValue {
        unsafe {
            call_event_on_attack_hit(self).into()
        }
    }

    pub fn notify_on_attack_hit(&mut self) {
        unsafe {
            notify_on_attack_hit(self)
        }
    }

    pub fn global__tactics___notify_on_attack_hit(&mut self) -> lib::L2CValue {
        unsafe {
            global__tactics___notify_on_attack_hit(self).into()
        }
    }

    pub fn is_normal_cp_type(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_normal_cp_type(self, arg1.into()).into()
        }
    }

    pub fn call_event_on_shield(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            call_event_on_shield(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn local_func__mode_line_4(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__mode_line_4(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn notify_on_shield(&mut self, arg1: lib::L2CValue) {
        unsafe {
            notify_on_shield(self, arg1.into())
        }
    }

    pub fn global__learning___notify_on_shield(&mut self) -> lib::L2CValue {
        unsafe {
            global__learning___notify_on_shield(self).into()
        }
    }

    pub fn global__tactics___notify_on_capture(&mut self) -> lib::L2CValue {
        unsafe {
            global__tactics___notify_on_capture(self).into()
        }
    }

    pub fn global__learning___notify_on_capture(&mut self) -> lib::L2CValue {
        unsafe {
            global__learning___notify_on_capture(self).into()
        }
    }

    pub fn call_event_on_damage(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue, arg7: lib::L2CValue, arg8: lib::L2CValue, arg9: lib::L2CValue, arg10: lib::L2CValue, arg11: lib::L2CValue, arg12: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            call_event_on_damage(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into(), arg7.into(), arg8.into(), arg9.into(), arg10.into(), arg11.into(), arg12.into()).into()
        }
    }

    pub fn local_func__mode_line_3(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue, arg7: lib::L2CValue, arg8: lib::L2CValue, arg9: lib::L2CValue, arg10: lib::L2CValue, arg11: lib::L2CValue, arg12: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__mode_line_3(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into(), arg7.into(), arg8.into(), arg9.into(), arg10.into(), arg11.into(), arg12.into()).into()
        }
    }

    pub fn notify_on_damage(&mut self, arg1: lib::L2CValue) {
        unsafe {
            notify_on_damage(self, arg1.into())
        }
    }

    pub fn global__damage___notify_on_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            global__damage___notify_on_damage(self, arg1.into()).into()
        }
    }

    pub fn global__tactics___notify_on_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            global__tactics___notify_on_damage(self, arg1.into()).into()
        }
    }

    pub fn global__learning___notify_on_damage(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            global__learning___notify_on_damage(self, arg1.into()).into()
        }
    }

    pub fn local_func__mode_global_variables_17(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_17(self).into()
        }
    }

    pub fn local_func__mode_line_1(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__mode_line_1(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn local_func__mode_line_2(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__mode_line_2(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn set_action_probability_mul_as_forward(&mut self) {
        unsafe {
            set_action_probability_mul_as_forward(self)
        }
    }

    pub fn set_action_probability_mul_as_charger(&mut self) {
        unsafe {
            set_action_probability_mul_as_charger(self)
        }
    }

    pub fn set_action_probability_mul_as_walker(&mut self) {
        unsafe {
            set_action_probability_mul_as_walker(self)
        }
    }

    pub fn set_action_probability_mul_3rd_foreach(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            set_action_probability_mul_3rd_foreach(self, arg1.into(), arg2.into())
        }
    }

    pub fn is_run_mode_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_run_mode_action(self, arg1.into()).into()
        }
    }

    pub fn set_action_probability_mul_as_dasher(&mut self) {
        unsafe {
            set_action_probability_mul_as_dasher(self)
        }
    }

    pub fn set_action_probability_mul_as_jumper(&mut self) {
        unsafe {
            set_action_probability_mul_as_jumper(self)
        }
    }

    pub fn set_action_probability_mul_as_grounder(&mut self) {
        unsafe {
            set_action_probability_mul_as_grounder(self)
        }
    }

    pub fn is_jump_mode_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_jump_mode_action(self, arg1.into()).into()
        }
    }

    pub fn is_small_jump_mode_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_small_jump_mode_action(self, arg1.into()).into()
        }
    }

    pub fn call_function_is_cancelable_mode_action_by_attack(&mut self) -> lib::L2CValue {
        unsafe {
            call_function_is_cancelable_mode_action_by_attack(self).into()
        }
    }

    pub fn call_function_update_coroutine(&mut self) {
        unsafe {
            call_function_update_coroutine(self)
        }
    }

    pub fn update_action(&mut self) {
        unsafe {
            update_action(self)
        }
    }

    pub fn global__difficulty___is_action_wait(&mut self) -> lib::L2CValue {
        unsafe {
            global__difficulty___is_action_wait(self).into()
        }
    }

    pub fn select_action(&mut self) {
        unsafe {
            select_action(self)
        }
    }

    pub fn deadzone_distance(&mut self) -> lib::L2CValue {
        unsafe {
            deadzone_distance(self).into()
        }
    }

    pub fn check_line_segment_is_danger_with_info(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            check_line_segment_is_danger_with_info(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn check_line_segment_is_danger(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            check_line_segment_is_danger(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn local_func__mode_action_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_action_1(self).into()
        }
    }

    pub fn local_func__mode_predict_utility_13(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_predict_utility_13(self).into()
        }
    }

    pub fn analyst_is_attacking(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            analyst_is_attacking(self, arg1.into()).into()
        }
    }

    pub fn analyst_is_attacking_only_ground(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            analyst_is_attacking_only_ground(self, arg1.into()).into()
        }
    }

    pub fn local_func__mode_predict_utility_15(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_predict_utility_15(self).into()
        }
    }

    pub fn local_func__mode_predict_utility_14(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_predict_utility_14(self).into()
        }
    }

    pub fn local_func__mode_predict_utility_12(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_predict_utility_12(self).into()
        }
    }

    pub fn set_action_probability_mul_1st_foreach(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            set_action_probability_mul_1st_foreach(self, arg1.into(), arg2.into())
        }
    }

    pub fn specializer_can_buildup(&mut self) -> lib::L2CValue {
        unsafe {
            specializer_can_buildup(self).into()
        }
    }

    pub fn local_func__mode_action_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_action_2(self).into()
        }
    }

    pub fn escape_distance(&mut self) -> lib::L2CValue {
        unsafe {
            escape_distance(self).into()
        }
    }

    pub fn select_mode_action_ground(&mut self) {
        unsafe {
            select_mode_action_ground(self)
        }
    }

    pub fn reset_action_probability_mul_1st(&mut self) {
        unsafe {
            reset_action_probability_mul_1st(self)
        }
    }

    pub fn reset_action_probability_mul_2nd(&mut self) {
        unsafe {
            reset_action_probability_mul_2nd(self)
        }
    }

    pub fn select_mode_action_air(&mut self) {
        unsafe {
            select_mode_action_air(self)
        }
    }

    pub fn dive(&mut self) {
        unsafe {
            dive(self)
        }
    }

    pub fn select_action_from_list(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            select_action_from_list(self, arg1.into()).into()
        }
    }

    pub fn is_dash_forward_mode_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_dash_forward_mode_action(self, arg1.into()).into()
        }
    }

    pub fn is_roll_mode_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_roll_mode_action(self, arg1.into()).into()
        }
    }

    pub fn is_feint_mode_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_feint_mode_action(self, arg1.into()).into()
        }
    }

    pub fn update_global_variables(&mut self) {
        unsafe {
            update_global_variables(self)
        }
    }

    pub fn update_interrupt(&mut self) {
        unsafe {
            update_interrupt(self)
        }
    }

    pub fn global__difficulty___is_attack_wait(&mut self) -> lib::L2CValue {
        unsafe {
            global__difficulty___is_attack_wait(self).into()
        }
    }

    pub fn select_weapon_guard_action(&mut self) -> lib::L2CValue {
        unsafe {
            select_weapon_guard_action(self).into()
        }
    }

    pub fn specializer_interrupt_action_on_target_rebirth(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            specializer_interrupt_action_on_target_rebirth(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn change_mode_action_no_restart_by_interrupt(&mut self, arg1: lib::L2CValue) {
        unsafe {
            change_mode_action_no_restart_by_interrupt(self, arg1.into())
        }
    }

    pub fn local_func__mode_action_interrupt_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_action_interrupt_1(self).into()
        }
    }

    pub fn restrict_always_command(&mut self) {
        unsafe {
            restrict_always_command(self)
        }
    }

    pub fn select_aerial_jump_action_air(&mut self) -> lib::L2CValue {
        unsafe {
            select_aerial_jump_action_air(self).into()
        }
    }

    pub fn select_interrupt_action_ground_c(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            select_interrupt_action_ground_c(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn analyst_is_dash(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            analyst_is_dash(self, arg1.into()).into()
        }
    }

    pub fn select_interrupt_action_ground_dash(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            select_interrupt_action_ground_dash(self, arg1.into(), arg2.into())
        }
    }

    pub fn select_interrupt_action_ground(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            select_interrupt_action_ground(self, arg1.into(), arg2.into())
        }
    }

    pub fn select_interrupt_action_air(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            select_interrupt_action_air(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn select_dive_action_air(&mut self) -> lib::L2CValue {
        unsafe {
            select_dive_action_air(self).into()
        }
    }

    pub fn local_func__mode_action_interrupt_8(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_action_interrupt_8(self).into()
        }
    }

    pub fn select_interrupt_action_air_internal(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) {
        unsafe {
            select_interrupt_action_air_internal(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into())
        }
    }

    pub fn predict_able_to_return_from_here(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            predict_able_to_return_from_here(self, arg1.into()).into()
        }
    }

    pub fn restrict_buildmax_command(&mut self) {
        unsafe {
            restrict_buildmax_command(self)
        }
    }

    pub fn restrict_air_command_by_no_ground(&mut self) {
        unsafe {
            restrict_air_command_by_no_ground(self)
        }
    }

    pub fn select_attack_action_air(&mut self, arg1: lib::L2CValue) {
        unsafe {
            select_attack_action_air(self, arg1.into())
        }
    }

    pub fn predict_guard_opponent_attack_probability_air(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            predict_guard_opponent_attack_probability_air(self, arg1.into()).into()
        }
    }

    pub fn select_guard_action_air(&mut self) {
        unsafe {
            select_guard_action_air(self)
        }
    }

    pub fn local_func__mode_action_interrupt_9(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_action_interrupt_9(self).into()
        }
    }

    pub fn global__learning___update_guard_rate_on_nothing_happend(&mut self) -> lib::L2CValue {
        unsafe {
            global__learning___update_guard_rate_on_nothing_happend(self).into()
        }
    }

    pub fn global__learning___start_observe_guard_status(&mut self) -> lib::L2CValue {
        unsafe {
            global__learning___start_observe_guard_status(self).into()
        }
    }

    pub fn predict_guard_opponent_attack_probability_ground(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            predict_guard_opponent_attack_probability_ground(self, arg1.into()).into()
        }
    }

    pub fn local_func__mode_action_interrupt_10(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_action_interrupt_10(self).into()
        }
    }

    pub fn floor_pos_with_adjust(&mut self) -> lib::L2CValue {
        unsafe {
            floor_pos_with_adjust(self).into()
        }
    }

    pub fn specializer_aerial_jump_high(&mut self) -> lib::L2CValue {
        unsafe {
            specializer_aerial_jump_high(self).into()
        }
    }

    pub fn local_func__mode_predict_utility_11(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_predict_utility_11(self).into()
        }
    }

    pub fn local_func__mode_action_interrupt_4(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_action_interrupt_4(self).into()
        }
    }

    pub fn select_interrupt_action_ground_common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) {
        unsafe {
            select_interrupt_action_ground_common(self, arg1.into(), arg2.into(), arg3.into(), arg4.into())
        }
    }

    pub fn analyst_is_free_to_attack(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            analyst_is_free_to_attack(self, arg1.into()).into()
        }
    }

    pub fn select_attack_action_ground(&mut self, arg1: lib::L2CValue) {
        unsafe {
            select_attack_action_ground(self, arg1.into())
        }
    }

    pub fn change_ground_guard_action(&mut self) {
        unsafe {
            change_ground_guard_action(self)
        }
    }

    pub fn analyst_is_or_will_be_attacking(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            analyst_is_or_will_be_attacking(self, arg1.into()).into()
        }
    }

    pub fn analyst_is_in_rest_to_combo(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            analyst_is_in_rest_to_combo(self, arg1.into()).into()
        }
    }

    pub fn global__difficulty___start_action(&mut self) -> lib::L2CValue {
        unsafe {
            global__difficulty___start_action(self).into()
        }
    }

    pub fn local_func__mode_action_interrupt_11(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_action_interrupt_11(self).into()
        }
    }

    pub fn local_func__mode_action_interrupt_12(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_action_interrupt_12(self).into()
        }
    }

    pub fn deadzone_nearest_distance(&mut self) -> lib::L2CValue {
        unsafe {
            deadzone_nearest_distance(self).into()
        }
    }

    pub fn select_weapon_guard_action_ground(&mut self) -> lib::L2CValue {
        unsafe {
            select_weapon_guard_action_ground(self).into()
        }
    }

    pub fn select_weapon_guard_action_air(&mut self) -> lib::L2CValue {
        unsafe {
            select_weapon_guard_action_air(self).into()
        }
    }

    pub fn local_func__mode_action_interrupt_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_action_interrupt_2(self).into()
        }
    }

    pub fn analyst_is_kind_of_attack(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            analyst_is_kind_of_attack(self, arg1.into()).into()
        }
    }

    pub fn update_global_difficulty(&mut self) {
        unsafe {
            update_global_difficulty(self)
        }
    }

    pub fn update_global_damage(&mut self) {
        unsafe {
            update_global_damage(self)
        }
    }

    pub fn update_global_situation(&mut self) {
        unsafe {
            update_global_situation(self)
        }
    }

    pub fn update_global_tactics(&mut self) {
        unsafe {
            update_global_tactics(self)
        }
    }

    pub fn update_global_learning(&mut self) {
        unsafe {
            update_global_learning(self)
        }
    }

    pub fn update_global_predict_self(&mut self) {
        unsafe {
            update_global_predict_self(self)
        }
    }

    pub fn update_global_predict_target(&mut self) {
        unsafe {
            update_global_predict_target(self)
        }
    }

    pub fn update_global_predict_weapon(&mut self) {
        unsafe {
            update_global_predict_weapon(self)
        }
    }

    pub fn update_global_koopag(&mut self) {
        unsafe {
            update_global_koopag(self)
        }
    }

    pub fn predict_target_any_attack_hit(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            predict_target_any_attack_hit(self, arg1.into()).into()
        }
    }

    pub fn predict_target_any_catch_hit(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            predict_target_any_catch_hit(self, arg1.into()).into()
        }
    }

    pub fn local_func__mode_predict_utility_6(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__mode_predict_utility_6(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn predict_target_catch_hit_on_list(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            predict_target_catch_hit_on_list(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn local_func__mode_predict_utility_7(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__mode_predict_utility_7(self, arg1.into()).into()
        }
    }

    pub fn local_func__mode_predict_utility_8(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__mode_predict_utility_8(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn local_func__mode_predict_utility_9(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__mode_predict_utility_9(self, arg1.into()).into()
        }
    }

    pub fn local_func__mode_predict_utility_10(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_predict_utility_10(self).into()
        }
    }

    pub fn predict_target_attack_hit(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            predict_target_attack_hit(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn local_func__mode_predict_utility_1(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__mode_predict_utility_1(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn predict_target_attack_hit_on_list(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            predict_target_attack_hit_on_list(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn local_func__mode_predict_utility_2(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__mode_predict_utility_2(self, arg1.into()).into()
        }
    }

    pub fn local_func__mode_predict_utility_3(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__mode_predict_utility_3(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn local_func__mode_predict_utility_4(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__mode_predict_utility_4(self, arg1.into()).into()
        }
    }

    pub fn local_func__mode_predict_utility_5(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__mode_predict_utility_5(self, arg1.into()).into()
        }
    }

    pub fn local_func__mode_global_variables_18(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_18(self).into()
        }
    }

    pub fn call_function_change_mode(&mut self) -> lib::L2CValue {
        unsafe {
            call_function_change_mode(self).into()
        }
    }

    pub fn call_function_reset(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            call_function_reset(self, arg1.into()).into()
        }
    }

    pub fn init_global_variables(&mut self) {
        unsafe {
            init_global_variables(self)
        }
    }

    pub fn init_private_variables(&mut self) {
        unsafe {
            init_private_variables(self)
        }
    }

    pub fn local_func__mode_private_variables_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_3(self).into()
        }
    }

    pub fn local_func__mode_private_variables_4(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_4(self).into()
        }
    }

    pub fn local_func__mode_private_variables_5(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_5(self).into()
        }
    }

    pub fn local_func__mode_private_variables_6(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_6(self).into()
        }
    }

    pub fn local_func__mode_private_variables_7(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_7(self).into()
        }
    }

    pub fn local_func__mode_private_variables_8(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_8(self).into()
        }
    }

    pub fn local_func__mode_private_variables_9(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_9(self).into()
        }
    }

    pub fn local_func__mode_private_variables_10(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_10(self).into()
        }
    }

    pub fn local_func__mode_private_variables_11(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_11(self).into()
        }
    }

    pub fn local_func__mode_private_variables_12(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_12(self).into()
        }
    }

    pub fn local_func__mode_private_variables_13(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_13(self).into()
        }
    }

    pub fn local_func__mode_private_variables_14(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_14(self).into()
        }
    }

    pub fn local_func__mode_private_variables_15(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_15(self).into()
        }
    }

    pub fn local_func__mode_private_variables_16(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_16(self).into()
        }
    }

    pub fn local_func__mode_private_variables_17(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_17(self).into()
        }
    }

    pub fn local_func__mode_private_variables_18(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_18(self).into()
        }
    }

    pub fn local_func__mode_private_variables_19(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_19(self).into()
        }
    }

    pub fn local_func__mode_private_variables_20(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_20(self).into()
        }
    }

    pub fn local_func__mode_private_variables_21(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_21(self).into()
        }
    }

    pub fn local_func__mode_private_variables_22(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_22(self).into()
        }
    }

    pub fn local_func__mode_private_variables_23(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_23(self).into()
        }
    }

    pub fn local_func__mode_private_variables_24(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_24(self).into()
        }
    }

    pub fn local_func__mode_private_variables_25(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_25(self).into()
        }
    }

    pub fn local_func__mode_private_variables_26(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_26(self).into()
        }
    }

    pub fn local_func__mode_private_variables_27(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_27(self).into()
        }
    }

    pub fn local_func__mode_private_variables_28(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_28(self).into()
        }
    }

    pub fn local_func__mode_private_variables_29(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_29(self).into()
        }
    }

    pub fn local_func__mode_private_variables_30(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_30(self).into()
        }
    }

    pub fn local_func__mode_private_variables_31(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_31(self).into()
        }
    }

    pub fn local_func__mode_private_variables_32(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_32(self).into()
        }
    }

    pub fn local_func__mode_private_variables_33(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_33(self).into()
        }
    }

    pub fn local_func__mode_private_variables_34(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_34(self).into()
        }
    }

    pub fn local_func__mode_private_variables_35(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_35(self).into()
        }
    }

    pub fn local_func__mode_private_variables_36(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_36(self).into()
        }
    }

    pub fn local_func__mode_private_variables_37(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_37(self).into()
        }
    }

    pub fn local_func__mode_private_variables_38(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_38(self).into()
        }
    }

    pub fn local_func__mode_private_variables_39(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_39(self).into()
        }
    }

    pub fn local_func__mode_private_variables_40(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_40(self).into()
        }
    }

    pub fn local_func__mode_private_variables_41(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_41(self).into()
        }
    }

    pub fn local_func__mode_private_variables_42(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_42(self).into()
        }
    }

    pub fn local_func__mode_private_variables_43(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_43(self).into()
        }
    }

    pub fn local_func__mode_private_variables_44(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_44(self).into()
        }
    }

    pub fn local_func__mode_private_variables_45(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_45(self).into()
        }
    }

    pub fn local_func__mode_private_variables_46(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_46(self).into()
        }
    }

    pub fn local_func__mode_private_variables_47(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_47(self).into()
        }
    }

    pub fn local_func__mode_private_variables_48(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_48(self).into()
        }
    }

    pub fn local_func__mode_private_variables_49(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_49(self).into()
        }
    }

    pub fn local_func__mode_private_variables_50(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_50(self).into()
        }
    }

    pub fn local_func__mode_private_variables_51(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_51(self).into()
        }
    }

    pub fn local_func__mode_private_variables_52(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_52(self).into()
        }
    }

    pub fn local_func__mode_private_variables_53(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_53(self).into()
        }
    }

    pub fn local_func__mode_private_variables_54(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_54(self).into()
        }
    }

    pub fn local_func__mode_private_variables_55(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_55(self).into()
        }
    }

    pub fn local_func__mode_private_variables_56(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_56(self).into()
        }
    }

    pub fn local_func__mode_private_variables_57(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_57(self).into()
        }
    }

    pub fn local_func__mode_private_variables_58(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_58(self).into()
        }
    }

    pub fn local_func__mode_private_variables_59(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_59(self).into()
        }
    }

    pub fn local_func__mode_private_variables_60(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_60(self).into()
        }
    }

    pub fn local_func__mode_private_variables_61(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_61(self).into()
        }
    }

    pub fn local_func__mode_private_variables_62(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_62(self).into()
        }
    }

    pub fn local_func__mode_private_variables_63(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_63(self).into()
        }
    }

    pub fn local_func__mode_private_variables_64(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_64(self).into()
        }
    }

    pub fn local_func__mode_private_variables_65(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_65(self).into()
        }
    }

    pub fn local_func__mode_private_variables_66(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_66(self).into()
        }
    }

    pub fn local_func__mode_private_variables_67(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_67(self).into()
        }
    }

    pub fn local_func__mode_private_variables_68(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_68(self).into()
        }
    }

    pub fn local_func__mode_private_variables_69(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_69(self).into()
        }
    }

    pub fn local_func__mode_private_variables_70(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_70(self).into()
        }
    }

    pub fn local_func__mode_private_variables_71(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_71(self).into()
        }
    }

    pub fn local_func__mode_private_variables_72(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_72(self).into()
        }
    }

    pub fn local_func__mode_private_variables_73(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_73(self).into()
        }
    }

    pub fn local_func__mode_private_variables_74(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_74(self).into()
        }
    }

    pub fn local_func__mode_private_variables_75(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_75(self).into()
        }
    }

    pub fn local_func__mode_private_variables_76(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_76(self).into()
        }
    }

    pub fn local_func__mode_private_variables_77(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_77(self).into()
        }
    }

    pub fn local_func__mode_private_variables_78(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_78(self).into()
        }
    }

    pub fn local_func__mode_private_variables_79(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_79(self).into()
        }
    }

    pub fn local_func__mode_private_variables_80(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_80(self).into()
        }
    }

    pub fn local_func__mode_private_variables_81(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_81(self).into()
        }
    }

    pub fn local_func__mode_private_variables_82(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_82(self).into()
        }
    }

    pub fn local_func__mode_private_variables_83(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_83(self).into()
        }
    }

    pub fn local_func__mode_private_variables_84(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_84(self).into()
        }
    }

    pub fn local_func__mode_private_variables_85(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_85(self).into()
        }
    }

    pub fn local_func__mode_private_variables_86(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_86(self).into()
        }
    }

    pub fn local_func__mode_private_variables_87(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_87(self).into()
        }
    }

    pub fn local_func__mode_private_variables_88(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_88(self).into()
        }
    }

    pub fn local_func__mode_private_variables_89(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_89(self).into()
        }
    }

    pub fn local_func__mode_private_variables_90(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_90(self).into()
        }
    }

    pub fn local_func__mode_private_variables_91(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_91(self).into()
        }
    }

    pub fn local_func__mode_private_variables_92(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_92(self).into()
        }
    }

    pub fn local_func__mode_private_variables_93(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_93(self).into()
        }
    }

    pub fn local_func__mode_private_variables_94(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_94(self).into()
        }
    }

    pub fn local_func__mode_private_variables_95(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_95(self).into()
        }
    }

    pub fn local_func__mode_private_variables_96(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_96(self).into()
        }
    }

    pub fn local_func__mode_private_variables_97(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_97(self).into()
        }
    }

    pub fn local_func__mode_private_variables_98(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_98(self).into()
        }
    }

    pub fn local_func__mode_private_variables_99(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_99(self).into()
        }
    }

    pub fn local_func__mode_private_variables_100(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_100(self).into()
        }
    }

    pub fn local_func__mode_private_variables_101(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_101(self).into()
        }
    }

    pub fn local_func__mode_private_variables_102(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_102(self).into()
        }
    }

    pub fn local_func__mode_private_variables_103(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_103(self).into()
        }
    }

    pub fn local_func__mode_private_variables_104(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_104(self).into()
        }
    }

    pub fn local_func__mode_private_variables_105(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_105(self).into()
        }
    }

    pub fn local_func__mode_private_variables_106(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_106(self).into()
        }
    }

    pub fn local_func__mode_private_variables_107(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_107(self).into()
        }
    }

    pub fn local_func__mode_private_variables_108(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_108(self).into()
        }
    }

    pub fn local_func__mode_private_variables_109(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_109(self).into()
        }
    }

    pub fn local_func__mode_private_variables_110(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_110(self).into()
        }
    }

    pub fn local_func__mode_private_variables_111(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_111(self).into()
        }
    }

    pub fn local_func__mode_private_variables_112(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_112(self).into()
        }
    }

    pub fn local_func__mode_private_variables_113(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_113(self).into()
        }
    }

    pub fn local_func__mode_private_variables_114(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_114(self).into()
        }
    }

    pub fn local_func__mode_private_variables_115(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_115(self).into()
        }
    }

    pub fn local_func__mode_private_variables_116(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_116(self).into()
        }
    }

    pub fn local_func__mode_private_variables_117(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_117(self).into()
        }
    }

    pub fn local_func__mode_private_variables_118(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_118(self).into()
        }
    }

    pub fn local_func__mode_private_variables_119(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_119(self).into()
        }
    }

    pub fn local_func__mode_private_variables_120(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_120(self).into()
        }
    }

    pub fn local_func__mode_private_variables_121(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_121(self).into()
        }
    }

    pub fn local_func__mode_private_variables_122(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_122(self).into()
        }
    }

    pub fn local_func__mode_private_variables_123(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_123(self).into()
        }
    }

    pub fn local_func__mode_private_variables_124(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_124(self).into()
        }
    }

    pub fn local_func__mode_private_variables_125(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_125(self).into()
        }
    }

    pub fn local_func__mode_private_variables_126(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_126(self).into()
        }
    }

    pub fn local_func__mode_private_variables_127(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_127(self).into()
        }
    }

    pub fn local_func__mode_private_variables_128(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_128(self).into()
        }
    }

    pub fn local_func__mode_private_variables_129(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_129(self).into()
        }
    }

    pub fn local_func__mode_private_variables_130(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_130(self).into()
        }
    }

    pub fn local_func__mode_private_variables_131(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_131(self).into()
        }
    }

    pub fn local_func__mode_private_variables_132(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_132(self).into()
        }
    }

    pub fn local_func__mode_private_variables_133(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_133(self).into()
        }
    }

    pub fn local_func__mode_private_variables_134(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_134(self).into()
        }
    }

    pub fn local_func__mode_private_variables_135(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_135(self).into()
        }
    }

    pub fn local_func__mode_private_variables_136(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_136(self).into()
        }
    }

    pub fn local_func__mode_private_variables_137(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_137(self).into()
        }
    }

    pub fn local_func__mode_private_variables_138(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_138(self).into()
        }
    }

    pub fn local_func__mode_private_variables_139(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_139(self).into()
        }
    }

    pub fn local_func__mode_private_variables_140(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_140(self).into()
        }
    }

    pub fn local_func__mode_private_variables_141(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_141(self).into()
        }
    }

    pub fn local_func__mode_private_variables_142(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_142(self).into()
        }
    }

    pub fn local_func__mode_private_variables_143(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_143(self).into()
        }
    }

    pub fn local_func__mode_private_variables_144(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_144(self).into()
        }
    }

    pub fn local_func__mode_private_variables_145(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_145(self).into()
        }
    }

    pub fn local_func__mode_private_variables_146(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_146(self).into()
        }
    }

    pub fn local_func__mode_private_variables_147(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_147(self).into()
        }
    }

    pub fn local_func__mode_private_variables_148(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_148(self).into()
        }
    }

    pub fn local_func__mode_private_variables_149(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_149(self).into()
        }
    }

    pub fn local_func__mode_private_variables_150(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_150(self).into()
        }
    }

    pub fn local_func__mode_private_variables_151(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_151(self).into()
        }
    }

    pub fn local_func__mode_private_variables_152(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_152(self).into()
        }
    }

    pub fn local_func__mode_private_variables_153(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_153(self).into()
        }
    }

    pub fn local_func__mode_private_variables_154(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_154(self).into()
        }
    }

    pub fn is_chase_mode_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_chase_mode_action(self, arg1.into()).into()
        }
    }

    pub fn ENTRY(&mut self) -> lib::L2CValue {
        unsafe {
            ENTRY(self).into()
        }
    }

    pub fn init_lines(&mut self) {
        unsafe {
            init_lines(self)
        }
    }

    pub fn local_func__action_walk_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_walk_1(self).into()
        }
    }

    pub fn ACTION_WALK__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_WALK__update(self).into()
        }
    }

    pub fn ACTION_WALK__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_WALK__init(self).into()
        }
    }

    pub fn local_func__action_wait_squat_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_wait_squat_1(self).into()
        }
    }

    pub fn ACTION_WAIT_SQUAT__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_WAIT_SQUAT__update(self).into()
        }
    }

    pub fn ACTION_WAIT_SQUAT__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_WAIT_SQUAT__init(self).into()
        }
    }

    pub fn local_func__action_wait_s_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_wait_s_1(self).into()
        }
    }

    pub fn ACTION_WAIT_S__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_WAIT_S__update(self).into()
        }
    }

    pub fn ACTION_WAIT_S__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_WAIT_S__init(self).into()
        }
    }

    pub fn local_func__action_wait_l_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_wait_l_1(self).into()
        }
    }

    pub fn ACTION_WAIT_L__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_WAIT_L__update(self).into()
        }
    }

    pub fn ACTION_WAIT_L__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_WAIT_L__init(self).into()
        }
    }

    pub fn local_func__action_turn_turn_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_turn_turn_1(self).into()
        }
    }

    pub fn ACTION_TURN_TURN__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_TURN_TURN__update(self).into()
        }
    }

    pub fn local_func__action_stroll_squat_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_stroll_squat_1(self).into()
        }
    }

    pub fn local_func__action_stroll_s_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_stroll_s_1(self).into()
        }
    }

    pub fn local_func__action_stroll_l_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_stroll_l_1(self).into()
        }
    }

    pub fn local_func__action_run_attack_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_run_attack_1(self).into()
        }
    }

    pub fn ACTION_RUN_ATTACK__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RUN_ATTACK__update(self).into()
        }
    }

    pub fn ACTION_RUN_ATTACK__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RUN_ATTACK__init(self).into()
        }
    }

    pub fn local_func__action_run_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_run_1(self).into()
        }
    }

    pub fn ACTION_RUN__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RUN__update(self).into()
        }
    }

    pub fn ACTION_RUN__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RUN__init(self).into()
        }
    }

    pub fn local_func__action_roll_f_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_roll_f_1(self).into()
        }
    }

    pub fn ACTION_ROLL_F__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ROLL_F__update(self).into()
        }
    }

    pub fn ACTION_ROLL_F__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ROLL_F__init(self).into()
        }
    }

    pub fn local_func__action_roll_b_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_roll_b_1(self).into()
        }
    }

    pub fn ACTION_ROLL_B__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ROLL_B__update(self).into()
        }
    }

    pub fn ACTION_ROLL_B__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ROLL_B__init(self).into()
        }
    }

    pub fn local_func__action_return_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_return_1(self).into()
        }
    }

    pub fn ACTION_RETURN__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RETURN__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_RETURN__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RETURN__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_RETURN__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RETURN__update(self).into()
        }
    }

    pub fn local_func__action_return_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_return_2(self).into()
        }
    }

    pub fn specializer_interrupt_action_on_return(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            specializer_interrupt_action_on_return(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn local_func__action_return_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_return_3(self).into()
        }
    }

    pub fn damage_motion_cancel_cmd_id(&mut self) -> lib::L2CValue {
        unsafe {
            damage_motion_cancel_cmd_id(self).into()
        }
    }

    pub fn ACTION_RETURN__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RETURN__init(self).into()
        }
    }

    pub fn local_func__action_rebirth_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_rebirth_1(self).into()
        }
    }

    pub fn ACTION_REBIRTH__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_REBIRTH__update(self).into()
        }
    }

    pub fn local_func__action_pursue_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_pursue_1(self).into()
        }
    }

    pub fn ACTION_PURSUE__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_PURSUE__update(self).into()
        }
    }

    pub fn ACTION_PURSUE__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_PURSUE__init(self).into()
        }
    }

    pub fn local_func__action_pass_floor_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_pass_floor_1(self).into()
        }
    }

    pub fn ACTION_PASS_FLOOR__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_PASS_FLOOR__update(self).into()
        }
    }

    pub fn local_func__action_pass_floor_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_pass_floor_2(self).into()
        }
    }

    pub fn local_func__action_meteor_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_meteor_2(self).into()
        }
    }

    pub fn ACTION_METEOR__interrupt_aerial_jump(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_METEOR__interrupt_aerial_jump(self).into()
        }
    }

    pub fn ACTION_METEOR__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_METEOR__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_METEOR__set_no_return_frame(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_METEOR__set_no_return_frame(self).into()
        }
    }

    pub fn ACTION_METEOR__lot_action(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_METEOR__lot_action(self).into()
        }
    }

    pub fn local_func__action_meteor_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_meteor_3(self).into()
        }
    }

    pub fn ACTION_METEOR__is_target_on_return(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_METEOR__is_target_on_return(self).into()
        }
    }

    pub fn ACTION_METEOR__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_METEOR__update(self).into()
        }
    }

    pub fn ACTION_METEOR__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_METEOR__init(self).into()
        }
    }

    pub fn local_func__action_meteor_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_meteor_1(self).into()
        }
    }

    pub fn local_func__action_machstamp_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_machstamp_1(self).into()
        }
    }

    pub fn ACTION_MACHSTAMP__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_MACHSTAMP__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_MACHSTAMP__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_MACHSTAMP__update(self).into()
        }
    }

    pub fn ACTION_MACHSTAMP__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_MACHSTAMP__init(self).into()
        }
    }

    pub fn local_func__action_jump_s_n_attack_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_jump_s_n_attack_1(self).into()
        }
    }

    pub fn ACTION_JUMP_S_N_ATTACK__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_N_ATTACK__update(self).into()
        }
    }

    pub fn ACTION_JUMP_S_N_ATTACK__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_N_ATTACK__init(self).into()
        }
    }

    pub fn is_cmd_jump_attack(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_cmd_jump_attack(self, arg1.into()).into()
        }
    }

    pub fn local_func__action_jump_s_b_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_jump_s_b_1(self).into()
        }
    }

    pub fn ACTION_JUMP_S_B__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_B__update(self).into()
        }
    }

    pub fn local_func__action_jump_s_attack_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_jump_s_attack_1(self).into()
        }
    }

    pub fn ACTION_JUMP_S_ATTACK__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_ATTACK__update(self).into()
        }
    }

    pub fn ACTION_JUMP_S_ATTACK__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_ATTACK__init(self).into()
        }
    }

    pub fn local_func__action_jump_attack_f_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_jump_attack_f_1(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK_F__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK_F__update(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK_F__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK_F__init(self).into()
        }
    }

    pub fn local_func__action_jump_attack_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_jump_attack_1(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK__update(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK__init(self).into()
        }
    }

    pub fn local_func__action_high_speed_dash_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_high_speed_dash_1(self).into()
        }
    }

    pub fn ACTION_HIGH_SPEED_DASH__interrupt_combo(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_HIGH_SPEED_DASH__interrupt_combo(self).into()
        }
    }

    pub fn ACTION_HIGH_SPEED_DASH__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_HIGH_SPEED_DASH__update(self).into()
        }
    }

    pub fn local_func__action_guard_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_guard_1(self).into()
        }
    }

    pub fn ACTION_GUARD__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_GUARD__update(self).into()
        }
    }

    pub fn analyst_is_not_need_shield(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            analyst_is_not_need_shield(self, arg1.into()).into()
        }
    }

    pub fn predict_attack_hit(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            predict_attack_hit(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn ACTION_GUARD__update_on_stop(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_GUARD__update_on_stop(self).into()
        }
    }

    pub fn ACTION_GUARD__notify_on_shield(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            ACTION_GUARD__notify_on_shield(self, arg1.into()).into()
        }
    }

    pub fn ACTION_GUARD__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_GUARD__init(self).into()
        }
    }

    pub fn ACTION_FINAL_END_JUMP__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_FINAL_END_JUMP__update(self).into()
        }
    }

    pub fn local_func__action_fall_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_fall_1(self).into()
        }
    }

    pub fn ACTION_FALL__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_FALL__update(self).into()
        }
    }

    pub fn local_func__action_fall_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_fall_2(self).into()
        }
    }

    pub fn ACTION_FALL__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_FALL__init(self).into()
        }
    }

    pub fn local_func__action_escape_near_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_escape_near_1(self).into()
        }
    }

    pub fn ACTION_ESCAPE_NEAR__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_NEAR__update(self).into()
        }
    }

    pub fn local_func__action_escape_far_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_escape_far_1(self).into()
        }
    }

    pub fn ACTION_ESCAPE_FAR__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_FAR__update(self).into()
        }
    }

    pub fn local_func__action_escape_air_move_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_escape_air_move_1(self).into()
        }
    }

    pub fn ACTION_ESCAPE_AIR_MOVE__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_AIR_MOVE__update(self).into()
        }
    }

    pub fn ACTION_ESCAPE_AIR_MOVE__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_AIR_MOVE__init(self).into()
        }
    }

    pub fn local_func__action_escape_air_move_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_escape_air_move_2(self).into()
        }
    }

    pub fn local_func__action_escape_air_move_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_escape_air_move_3(self).into()
        }
    }

    pub fn local_func__action_escape_air_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_escape_air_1(self).into()
        }
    }

    pub fn ACTION_ESCAPE_AIR__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_AIR__update(self).into()
        }
    }

    pub fn local_func__action_escape_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_escape_1(self).into()
        }
    }

    pub fn ACTION_ENTRY__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ENTRY__update(self).into()
        }
    }

    pub fn local_func__action_drill_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_drill_1(self).into()
        }
    }

    pub fn ACTION_DRILL__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DRILL__update(self).into()
        }
    }

    pub fn ACTION_DRILL__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DRILL__init(self).into()
        }
    }

    pub fn local_func__action_dragoon_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_dragoon_1(self).into()
        }
    }

    pub fn ACTION_DRAGOON__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DRAGOON__update(self).into()
        }
    }

    pub fn ACTION_DRAGOON__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DRAGOON__init(self).into()
        }
    }

    pub fn local_func__action_down_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_down_2(self).into()
        }
    }

    pub fn ACTION_DOWN__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DOWN__update(self).into()
        }
    }

    pub fn ACTION_DOWN__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DOWN__init(self).into()
        }
    }

    pub fn local_func__action_down_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_down_3(self).into()
        }
    }

    pub fn local_func__action_down_4(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_down_4(self).into()
        }
    }

    pub fn local_func__action_down_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_down_1(self).into()
        }
    }

    pub fn local_func__action_deadarea_dash_attack_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_deadarea_dash_attack_1(self).into()
        }
    }

    pub fn ACTION_DEADAREA_DASH_ATTACK__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DEADAREA_DASH_ATTACK__update(self).into()
        }
    }

    pub fn local_func__action_deadarea_air_attack_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_deadarea_air_attack_1(self).into()
        }
    }

    pub fn ACTION_DEADAREA_AIR_ATTACK__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DEADAREA_AIR_ATTACK__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_DEADAREA_AIR_ATTACK__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DEADAREA_AIR_ATTACK__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_DEADAREA_AIR_ATTACK__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DEADAREA_AIR_ATTACK__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_DEADAREA_AIR_ATTACK__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DEADAREA_AIR_ATTACK__update(self).into()
        }
    }

    pub fn ACTION_DEADAREA_AIR_ATTACK__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DEADAREA_AIR_ATTACK__init(self).into()
        }
    }

    pub fn local_func__action_dash_roll_f_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_dash_roll_f_1(self).into()
        }
    }

    pub fn ACTION_DASH_ROLL_F__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ROLL_F__update(self).into()
        }
    }

    pub fn local_func__action_dash_roll_b_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_dash_roll_b_1(self).into()
        }
    }

    pub fn ACTION_DASH_ROLL_B__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ROLL_B__update(self).into()
        }
    }

    pub fn local_func__action_dash_guard_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_dash_guard_1(self).into()
        }
    }

    pub fn ACTION_DASH_GUARD__next_action(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_GUARD__next_action(self).into()
        }
    }

    pub fn ACTION_DASH_GUARD__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_GUARD__update(self).into()
        }
    }

    pub fn local_func__action_dash_f_wait_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_dash_f_wait_1(self).into()
        }
    }

    pub fn ACTION_DASH_F_WAIT__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_WAIT__update(self).into()
        }
    }

    pub fn local_func__action_dash_f_jump_b_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_dash_f_jump_b_1(self).into()
        }
    }

    pub fn ACTION_DASH_F_JUMP_B__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_JUMP_B__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_DASH_F_JUMP_B__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_JUMP_B__update(self).into()
        }
    }

    pub fn local_func__action_dash_f_dash_b_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_dash_f_dash_b_1(self).into()
        }
    }

    pub fn global__tactics___unlock_waza(&mut self) -> lib::L2CValue {
        unsafe {
            global__tactics___unlock_waza(self).into()
        }
    }

    pub fn ACTION_DASH_F_DASH_B__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_DASH_B__update(self).into()
        }
    }

    pub fn ACTION_DASH_F_DASH_B__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_DASH_B__init(self).into()
        }
    }

    pub fn global__tactics___lock_waza_dash(&mut self) -> lib::L2CValue {
        unsafe {
            global__tactics___lock_waza_dash(self).into()
        }
    }

    pub fn local_func__mode_global_variables_19(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_19(self).into()
        }
    }

    pub fn local_func__action_dash_b_jump_s_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_dash_b_jump_s_1(self).into()
        }
    }

    pub fn ACTION_DASH_B_JUMP_S__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B_JUMP_S__update(self).into()
        }
    }

    pub fn local_func__action_dash_b_dash_f_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_dash_b_dash_f_1(self).into()
        }
    }

    pub fn ACTION_DASH_B_DASH_F__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B_DASH_F__update(self).into()
        }
    }

    pub fn ACTION_DASH_B_DASH_F__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B_DASH_F__init(self).into()
        }
    }

    pub fn local_func__action_dash_b_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_dash_b_1(self).into()
        }
    }

    pub fn ACTION_DASH_B__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B__update(self).into()
        }
    }

    pub fn ACTION_DASH_B__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B__init(self).into()
        }
    }

    pub fn local_func__action_dash_attack_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_dash_attack_1(self).into()
        }
    }

    pub fn local_func__action_damaged_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_damaged_1(self).into()
        }
    }

    pub fn ACTION_DAMAGED__function_input_ground(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DAMAGED__function_input_ground(self).into()
        }
    }

    pub fn ACTION_DAMAGED__function_input_air(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DAMAGED__function_input_air(self).into()
        }
    }

    pub fn ACTION_DAMAGED__hit_stop_slide(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DAMAGED__hit_stop_slide(self).into()
        }
    }

    pub fn ACTION_DAMAGED__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DAMAGED__update(self).into()
        }
    }

    pub fn ACTION_DAMAGED__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DAMAGED__init(self).into()
        }
    }

    pub fn local_func__action_cliff_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_cliff_2(self).into()
        }
    }

    pub fn ACTION_CLIFF__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_CLIFF__update(self).into()
        }
    }

    pub fn ACTION_CLIFF__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_CLIFF__init(self).into()
        }
    }

    pub fn local_func__action_cliff_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_cliff_3(self).into()
        }
    }

    pub fn local_func__action_cliff_4(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_cliff_4(self).into()
        }
    }

    pub fn local_func__action_cliff_5(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_cliff_5(self).into()
        }
    }

    pub fn local_func__action_cliff_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_cliff_1(self).into()
        }
    }

    pub fn local_func__action_buildup_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_buildup_1(self).into()
        }
    }

    pub fn local_func__action_avoid_koopa_special_s_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_avoid_koopa_special_s_1(self).into()
        }
    }

    pub fn ACTION_AVOID_KOOPA_SPECIAL_S__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AVOID_KOOPA_SPECIAL_S__update(self).into()
        }
    }

    pub fn ACTION_AVOID_KOOPA_SPECIAL_S__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AVOID_KOOPA_SPECIAL_S__init(self).into()
        }
    }

    pub fn local_func__action_avoid_ganon_special_air_s_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_avoid_ganon_special_air_s_1(self).into()
        }
    }

    pub fn ACTION_AVOID_GANON_SPECIAL_AIR_S__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AVOID_GANON_SPECIAL_AIR_S__update(self).into()
        }
    }

    pub fn ACTION_AVOID_GANON_SPECIAL_AIR_S__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AVOID_GANON_SPECIAL_AIR_S__init(self).into()
        }
    }

    pub fn local_func__action_avoid_donkey_special_lw_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_avoid_donkey_special_lw_1(self).into()
        }
    }

    pub fn ACTION_AVOID_DONKEY_SPECIAL_LW__functon_common_input(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AVOID_DONKEY_SPECIAL_LW__functon_common_input(self).into()
        }
    }

    pub fn ACTION_AVOID_DONKEY_SPECIAL_LW__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AVOID_DONKEY_SPECIAL_LW__update(self).into()
        }
    }

    pub fn ACTION_AVOID_DONKEY_SPECIAL_LW__notify_on_shield(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            ACTION_AVOID_DONKEY_SPECIAL_LW__notify_on_shield(self, arg1.into()).into()
        }
    }

    pub fn ACTION_AVOID_DONKEY_SPECIAL_LW__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AVOID_DONKEY_SPECIAL_LW__init(self).into()
        }
    }

    pub fn local_func__action_attack_123_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_attack_123_1(self).into()
        }
    }

    pub fn ACTION_ATTACK_123__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ATTACK_123__update(self).into()
        }
    }

    pub fn ACTION_ATTACK_123__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ATTACK_123__init(self).into()
        }
    }

    pub fn ACTION_APPEAL__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_APPEAL__update(self).into()
        }
    }

    pub fn local_func__action_appeal_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_appeal_1(self).into()
        }
    }

    pub fn local_func__action_air_attack_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_air_attack_1(self).into()
        }
    }

    pub fn ACTION_AIR_ATTACK__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AIR_ATTACK__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_AIR_ATTACK__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AIR_ATTACK__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_AIR_ATTACK__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AIR_ATTACK__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_AIR_ATTACK__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AIR_ATTACK__update(self).into()
        }
    }

    pub fn ACTION_AIR_ATTACK__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AIR_ATTACK__init(self).into()
        }
    }

    pub fn enum_(&mut self) -> lib::L2CValue {
        unsafe {
            enum_(self).into()
        }
    }

    pub fn local_func__mode_private_variables_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_2(self).into()
        }
    }

    pub fn local_func__mode_private_variables_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_private_variables_1(self).into()
        }
    }

    pub fn local_func__mode_global_variables_16(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_16(self).into()
        }
    }

    pub fn global__learning___set_guard_rate(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            global__learning___set_guard_rate(self, arg1.into()).into()
        }
    }

    pub fn global__learning___update_guard_rate_on_damaged(&mut self) -> lib::L2CValue {
        unsafe {
            global__learning___update_guard_rate_on_damaged(self).into()
        }
    }

    pub fn global__learning___end_observe_guard_status(&mut self) -> lib::L2CValue {
        unsafe {
            global__learning___end_observe_guard_status(self).into()
        }
    }

    pub fn local_func__mode_global_variables_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_3(self).into()
        }
    }

    pub fn local_func__mode_global_variables_4(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_4(self).into()
        }
    }

    pub fn local_func__mode_global_variables_5(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_5(self).into()
        }
    }

    pub fn local_func__mode_global_variables_6(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_6(self).into()
        }
    }

    pub fn local_func__mode_global_variables_7(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_7(self).into()
        }
    }

    pub fn local_func__mode_global_variables_10(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_10(self).into()
        }
    }

    pub fn local_func__mode_global_variables_11(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_11(self).into()
        }
    }

    pub fn local_func__mode_global_variables_12(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_12(self).into()
        }
    }

    pub fn local_func__mode_global_variables_13(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_13(self).into()
        }
    }

    pub fn local_func__mode_global_variables_14(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_14(self).into()
        }
    }

    pub fn local_func__mode_global_variables_15(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_15(self).into()
        }
    }

    pub fn local_func__mode_global_variables_8(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_8(self).into()
        }
    }

    pub fn local_func__mode_global_variables_9(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_9(self).into()
        }
    }

    pub fn local_func__mode_global_variables_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_1(self).into()
        }
    }

    pub fn local_func__mode_global_variables_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__mode_global_variables_2(self).into()
        }
    }

    pub fn specializer_interrupt_action(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            specializer_interrupt_action(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn ACTION_BUILDUP__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_BUILDUP__name(self).into()
        }
    }

    pub fn ACTION_BUILDUP__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_BUILDUP__init(self).into()
        }
    }

    pub fn ACTION_BUILDUP__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_BUILDUP__update(self).into()
        }
    }

    pub fn ACTION_BUILDUP__cancel(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_BUILDUP__cancel(self).into()
        }
    }

    pub fn ACTION_BUILDUP__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_BUILDUP__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_COMBO__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_COMBO__name(self).into()
        }
    }

    pub fn ACTION_COMBO__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_COMBO__init(self).into()
        }
    }

    pub fn ACTION_COMBO__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_COMBO__update(self).into()
        }
    }

    pub fn ACTION_COMBO__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_COMBO__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_COMBO__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_COMBO__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_COMBO__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_COMBO__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_COMBO__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_COMBO__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_COMBO__interrupt_aerial_jump(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_COMBO__interrupt_aerial_jump(self).into()
        }
    }

    pub fn ACTION_DASH_ATTACK__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ATTACK__name(self).into()
        }
    }

    pub fn ACTION_DASH_ATTACK__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ATTACK__init(self).into()
        }
    }

    pub fn ACTION_DASH_ATTACK__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ATTACK__update(self).into()
        }
    }

    pub fn ACTION_DASH_ATTACK__exit(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ATTACK__exit(self).into()
        }
    }

    pub fn ACTION_ESCAPE__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE__name(self).into()
        }
    }

    pub fn ACTION_ESCAPE__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE__init(self).into()
        }
    }

    pub fn ACTION_ESCAPE__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE__update(self).into()
        }
    }

    pub fn ACTION_ESCAPE__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_ESCAPE__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_ESCAPE__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_ESCAPE__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_STROLL_L__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_STROLL_L__name(self).into()
        }
    }

    pub fn ACTION_STROLL_L__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_STROLL_L__init(self).into()
        }
    }

    pub fn ACTION_STROLL_L__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_STROLL_L__update(self).into()
        }
    }

    pub fn ACTION_STROLL_L__reset(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_STROLL_L__reset(self).into()
        }
    }

    pub fn ACTION_STROLL_S__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_STROLL_S__name(self).into()
        }
    }

    pub fn ACTION_STROLL_S__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_STROLL_S__init(self).into()
        }
    }

    pub fn ACTION_STROLL_S__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_STROLL_S__update(self).into()
        }
    }

    pub fn ACTION_STROLL_S__reset(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_STROLL_S__reset(self).into()
        }
    }

    pub fn ACTION_STROLL_SQUAT__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_STROLL_SQUAT__name(self).into()
        }
    }

    pub fn ACTION_STROLL_SQUAT__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_STROLL_SQUAT__init(self).into()
        }
    }

    pub fn ACTION_STROLL_SQUAT__update(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_STROLL_SQUAT__update(self).into()
        }
    }

    pub fn ACTION_STROLL_SQUAT__reset(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_STROLL_SQUAT__reset(self).into()
        }
    }

    pub fn ACTION_DASH_ATTACK__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ATTACK__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn local_func__action_buildup_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_buildup_2(self).into()
        }
    }

    pub fn local_func__action_buildup_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_buildup_3(self).into()
        }
    }

    pub fn specializer_buildup(&mut self) -> lib::L2CValue {
        unsafe {
            specializer_buildup(self).into()
        }
    }

    pub fn change_mode_action_none(&mut self) {
        unsafe {
            change_mode_action_none(self)
        }
    }

    pub fn set_action_probability_mul_1st(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            set_action_probability_mul_1st(self, arg1.into(), arg2.into())
        }
    }

    pub fn change_mode_action_no_restart(&mut self, arg1: lib::L2CValue) {
        unsafe {
            change_mode_action_no_restart(self, arg1.into())
        }
    }

    pub fn specializer_interrupt_combo(&mut self) -> lib::L2CValue {
        unsafe {
            specializer_interrupt_combo(self).into()
        }
    }

    pub fn notify_on_capture(&mut self) {
        unsafe {
            notify_on_capture(self)
        }
    }

    pub fn change_mode_action_by_interrupt(&mut self, arg1: lib::L2CValue) {
        unsafe {
            change_mode_action_by_interrupt(self, arg1.into())
        }
    }

    pub fn specializer_interrupt_always(&mut self) {
        unsafe {
            specializer_interrupt_always(self)
        }
    }

    pub fn specializer_interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            specializer_interrupt_dive(self).into()
        }
    }

    pub fn specializer_interrupt_action_on_meteor(&mut self) -> lib::L2CValue {
        unsafe {
            specializer_interrupt_action_on_meteor(self).into()
        }
    }

    pub fn change_mode_action_none_by_interrupt(&mut self) {
        unsafe {
            change_mode_action_none_by_interrupt(self)
        }
    }

    pub fn specializer_interrupt_action_post(&mut self) -> lib::L2CValue {
        unsafe {
            specializer_interrupt_action_post(self).into()
        }
    }

    pub fn specializer_has_reflector(&mut self) -> lib::L2CValue {
        unsafe {
            specializer_has_reflector(self).into()
        }
    }

    pub fn specializer_reflect_cmd_id(&mut self) -> lib::L2CValue {
        unsafe {
            specializer_reflect_cmd_id(self).into()
        }
    }

    pub fn specializer_has_absorber(&mut self) -> lib::L2CValue {
        unsafe {
            specializer_has_absorber(self).into()
        }
    }

    pub fn specializer_absorb_cmd_id(&mut self) -> lib::L2CValue {
        unsafe {
            specializer_absorb_cmd_id(self).into()
        }
    }

    pub fn select_guard_action_ground(&mut self) {
        unsafe {
            select_guard_action_ground(self)
        }
    }

    pub fn specializer_can_use_before_buildmax(&mut self) -> lib::L2CValue {
        unsafe {
            specializer_can_use_before_buildmax(self).into()
        }
    }

    pub fn specializer_disable_buildmax_cmd_id(&mut self) {
        unsafe {
            specializer_disable_buildmax_cmd_id(self)
        }
    }

    pub fn specializer_restrict_always_command(&mut self) {
        unsafe {
            specializer_restrict_always_command(self)
        }
    }

    pub fn get_action_probability_mul_1st(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            get_action_probability_mul_1st(self, arg1.into()).into()
        }
    }

    pub fn set_action_probability_mul_2nd(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            set_action_probability_mul_2nd(self, arg1.into(), arg2.into())
        }
    }

    pub fn get_action_probability_mul_2nd(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            get_action_probability_mul_2nd(self, arg1.into()).into()
        }
    }

    pub fn set_action_probability_mul_3rd(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            set_action_probability_mul_3rd(self, arg1.into(), arg2.into())
        }
    }

    pub fn get_action_probability_mul_3rd(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            get_action_probability_mul_3rd(self, arg1.into()).into()
        }
    }

    pub fn set_action_probability_mul_2nd_foreach(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            set_action_probability_mul_2nd_foreach(self, arg1.into(), arg2.into())
        }
    }

    pub fn reset_action_probability_mul_3rd(&mut self) {
        unsafe {
            reset_action_probability_mul_3rd(self)
        }
    }

    pub fn is_movement_mode_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_movement_mode_action(self, arg1.into()).into()
        }
    }

    pub fn is_wait_mode_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_wait_mode_action(self, arg1.into()).into()
        }
    }

    pub fn is_stroll_mode_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_stroll_mode_action(self, arg1.into()).into()
        }
    }

    pub fn is_dash_backward_mode_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_dash_backward_mode_action(self, arg1.into()).into()
        }
    }

    pub fn is_jump_s_attack_mode_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_jump_s_attack_mode_action(self, arg1.into()).into()
        }
    }

    pub fn is_guard_mode_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_guard_mode_action(self, arg1.into()).into()
        }
    }

    pub fn is_escape_mode_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_escape_mode_action(self, arg1.into()).into()
        }
    }

    pub fn is_large_jump_mode_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_large_jump_mode_action(self, arg1.into()).into()
        }
    }

    pub fn set_action_probability_weapondefmax(&mut self) {
        unsafe {
            set_action_probability_weapondefmax(self)
        }
    }

    pub fn init_events(&mut self) {
        unsafe {
            init_events(self)
        }
    }

    pub fn init_specializer(&mut self) {
        unsafe {
            init_specializer(self)
        }
    }

    pub fn global__tactics___change_tactics(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            global__tactics___change_tactics(self, arg1.into()).into()
        }
    }

    pub fn call_function_update(&mut self) -> lib::L2CValue {
        unsafe {
            call_function_update(self).into()
        }
    }

    pub fn call_function_is_running_mode_action(&mut self) -> lib::L2CValue {
        unsafe {
            call_function_is_running_mode_action(self).into()
        }
    }

    pub fn call_function_set_action_probability_mul_as_grounder(&mut self) -> lib::L2CValue {
        unsafe {
            call_function_set_action_probability_mul_as_grounder(self).into()
        }
    }

    pub fn call_function_set_action_probability_mul_as_jumper(&mut self) -> lib::L2CValue {
        unsafe {
            call_function_set_action_probability_mul_as_jumper(self).into()
        }
    }

    pub fn call_function_set_action_probability_mul_as_dasher(&mut self) -> lib::L2CValue {
        unsafe {
            call_function_set_action_probability_mul_as_dasher(self).into()
        }
    }

    pub fn call_function_set_action_probability_mul_as_walker(&mut self) -> lib::L2CValue {
        unsafe {
            call_function_set_action_probability_mul_as_walker(self).into()
        }
    }

    pub fn call_function_set_action_probability_mul_as_charger(&mut self) -> lib::L2CValue {
        unsafe {
            call_function_set_action_probability_mul_as_charger(self).into()
        }
    }

    pub fn call_function_set_action_probability_mul_as_forward(&mut self) -> lib::L2CValue {
        unsafe {
            call_function_set_action_probability_mul_as_forward(self).into()
        }
    }

    pub fn call_function_set_action_probability_weapondefmax(&mut self) -> lib::L2CValue {
        unsafe {
            call_function_set_action_probability_weapondefmax(self).into()
        }
    }

    pub fn call_event_on_capture(&mut self) -> lib::L2CValue {
        unsafe {
            call_event_on_capture(self).into()
        }
    }

    pub fn regist_mode_action(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            regist_mode_action(self, arg1.into(), arg2.into())
        }
    }

    pub fn is_cmd_special(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_cmd_special(self, arg1.into()).into()
        }
    }

    pub fn ACTION_AIR_ATTACK__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AIR_ATTACK__name(self).into()
        }
    }

    pub fn ACTION_AIR_ATTACK__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AIR_ATTACK__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_AIR_ATTACK__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AIR_ATTACK__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_APPEAL__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_APPEAL__name(self).into()
        }
    }

    pub fn ACTION_APPEAL__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_APPEAL__init(self).into()
        }
    }

    pub fn ACTION_APPEAL__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_APPEAL__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_APPEAL__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_APPEAL__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_APPEAL__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_APPEAL__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_APPEAL__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_APPEAL__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_APPEAL__interrupt_aerial_jump(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_APPEAL__interrupt_aerial_jump(self).into()
        }
    }

    pub fn ACTION_ATTACK_123__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ATTACK_123__name(self).into()
        }
    }

    pub fn ACTION_ATTACK_123__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ATTACK_123__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_ATTACK_123__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ATTACK_123__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_ATTACK_123__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ATTACK_123__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_ATTACK_123__interrupt_combo(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ATTACK_123__interrupt_combo(self).into()
        }
    }

    pub fn ACTION_AVOID_DONKEY_SPECIAL_LW__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AVOID_DONKEY_SPECIAL_LW__name(self).into()
        }
    }

    pub fn ACTION_AVOID_DONKEY_SPECIAL_LW__update_on_stop(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AVOID_DONKEY_SPECIAL_LW__update_on_stop(self).into()
        }
    }

    pub fn ACTION_AVOID_DONKEY_SPECIAL_LW__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AVOID_DONKEY_SPECIAL_LW__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_AVOID_DONKEY_SPECIAL_LW__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AVOID_DONKEY_SPECIAL_LW__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_AVOID_DONKEY_SPECIAL_LW__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AVOID_DONKEY_SPECIAL_LW__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_AVOID_GANON_SPECIAL_AIR_S__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AVOID_GANON_SPECIAL_AIR_S__name(self).into()
        }
    }

    pub fn ACTION_AVOID_KOOPA_SPECIAL_S__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_AVOID_KOOPA_SPECIAL_S__name(self).into()
        }
    }

    pub fn ACTION_CLIFF__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_CLIFF__name(self).into()
        }
    }

    pub fn ACTION_CLIFF__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_CLIFF__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_CLIFF__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_CLIFF__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_CLIFF__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_CLIFF__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_CLIFF__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_CLIFF__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_DAMAGED__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DAMAGED__name(self).into()
        }
    }

    pub fn ACTION_DAMAGED__update_on_stop(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DAMAGED__update_on_stop(self).into()
        }
    }

    pub fn ACTION_DAMAGED__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DAMAGED__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_DAMAGED__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DAMAGED__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_DAMAGED__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DAMAGED__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_DAMAGED__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DAMAGED__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_DAMAGED__interrupt_aerial_jump(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DAMAGED__interrupt_aerial_jump(self).into()
        }
    }

    pub fn ACTION_DASH_B__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B__name(self).into()
        }
    }

    pub fn ACTION_DASH_B__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_DASH_B__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_DASH_B__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_DASH_B__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_DASH_B_DASH_F__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B_DASH_F__name(self).into()
        }
    }

    pub fn ACTION_DASH_B_DASH_F__exit(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B_DASH_F__exit(self).into()
        }
    }

    pub fn ACTION_DASH_B_DASH_F__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B_DASH_F__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_DASH_B_DASH_F__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B_DASH_F__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_DASH_B_JUMP_S__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B_JUMP_S__name(self).into()
        }
    }

    pub fn ACTION_DASH_B_JUMP_S__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B_JUMP_S__init(self).into()
        }
    }

    pub fn ACTION_DASH_B_JUMP_S__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B_JUMP_S__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_DASH_B_JUMP_S__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B_JUMP_S__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_DASH_B_JUMP_S__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B_JUMP_S__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_DASH_B_JUMP_S__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B_JUMP_S__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_DASH_B_JUMP_S__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_B_JUMP_S__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_DASH_F_DASH_B__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_DASH_B__name(self).into()
        }
    }

    pub fn ACTION_DASH_F_DASH_B__exit(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_DASH_B__exit(self).into()
        }
    }

    pub fn ACTION_DASH_F_DASH_B__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_DASH_B__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_DASH_F_DASH_B__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_DASH_B__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_DASH_F_JUMP_B__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_JUMP_B__name(self).into()
        }
    }

    pub fn ACTION_DASH_F_JUMP_B__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_JUMP_B__init(self).into()
        }
    }

    pub fn ACTION_DASH_F_JUMP_B__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_JUMP_B__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_DASH_F_JUMP_B__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_JUMP_B__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_DASH_F_WAIT__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_WAIT__name(self).into()
        }
    }

    pub fn ACTION_DASH_F_WAIT__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_WAIT__init(self).into()
        }
    }

    pub fn ACTION_DASH_F_WAIT__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_F_WAIT__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_DASH_GUARD__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_GUARD__name(self).into()
        }
    }

    pub fn ACTION_DASH_GUARD__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_GUARD__init(self).into()
        }
    }

    pub fn ACTION_DASH_GUARD__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_GUARD__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_DASH_GUARD__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_GUARD__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_DASH_GUARD__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_GUARD__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_DASH_ROLL_B__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ROLL_B__name(self).into()
        }
    }

    pub fn ACTION_DASH_ROLL_B__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ROLL_B__init(self).into()
        }
    }

    pub fn ACTION_DASH_ROLL_B__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ROLL_B__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_DASH_ROLL_B__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ROLL_B__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_DASH_ROLL_B__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ROLL_B__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_DASH_ROLL_F__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ROLL_F__name(self).into()
        }
    }

    pub fn ACTION_DASH_ROLL_F__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ROLL_F__init(self).into()
        }
    }

    pub fn ACTION_DASH_ROLL_F__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ROLL_F__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_DASH_ROLL_F__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ROLL_F__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_DASH_ROLL_F__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DASH_ROLL_F__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_DEADAREA_AIR_ATTACK__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DEADAREA_AIR_ATTACK__name(self).into()
        }
    }

    pub fn ACTION_DEADAREA_AIR_ATTACK__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DEADAREA_AIR_ATTACK__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_DEADAREA_DASH_ATTACK__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DEADAREA_DASH_ATTACK__name(self).into()
        }
    }

    pub fn ACTION_DEADAREA_DASH_ATTACK__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DEADAREA_DASH_ATTACK__init(self).into()
        }
    }

    pub fn ACTION_DOWN__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DOWN__name(self).into()
        }
    }

    pub fn ACTION_DOWN__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DOWN__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_DOWN__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DOWN__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_DOWN__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DOWN__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_DOWN__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DOWN__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_DRAGOON__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DRAGOON__name(self).into()
        }
    }

    pub fn ACTION_DRILL__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DRILL__name(self).into()
        }
    }

    pub fn ACTION_DRILL__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_DRILL__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_ENTRY__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ENTRY__name(self).into()
        }
    }

    pub fn ACTION_ENTRY__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ENTRY__init(self).into()
        }
    }

    pub fn ACTION_ENTRY__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ENTRY__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_ENTRY__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ENTRY__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_ENTRY__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ENTRY__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_ENTRY__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ENTRY__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_ENTRY__interrupt_aerial_jump(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ENTRY__interrupt_aerial_jump(self).into()
        }
    }

    pub fn ACTION_ESCAPE_AIR__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_AIR__name(self).into()
        }
    }

    pub fn ACTION_ESCAPE_AIR__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_AIR__init(self).into()
        }
    }

    pub fn ACTION_ESCAPE_AIR__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_AIR__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_ESCAPE_AIR__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_AIR__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_ESCAPE_AIR__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_AIR__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_ESCAPE_AIR__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_AIR__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_ESCAPE_AIR_MOVE__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_AIR_MOVE__name(self).into()
        }
    }

    pub fn ACTION_ESCAPE_AIR_MOVE__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_AIR_MOVE__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_ESCAPE_AIR_MOVE__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_AIR_MOVE__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_ESCAPE_AIR_MOVE__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_AIR_MOVE__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_ESCAPE_AIR_MOVE__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_AIR_MOVE__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_ESCAPE_FAR__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_FAR__name(self).into()
        }
    }

    pub fn ACTION_ESCAPE_FAR__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_FAR__init(self).into()
        }
    }

    pub fn ACTION_ESCAPE_FAR__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_FAR__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_ESCAPE_FAR__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_FAR__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_ESCAPE_FAR__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_FAR__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_ESCAPE_FAR__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_FAR__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_ESCAPE_NEAR__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_NEAR__name(self).into()
        }
    }

    pub fn ACTION_ESCAPE_NEAR__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_NEAR__init(self).into()
        }
    }

    pub fn ACTION_ESCAPE_NEAR__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_NEAR__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_ESCAPE_NEAR__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_NEAR__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_ESCAPE_NEAR__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_NEAR__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_ESCAPE_NEAR__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ESCAPE_NEAR__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_FALL__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_FALL__name(self).into()
        }
    }

    pub fn ACTION_FALL__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_FALL__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_FALL__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_FALL__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_FALL__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_FALL__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_FALL__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_FALL__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_FALL__interrupt_aerial_jump(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_FALL__interrupt_aerial_jump(self).into()
        }
    }

    pub fn ACTION_FALL__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_FALL__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_FINAL_END_JUMP__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_FINAL_END_JUMP__name(self).into()
        }
    }

    pub fn ACTION_FINAL_END_JUMP__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_FINAL_END_JUMP__init(self).into()
        }
    }

    pub fn ACTION_FINAL_END_JUMP__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_FINAL_END_JUMP__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_GUARD__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_GUARD__name(self).into()
        }
    }

    pub fn ACTION_GUARD__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_GUARD__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_GUARD__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_GUARD__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_GUARD__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_GUARD__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_HIGH_SPEED_DASH__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_HIGH_SPEED_DASH__name(self).into()
        }
    }

    pub fn ACTION_HIGH_SPEED_DASH__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_HIGH_SPEED_DASH__init(self).into()
        }
    }

    pub fn ACTION_HIGH_SPEED_DASH__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_HIGH_SPEED_DASH__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_HIGH_SPEED_DASH__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_HIGH_SPEED_DASH__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_HIGH_SPEED_DASH__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_HIGH_SPEED_DASH__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK__name(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK_F__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK_F__name(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK_F__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK_F__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK_F__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK_F__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK_F__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK_F__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK_F__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK_F__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_JUMP_ATTACK_F__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_ATTACK_F__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_JUMP_S_ATTACK__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_ATTACK__name(self).into()
        }
    }

    pub fn ACTION_JUMP_S_ATTACK__notify_on_attack_hit(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_ATTACK__notify_on_attack_hit(self).into()
        }
    }

    pub fn ACTION_JUMP_S_ATTACK__notify_on_attack_shield(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_ATTACK__notify_on_attack_shield(self).into()
        }
    }

    pub fn ACTION_JUMP_S_ATTACK__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_ATTACK__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_JUMP_S_ATTACK__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_ATTACK__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_JUMP_S_ATTACK__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_ATTACK__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_JUMP_S_ATTACK__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_ATTACK__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_JUMP_S_B__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_B__name(self).into()
        }
    }

    pub fn ACTION_JUMP_S_B__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_B__init(self).into()
        }
    }

    pub fn ACTION_JUMP_S_B__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_B__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_JUMP_S_B__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_B__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_JUMP_S_B__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_B__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_JUMP_S_B__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_B__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_JUMP_S_N_ATTACK__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_N_ATTACK__name(self).into()
        }
    }

    pub fn ACTION_JUMP_S_N_ATTACK__notify_on_attack_hit(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_N_ATTACK__notify_on_attack_hit(self).into()
        }
    }

    pub fn ACTION_JUMP_S_N_ATTACK__notify_on_attack_shield(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_N_ATTACK__notify_on_attack_shield(self).into()
        }
    }

    pub fn ACTION_JUMP_S_N_ATTACK__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_N_ATTACK__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_JUMP_S_N_ATTACK__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_N_ATTACK__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_JUMP_S_N_ATTACK__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_N_ATTACK__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_JUMP_S_N_ATTACK__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_JUMP_S_N_ATTACK__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_MACHSTAMP__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_MACHSTAMP__name(self).into()
        }
    }

    pub fn ACTION_MACHSTAMP__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_MACHSTAMP__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_MACHSTAMP__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_MACHSTAMP__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_MACHSTAMP__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_MACHSTAMP__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_MACHSTAMP__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_MACHSTAMP__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_METEOR__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_METEOR__name(self).into()
        }
    }

    pub fn ACTION_METEOR__is_cancelable_phase(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_METEOR__is_cancelable_phase(self).into()
        }
    }

    pub fn ACTION_METEOR__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_METEOR__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_METEOR__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_METEOR__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_METEOR__interrupt_combo(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_METEOR__interrupt_combo(self).into()
        }
    }

    pub fn ACTION_METEOR__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_METEOR__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_METEOR__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_METEOR__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_PASS_FLOOR__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_PASS_FLOOR__name(self).into()
        }
    }

    pub fn ACTION_PASS_FLOOR__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_PASS_FLOOR__init(self).into()
        }
    }

    pub fn ACTION_PASS_FLOOR__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_PASS_FLOOR__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_PASS_FLOOR__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_PASS_FLOOR__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_PASS_FLOOR__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_PASS_FLOOR__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_PASS_FLOOR__interrupt_aerial_jump(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_PASS_FLOOR__interrupt_aerial_jump(self).into()
        }
    }

    pub fn ACTION_PURSUE__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_PURSUE__name(self).into()
        }
    }

    pub fn ACTION_PURSUE__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_PURSUE__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_PURSUE__interrupt_aerial_jump(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_PURSUE__interrupt_aerial_jump(self).into()
        }
    }

    pub fn ACTION_PURSUE__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_PURSUE__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_REBIRTH__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_REBIRTH__name(self).into()
        }
    }

    pub fn ACTION_REBIRTH__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_REBIRTH__init(self).into()
        }
    }

    pub fn ACTION_REBIRTH__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_REBIRTH__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_REBIRTH__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_REBIRTH__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_REBIRTH__interrupt_aerial_jump(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_REBIRTH__interrupt_aerial_jump(self).into()
        }
    }

    pub fn ACTION_REBIRTH__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_REBIRTH__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_RETURN__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RETURN__name(self).into()
        }
    }

    pub fn ACTION_RETURN__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RETURN__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_RETURN__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RETURN__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_RETURN__interrupt_aerial_jump(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RETURN__interrupt_aerial_jump(self).into()
        }
    }

    pub fn ACTION_RETURN__interrupt_combo(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RETURN__interrupt_combo(self).into()
        }
    }

    pub fn ACTION_ROLL_B__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ROLL_B__name(self).into()
        }
    }

    pub fn ACTION_ROLL_B__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ROLL_B__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_ROLL_B__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ROLL_B__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_ROLL_B__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ROLL_B__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_ROLL_B__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ROLL_B__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_ROLL_F__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ROLL_F__name(self).into()
        }
    }

    pub fn ACTION_ROLL_F__interrupt_attack(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ROLL_F__interrupt_attack(self).into()
        }
    }

    pub fn ACTION_ROLL_F__interrupt_guard(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ROLL_F__interrupt_guard(self).into()
        }
    }

    pub fn ACTION_ROLL_F__interrupt_guard_weapon(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ROLL_F__interrupt_guard_weapon(self).into()
        }
    }

    pub fn ACTION_ROLL_F__interrupt_dive(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_ROLL_F__interrupt_dive(self).into()
        }
    }

    pub fn ACTION_RUN__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RUN__name(self).into()
        }
    }

    pub fn ACTION_RUN__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RUN__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_RUN_ATTACK__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_RUN_ATTACK__name(self).into()
        }
    }

    pub fn ACTION_STROLL_L__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_STROLL_L__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_STROLL_S__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_STROLL_S__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_STROLL_SQUAT__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_STROLL_SQUAT__interrupt_auto_avoid_dead_zone(self).into()
        }
    }

    pub fn ACTION_TURN_TURN__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_TURN_TURN__name(self).into()
        }
    }

    pub fn ACTION_TURN_TURN__init(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_TURN_TURN__init(self).into()
        }
    }

    pub fn ACTION_WAIT_L__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_WAIT_L__name(self).into()
        }
    }

    pub fn ACTION_WAIT_S__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_WAIT_S__name(self).into()
        }
    }

    pub fn ACTION_WAIT_SQUAT__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_WAIT_SQUAT__name(self).into()
        }
    }

    pub fn ACTION_WALK__name(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_WALK__name(self).into()
        }
    }

    pub fn ACTION_WALK__interrupt_auto_avoid_dead_zone(&mut self) -> lib::L2CValue {
        unsafe {
            ACTION_WALK__interrupt_auto_avoid_dead_zone(self).into()
        }
    }
}

#[cfg(feature = "type_assert")]
impl L2CFighterAIModeBase {
    pub fn assert() {
        assert_eq!(size_of!(L2CFighterAIModeBase), 0x998);
        assert_eq!(offset_of!(L2CFighterAIModeBase, ai_base), 0x0);
        assert_eq!(offset_of!(L2CFighterAIModeBase, private_table), 0x158);
        assert_eq!(offset_of!(L2CFighterAIModeBase, zero_globals1), 0x168);
        assert_eq!(offset_of!(L2CFighterAIModeBase, one_globals1), 0x178);
        assert_eq!(offset_of!(L2CFighterAIModeBase, two_globals1), 0x188);
        assert_eq!(offset_of!(L2CFighterAIModeBase, three_globals1), 0x198);
        assert_eq!(offset_of!(L2CFighterAIModeBase, one_globals2), 0x1A8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, two_globals2), 0x1B8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, three_globals2), 0x1C8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, four_globals2), 0x1D8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, global_table), 0x1e8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_34), 0x1f8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_28), 0x208);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_3), 0x218);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_14), 0x228);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_15), 0x238);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_32), 0x248);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_23), 0x258);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_9), 0x268);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_7), 0x278);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_8), 0x288);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_1), 0x298);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_52), 0x2a8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_53), 0x2b8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_16), 0x2c8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_17), 0x2d8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_10), 0x2e8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_24), 0x2f8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_25), 0x308);
        assert_eq!(offset_of!(L2CFighterAIModeBase, func_table), 0x318);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_44), 0x328);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_2), 0x338);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_31), 0x348);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_27), 0x358);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_57), 0x368);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_58), 0x378);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_5), 0x388);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_6), 0x398);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_0), 0x3a8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_41), 0x3b8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_46), 0x3c8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_45), 0x3d8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_40), 0x3E8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_42), 0x3f8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_51), 0x408);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_55), 0x418);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_56), 0x428);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_36), 0x438);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_39), 0x448);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_54), 0x458);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_33), 0x468);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_37), 0x478);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_35), 0x488);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_38), 0x498);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_13), 0x4a8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_49), 0x4b8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_47), 0x4c8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_48), 0x4d8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_43), 0x4e8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, unk_val), 0x4f8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_59), 0x508);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_50), 0x518);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_12), 0x528);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_19), 0x538);
        assert_eq!(offset_of!(L2CFighterAIModeBase, unk_val2), 0x548);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_4), 0x558);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_11), 0x568);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_18), 0x578);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_20), 0x588);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_21), 0x598);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_22), 0x5a8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_26), 0x5b8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_29), 0x5c8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_mode_30), 0x5d8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_run), 0x5E8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, attack_air_attack), 0x5F8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_guard), 0x608);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_escape), 0x618);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_escape_near), 0x628);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_escape_far), 0x638);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_escape_air), 0x648);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_escape_air_move), 0x658);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_roll_f), 0x668);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_roll_b), 0x678);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_walk), 0x688);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_dash_attack), 0x698);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_dash_guard), 0x6a8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_dash_roll_f), 0x6b8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_dash_roll_b), 0x6c8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_dash_b), 0x6d8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_dash_b_dash_f), 0x6e8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_dash_f_dash_b), 0x6f8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_dash_f_jump_b), 0x708);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_dash_b_jump_s), 0x718);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_dash_f_wait), 0x728);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_turn_turn), 0x738);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_drill), 0x748);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_damaged), 0x758);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_fall), 0x768);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_stroll_s), 0x778);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_stroll_l), 0x788);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_stroll_squat), 0x798);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_wait_s), 0x7a8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_wait_l), 0x7b8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_wait_squat), 0x7c8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_pursue), 0x7d8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_return), 0x7e8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_jump_attack), 0x7f8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_jump_attack_f), 0x808);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_jump_s_attack_f), 0x818);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_jump_s_b), 0x828);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_jump_s_n_attack), 0x838);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_rebirth), 0x848);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_down), 0x858);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_cliff), 0x868);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_dragoon), 0x878);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_combo), 0x888);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_entry_l), 0x898);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_appeal), 0x8a8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_buildup), 0x8b8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_meteor), 0x8c8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_attack_123), 0x8d8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_pass_floor), 0x8e8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_run_attack), 0x8f8);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_final_end_jump), 0x908);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_machstamp), 0x918);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_high_speed_dash), 0x928);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_avoid_donkey_special_lw), 0x938);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_avoid_koopa_special_s), 0x948);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_avoid_ganon_special_air_s), 0x958);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_deadarea_air_attack), 0x968);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_deadarea_dash_attack), 0x978);
        assert_eq!(offset_of!(L2CFighterAIModeBase, action_count), 0x988);
    }
}