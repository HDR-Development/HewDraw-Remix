use super::*;
use utils::ext::*;
use super::super::energy::PaddedVec2;

#[skyline::hook(offset = 0x3453b10)]
unsafe extern "C" fn miigunner_grenadelauncher_init(vtable: u64, weapon: &mut smash::app::Weapon, init_struct: u64) {
    let boma = weapon.battle_object.module_accessor;
    let speed = WorkModule::get_param_float(boma, hash40("param_grenadelauncher"), hash40("speed"));
    let mul = *(init_struct as *const f32).add(0x88 / 0x4);
    let speed = speed * mul;
    let angle = WorkModule::get_param_float(boma, hash40("param_grenadelauncher"), hash40("angle"));
    let lr = PostureModule::lr(boma);
    let energy = KineticModule::get_energy(boma, 0) as *mut super::super::energy::KineticEnergy;
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

//d748d0 - on reflect (lw1)
//d74bc0 - on absorb (lw3)
//d74a20 - on reflect (lw1)
//d74fd0 - on absorb (lw3)
// #[skyline::hook(offset = 0x)]
// unsafe extern "C" fn miigunner_on_search(vtable: u64, fighter: &mut Fighter, param_3: u64) {
//     println!("h?");
//     return call_original!(vtable, fighter, param_3);
// }

// #[skyline::hook(offset = 0x33bdb70)]
// unsafe extern "C" fn miigunner_groundbomb_set_team_flags(vtable: u64, weapon: &mut smash::app::Weapon) {
//     println!("set team flags?");

//     return call_original!(vtable, weapon);
// }

// #[skyline::hook(offset = 0x33be150)]
// unsafe extern "C" fn miigunner_groundbomb_on_damage2(vtable: u64, weapon: &mut smash::app::Weapon) {
//     println!("on damage2?");

//     return call_original!(vtable, weapon);
// }

// #[skyline::hook(offset = 0x33be0b0)]
// unsafe extern "C" fn miigunner_groundbomb_on_reflect(vtable: u64, weapon: &mut smash::app::Weapon) {
//     println!("on reflect?");

//     return call_original!(vtable, weapon);
// }

pub fn install() {
    skyline::install_hooks!(
        miigunner_grenadelauncher_init,
        // miigunner_groundbomb_set_team_flags,
        // miigunner_groundbomb_on_damage2,
        // miigunner_groundbomb_on_reflect,
    );
}