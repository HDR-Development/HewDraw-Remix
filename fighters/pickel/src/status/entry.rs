use super::*;
use globals::*;
 
// FIGHTER_STATUS_KIND_ENTRY

// handles materials when steve is entering the match, to account for salty runbacks
pub unsafe extern "C" fn entry_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dirt = WorkModule::get_int(fighter.boma(), *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_GRADE_1);
    let wood = WorkModule::get_int(fighter.boma(), *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_WOOD);
    let stone = WorkModule::get_int(fighter.boma(), *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_STONE);
    let iron = WorkModule::get_int(fighter.boma(), *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_IRON);
    let gold = WorkModule::get_int(fighter.boma(), *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_GOLD);
    let diamond = WorkModule::get_int(fighter.boma(), *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_DIAMOND);
    let redstone = WorkModule::get_int(fighter.boma(), *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_RED_STONE);
    let remove_mats: [[i32;2];4] = [ // these materials will be removed on entry
        [*FIGHTER_PICKEL_MATERIAL_KIND_STONE, stone],
        [*FIGHTER_PICKEL_MATERIAL_KIND_IRON, iron],
        [*FIGHTER_PICKEL_MATERIAL_KIND_GOLD, dirt],
        [*FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND, wood]]; 
    for material in remove_mats {
        let (kind, has) = (material[0], material[1]);
        if has > 0 {
            FighterSpecializer_Pickel::sub_material_num(fighter.boma(), kind, has);
        }
    }    
    let init_dirt = WorkModule::get_param_int(fighter.boma(), hash40("param_private"), hash40("start_material_grade_1_num"));
    let init_wood = WorkModule::get_param_int(fighter.boma(), hash40("param_private"), hash40("start_material_wood_num")); 
    let init_rstn = WorkModule::get_param_int(fighter.boma(), hash40("param_private"), hash40("start_material_red_stone_num"));
    let init_mats: [[i32;3];3] = [ // these materials will be reverted to the initial amount defined in params
        [*FIGHTER_PICKEL_MATERIAL_KIND_GRADE_1, dirt, init_dirt],
        [*FIGHTER_PICKEL_MATERIAL_KIND_WOOD, wood, init_wood],
        [*FIGHTER_PICKEL_MATERIAL_KIND_RED_STONE, redstone, init_rstn]];
    for material in init_mats {
        let (kind, has, init) = (material[0], material[1], material[2]);
        if has > init {
            FighterSpecializer_Pickel::sub_material_num(fighter.boma(), kind, (has - init));
        } else if has < init {
            FighterSpecializer_Pickel::add_material_num(fighter.boma(), kind, (init - has));
        }
    }
    
    smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_ENTRY)(fighter)
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ENTRY, entry_main);
}