use std::ops::{Deref, DerefMut};

use crate::*;

use super::cpp::*;

#[repr(C)]
pub struct L2CFighterAIBase {
    agent_base: lua2cpp::L2CAgentBase,
    pub vector_methods:  lib::L2CValue, // dot, cross, length, length_square, normalize, distance_square, distance, and 2d variants
    pub constants:       lib::L2CValue,
    pub negative_one:    lib::L2CValue,
    pub zero:            lib::L2CValue,
    pub one:             lib::L2CValue,
    pub ninety:          lib::L2CValue,
    pub negative_nintey: lib::L2CValue,
    pub ninety_again:    lib::L2CValue,
    pub three_halves:    lib::L2CValue,
}

impl Deref for L2CFighterAIBase {
    type Target = lua2cpp::L2CAgentBase;

    fn deref(&self) -> &Self::Target {
        &self.agent_base
    }
}

impl DerefMut for L2CFighterAIBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.agent_base
    }
}

impl L2CFighterAIBase {
    pub fn main_func__constant_variables(&mut self) {
        unsafe {
            main_func__constant_variables(self)
        }
    }

    pub fn main_func__utility_1(&mut self) {
        unsafe {
            main_func__utility_1(self)
        }
    }

    pub fn vector__distance(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            vector__distance(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into()).into()
        }
    }

    pub fn vector__distance_square(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            vector__distance_square(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into()).into()
        }
    }

    pub fn vector__distance_2d(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            vector__distance_2d(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn vector__distance_square_2d(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            vector__distance_square_2d(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn vector__normalize(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            vector__normalize(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn vector__length_square(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            vector__length_square(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn vector__normalize_2d(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            vector__normalize_2d(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn vector__length_square_2d(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            vector__length_square_2d(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn vector__length(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            vector__length(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn vector__length_2d(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            vector__length_2d(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn vector__cross(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            vector__cross(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into()).into()
        }
    }

    pub fn vector__cross_2d(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            vector__cross_2d(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn vector__dot(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue, arg6: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            vector__dot(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into(), arg6.into()).into()
        }
    }

    pub fn vector__dot_2d(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            vector__dot_2d(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn local_func__constant_variables_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__constant_variables_1(self).into()
        }
    }

    pub fn yield_(&mut self) {
        unsafe {
            yield_(self)
        }
    }

    pub fn wait(&mut self, arg1: lib::L2CValue) {
        unsafe {
            wait(self, arg1.into())
        }
    }

    pub fn wait_random(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            wait_random(self, arg1.into(), arg2.into())
        }
    }

    pub fn wait_until(&mut self, arg1: lib::L2CValue) {
        unsafe {
            wait_until(self, arg1.into())
        }
    }

    pub fn add_stick(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            add_stick(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn reset_stick(&mut self, arg1: lib::L2CValue) {
        unsafe {
            reset_stick(self, arg1.into())
        }
    }

    pub fn add_stick_abs(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) {
        unsafe {
            add_stick_abs(self, arg1.into(), arg2.into(), arg3.into())
        }
    }

    pub fn add_stick_deg(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            add_stick_deg(self, arg1.into(), arg2.into())
        }
    }

    pub fn add_button(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            add_button(self, arg1.into(), arg2.into())
        }
    }

    pub fn is_1on1_auto_turn_fighter(&mut self) -> lib::L2CValue {
        unsafe {
            is_1on1_auto_turn_fighter(self).into()
        }
    }

    pub fn is_status_kind_walk(&mut self) -> lib::L2CValue {
        unsafe {
            is_status_kind_walk(self).into()
        }
    }

    pub fn is_status_kind_dash_start(&mut self) -> lib::L2CValue {
        unsafe {
            is_status_kind_dash_start(self).into()
        }
    }

    pub fn is_status_kind_dash_f_start(&mut self) -> lib::L2CValue {
        unsafe {
            is_status_kind_dash_f_start(self).into()
        }
    }

    pub fn is_status_kind_dash_b_start(&mut self) -> lib::L2CValue {
        unsafe {
            is_status_kind_dash_b_start(self).into()
        }
    }

    pub fn is_status_kind_run(&mut self) -> lib::L2CValue {
        unsafe {
            is_status_kind_run(self).into()
        }
    }

    pub fn is_status_kind_escape_able(&mut self) {
        unsafe {
            is_status_kind_escape_able(self)
        }
    }

    pub fn is_status_kind_escape_lr_able(&mut self) -> lib::L2CValue {
        unsafe {
            is_status_kind_escape_lr_able(self).into()
        }
    }

    pub fn is_absolutely_falling(&mut self) -> lib::L2CValue {
        unsafe {
            is_absolutely_falling(self).into()
        }
    }

    pub fn rank_rate(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            rank_rate(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn rank_rate_with_range(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            rank_rate_with_range(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn cp_level_to_cp_rank(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            cp_level_to_cp_rank(self, arg1.into()).into()
        }
    }

    pub fn check_range_from_floor(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            check_range_from_floor(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn local_func__utility_1_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__utility_1_1(self).into()
        }
    }

    pub fn is_target_waiting_you(&mut self) -> lib::L2CValue {
        unsafe {
            is_target_waiting_you(self).into()
        }
    }

    pub fn is_wire_available(&mut self) -> lib::L2CValue {
        unsafe {
            is_wire_available(self).into()
        }
    }

    pub fn local_func__utility_1_2(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__utility_1_2(self).into()
        }
    }

    pub fn local_func__utility_1_3(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__utility_1_3(self).into()
        }
    }

    pub fn local_func__utility_1_4(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__utility_1_4(self).into()
        }
    }

    pub fn local_func__utility_1_5(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__utility_1_5(self).into()
        }
    }

    pub fn is_wire_return(&mut self) -> lib::L2CValue {
        unsafe {
            is_wire_return(self).into()
        }
    }

    pub fn common_return_set_goal(&mut self, arg1: lib::L2CValue) {
        unsafe {
            common_return_set_goal(self, arg1.into())
        }
    }

    pub fn is_interrupt_sp_u_available(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_interrupt_sp_u_available(self, arg1.into()).into()
        }
    }

    pub fn is_equal(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_equal(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn is_zero(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_zero(self, arg1.into()).into()
        }
    }

    pub fn lerp(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            lerp(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn sign(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sign(self, arg1.into()).into()
        }
    }

    pub fn clamp(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            clamp(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn random_choice(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            random_choice(self, arg1.into()).into()
        }
    }
}

#[cfg(feature = "type_assert")]
impl L2CFighterAIBase {
    pub fn assert() {
        assert_eq!(size_of!(L2CFighterAIBase), 0x158);
        assert_eq!(offset_of!(L2CFighterAIBase, agent_base),      0x0);
        assert_eq!(offset_of!(L2CFighterAIBase, vector_methods),  0xC8);
        assert_eq!(offset_of!(L2CFighterAIBase, constants),       0xD8);
        assert_eq!(offset_of!(L2CFighterAIBase, negative_one),    0xE8);
        assert_eq!(offset_of!(L2CFighterAIBase, zero),            0xF8);
        assert_eq!(offset_of!(L2CFighterAIBase, one),             0x108);
        assert_eq!(offset_of!(L2CFighterAIBase, ninety),          0x118);
        assert_eq!(offset_of!(L2CFighterAIBase, negative_nintey), 0x128);
        assert_eq!(offset_of!(L2CFighterAIBase, ninety_again),    0x138);
        assert_eq!(offset_of!(L2CFighterAIBase, three_halves),    0x148);
    }
}