use super::*;

pub unsafe fn special_motion_helper(
    fighter: &mut L2CFighterCommon,
    mot_g: L2CValue,
    mot_a: L2CValue,
    kinetic_g: L2CValue,
    kinetic_a: L2CValue,
    inherit_const: L2CValue,
    correct_g: L2CValue
) {
    let mot;
    let kinetic;
    let correct;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        mot = mot_a.get_u64();
        kinetic = kinetic_a.get_i32();
        correct = *GROUND_CORRECT_KIND_AIR;
    }
    else {
        mot = mot_g.get_u64();
        kinetic = kinetic_g.get_i32();
        correct = correct_g.get_i32();
    }
    KineticModule::change_kinetic(fighter.module_accessor, kinetic);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    if !WorkModule::is_flag(fighter.module_accessor, inherit_const.get_i32()) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, inherit_const.get_i32());
    }
    else {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
}
