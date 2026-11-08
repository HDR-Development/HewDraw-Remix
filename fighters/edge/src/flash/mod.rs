use super::*;

mod acmd;
mod status;

#[no_mangle]
unsafe extern "C" fn edge_flash_on_search_inner(vtable: u64, weapon: &mut app::Weapon, log: *mut CollisionLog) {
    let object = &mut weapon.battle_object;
    let module_accessor = object.module_accessor;
    let collision_kind = (*log).collision_kind;
    let object_id = (*log).opponent_battle_object_id;
    let category = sv_battle_object::category(object_id);
    // println!("what's up");
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        // println!("fighter time");
        let collision_object = get_battle_object_from_id(object_id);
        let kind = (*collision_object).kind as i32;
        if kind == *FIGHTER_KIND_EDGE {
            let status = StatusModule::status_kind((*collision_object).module_accessor);
            if status == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_RUSH
            || status == *FIGHTER_EDGE_STATUS_KIND_SPECIAL_HI_CHARGED_RUSH {
                let lr = PostureModule::lr(module_accessor);
                let edge_lr = PostureModule::lr((*collision_object).module_accessor);
                if lr != edge_lr {
                    PostureModule::reverse_lr(module_accessor);
                }

                let team = TeamModule::team_no((*collision_object).module_accessor) as i32;
                TeamModule::set_team(module_accessor, team, false);

                StatusModule::change_status_request(module_accessor, statuses::edge_flash::BURST, false);
            }
        }
    }
    else if category == *BATTLE_OBJECT_CATEGORY_WEAPON {
        // println!("weapon time");
        let collision_object = get_battle_object_from_id(object_id);
        let kind = (*collision_object).kind as i32;
        if kind == *WEAPON_KIND_EDGE_FIRE {
            // println!("oh it's gettin hot in here");
            VarModule::on_flag(collision_object, vars::edge_fire::instance::REFINE);
        }
        if kind == *WEAPON_KIND_EDGE_FLARE1 {
            // println!("of it's gettin shadowy in here");
            VarModule::on_flag(collision_object, vars::edge_flare1::status::REFRACT);
        }
    }
}


pub fn install() {
    let agent = &mut Agent::new("edge_flash");
    acmd::install(agent);
    status::install(agent);
    agent.install();
}