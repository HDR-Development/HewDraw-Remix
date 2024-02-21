use super::*;
use globals::*;

// defines steve's material table
#[skyline::hook(offset = 0x0f10710)] // FighterSpecializer_Pickel::get_mining_material_table_result()
pub unsafe extern "C" fn material_table_hook(fighter: &mut Fighter, arg2: u64, arg3: u64) -> u32 {
    let dirt = 0; // this applies to whichever "grade 1" material is being used
    let wood = 1;
    let ston = 2; // stone
    let iron = 3;
    let gold = 4;
    let rstn = 5; // redstone
    let dmnd = 6; // diamond

    // order in which steve will obtain materials
    let material_table: [i32; 101] = [
        dirt, ston, iron, wood, dirt, ston, iron, dirt, wood, ston, 
        iron, dirt, wood, ston, iron, wood, dirt, wood, iron, dirt, 
        ston, wood, iron, dirt, ston, dirt, iron, wood, dirt, gold, 
        iron, dirt, ston, dirt, iron, wood, ston, iron, dirt, wood, 
        iron, dirt, ston, iron, dirt, iron, wood, iron, dirt, dmnd,
        wood, dirt, iron, ston, dirt, wood, iron, ston, dirt, gold, 
        iron, wood, dirt, ston, iron, dirt, wood, ston, iron, dirt, 
        wood, dirt, iron, dirt, wood, ston, iron, dirt, wood, ston, 
        iron, dirt, ston, wood, iron, dirt, wood, iron, dirt, gold, 
        iron, dirt, ston, iron, dirt, iron, wood, iron, dirt, dmnd, dirt // dummy entry as a failsafe
    ];
    
    let pickel = &mut (*fighter).battle_object;
    let kind = pickel.kind as i32;
    if kind != *FIGHTER_KIND_KIRBY {
        let index = VarModule::get_int(pickel, vars::pickel::instance::MATERIAL_INDEX) as i32;
        let material = material_table[index as usize] as u32; // stores current table index to return
        if VarModule::is_flag(pickel, vars::pickel::instance::SHOULD_CYCLE_MATERIAL) {
            if material == (gold as u32) { // add redstone to gold entries
                let pickel_boma = pickel.module_accessor;
                let current_redstone = WorkModule::get_int(pickel_boma, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_RED_STONE);
                let max_redstone = WorkModule::get_param_int(pickel_boma, hash40("param_private"), hash40("material_red_stone_max"));
                let mut gain_amount = 6; // number of redstone steve should receive when mining gold
                if (current_redstone + gain_amount) <= max_redstone {
                    FighterSpecializer_Pickel::add_material_num(pickel_boma, *FIGHTER_PICKEL_MATERIAL_KIND_RED_STONE, gain_amount);
                } else if current_redstone != max_redstone {
                    gain_amount = max_redstone - current_redstone;
                    FighterSpecializer_Pickel::add_material_num(pickel_boma, *FIGHTER_PICKEL_MATERIAL_KIND_RED_STONE, gain_amount);
                }
            }
            if (0..99).contains(&index) { // continue the cycle
                VarModule::inc_int(pickel, vars::pickel::instance::MATERIAL_INDEX);
            } else { // reset the cycle
                VarModule::set_int(pickel, vars::pickel::instance::MATERIAL_INDEX, 0);
            }
            VarModule::off_flag(pickel, vars::pickel::instance::SHOULD_CYCLE_MATERIAL);
            VarModule::set_int(pickel, vars::pickel::status::MINING_TIMER, 2);
            // logging for debug purposes
            // let mut mat_name = "nothing lol";
            // if material == 0 { mat_name = "dirt"; }
            // else if material == 1 { mat_name = "wood"; }
            // else if material == 2 { mat_name = "stone"; }
            // else if material == 3 { mat_name = "iron"; }
            // else if material == 4 { mat_name = "gold"; }
            // else if material == 5 { mat_name = "redstone"; }
            // else if material == 6 { mat_name = "diamond"; }
            // else { mat_name = "...what?"; }
            // println!("Steve mines entry {}/100, which is {}.", (index +1), mat_name);
        }

        return material;
    } else { // alternative logic for kirby
        let index = VarModule::get_int(pickel, vars::kirby::instance::MATERIAL_INDEX) as i32;
        let material = material_table[index as usize] as u32; // stores current table index to return
        if VarModule::is_flag(pickel, vars::kirby::instance::SHOULD_CYCLE_MATERIAL) {
            if (0..99).contains(&index) { // continue the cycle
                VarModule::inc_int(pickel, vars::kirby::instance::MATERIAL_INDEX);
            } else { // reset the cycle
                VarModule::set_int(pickel, vars::kirby::instance::MATERIAL_INDEX, 0);
            }
            VarModule::off_flag(pickel, vars::kirby::instance::SHOULD_CYCLE_MATERIAL);
            VarModule::set_int(pickel, vars::kirby::status::MINING_TIMER, 2);
            // logging for debug purposes
            // let mut mat_name = "nothing lol";
            // if material == 0 { mat_name = "dirt"; }
            // else if material == 1 { mat_name = "wood"; }
            // else if material == 2 { mat_name = "stone"; }
            // else if material == 3 { mat_name = "iron"; }
            // else if material == 4 { mat_name = "gold. He gets iron instead"; }
            // else if material == 6 { mat_name = "diamond. He gets iron instead"; }
            // else { mat_name = "...what?"; }
            // println!("Kirby mines entry {}/100, which is {}.", (index +1), mat_name);
        }
        if material == (4 | 6) {
            return 3; // give kirby iron instead of gold/diamond
        }
        return material;
    }
}

// overrides stage id check for things such as mining speed
#[skyline::hook(offset = 0x0f10680)] // FighterSpecializer_Pickel::is_mining_material_table_normal()
pub unsafe extern "C" fn stub_stage_check() -> bool {
    return false;
}


pub fn install() {
    skyline::install_hooks!(
        material_table_hook,
        stub_stage_check
    );
}
