use super::*;
use smash::lib::L2CAgent;

extern "C" {
    #[link_name = "\u{1}_ZN3app9holywater35HOLYWATER_FIRE_PILLAR_GRAVITY_ACCELENS_11FighterKindE"]
    pub fn FIRE_PILLAR_GRAVITY_ACCEL(kind: FighterKind) -> f32;

    #[link_name = "\u{1}_ZN3app9holywater39HOLYWATER_FIRE_PILLAR_GRAVITY_ACCEL_MAXENS_11FighterKindE"]
    pub fn FIRE_PILLAR_GRAVITY_ACCEL_MAX(kind: FighterKind) -> f32;

    #[link_name = "\u{1}_ZN3app9holywater29HOLYWATER_FIRE_PILLAR_SPEED_YENS_11FighterKindE"]
    pub fn FIRE_PILLAR_SPEED_Y(kind: FighterKind) -> f32;

    #[link_name = "\u{1}_ZN3app4item12disable_areaEP9lua_Statei"]
    pub fn ITEM_DISABLE_AREA(lua_state: u64, area_kind: i32);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_gravity9set_accelEP9lua_Statef"]
    pub fn KINETIC_ENERGY_GRAVITY_SET_ACCEL(lua_state: u64, accel: f32);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_gravity15set_limit_speedEP9lua_Statef"]
    pub fn KINETIC_ENERGY_GRAVITY_SET_LIMIT_SPEED(lua_state: u64, accel: f32);

    #[link_name = "\u{1}_ZN3app26kinetic_energy_control_rot12set_rotationEP9lua_StateRKN3phx8Vector3fE"]
    pub fn KINETIC_ENERGY_CONTROL_ROT_SET_ROTATION(lua_state: u64, rotation: *const Vector3f);

    #[link_name = "\u{1}_ZN3app18kinetic_energy_rot12set_rotationEP9lua_StateRKN3phx8Vector3fE"]
    pub fn KINETIC_ENERGY_ROT_SET_ROTATION(lua_state: u64, rotation: *const Vector3f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control9set_accelEP9lua_StateRKN3phx8Vector2fE"]
    pub fn KINETIC_ENERGY_CONTROL_SET_ACCEL(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control9set_brakeEP9lua_StateRKN3phx8Vector2fE"]
    pub fn KINETIC_ENERGY_CONTROL_SET_BRAKE(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control16set_stable_speedEP9lua_StateRKN3phx8Vector2fE"]
    pub fn KINETIC_ENERGY_CONTROL_SET_STABLE_SPEED(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control15set_limit_speedEP9lua_StateRKN3phx8Vector2fE"]
    pub fn KINETIC_ENERGY_CONTROL_SET_LIMIT_SPEED(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control9set_speedEP9lua_StateRKN3phx8Vector2fE"]
    pub fn KINETIC_ENERGY_CONTROL_SET_SPEED(lua_state: u64, accel: *const Vector2f);

    #[link_name = "\u{1}_ZN3app22kinetic_energy_control6enableEP9lua_State"]
    pub fn KINETIC_ENERGY_CONTROL_ENABLE(lua_state: u64);
}

#[no_mangle]
unsafe extern "C" fn richter_holywater_born_inner(item: &mut L2CAgent) -> L2CValue {
    // (item.unk20 as L2CValue)[0x1257816e00 as u64].assign(&L2CValue::I32(0));
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_BODY);
    ITEM_DISABLE_AREA(item.lua_state_agent, *ITEM_AREA_KIND_BODY);
    item.clear_lua_stack();
    lua_args!(item, ITEM_AREA_KIND_PICKUP);
    ITEM_DISABLE_AREA(item.lua_state_agent, *ITEM_AREA_KIND_PICKUP);
    HitModule::set_whole(item.module_accessor, HitStatus(*HIT_STATUS_OFF), 0);
    WorkModule::off_flag(item.module_accessor, *ITEM_INSTANCE_WORK_FLAG_AUTO_PLAY_LOST_EFFECT);
    KineticModule::clear_speed_all(item.module_accessor);
    let kind = richter_holywater_something(item).get_i32();
    let gravity_accel = FIRE_PILLAR_GRAVITY_ACCEL(FighterKind(kind));
    // println!("gravity: {}", -gravity_accel);
    item.clear_lua_stack();
    lua_args!(item, -gravity_accel);
    KINETIC_ENERGY_GRAVITY_SET_ACCEL(item.lua_state_agent, -gravity_accel);
    let gravity_accel_max = FIRE_PILLAR_GRAVITY_ACCEL_MAX(FighterKind(kind));
    // println!("gravity max: {}", gravity_accel_max);
    item.clear_lua_stack();
    lua_args!(item, gravity_accel_max);
    KINETIC_ENERGY_GRAVITY_SET_LIMIT_SPEED(item.lua_state_agent, gravity_accel_max);
    item.clear_lua_stack();
    lua_args!(item, 0, 0, 0);
    KINETIC_ENERGY_CONTROL_ROT_SET_ROTATION(item.lua_state_agent, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
    item.clear_lua_stack();
    lua_args!(item, 0, 0, 0);
    KINETIC_ENERGY_ROT_SET_ROTATION(item.lua_state_agent, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
    if !GroundModule::is_touch(item.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        let speed_y = FIRE_PILLAR_SPEED_Y(FighterKind(kind));
        // println!("speed y: {}", speed_y);
        KineticModule::add_speed(item.module_accessor, &Vector3f{x: 0.0, y: speed_y, z: 0.0});
    }
    // <ported from WuBor>
    if !GroundModule::is_touch(item.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32) {
        let normal_x = GroundModule::get_touch_normal_x(item.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let normal_y = GroundModule::get_touch_normal_y(item.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        // println!("normal: {}, {}", normal_x, normal_y);
        let angle = normal_x.atan2(normal_y);
        // println!("angle: {}", angle.to_degrees());
        let speed = 1.29;
        let speed_x = speed * angle.cos();
        let speed_y = speed * angle.sin();
        // println!("speed: {}, {}", speed_x, speed_y);
        let lr = PostureModule::lr(item.module_accessor);
        item.clear_lua_stack();
        lua_args!(item, 0, 0);
        KINETIC_ENERGY_CONTROL_SET_ACCEL(item.lua_state_agent, &Vector2f{x: 0.0, y: 0.0});
        item.clear_lua_stack();
        lua_args!(item, 0, 0);
        KINETIC_ENERGY_CONTROL_SET_BRAKE(item.lua_state_agent, &Vector2f{x: 0.0248, y: 0.0});
        item.clear_lua_stack();
        lua_args!(item, speed, speed);
        KINETIC_ENERGY_CONTROL_SET_STABLE_SPEED(item.lua_state_agent, &Vector2f{x: 0.0, y: speed});
        item.clear_lua_stack();
        lua_args!(item, speed, speed);
        KINETIC_ENERGY_CONTROL_SET_LIMIT_SPEED(item.lua_state_agent, &Vector2f{x: speed, y: speed});
        item.clear_lua_stack();
        lua_args!(item, speed_x.abs() * lr, -speed_y * lr);
        KINETIC_ENERGY_CONTROL_SET_SPEED(item.lua_state_agent, &Vector2f{x: speed_x.abs() * lr, y: -speed_y * lr});
        item.clear_lua_stack();
        KINETIC_ENERGY_CONTROL_ENABLE(item.lua_state_agent);
    }
    // </ported from WuBor>
    PostureModule::set_rot(item.module_accessor, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0);
    0.into()
}

unsafe extern "C" fn richter_holywater_something(_item: &mut L2CAgent) -> L2CValue {
    // Checks which holywater, but I'm lazy...
    0x44.into()
}

#[no_mangle]
unsafe extern "C" fn richter_holywater_born_loop_inner(item: &mut L2CAgent) -> L2CValue {
    if GroundModule::is_touch(item.module_accessor, (*GROUND_TOUCH_FLAG_LEFT | *GROUND_TOUCH_FLAG_RIGHT) as u32) {
        item.clear_lua_stack();
        lua_args!(item, 0.0, 0.0);
        KINETIC_ENERGY_CONTROL_SET_SPEED(item.lua_state_agent, &Vector2f{x: 0.0, y: 0.0});
    }
    // else if GroundModule::is_touch(item.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
    //     let normal_x = GroundModule::get_touch_normal_x(item.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    //     let normal_y = GroundModule::get_touch_normal_y(item.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
    //     let angle = normal_x.atan2(normal_y);
    //     let speed = 1.29;
    //     let speed_x = speed * angle.cos();
    //     let speed_y = speed * angle.sin();
    //     let lr = PostureModule::lr(item.module_accessor);
    //     item.clear_lua_stack();
    //     lua_args!(item, speed_x.abs() * lr, -speed_y * lr);
    //     KINETIC_ENERGY_CONTROL_SET_SPEED(item.lua_state_agent, &Vector2f{x: speed_x.abs() * lr, y: -speed_y * lr});
    // }
    0.into()
}