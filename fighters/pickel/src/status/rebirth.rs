use super::*;
use globals::*;
 
// FIGHTER_STATUS_KIND_REBIRTH

// handles the removal of steves resources when respawning
pub unsafe extern "C" fn rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dirt = WorkModule::get_int(fighter.boma(), *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_GRADE_1);
    let wood = WorkModule::get_int(fighter.boma(), *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_WOOD);
    let stone = WorkModule::get_int(fighter.boma(), *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_STONE);
    let iron = WorkModule::get_int(fighter.boma(), *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_IRON);
    let gold = WorkModule::get_int(fighter.boma(), *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_GOLD);
    let diamond = WorkModule::get_int(fighter.boma(), *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_DIAMOND);
    let redstone = WorkModule::get_int(fighter.boma(), *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_RED_STONE);
    let reduce_half: [[i32;2];4] = [ // these materials will be reduced by 50% on respawn
        [*FIGHTER_PICKEL_MATERIAL_KIND_GRADE_1, dirt],
        [*FIGHTER_PICKEL_MATERIAL_KIND_WOOD, wood],
        [*FIGHTER_PICKEL_MATERIAL_KIND_STONE, stone],
        [*FIGHTER_PICKEL_MATERIAL_KIND_IRON, iron]]; 
    for material in reduce_half {
        if material[1] > 1 {
            let reduction = (material[1] / 2);
            FighterSpecializer_Pickel::sub_material_num(fighter.boma(), material[0], reduction);
        }
    }     
    if gold > 0 { // remove all gold
        FighterSpecializer_Pickel::sub_material_num(fighter.boma(), *FIGHTER_PICKEL_MATERIAL_KIND_GOLD, gold);
    }
    if diamond > 0 { // remove all diamonds
        FighterSpecializer_Pickel::sub_material_num(fighter.boma(), *FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, diamond);
    }
    let init_rstn = WorkModule::get_param_int(fighter.boma(), hash40("param_private"), hash40("start_material_red_stone_num"));
    if redstone > init_rstn { // reduce redstone to starting value
        FighterSpecializer_Pickel::sub_material_num(fighter.boma(), *FIGHTER_PICKEL_MATERIAL_KIND_RED_STONE, (redstone - init_rstn));
    }
    
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_REBIRTH)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_REBIRTH, rebirth_main);
}