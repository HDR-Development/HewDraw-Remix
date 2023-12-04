use super::*;
use globals::*;

// defines steve's material table
#[skyline::hook(offset = 0x0f106f0)] // FighterSpecializer_Pickel::get_mining_material_table_result()
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
        dirt, wood, ston, iron, wood, ston, dirt, iron, rstn, wood, 
        iron, dirt, ston, wood, rstn, iron, dirt, wood, dirt, iron, 
        wood, iron, dirt, rstn, dirt, iron, wood, ston, dirt, gold, 
        dirt, iron, dirt, wood, ston, iron, dirt, rstn, ston, dirt, 
        wood, iron, dirt, wood, ston, iron, dirt, rstn, ston, dmnd,
        dirt, iron, wood, rstn, ston, dirt, wood, iron, ston, gold, 
        ston, rstn, wood, dirt, ston, iron, wood, ston, dirt, rstn, 
        wood, iron, dirt, ston, wood, iron, dirt, rstn, dirt, ston, 
        wood, iron, dirt, iron, ston, wood, dirt, iron, ston, gold, 
        wood, iron, dirt, rstn, ston, wood, dirt, iron, dirt, dmnd, dirt // dummy entry as a failsafe
    ];
    
    let pickel = &mut (*fighter).battle_object;
    let index = VarModule::get_int(pickel, vars::pickel::instance::MATERIAL_INDEX) as i32;
    let material = material_table[index as usize] as u32; // stores current table index to return
    if VarModule::is_flag(pickel, vars::pickel::instance::SHOULD_CYCLE_MATERIAL) {
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
}

// overrides stage id check for things such as mining speed
#[skyline::hook(offset = 0x0f10660)] // FighterSpecializer_Pickel::is_mining_material_table_normal()
pub unsafe extern "C" fn stub_stage_check() -> bool {
    return false;
}

pub fn install() {
    skyline::install_hooks!(
        material_table_hook,
        stub_stage_check
    );
}