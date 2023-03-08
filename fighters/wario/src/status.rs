use super::*;
use globals::*;
 

pub fn install() {
    install_status_scripts!(
        catch_attack_exec,
        catch_attack_end,
    );
}


//Force opponent rotation
#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_CATCH_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn catch_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue {

    let boma = fighter.boma();

    let mut vec =Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let offset = ModelModule::joint_global_rotation(boma,Hash40::new("throw"),&mut vec,false);
    let rot = Vector3f{x: vec.x, y: 0.0, z: 0.0};
    PostureModule::set_rot(
        boma.get_grabbed_opponent_boma(),
        &rot,
        0
    );
    return false.into();
}
//Reset opponent rotation
#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_CATCH_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn catch_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.boma();

    PostureModule::set_rot(
        boma.get_grabbed_opponent_boma(),
        &Vector3f::zero(),
        0
    );
    return original!(fighter);
}