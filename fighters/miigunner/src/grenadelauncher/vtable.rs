use super::*;

// Mirrors the layout in common's function_hooks/energy module.
#[repr(C)]
struct PaddedVec2 {
    x: f32,
    y: f32,
    padding: u64
}

impl PaddedVec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y, padding: 0 }
    }
}

#[repr(C)]
struct KineticEnergy {
    vtable: u64,
    _x8: u64,
    speed: PaddedVec2,
    rot_speed: PaddedVec2,
    enable: bool,
    unk2: [u8; 0xF],
    accel: PaddedVec2,
    speed_max: PaddedVec2,
    speed_brake: PaddedVec2,
    speed_limit: PaddedVec2,
}

// WeaponVtableSlot::WeaponGenerate3
unsafe extern "C" fn miigunner_grenadelauncher_init(_vtable: u64, weapon: &mut app::Weapon, init_struct: u64) {
    let boma = weapon.battle_object.module_accessor;
    let speed = WorkModule::get_param_float(boma, hash40("param_grenadelauncher"), hash40("speed"));
    let mul = *(init_struct as *const f32).add(0x88 / 0x4);
    let speed = speed * mul;
    let angle = WorkModule::get_param_float(boma, hash40("param_grenadelauncher"), hash40("angle"));
    let lr = PostureModule::lr(boma);
    let energy = KineticModule::get_energy(boma, 0) as *mut KineticEnergy;
    let brake_x = WorkModule::get_param_float(boma, hash40("param_grenadelauncher"), hash40("brake_x"));
    let gravity_acl_max = WorkModule::get_param_float(boma, hash40("param_grenadelauncher"), hash40("gravity_acl_max"));
    let gravity_accel = WorkModule::get_param_float(boma, hash40("param_grenadelauncher"), hash40("gravity_accel"));

    let angle_adjust;
    //let brake_adjust;
    let owner_boma = (&mut *(boma)).get_owner_boma();
    if owner_boma.kind() == *FIGHTER_KIND_MIIGUNNER {
        let charge = VarModule::get_float(owner_boma.object(), vars::miigunner::instance::SPECIAL_N3_CHARGE);
        let max_charge_frames = ParamModule::get_float(owner_boma.object(), ParamType::Agent, "param_charge.max_charge_frames");

        let charge_angle = ParamModule::get_float(owner_boma.object(), ParamType::Agent, "param_charge.special_n3_charge_max_angle");
        angle_adjust = angle - ((angle - charge_angle) * (charge / max_charge_frames));
        // let charge_brake = ParamModule::get_float(owner_boma.object(), ParamType::Agent, "param_charge.special_n3_max_brake");
        // brake_adjust = brake + ((brake - charge_brake) * (charge / max_charge_frames));
    }
    else {
        angle_adjust = angle - 10.0;
        //brake_adjust = brake;
    }

    let angle_cos = (angle_adjust).to_radians().cos();
    let angle_sin = (angle_adjust).to_radians().sin();

    (*energy).speed = PaddedVec2::new(speed * lr * angle_cos, speed * angle_sin);
    (*energy).speed_limit = PaddedVec2::new(-1.0, gravity_acl_max);
    (*energy).speed_max = PaddedVec2::new(0.0, 0.0);
    (*energy).speed_brake = PaddedVec2::new(brake_x, 0.0);
    (*energy).accel = PaddedVec2::new(0.0, -gravity_accel);
}

pub fn install() {
    replace_weapon_vtable(
        *WEAPON_KIND_MIIGUNNER_GRENADELAUNCHER,
        WeaponVtableSlot::WeaponGenerate3,
        miigunner_grenadelauncher_init as *const () as _
    );
}
