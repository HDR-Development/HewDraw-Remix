use super::*;
use super::*;

#for _ in 0..1000 {
		wait(lua_state, 1.0);
		if is_excute(fighter) {
			let motion_vec = Vector3f{x: 0.5, y: 1.0, z: 1.0};
			KineticModule::mul_speed(boma, &motion_vec, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
		}
	}
    
}

pub fn install() {
    install_acmd_scripts!(
        dash_game,
		dash_effect,
        turn_dash_game,
		catch_game,
		catch_dash_game,
		catch_turn_game,
		miigunner_gunnercharge_shoot_game,
		miigunner_rapidshot_bullet_flythrowb_game,
		//miigunner_fullthrottle_final_game,
		miigunner_stealthbomb_tame_effect,
		//miigunner_stealthbomb_s_move_game,
		//miigunner_stealthbomb_s_move_effect,
		miigunner_bottomshoot_shoot_game,
    );
}

