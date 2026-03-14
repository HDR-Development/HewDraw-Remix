use std::ops::{Deref, DerefMut};

use crate::*;

use super::cpp::*;

#[repr(C)]
pub struct L2CFighterBase {
    agent_base: lua2cpp::L2CAgentBase,
    pub global_table: lib::L2CValue,
    pub line_state: lib::L2CValue,
    pub vector2_metatable: lib::L2CValue,
    pub vector3_metatable: lib::L2CValue,
    pub vector4_metatable: lib::L2CValue
}

impl Deref for L2CFighterBase {
    type Target = lua2cpp::L2CAgentBase;

    fn deref(&self) -> &Self::Target {
        &self.agent_base
    }
}

impl DerefMut for L2CFighterBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.agent_base
    }
}

impl L2CFighterBase {
    pub fn main_func__vector2(&mut self) {
        unsafe {
            main_func__vector2(self)
        }
    }

    pub fn main_func__vector3(&mut self) {
        unsafe {
            main_func__vector3(self)
        }
    }

    pub fn main_func__vector4(&mut self) {
        unsafe {
            main_func__vector4(self)
        }
    }

    pub fn main_func__base_global_variable(&mut self) {
        unsafe {
            main_func__base_global_variable(self)
        }
    }

    pub fn ENTRY(&mut self) -> lib::L2CValue {
        unsafe {
            ENTRY(self).into()
        }
    }

    pub fn begin_line(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            begin_line(self, arg1.into(), arg2.into())
        }
    }

    pub fn shift(&mut self, arg1: lib::L2CValue) {
        unsafe {
            shift(self, arg1.into())
        }
    }

    pub fn line_state__set_value_from_index(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            line_state__set_value_from_index(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn sys_line_status_system_init(&mut self) -> lib::L2CValue {
        unsafe {
            sys_line_status_system_init(self).into()
        }
    }

    pub fn sys_line_status_system_control(&mut self) -> lib::L2CValue {
        unsafe {
            sys_line_status_system_control(self).into()
        }
    }

    pub fn sys_line_status_shift_init(&mut self) -> lib::L2CValue {
        unsafe {
            sys_line_status_shift_init(self).into()
        }
    }

    pub fn sys_line_status_shift_control(&mut self) -> lib::L2CValue {
        unsafe {
            sys_line_status_shift_control(self).into()
        }
    }

    pub fn sub_pre_status_msc(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            sub_pre_status_msc(self, arg1.into()).into()
        }
    }

    pub fn sub_set_status_msc(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_set_status_msc(self, arg1.into())
        }
    }

    pub fn sys_line_status_end_init(&mut self) -> lib::L2CValue {
        unsafe {
            sys_line_status_end_init(self).into()
        }
    }

    pub fn SETUP(&mut self) -> lib::L2CValue {
        unsafe {
            SETUP(self).into()
        }
    }

    pub fn line_state__init(&mut self) -> lib::L2CValue {
        unsafe {
            line_state__init(self).into()
        }
    }

    pub fn global_init(&mut self) {
        unsafe {
            global_init(self)
        }
    }

    pub fn local_func__base_global_variable_1(&mut self) -> lib::L2CValue {
        unsafe {
            local_func__base_global_variable_1(self).into()
        }
    }

    pub fn line_state__get_value_from_index(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            line_state__get_value_from_index(self, arg1.into()).into()
        }
    }

    pub fn Vector4_mt____eq(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector4_mt____eq(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector4_mt____unm(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector4_mt____unm(self, arg1.into()).into()
        }
    }

    pub fn Vector4__create(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector4__create(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn local_func__vector4_1(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__vector4_1(self, arg1.into(), arg2.into(), arg3.into(), arg4.into()).into()
        }
    }

    pub fn Vector4_mt____div(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector4_mt____div(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector4_mt____mul(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector4_mt____mul(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector4_mt____sub(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector4_mt____sub(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector4_mt____add(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector4_mt____add(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector3_mt____eq(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3_mt____eq(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector3_mt____unm(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3_mt____unm(self, arg1.into()).into()
        }
    }

    pub fn Vector3__create(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3__create(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn local_func__vector3_1(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__vector3_1(self, arg1.into(), arg2.into(), arg3.into()).into()
        }
    }

    pub fn Vector3_mt____div(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3_mt____div(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector3_mt____mul(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3_mt____mul(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector3_mt____sub(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3_mt____sub(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector3_mt____add(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3_mt____add(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector2_mt____unm(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector2_mt____unm(self, arg1.into()).into()
        }
    }

    pub fn Vector2__create(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector2__create(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn local_func__vector2_1(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            local_func__vector2_1(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector2_mt____div(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector2_mt____div(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector2_mt____mul(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector2_mt____mul(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector2_mt____sub(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector2_mt____sub(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector2_mt____add(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector2_mt____add(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn sys_line_system_init(&mut self) -> lib::L2CValue {
        unsafe {
            sys_line_system_init(self).into()
        }
    }

    pub fn sub_begin_added_lines(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_begin_added_lines(self, arg1.into())
        }
    }

    pub fn sys_line_status_end_control(&mut self) -> lib::L2CValue {
        unsafe {
            sys_line_status_end_control(self).into()
        }
    }

    pub fn sub_end_added_lines(&mut self) {
        unsafe {
            sub_end_added_lines(self)
        }
    }

    pub fn RESET(&mut self) {
        unsafe {
            RESET(self)
        }
    }

    pub fn global_reset(&mut self) {
        unsafe {
            global_reset(self)
        }
    }

    pub fn sub_end_added_lines_common(&mut self) {
        unsafe {
            sub_end_added_lines_common(self)
        }
    }

    pub fn end_line(&mut self, arg1: lib::L2CValue) {
        unsafe {
            end_line(self, arg1.into())
        }
    }

    pub fn sub_end_status_msc(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_end_status_msc(self, arg1.into())
        }
    }

    pub fn sub_begin_added_lines_common(&mut self, arg1: lib::L2CValue) {
        unsafe {
            sub_begin_added_lines_common(self, arg1.into())
        }
    }

    pub fn sys_line_system_control(&mut self) -> lib::L2CValue {
        unsafe {
            sys_line_system_control(self).into()
        }
    }

    pub fn clamp(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            clamp(self, arg1.into(), arg2.into(), arg3.into()).into()
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

    pub fn Vector2_mt____eq(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector2_mt____eq(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector2__dot(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector2__dot(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector2__cross(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector2__cross(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector2__length_square(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector2__length_square(self, arg1.into()).into()
        }
    }

    pub fn Vector2__length(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector2__length(self, arg1.into()).into()
        }
    }

    pub fn Vector2__normalize(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector2__normalize(self, arg1.into()).into()
        }
    }

    pub fn Vector2__distance_square(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector2__distance_square(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector2__distance(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector2__distance(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector2__xy(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector2__xy(self, arg1.into()).into()
        }
    }

    pub fn Vector3__dot(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3__dot(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector3__cross(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3__cross(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector3__length_square(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3__length_square(self, arg1.into()).into()
        }
    }

    pub fn Vector3__length(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3__length(self, arg1.into()).into()
        }
    }

    pub fn Vector3__normalize(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3__normalize(self, arg1.into()).into()
        }
    }

    pub fn Vector3__distance_square(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3__distance_square(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector3__distance(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3__distance(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector3__create_bezier_curve(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue, arg3: lib::L2CValue, arg4: lib::L2CValue, arg5: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3__create_bezier_curve(self, arg1.into(), arg2.into(), arg3.into(), arg4.into(), arg5.into()).into()
        }
    }

    pub fn Vector3__xyz(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector3__xyz(self, arg1.into()).into()
        }
    }

    pub fn Vector4__dot(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector4__dot(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector4__length_square(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector4__length_square(self, arg1.into()).into()
        }
    }

    pub fn Vector4__length(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector4__length(self, arg1.into()).into()
        }
    }

    pub fn Vector4__normalize(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector4__normalize(self, arg1.into()).into()
        }
    }

    pub fn Vector4__distance_square(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector4__distance_square(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector4__distance(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector4__distance(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn Vector4__xyzw(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            Vector4__xyzw(self, arg1.into()).into()
        }
    }

    pub fn call_line_system(&mut self) -> lib::L2CValue {
        unsafe {
            call_line_system(self).into()
        }
    }

    pub fn call_line_system_post(&mut self) -> lib::L2CValue {
        unsafe {
            call_line_system_post(self).into()
        }
    }

    pub fn call_line_status(&mut self) -> lib::L2CValue {
        unsafe {
            call_line_status(self).into()
        }
    }

    pub fn call_line_status_system(&mut self) -> lib::L2CValue {
        unsafe {
            call_line_status_system(self).into()
        }
    }

    pub fn call_line_status_end(&mut self) -> lib::L2CValue {
        unsafe {
            call_line_status_end(self).into()
        }
    }

    pub fn call_line_status_shift(&mut self) -> lib::L2CValue {
        unsafe {
            call_line_status_shift(self).into()
        }
    }

    pub fn call_line_fix_camera(&mut self) -> lib::L2CValue {
        unsafe {
            call_line_fix_camera(self).into()
        }
    }

    pub fn call_line_map_correction(&mut self) -> lib::L2CValue {
        unsafe {
            call_line_map_correction(self).into()
        }
    }

    pub fn call_line_fix_pos_slow(&mut self) -> lib::L2CValue {
        unsafe {
            call_line_fix_pos_slow(self).into()
        }
    }

    pub fn call_line_waza_customize(&mut self) -> lib::L2CValue {
        unsafe {
            call_line_waza_customize(self).into()
        }
    }

    pub fn fastshift(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            fastshift(self, arg1.into()).into()
        }
    }

    pub fn is_line_running(&mut self, arg1: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            is_line_running(self, arg1.into()).into()
        }
    }

    pub fn change_status(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            change_status(self, arg1.into(), arg2.into())
        }
    }

    pub fn change_status_common(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            change_status_common(self, arg1.into(), arg2.into()).into()
        }
    }

    pub fn set_situation(&mut self, arg1: lib::L2CValue) {
        unsafe {
            set_situation(self, arg1.into())
        }
    }

    pub fn set_situation_keep(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) {
        unsafe {
            set_situation_keep(self, arg1.into(), arg2.into())
        }
    }

    pub fn sub_situation_passible(&mut self) -> lib::L2CValue {
        unsafe {
            sub_situation_passible(self).into()
        }
    }

    pub fn get_stick_rate(&mut self, arg1: lib::L2CValue, arg2: lib::L2CValue) -> lib::L2CValue {
        unsafe {
            get_stick_rate(self, arg1.into(), arg2.into()).into()
        }
    }
}

#[cfg(feature = "type_assert")]
impl L2CFighterBase {
    pub fn assert() {
        assert_eq!(size_of!(L2CFighterBase), 0x118);
        assert_eq!(offset_of!(L2CFighterBase, global_table), 0xC8);
        assert_eq!(offset_of!(L2CFighterBase, line_state), 0xD8);
        assert_eq!(offset_of!(L2CFighterBase, vector2_metatable), 0xE8);
        assert_eq!(offset_of!(L2CFighterBase, vector3_metatable), 0xF8);
        assert_eq!(offset_of!(L2CFighterBase, vector4_metatable), 0x108);
    }
}