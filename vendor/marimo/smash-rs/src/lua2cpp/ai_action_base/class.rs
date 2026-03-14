use std::ops::{Deref, DerefMut};

use crate::*;

use super::cpp::*;

#[repr(C)]
pub struct L2CFighterAIActionBase {
    ai_base: lua2cpp::L2CFighterAIBase,
    pub global_table: lib::L2CValue,
    pub private_table: lib::L2CValue,
    // misc actions?
    pub act_a_goro: lib::L2CValue,
    pub act_a_catch_atk: lib::L2CValue,
    pub act_a_final: lib::L2CValue,
    pub act_r_jump: lib::L2CValue,
    pub act_r_act_last: lib::L2CValue,
    pub act_combo: lib::L2CValue,
    // ground actions
    // Note that the way these work is that act_c_ground is the default for *all* ground actions and then each of the following
    // are set as overrides if they are not nil
    pub act_c_ground: lib::L2CValue,
    pub act_c_g_atk_n: lib::L2CValue,
    pub act_c_g_atk_f: lib::L2CValue,
    pub act_c_g_atk_u: lib::L2CValue,
    pub act_c_g_atk_d: lib::L2CValue,
    pub act_c_g_smash_f: lib::L2CValue,
    pub act_c_g_smash_u: lib::L2CValue,
    pub act_c_g_smash_d: lib::L2CValue,
    pub act_c_g_blow_n: lib::L2CValue,
    pub act_c_g_blow_f: lib::L2CValue,
    pub act_c_g_blow_u: lib::L2CValue,
    pub act_c_g_blow_d: lib::L2CValue,
    pub act_c_g_blow_catch: lib::L2CValue,
    pub act_c_g_none: lib::L2CValue,
    // air actions
    // Note that the way these works is that act_c_air is the default for *all* air actions and then each of the following
    // are set as overrides if they are not nil
    pub act_c_air: lib::L2CValue,
    pub act_c_a_atk_n: lib::L2CValue,
    pub act_c_a_atk_f: lib::L2CValue,
    pub act_c_a_atk_b: lib::L2CValue,
    pub act_c_a_atk_u: lib::L2CValue,
    pub act_c_a_atk_d: lib::L2CValue,
    pub act_c_a_blow_n: lib::L2CValue,
    pub act_c_a_blow_f: lib::L2CValue,
    pub act_c_a_blow_u: lib::L2CValue,
    pub act_c_a_blow_d: lib::L2CValue,
    // these are also defaulted to act_c_ground and then overriden if not nil
    pub act_c_g_atk2_n: lib::L2CValue,
    pub act_c_g_atk2_f: lib::L2CValue,
    pub act_c_g_atk2_u: lib::L2CValue,
    pub act_c_g_atk2_d: lib::L2CValue,
    pub act_c_g_dash_attack: lib::L2CValue,
    pub act_c_g_dash_catch: lib::L2CValue,
    pub act_c_g_turn_catch: lib::L2CValue,
    // defaults to act_c_air and overriden if not nil
    pub act_c_a_catch: lib::L2CValue,
    // the following are not necessarily defaulting to act_c_ground anymore, they pull values from `private_table` before
    // attempting to set these as overrides
    // they are also all DLC lol
    pub act_c_g_brave_blow_f2: lib::L2CValue,
    pub act_c_g_brave_blow_f3: lib::L2CValue,
    pub act_c_a_brave_blow_f2: lib::L2CValue,
    pub act_c_a_brave_blow_f3: lib::L2CValue,
    pub act_c_g_blow_b: lib::L2CValue,
    pub act_c_g_blow_super1: lib::L2CValue,
    pub act_c_g_blow_super2: lib::L2CValue,
    pub act_c_g_blow3_n: lib::L2CValue,
    pub act_c_g_blow3_f: lib::L2CValue,
    pub act_c_g_blow3_b: lib::L2CValue,
    pub act_c_g_blow3_u: lib::L2CValue,
    pub act_c_g_blow3_d: lib::L2CValue,
    pub act_c_a_blow_b: lib::L2CValue,
    pub act_c_a_blow3_n: lib::L2CValue,
    pub act_c_a_blow3_f: lib::L2CValue,
    pub act_c_a_blow3_b: lib::L2CValue,
    pub act_c_a_blow3_u: lib::L2CValue,
    pub act_c_a_blow3_d: lib::L2CValue,
    pub act_c_g_master_blow_f2: lib::L2CValue,
    pub act_c_a_master_blow_f2: lib::L2CValue,
    pub act_c_g_tantan_atk_f_r: lib::L2CValue,
    pub act_c_g_tantan_smash_f_r: lib::L2CValue,
    pub act_c_a_tantan_atk_f2: lib::L2CValue,
    pub act_c_a_tantan_atk_b2: lib::L2CValue,
    pub act_c_a_tantan_atk_n_r: lib::L2CValue,
    pub act_c_a_tantan_atk_f_r: lib::L2CValue,
    pub act_c_a_tantan_atk_b_r: lib::L2CValue,
    pub act_c_a_tantan_atk_f_r2: lib::L2CValue,
    pub act_c_a_tantan_atk_b_r2: lib::L2CValue,
    pub act_c_g_pickel_blow_f2: lib::L2CValue,
    pub act_c_a_pickel_blow_f2: lib::L2CValue,
    pub act_c_g_demon_atk_stand_1: lib::L2CValue,
    pub act_c_g_demon_atk_stand_2: lib::L2CValue,
    pub act_c_g_demon_atk_stand_3: lib::L2CValue,
    pub act_c_g_demon_atk_stand_4: lib::L2CValue,
    pub act_c_g_demon_atk_stand_5: lib::L2CValue,
    pub act_c_g_demon_atk_stand_6: lib::L2CValue,
    pub act_c_g_demon_atk_squat_1: lib::L2CValue,
    pub act_c_g_demon_atk_squat_2: lib::L2CValue,
    pub act_c_g_demon_atk_squat_3: lib::L2CValue,
    pub act_c_g_demon_atk_squat_4: lib::L2CValue,
    pub act_c_g_demon_atk_catch: lib::L2CValue,
    pub act_c_g_demon_atk_step_2: lib::L2CValue,
    pub act_c_g_demon_atk_step_2f: lib::L2CValue,
    pub act_c_g_demon_atk_step_2l: lib::L2CValue,
    pub act_c_g_demon_atk_step_2s: lib::L2CValue,
    pub act_c_g_demon_atk_rage: lib::L2CValue,
}

impl Deref for L2CFighterAIActionBase {
    type Target = lua2cpp::L2CFighterAIBase;

    fn deref(&self) -> &Self::Target {
        &self.ai_base
    }
}

impl DerefMut for L2CFighterAIActionBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ai_base
    }
}

impl L2CFighterAIActionBase {
    pub fn local_func__action_global_variables_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_global_variables_1(self).into()
        }
    }

    pub fn local_func__action_private_variables_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__action_private_variables_1(self).into()
        }
    }

    pub fn main_func__act_a_catch_atk(&mut self) {
        unsafe {
            main_func__act_a_catch_atk(self)
        }
    }

    pub fn main_func__act_a_final(&mut self) {
        unsafe {
            main_func__act_a_final(self)
        }
    }

    pub fn main_func__act_c_air(&mut self) {
        unsafe {
            main_func__act_c_air(self)
        }
    }

    pub fn main_func__act_c_ground(&mut self) {
        unsafe {
            main_func__act_c_ground(self)
        }
    }

    pub fn main_func__act_r_act_last(&mut self) {
        unsafe {
            main_func__act_r_act_last(self)
        }
    }

    pub fn main_func__act_r_jump(&mut self) {
        unsafe {
            main_func__act_r_jump(self)
        }
    }

    pub fn main_func__phase_c_command(&mut self) {
        unsafe {
            main_func__phase_c_command(self)
        }
    }

    pub fn call_event_on_attack_shield(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            call_event_on_attack_shield(self, arg1.into()).into()
        }
    }

    pub fn call_event_on_attack_hit(&mut self) -> lib::L2CValue {
        unsafe {
            call_event_on_attack_hit(self).into()
        }
    }

    pub fn call_function_update_coroutine(&mut self) -> lib::L2CValue {
        unsafe {
            call_function_update_coroutine(self).into()
        }
    }

    pub fn call_function_change_action(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            call_function_change_action(self, arg1.into()).into()
        }
    }

    pub fn init_private_variables(&mut self) {
        unsafe {
            init_private_variables(self)
        }
    }

    pub fn regist_action(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            regist_action(self, arg1.into(), arg2.into())
        }
    }

    pub fn regist_action_override(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            regist_action_override(self, arg1.into(), arg2.into())
        }
    }

    pub fn global__COMMON__phase_sp_after(&mut self) -> lib::L2CValue {
        unsafe {
            global__COMMON__phase_sp_after(self).into()
        }
    }

    pub fn exit(&mut self) {
        unsafe {
            exit(self)
        }
    }

    pub fn local_func__phase_sp_after_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__phase_sp_after_1(self).into()
        }
    }

    pub fn local_func__phase_sp_after_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__phase_sp_after_2(self).into()
        }
    }

    pub fn local_func__phase_sp_after_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__phase_sp_after_3(self).into()
        }
    }

    pub fn global__COMMON__phase_r_jump(&mut self) -> lib::L2CValue {
        unsafe {
            global__COMMON__phase_r_jump(self).into()
        }
    }

    pub fn global__COMMON__function_set_goal(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            global__COMMON__function_set_goal(self, arg1.into()).into()
        }
    }

    pub fn local_func__phase_c_command_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__phase_c_command_1(self).into()
        }
    }

    pub fn private___COMMON__phase_command_end(&mut self) -> lib::L2CValue {
        unsafe {
            private___COMMON__phase_command_end(self).into()
        }
    }

    pub fn change_phase(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            change_phase(self, arg1.into(), arg2.into())
        }
    }

    pub fn private___COMMON__phase_command_start2(&mut self) -> lib::L2CValue {
        unsafe {
            private___COMMON__phase_command_start2(self).into()
        }
    }

    pub fn private___COMMON__phase_command_start(&mut self) -> lib::L2CValue {
        unsafe {
            private___COMMON__phase_command_start(self).into()
        }
    }

    pub fn private___COMMON__phase_button_or_command(&mut self) -> lib::L2CValue {
        unsafe {
            private___COMMON__phase_button_or_command(self).into()
        }
    }

    pub fn global__COMMON__function_change_phase_to_c_command(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            global__COMMON__function_change_phase_to_c_command(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn global__COMMON__function_sp_u_check(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            global__COMMON__function_sp_u_check(self, arg1.into()).into()
        }
    }

    pub fn local_func__function_sp_u_check_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__function_sp_u_check_1(self).into()
        }
    }

    pub fn global__COMMON__function_sp_u(&mut self) -> lib::L2CValue {
        unsafe {
            global__COMMON__function_sp_u(self).into()
        }
    }

    pub fn local_func__function_sp_u_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__function_sp_u_1(self).into()
        }
    }

    pub fn local_func__function_sp_u_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__function_sp_u_2(self).into()
        }
    }

    pub fn global__COMMON__function_r_jump_timing_check(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            global__COMMON__function_r_jump_timing_check(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn global__COMMON__function_r_jump_sp_u_check(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            global__COMMON__function_r_jump_sp_u_check(self, arg1.into()).into()
        }
    }

    pub fn local_func__function_r_jump_sp_u_check_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__function_r_jump_sp_u_check_1(self).into()
        }
    }

    pub fn global__COMMON__function_r_jump_exit_check(&mut self) -> lib::L2CValue {
        unsafe {
            global__COMMON__function_r_jump_exit_check(self).into()
        }
    }

    pub fn local_func__act_r_jump_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__act_r_jump_2(self).into()
        }
    }

    pub fn private___R_JUMP__phase_1(&mut self) -> lib::L2CValue {
        unsafe {
            private___R_JUMP__phase_1(self).into()
        }
    }

    pub fn private___R_JUMP__init(&mut self) -> lib::L2CValue {
        unsafe {
            private___R_JUMP__init(self).into()
        }
    }

    pub fn local_func__act_r_jump_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__act_r_jump_1(self).into()
        }
    }

    pub fn local_func__act_r_act_last_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__act_r_act_last_2(self).into()
        }
    }

    pub fn private___R_ACT_LAST__phase_2(&mut self) -> lib::L2CValue {
        unsafe {
            private___R_ACT_LAST__phase_2(self).into()
        }
    }

    pub fn private___R_ACT_LAST__phase_1(&mut self) -> lib::L2CValue {
        unsafe {
            private___R_ACT_LAST__phase_1(self).into()
        }
    }

    pub fn local_func__act_r_act_last_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__act_r_act_last_3(self).into()
        }
    }

    pub fn private___R_ACT_LAST__init(&mut self) -> lib::L2CValue {
        unsafe {
            private___R_ACT_LAST__init(self).into()
        }
    }

    pub fn local_func__act_r_act_last_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__act_r_act_last_1(self).into()
        }
    }

    pub fn local_func__act_c_ground_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__act_c_ground_1(self).into()
        }
    }

    pub fn private___C_GROUND__function_is_special_command(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__function_is_special_command(self, arg1.into()).into()
        }
    }

    pub fn private___C_GROUND__function_force_end_check(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__function_force_end_check(self).into()
        }
    }

    pub fn private___C_GROUND__function_hi_low_use_check(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__function_hi_low_use_check(self).into()
        }
    }

    pub fn private___C_GROUND__function_set_hi_low(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__function_set_hi_low(self).into()
        }
    }

    pub fn private___C_GROUND__function_set_stick(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__function_set_stick(self).into()
        }
    }

    pub fn private___C_GROUND__function_set_end_frame(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__function_set_end_frame(self).into()
        }
    }

    pub fn private___C_GROUND__function_set_key_frame(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__function_set_key_frame(self).into()
        }
    }

    pub fn private___C_GROUND__function_check_stat_air(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__function_check_stat_air(self).into()
        }
    }

    pub fn private___C_GROUND__phase_end_wait(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__phase_end_wait(self).into()
        }
    }

    pub fn private___C_GROUND__phase_hi_low_wait(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__phase_hi_low_wait(self).into()
        }
    }

    pub fn private___C_GROUND__phase_start(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__phase_start(self).into()
        }
    }

    pub fn private___C_GROUND__function_is_status_shield(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__function_is_status_shield(self).into()
        }
    }

    pub fn private___C_GROUND__phase_start_dash(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__phase_start_dash(self).into()
        }
    }

    pub fn private___C_GROUND__phase_check_lr(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__phase_check_lr(self).into()
        }
    }

    pub fn private___C_GROUND__phase_neutral(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__phase_neutral(self).into()
        }
    }

    pub fn private___C_GROUND__phase_init(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__phase_init(self).into()
        }
    }

    pub fn private___C_GROUND__init(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_GROUND__init(self).into()
        }
    }

    pub fn local_func__act_c_air_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__act_c_air_1(self).into()
        }
    }

    pub fn private___C_AIR__function_set_lr(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_AIR__function_set_lr(self).into()
        }
    }

    pub fn private___C_AIR__function_set_hi_low(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_AIR__function_set_hi_low(self).into()
        }
    }

    pub fn private___C_AIR__phase_lr_wait(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_AIR__phase_lr_wait(self).into()
        }
    }

    pub fn private___C_AIR__phase_end_wait(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_AIR__phase_end_wait(self).into()
        }
    }

    pub fn private___C_AIR__phase_hi_low_wait(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_AIR__phase_hi_low_wait(self).into()
        }
    }

    pub fn private___C_AIR__phase_start(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_AIR__phase_start(self).into()
        }
    }

    pub fn private___C_AIR__phase_pre_start(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_AIR__phase_pre_start(self).into()
        }
    }

    pub fn private___C_AIR__phase_init(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_AIR__phase_init(self).into()
        }
    }

    pub fn private___C_AIR__init(&mut self) -> lib::L2CValue {
        unsafe {
            private___C_AIR__init(self).into()
        }
    }

    pub fn private___A_FINAL__init(&mut self) -> lib::L2CValue {
        unsafe {
            private___A_FINAL__init(self).into()
        }
    }

    pub fn local_func__act_a_catch_atk_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__act_a_catch_atk_1(self).into()
        }
    }

    pub fn private___A_CATCH_ATK__function_after_wait(&mut self) -> lib::L2CValue {
        unsafe {
            private___A_CATCH_ATK__function_after_wait(self).into()
        }
    }

    pub fn private___A_CATCH_ATK__phase_right(&mut self) -> lib::L2CValue {
        unsafe {
            private___A_CATCH_ATK__phase_right(self).into()
        }
    }

    pub fn private___A_CATCH_ATK__phase_left(&mut self) -> lib::L2CValue {
        unsafe {
            private___A_CATCH_ATK__phase_left(self).into()
        }
    }

    pub fn private___A_CATCH_ATK__phase_bottom(&mut self) -> lib::L2CValue {
        unsafe {
            private___A_CATCH_ATK__phase_bottom(self).into()
        }
    }

    pub fn private___A_CATCH_ATK__phase_up(&mut self) -> lib::L2CValue {
        unsafe {
            private___A_CATCH_ATK__phase_up(self).into()
        }
    }

    pub fn private___A_CATCH_ATK__phase_back(&mut self) -> lib::L2CValue {
        unsafe {
            private___A_CATCH_ATK__phase_back(self).into()
        }
    }

    pub fn private___A_CATCH_ATK__phase_front(&mut self) -> lib::L2CValue {
        unsafe {
            private___A_CATCH_ATK__phase_front(self).into()
        }
    }

    pub fn private___A_CATCH_ATK__phase_throw(&mut self) -> lib::L2CValue {
        unsafe {
            private___A_CATCH_ATK__phase_throw(self).into()
        }
    }

    pub fn private___A_CATCH_ATK__function_get_padding_wait(&mut self) -> lib::L2CValue {
        unsafe {
            private___A_CATCH_ATK__function_get_padding_wait(self).into()
        }
    }

    pub fn private___A_CATCH_ATK__phase_attack(&mut self) -> lib::L2CValue {
        unsafe {
            private___A_CATCH_ATK__phase_attack(self).into()
        }
    }

    pub fn private___A_CATCH_ATK__phase_wait(&mut self) -> lib::L2CValue {
        unsafe {
            private___A_CATCH_ATK__phase_wait(self).into()
        }
    }

    pub fn private___A_CATCH_ATK__init(&mut self) -> lib::L2CValue {
        unsafe {
            private___A_CATCH_ATK__init(self).into()
        }
    }

    pub fn main_func__action_global_variables(&mut self) {
        unsafe {
            main_func__action_global_variables(self)
        }
    }

    pub fn main_func__action_private_variables(&mut self) {
        unsafe {
            main_func__action_private_variables(self)
        }
    }

    pub fn main_func__function_r_jump_exit_check(&mut self) {
        unsafe {
            main_func__function_r_jump_exit_check(self)
        }
    }

    pub fn main_func__function_r_jump_sp_u_check(&mut self) {
        unsafe {
            main_func__function_r_jump_sp_u_check(self)
        }
    }

    pub fn main_func__function_r_jump_timing_check(&mut self) {
        unsafe {
            main_func__function_r_jump_timing_check(self)
        }
    }

    pub fn main_func__function_set_goal(&mut self) {
        unsafe {
            main_func__function_set_goal(self)
        }
    }

    pub fn main_func__function_sp_u(&mut self) {
        unsafe {
            main_func__function_sp_u(self)
        }
    }

    pub fn main_func__function_sp_u_check(&mut self) {
        unsafe {
            main_func__function_sp_u_check(self)
        }
    }

    pub fn main_func__phase_r_jump(&mut self) {
        unsafe {
            main_func__phase_r_jump(self)
        }
    }

    pub fn main_func__phase_sp_after(&mut self) {
        unsafe {
            main_func__phase_sp_after(self)
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

    pub fn init_global_variables(&mut self) {
        unsafe {
            init_global_variables(self)
        }
    }

    pub fn function_phase_init_for_ground(&mut self) -> lib::L2CValue {
        unsafe {
            function_phase_init_for_ground(self).into()
        }
    }

    pub fn function_phase_init_wait_for_dash_cancel(&mut self) -> lib::L2CValue {
        unsafe {
            function_phase_init_wait_for_dash_cancel(self).into()
        }
    }

    pub fn function_phase_init_for_air(&mut self) -> lib::L2CValue {
        unsafe {
            function_phase_init_for_air(self).into()
        }
    }

    pub fn private___A_FINAL__phase_init(&mut self) -> lib::L2CValue {
        unsafe {
            private___A_FINAL__phase_init(self).into()
        }
    }
}

#[cfg(feature = "type_assert")]
impl L2CFighterAIActionBase {
    pub fn assert() {
        assert_eq!(size_of!(L2CFighterAIActionBase), 0x6C8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, ai_base), 0x0);
        assert_eq!(offset_of!(L2CFighterAIActionBase, global_table), 0x158);
        assert_eq!(offset_of!(L2CFighterAIActionBase, private_table), 0x168);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_a_goro), 0x178);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_a_catch_atk), 0x188);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_a_final), 0x198);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_r_jump), 0x1A8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_r_act_last), 0x1B8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_combo), 0x1C8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_ground), 0x1D8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_atk_n), 0x1E8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_atk_f), 0x1F8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_atk_u), 0x208);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_atk_d), 0x218);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_smash_f), 0x228);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_smash_u), 0x238);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_smash_d), 0x248);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_blow_n), 0x258);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_blow_f), 0x268);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_blow_u), 0x278);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_blow_d), 0x288);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_blow_catch), 0x298);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_none), 0x2A8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_air), 0x2B8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_atk_n), 0x2C8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_atk_f), 0x2D8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_atk_b), 0x2E8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_atk_u), 0x2F8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_atk_d), 0x308);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_blow_n), 0x318);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_blow_f), 0x328);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_blow_u), 0x338);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_blow_d), 0x348);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_atk2_n), 0x358);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_atk2_f), 0x368);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_atk2_u), 0x378);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_atk2_d), 0x388);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_dash_attack), 0x398);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_dash_catch), 0x3A8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_turn_catch), 0x3B8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_catch), 0x3C8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_brave_blow_f2), 0x3D8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_brave_blow_f3), 0x3E8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_brave_blow_f2), 0x3F8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_brave_blow_f3), 0x408);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_blow_b), 0x418);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_blow_super1), 0x428);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_blow_super2), 0x438);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_blow3_n), 0x448);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_blow3_f), 0x458);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_blow3_b), 0x468);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_blow3_u), 0x478);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_blow3_d), 0x488);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_blow_b), 0x498);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_blow3_n), 0x4A8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_blow3_f), 0x4B8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_blow3_b), 0x4C8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_blow3_u), 0x4D8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_blow3_d), 0x4E8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_master_blow_f2), 0x4F8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_master_blow_f2), 0x508);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_tantan_atk_f_r), 0x518);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_tantan_smash_f_r), 0x528);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_tantan_atk_f2), 0x538);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_tantan_atk_b2), 0x548);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_tantan_atk_n_r), 0x558);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_tantan_atk_f_r), 0x568);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_tantan_atk_b_r), 0x578);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_tantan_atk_f_r2), 0x588);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_tantan_atk_b_r2), 0x598);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_pickel_blow_f2), 0x5A8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_a_pickel_blow_f2), 0x5B8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_stand_1), 0x5C8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_stand_2), 0x5D8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_stand_3), 0x5E8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_stand_4), 0x5F8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_stand_5), 0x608);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_stand_6), 0x618);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_squat_1), 0x628);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_squat_2), 0x638);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_squat_3), 0x648);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_squat_4), 0x658);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_catch), 0x668);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_step_2), 0x678);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_step_2f), 0x688);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_step_2l), 0x698);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_step_2s), 0x6A8);
        assert_eq!(offset_of!(L2CFighterAIActionBase, act_c_g_demon_atk_rage), 0x6B8);
    }
}