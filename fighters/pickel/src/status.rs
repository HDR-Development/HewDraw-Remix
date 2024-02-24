use super::*;
use globals::*;
// status script import

pub fn install() {
    install_status_scripts!(
        recreate_table,
        guarddamage_pre,
        guarddamage_end,
        guard,
        rebirth,
        entry,
        attack_air_lw_start_main,
        special_s_pre,
        pre_jump
    );
    smashline::install_agent_init_callbacks!(pickel_init);
    skyline::install_hooks!(stuff_hook);
}

// prevent steve from spawning the crafting table through vanilla circumstances
#[status_script(agent = "pickel", status = FIGHTER_PICKEL_STATUS_KIND_RECREATE_TABLE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn recreate_table(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_prev_status(*FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WAIT)
    || !VarModule::is_flag(fighter.object(), vars::pickel::instance::CAN_RESPAWN_TABLE) {
        VarModule::on_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF);
        StatusModule::change_status_force(fighter.boma(), *FIGHTER_STATUS_KIND_GUARD_OFF, true); // steve will instead parry
        
        return 1.into();
    }

    return original!(fighter);
}

// prevent steve's shield from being locked in place after it is damaged (vanilla bug) 
#[status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_GUARD_DAMAGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn guarddamage_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.boma(), *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_2ND_MOTION);

    return original!(fighter);
}

#[status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_GUARD_DAMAGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn guarddamage_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_GuardDamage()
}

// lets the "stuff" article generate in new statuses
#[skyline::hook(offset = 0xf13d3c, inline)]
unsafe fn stuff_hook(ctx: &mut skyline::hooks::InlineCtx) {
    let new_shield_statuses = &[
        0x1B, // GUARD_ON
        0x1C // GUARD
        ];
    let status = *ctx.registers[0].x.as_ref();
    if new_shield_statuses.contains(&status) {
        *ctx.registers[0].x.as_mut() = 0x1E;
    } 
}

// keep shield article visible while shielding
#[status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_GUARD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn guard(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !ArticleModule::is_exist(fighter.boma(), *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF){
        ArticleModule::generate_article(fighter.boma(), *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, false, -1);
        ArticleModule::set_rate(fighter.boma(), *FIGHTER_PICKEL_GENERATE_ARTICLE_STUFF, 0.0);
    }

    return original!(fighter);
}

// handles the removal of steves resources when respawning
#[status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_REBIRTH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn rebirth(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    
    return original!(fighter);
}

// handles materials when steve is entering the match, to account for salty runbacks
#[status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_ENTRY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn entry(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    
    return original!(fighter);
}

#[status_script(agent = "pickel", status = FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn attack_air_lw_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if pickel_attack_que(fighter).get_bool() {
        return 0.into();
    }
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_ATTACK_FRAME);
    let mut generate = false;
    if ArticleModule::is_generatable(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_ATTACK_AIR_LW_FORBID_FRAME) <= 0
        || !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE) {
            generate = true;
        }
    }
    if !FighterSpecializer_Pickel::check_material_attack_air_lw_generate(fighter.module_accessor) {
        generate = false;
    }
    let mot = if generate {
        KineticModule::clear_speed_attr(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        Hash40::new("attack_air_lw")
    }
    else {
        Hash40::new("attack_air_lw_fail")
    };
    MotionModule::change_motion(
        fighter.module_accessor,
        mot,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    WorkModule::set_flag(fighter.module_accessor, generate, *FIGHTER_PICKEL_STATUS_ATTACK_FLAG_IS_GENERATE_FORGE);
    if generate {
        let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        WorkModule::set_int(fighter.module_accessor, jump_count, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_JUMP_COUNT_BACKUP);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_IGNORE_LAMDING_RECOVER);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_FORGE_LANDING);
        fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_lw_start_main_loop as *const () as _))
    }
    else {
        fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_lw_fail_main_status_loop as *const () as _))
    }
}

unsafe extern "C" fn attack_air_lw_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
            fighter.change_status(FIGHTER_PICKEL_STATUS_KIND_ATTACK_AIR_LW_LOOP.into(), false.into());
            return 0.into();
        }
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 0.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_ATTACK_FLAG_FORGE_GENERATE_ENABLE) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_ATTACK_FLAG_FORGE_GENERATE_ENABLE);
        if FighterSpecializer_Pickel::check_material_attack_air_lw_generate(fighter.module_accessor) {
            let attack_air_lw_interval = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("attack_air_lw_interval"));
            WorkModule::set_int(fighter.module_accessor, attack_air_lw_interval, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_ATTACK_AIR_LW_FORBID_FRAME);
            let anvil_iron_count = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), 0x188e0b0db2) + 2;
            FighterSpecializer_Pickel::sub_material_num(fighter.module_accessor, *FIGHTER_PICKEL_MATERIAL_KIND_IRON, anvil_iron_count);
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICKEL_GENERATE_ARTICLE_FORGE, false, -1);
            if LinkModule::is_link(fighter.module_accessor, *FIGHTER_PICKEL_LINK_NO_FORGE) {
                LinkModule::send_event_parents(fighter.module_accessor, *FIGHTER_PICKEL_LINK_NO_FORGE, Hash40::new_raw(0x11d608f91f));
            }
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    pickel_attack_catch_item(fighter);
    attack_air_lw_dead_area(fighter);
    0.into()
}

pub unsafe extern "C" fn pickel_attack_que(fighter: &mut L2CFighterCommon) -> L2CValue {
    let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if !FighterSpecializer_Pickel::is_status_kind_attack(prev_status) {
        fighter.sub_GetLightItemImm(L2CValue::Void());
        if StatusModule::status_kind_que_from_script(fighter.module_accessor) as i32 != *STATUS_KIND_NONE {
            return true.into();
        }
    }
    false.into()
}

pub unsafe extern "C" fn pickel_attack_catch_item(fighter: &mut L2CFighterCommon) {
    let catch_frame_param = if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        hash40("item_catch_frame_attack_3")
    } else {
        hash40("item_air_catch_frame")
    };
    let catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), catch_frame_param);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_ATTACK_FRAME) <= catch_frame {
        fighter.sub_GetLightItemImm(L2CValue::Void());
    }
}

unsafe extern "C" fn attack_air_lw_dead_area(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let pos = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
        ModelModule::joint_global_position(
            fighter.module_accessor,
            Hash40::new("hip"),
            pos,
            true
        );
        let check_dead = GroundUtility::check_dead_area(pos);
        if check_dead != *GROUND_DEAD_AREA_CHECK_RESULT_OUTSIDE_UP {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
        }
    }
}

pub unsafe extern "C" fn attack_air_lw_fail_main_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.status_AttackAir_Main_common().get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        if !fighter.global_table[IS_STOPPING].get_bool() {
            fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
        }
        0.into()
    } else {
        1.into()
    }
}

#[status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue{
    let hasIron = WorkModule::get_int(fighter.module_accessor,*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_MATERIAL_NUM_IRON) > 0;
    let trolleyArticle = ArticleModule::is_exist(fighter.module_accessor,*FIGHTER_PICKEL_GENERATE_ARTICLE_TROLLEY);
    let canCart = hasIron && !trolleyArticle;
    if canCart {
        VarModule::on_flag(fighter.battle_object, vars::pickel::instance::DISABLE_SPECIAL_S);
    }

    return original!(fighter);
}

// Prevents side special from being used again if it has already been used once in the current airtime
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR)
    && VarModule::is_flag(fighter.battle_object, vars::pickel::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

// Re-enables the ability to use side special when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let still_SideSpecial = fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_JUMP,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_RIDE,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_DRIVE
        ]
    );

    if (!fighter.is_situation(*SITUATION_KIND_AIR) && !still_SideSpecial) 
    || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::pickel::instance::DISABLE_SPECIAL_S);
    }

    return true.into();
}

unsafe extern "C" fn pickel_jump_status_check(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.is_prev_status_one_of(&[
        *FIGHTER_PICKEL_STATUS_KIND_ATTACK_JUMP,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_JUMP,
        *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N3_JUMP]) {
        return false.into();
    } else {
        return true.into();
    }
}

#[status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_jump(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.status_pre_Jump_Common_param(L2CValue::Bool(true)).get_bool()
    {
        return 1.into();
    } else {
        if pickel_jump_status_check(fighter).get_bool() {
            fighter.status_pre_Jump_sub_param(
                L2CValue::I32(-1),
                L2CValue::I32(-1),
                L2CValue::I32(-1),
                L2CValue::I32(*KINETIC_TYPE_NONE),
                L2CValue::I32(
                    *FS_SUCCEEDS_KEEP_EFFECT
                    | *FS_SUCCEEDS_KEEP_SOUND
                    | *FS_SUCCEEDS_KEEP_TRANSITION
                    | *FS_SUCCEEDS_KEEP_CANCEL,
                ),
            );
        } else {
            fighter.status_pre_Jump_sub_param(
                L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLAG),
                L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_INT),
                L2CValue::I32(*FIGHTER_STATUS_WORK_KEEP_FLAG_JUMP_FLOAT),
                L2CValue::I32(*FIGHTER_KINETIC_TYPE_JUMP),
                L2CValue::I32(0),
            );
        }
        return 0.into();
    }
}

#[fighter_init]
fn pickel_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        if fighter.kind() != FIGHTER_KIND_PICKEL {
            return;
        }
        
        // set callbacks and variables on fighter init
        fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
        fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));
        VarModule::on_flag(fighter.battle_object, vars::pickel::instance::SHOULD_CYCLE_MATERIAL);
        VarModule::off_flag(fighter.battle_object, vars::pickel::instance::SHOULD_RESET_ROT);
        VarModule::set_int(fighter.battle_object, vars::pickel::instance::MATERIAL_INDEX, 0);
        VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 0);
        VarModule::set_int(fighter.battle_object, vars::pickel::instance::HITSTUN_TIMER, 0);
        VarModule::set_float(fighter.battle_object, vars::pickel::instance::DAMAGE_TRACKER, 0.0);
        VarModule::set_float(fighter.battle_object, vars::pickel::instance::TABLE_HP_TRACKER, 20.0);
        
    }
}