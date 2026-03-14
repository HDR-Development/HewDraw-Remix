use crate::*;
use super::class::*;

extern "C" {
    #[link_name = "_ZN7lua2cpp14L2CFighterBase18main_func__vector2Ev"]
    pub(super) fn main_func__vector2(this: *mut L2CFighterBase);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase18main_func__vector3Ev"]
    pub(super) fn main_func__vector3(this: *mut L2CFighterBase);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase18main_func__vector4Ev"]
    pub(super) fn main_func__vector4(this: *mut L2CFighterBase);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase31main_func__base_global_variableEv"]
    pub(super) fn main_func__base_global_variable(this: *mut L2CFighterBase);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase5ENTRYEv"]
    pub(super) fn ENTRY(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase10begin_lineEN3lib8L2CValueES2_"]
    pub(super) fn begin_line(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase5shiftEN3lib8L2CValueE"]
    pub(super) fn shift(this: *mut L2CFighterBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase32line_state__set_value_from_indexEN3lib8L2CValueES2_"]
    pub(super) fn line_state__set_value_from_index(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase27sys_line_status_system_initEv"]
    pub(super) fn sys_line_status_system_init(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase30sys_line_status_system_controlEv"]
    pub(super) fn sys_line_status_system_control(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase26sys_line_status_shift_initEv"]
    pub(super) fn sys_line_status_shift_init(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase29sys_line_status_shift_controlEv"]
    pub(super) fn sys_line_status_shift_control(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase18sub_pre_status_mscEN3lib8L2CValueE"]
    pub(super) fn sub_pre_status_msc(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase18sub_set_status_mscEN3lib8L2CValueE"]
    pub(super) fn sub_set_status_msc(this: *mut L2CFighterBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase24sys_line_status_end_initEv"]
    pub(super) fn sys_line_status_end_init(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase5SETUPEv"]
    pub(super) fn SETUP(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase16line_state__initEv"]
    pub(super) fn line_state__init(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase11global_initEv"]
    pub(super) fn global_init(this: *mut L2CFighterBase);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase34local_func__base_global_variable_1Ev"]
    pub(super) fn local_func__base_global_variable_1(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase32line_state__get_value_from_indexEN3lib8L2CValueE"]
    pub(super) fn line_state__get_value_from_index(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase16Vector4_mt____eqEN3lib8L2CValueES2_"]
    pub(super) fn Vector4_mt____eq(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector4_mt____unmEN3lib8L2CValueE"]
    pub(super) fn Vector4_mt____unm(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase15Vector4__createEN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn Vector4__create(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase21local_func__vector4_1EN3lib8L2CValueES2_S2_S2_"]
    pub(super) fn local_func__vector4_1(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector4_mt____divEN3lib8L2CValueES2_"]
    pub(super) fn Vector4_mt____div(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector4_mt____mulEN3lib8L2CValueES2_"]
    pub(super) fn Vector4_mt____mul(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector4_mt____subEN3lib8L2CValueES2_"]
    pub(super) fn Vector4_mt____sub(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector4_mt____addEN3lib8L2CValueES2_"]
    pub(super) fn Vector4_mt____add(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase16Vector3_mt____eqEN3lib8L2CValueES2_"]
    pub(super) fn Vector3_mt____eq(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector3_mt____unmEN3lib8L2CValueE"]
    pub(super) fn Vector3_mt____unm(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase15Vector3__createEN3lib8L2CValueES2_S2_"]
    pub(super) fn Vector3__create(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase21local_func__vector3_1EN3lib8L2CValueES2_S2_"]
    pub(super) fn local_func__vector3_1(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector3_mt____divEN3lib8L2CValueES2_"]
    pub(super) fn Vector3_mt____div(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector3_mt____mulEN3lib8L2CValueES2_"]
    pub(super) fn Vector3_mt____mul(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector3_mt____subEN3lib8L2CValueES2_"]
    pub(super) fn Vector3_mt____sub(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector3_mt____addEN3lib8L2CValueES2_"]
    pub(super) fn Vector3_mt____add(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector2_mt____unmEN3lib8L2CValueE"]
    pub(super) fn Vector2_mt____unm(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase15Vector2__createEN3lib8L2CValueES2_"]
    pub(super) fn Vector2__create(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase21local_func__vector2_1EN3lib8L2CValueES2_"]
    pub(super) fn local_func__vector2_1(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector2_mt____divEN3lib8L2CValueES2_"]
    pub(super) fn Vector2_mt____div(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector2_mt____mulEN3lib8L2CValueES2_"]
    pub(super) fn Vector2_mt____mul(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector2_mt____subEN3lib8L2CValueES2_"]
    pub(super) fn Vector2_mt____sub(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector2_mt____addEN3lib8L2CValueES2_"]
    pub(super) fn Vector2_mt____add(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase20sys_line_system_initEv"]
    pub(super) fn sys_line_system_init(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase21sub_begin_added_linesEN3lib8L2CValueE"]
    pub(super) fn sub_begin_added_lines(this: *mut L2CFighterBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase27sys_line_status_end_controlEv"]
    pub(super) fn sys_line_status_end_control(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase19sub_end_added_linesEv"]
    pub(super) fn sub_end_added_lines(this: *mut L2CFighterBase);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase5RESETEv"]
    pub(super) fn RESET(this: *mut L2CFighterBase);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase12global_resetEv"]
    pub(super) fn global_reset(this: *mut L2CFighterBase);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase26sub_end_added_lines_commonEv"]
    pub(super) fn sub_end_added_lines_common(this: *mut L2CFighterBase);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase8end_lineEN3lib8L2CValueE"]
    pub(super) fn end_line(this: *mut L2CFighterBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase18sub_end_status_mscEN3lib8L2CValueE"]
    pub(super) fn sub_end_status_msc(this: *mut L2CFighterBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase28sub_begin_added_lines_commonEN3lib8L2CValueE"]
    pub(super) fn sub_begin_added_lines_common(this: *mut L2CFighterBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase23sys_line_system_controlEv"]
    pub(super) fn sys_line_system_control(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase5clampEN3lib8L2CValueES2_S2_"]
    pub(super) fn clamp(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase4lerpEN3lib8L2CValueES2_S2_"]
    pub(super) fn lerp(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase4signEN3lib8L2CValueE"]
    pub(super) fn sign(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase16Vector2_mt____eqEN3lib8L2CValueES2_"]
    pub(super) fn Vector2_mt____eq(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase12Vector2__dotEN3lib8L2CValueES2_"]
    pub(super) fn Vector2__dot(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase14Vector2__crossEN3lib8L2CValueES2_"]
    pub(super) fn Vector2__cross(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase22Vector2__length_squareEN3lib8L2CValueE"]
    pub(super) fn Vector2__length_square(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase15Vector2__lengthEN3lib8L2CValueE"]
    pub(super) fn Vector2__length(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase18Vector2__normalizeEN3lib8L2CValueE"]
    pub(super) fn Vector2__normalize(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase24Vector2__distance_squareEN3lib8L2CValueES2_"]
    pub(super) fn Vector2__distance_square(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector2__distanceEN3lib8L2CValueES2_"]
    pub(super) fn Vector2__distance(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase11Vector2__xyEN3lib8L2CValueE"]
    pub(super) fn Vector2__xy(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase12Vector3__dotEN3lib8L2CValueES2_"]
    pub(super) fn Vector3__dot(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase14Vector3__crossEN3lib8L2CValueES2_"]
    pub(super) fn Vector3__cross(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase22Vector3__length_squareEN3lib8L2CValueE"]
    pub(super) fn Vector3__length_square(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase15Vector3__lengthEN3lib8L2CValueE"]
    pub(super) fn Vector3__length(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase18Vector3__normalizeEN3lib8L2CValueE"]
    pub(super) fn Vector3__normalize(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase24Vector3__distance_squareEN3lib8L2CValueES2_"]
    pub(super) fn Vector3__distance_square(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector3__distanceEN3lib8L2CValueES2_"]
    pub(super) fn Vector3__distance(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase28Vector3__create_bezier_curveEN3lib8L2CValueES2_S2_S2_S2_"]
    pub(super) fn Vector3__create_bezier_curve(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack, arg3: lib::L2CValueHack, arg4: lib::L2CValueHack, arg5: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase12Vector3__xyzEN3lib8L2CValueE"]
    pub(super) fn Vector3__xyz(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase12Vector4__dotEN3lib8L2CValueES2_"]
    pub(super) fn Vector4__dot(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase22Vector4__length_squareEN3lib8L2CValueE"]
    pub(super) fn Vector4__length_square(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase15Vector4__lengthEN3lib8L2CValueE"]
    pub(super) fn Vector4__length(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase18Vector4__normalizeEN3lib8L2CValueE"]
    pub(super) fn Vector4__normalize(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase24Vector4__distance_squareEN3lib8L2CValueES2_"]
    pub(super) fn Vector4__distance_square(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase17Vector4__distanceEN3lib8L2CValueES2_"]
    pub(super) fn Vector4__distance(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase13Vector4__xyzwEN3lib8L2CValueE"]
    pub(super) fn Vector4__xyzw(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase16call_line_systemEv"]
    pub(super) fn call_line_system(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase21call_line_system_postEv"]
    pub(super) fn call_line_system_post(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase16call_line_statusEv"]
    pub(super) fn call_line_status(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase23call_line_status_systemEv"]
    pub(super) fn call_line_status_system(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase20call_line_status_endEv"]
    pub(super) fn call_line_status_end(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase22call_line_status_shiftEv"]
    pub(super) fn call_line_status_shift(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase20call_line_fix_cameraEv"]
    pub(super) fn call_line_fix_camera(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase24call_line_map_correctionEv"]
    pub(super) fn call_line_map_correction(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase22call_line_fix_pos_slowEv"]
    pub(super) fn call_line_fix_pos_slow(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase24call_line_waza_customizeEv"]
    pub(super) fn call_line_waza_customize(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase9fastshiftEN3lib8L2CValueE"]
    pub(super) fn fastshift(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase15is_line_runningEN3lib8L2CValueE"]
    pub(super) fn is_line_running(this: *mut L2CFighterBase, arg1: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase13change_statusEN3lib8L2CValueES2_"]
    pub(super) fn change_status(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase20change_status_commonEN3lib8L2CValueES2_"]
    pub(super) fn change_status_common(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase13set_situationEN3lib8L2CValueE"]
    pub(super) fn set_situation(this: *mut L2CFighterBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase18set_situation_keepEN3lib8L2CValueES2_"]
    pub(super) fn set_situation_keep(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp14L2CFighterBase22sub_situation_passibleEv"]
    pub(super) fn sub_situation_passible(this: *mut L2CFighterBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp14L2CFighterBase14get_stick_rateEN3lib8L2CValueES2_"]
    pub(super) fn get_stick_rate(this: *mut L2CFighterBase, arg1: lib::L2CValueHack, arg2: lib::L2CValueHack) -> lib::L2CValueHack;
}