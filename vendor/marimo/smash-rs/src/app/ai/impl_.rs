use crate::*;

extern "C" {
    #[link_name = "_ZN3app2ai9target_idEP9lua_State"]
    pub(super) fn target_id(state: *mut lua_State) -> u32;

    #[link_name = "_ZN3app2ai15is_valid_targetEP9lua_State"]
    pub(super) fn is_valid_target(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai12fighter_kindEP9lua_State"]
    pub(super) fn fighter_kind(state: *mut lua_State) -> app::FighterKind;

    #[link_name = "_ZN3app2ai17copy_fighter_kindEP9lua_State"]
    pub(super) fn copy_fighter_kind(state: *mut lua_State) -> app::FighterKind;

    #[link_name = "_ZN3app2ai4rankEP9lua_State"]
    pub(super) fn rank(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai7cp_typeEP9lua_State"]
    pub(super) fn cp_type(state: *mut lua_State) -> app::FighterAICPType;

    #[link_name = "_ZN3app2ai7cp_flagEP9lua_State"]
    pub(super) fn cp_flag(state: *mut lua_State) -> app::FighterAICPFlag;

    #[link_name = "_ZN3app2ai13cp_slide_typeEP9lua_State"]
    pub(super) fn cp_slide_type(state: *mut lua_State) -> app::FighterAICPType;

    #[link_name = "_ZN3app2ai6act_idEP9lua_State"]
    pub(super) fn act_id(state: *mut lua_State) -> app::FighterAIAct::CmdId;

    #[link_name = "_ZN3app2ai27current_attack_cancel_frameEP9lua_State"]
    pub(super) fn current_attack_cancel_frame(state: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai9uniq_statEP9lua_State"]
    pub(super) fn uniq_stat(state: *mut lua_State) -> app::FighterAIUniqStat;

    #[link_name = "_ZN3app2ai12attack_phaseEP9lua_State"]
    pub(super) fn attack_phase(state: *mut lua_State) -> app::FighterAIAttackPhase;

    #[link_name = "_ZN3app2ai14check_stat_airEP9lua_State"]
    pub(super) fn check_stat_air(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai20check_stat_build_maxEP9lua_State"]
    pub(super) fn check_stat_build_max(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai19check_stat_build_upEP9lua_State"]
    pub(super) fn check_stat_build_up(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai19check_stat_gorogoroEP9lua_State"]
    pub(super) fn check_stat_gorogoro(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai20check_stat_attentionEP9lua_State"]
    pub(super) fn check_stat_attention(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai16check_stat_finalEP9lua_State"]
    pub(super) fn check_stat_final(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai20check_stat_final_actEP9lua_State"]
    pub(super) fn check_stat_final_act(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai21check_stat_invincibleEP9lua_State"]
    pub(super) fn check_stat_invincible(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_stat_invincible_lEP9lua_State"]
    pub(super) fn check_stat_invincible_l(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_stat_damage_elecEP9lua_State"]
    pub(super) fn check_stat_damage_elec(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai17check_stat_sp_dirEP9lua_State"]
    pub(super) fn check_stat_sp_dir(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_stat_unguarded_hindEP9lua_State"]
    pub(super) fn check_stat_unguarded_hind(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai20check_stat_unguardedEP9lua_State"]
    pub(super) fn check_stat_unguarded(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai15check_stat_dashEP9lua_State"]
    pub(super) fn check_stat_dash(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai15check_stat_downEP9lua_State"]
    pub(super) fn check_stat_down(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai15check_stat_piyoEP9lua_State"]
    pub(super) fn check_stat_piyo(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18check_stat_dragoonEP9lua_State"]
    pub(super) fn check_stat_dragoon(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18check_stat_genesisEP9lua_State"]
    pub(super) fn check_stat_genesis(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai16check_stat_catchEP9lua_State"]
    pub(super) fn check_stat_catch(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai17check_stat_damageEP9lua_State"]
    pub(super) fn check_stat_damage(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai16check_stat_guardEP9lua_State"]
    pub(super) fn check_stat_guard(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_stat_attack_holdEP9lua_State"]
    pub(super) fn check_stat_attack_hold(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai21check_stat_floor_passEP9lua_State"]
    pub(super) fn check_stat_floor_pass(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_stat_floor_damageEP9lua_State"]
    pub(super) fn check_stat_floor_damage(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_stat_ground_freeEP9lua_State"]
    pub(super) fn check_stat_ground_free(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_stat_ground_free2EP9lua_State"]
    pub(super) fn check_stat_ground_free2(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai19check_stat_air_freeEP9lua_State"]
    pub(super) fn check_stat_air_free(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18check_stat_touch_uEP9lua_State"]
    pub(super) fn check_stat_touch_u(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18check_stat_touch_lEP9lua_State"]
    pub(super) fn check_stat_touch_l(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18check_stat_touch_rEP9lua_State"]
    pub(super) fn check_stat_touch_r(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai29check_stat_cannot_catch_cliffEP9lua_State"]
    pub(super) fn check_stat_cannot_catch_cliff(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai15check_stat_diveEP9lua_State"]
    pub(super) fn check_stat_dive(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai27check_stat_unable_cliff_xluEP9lua_State"]
    pub(super) fn check_stat_unable_cliff_xlu(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai28check_stat_unable_escape_airEP9lua_State"]
    pub(super) fn check_stat_unable_escape_air(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_stat_unable_attackEP9lua_State"]
    pub(super) fn check_stat_unable_attack(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_stat_unable_specialEP9lua_State"]
    pub(super) fn check_stat_unable_special(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_stat_unable_jumpEP9lua_State"]
    pub(super) fn check_stat_unable_jump(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_stat_unable_shieldEP9lua_State"]
    pub(super) fn check_stat_unable_shield(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai15check_stat_haveEP9lua_State"]
    pub(super) fn check_stat_have(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai19check_stat_put_bombEP9lua_State"]
    pub(super) fn check_stat_put_bomb(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai28check_stat_can_use_superleafEP9lua_State"]
    pub(super) fn check_stat_can_use_superleaf(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai29check_stat_can_use_rocketbeltEP9lua_State"]
    pub(super) fn check_stat_can_use_rocketbelt(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai21check_stat_have_throwEP9lua_State"]
    pub(super) fn check_stat_have_throw(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai21check_stat_have_shootEP9lua_State"]
    pub(super) fn check_stat_have_shoot(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai21check_stat_have_swingEP9lua_State"]
    pub(super) fn check_stat_have_swing(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_stat_dogs_blind_ownEP9lua_State"]
    pub(super) fn check_stat_dogs_blind_own(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai27check_stat_target_invisibleEP9lua_State"]
    pub(super) fn check_stat_target_invisible(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai16check_skill_statEP9lua_StateNS_13FighterAIStat11SkillStatusE"]
    pub(super) fn check_skill_stat(state: *mut lua_State, stat: app::FighterAIStat::SkillStatus) -> bool;

    #[link_name = "_ZN3app2ai24check_spirits_event_statEP9lua_StateNS_13FighterAIStat18SpiritsEventStatusE"]
    pub(super) fn check_spirits_event_stat(state: *mut lua_State, stat: app::FighterAIStat::SpiritsEventStatus) -> bool;

    #[link_name = "_ZN3app2ai14check_chr_statEP9lua_StateNS_13FighterAIStat15CharacterStatusENS_11FighterKindE"]
    pub(super) fn check_chr_stat(state: *mut lua_State, stat: app::FighterAIStat::CharacterStatus, fighter_kind: app::FighterKind) -> bool;

    #[link_name = "_ZN3app2ai15air_lasso_countEP9lua_State"]
    pub(super) fn air_lasso_count(state: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai15check_cliffableEP9lua_State"]
    pub(super) fn check_cliffable(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_cliffable_floor_lrEP9lua_Statef"]
    pub(super) fn check_cliffable_floor_lr(state: *mut lua_State, lr: f32) -> bool;

    #[link_name = "_ZN3app2ai14check_passableEP9lua_State"]
    pub(super) fn check_passable(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai11shield_rateEP9lua_State"]
    pub(super) fn shield_rate(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai19damage_reaction_mulEP9lua_State"]
    pub(super) fn damage_reaction_mul(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai10stop_frameEP9lua_State"]
    pub(super) fn stop_frame(state: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai6heightEP9lua_State"]
    pub(super) fn height(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai5pos_xEP9lua_State"]
    pub(super) fn pos_x(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai5pos_yEP9lua_State"]
    pub(super) fn pos_y(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai7speed_xEP9lua_State"]
    pub(super) fn speed_x(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai7speed_yEP9lua_State"]
    pub(super) fn speed_y(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai5scaleEP9lua_State"]
    pub(super) fn scale(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai11status_kindEP9lua_State"]
    pub(super) fn status_kind(state: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai16prev_status_kindEP9lua_State"]
    pub(super) fn prev_status_kind(state: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai11motion_kindEP9lua_State"]
    pub(super) fn motion_kind(state: *mut lua_State) -> phx::Hash40;

    #[link_name = "_ZN3app2ai12motion_frameEP9lua_State"]
    pub(super) fn motion_frame(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai11motion_rateEP9lua_State"]
    pub(super) fn motion_rate(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai19motion_cancel_frameEP9lua_State"]
    pub(super) fn motion_cancel_frame(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai19jump_rest_availableEP9lua_State"]
    pub(super) fn jump_rest_available(state: *mut lua_State) -> u16;

    #[link_name = "_ZN3app2ai17is_sp_u_availableEP9lua_State"]
    pub(super) fn is_sp_u_available(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24is_sp_u_weaken_availableEP9lua_State"]
    pub(super) fn is_sp_u_weaken_available(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai6damageEP9lua_State"]
    pub(super) fn damage(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai2hpEP9lua_State"]
    pub(super) fn hp(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai2lrEP9lua_State"]
    pub(super) fn lr(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai11customize_nEP9lua_State"]
    pub(super) fn customize_n(state: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai11customize_sEP9lua_State"]
    pub(super) fn customize_s(state: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai12customize_hiEP9lua_State"]
    pub(super) fn customize_hi(state: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai12customize_lwEP9lua_State"]
    pub(super) fn customize_lw(state: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai17check_use_commandEP9lua_State"]
    pub(super) fn check_use_command(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_use_command_typeEP9lua_State"]
    pub(super) fn check_use_command_type(state: *mut lua_State) -> u8;

    #[link_name = "_ZN3app2ai22check_command_236_stepEP9lua_State"]
    pub(super) fn check_command_236_step(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_command_41236_stepEP9lua_State"]
    pub(super) fn check_command_41236_step(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_command_214_stepEP9lua_State"]
    pub(super) fn check_command_214_step(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_command_623_stepEP9lua_State"]
    pub(super) fn check_command_623_step(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_command_236236_stepEP9lua_State"]
    pub(super) fn check_command_236236_step(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_command_21416_stepEP9lua_State"]
    pub(super) fn check_command_21416_step(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_command_214214_stepEP9lua_State"]
    pub(super) fn check_command_214214_step(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_command_23634_stepEP9lua_State"]
    pub(super) fn check_command_23634_step(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18fighter_uniq_valueEP9lua_State"]
    pub(super) fn fighter_uniq_value(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai19fighter_uniq_value2EP9lua_Statei"]
    pub(super) fn fighter_uniq_value2(state: *mut lua_State, uniq_value_idx: i32) -> f32;

    #[link_name = "_ZN3app2ai18line_segment_checkEP9lua_StateRKN3phx8Vector2fE"]
    pub(super) fn line_segment_check(state: *mut lua_State, line: *const phx::Vector2f) -> bool;

    #[link_name = "_ZN3app2ai29line_segment_check_from_top_nEP9lua_StateRKN3phx8Vector2fES6_"]
    pub(super) fn line_segment_check_from_top_n(state: *mut lua_State, line: *const phx::Vector2f, top: *const phx::Vector2f) -> bool;

    #[link_name = "_ZN3app2ai28line_segment_check_only_roofEP9lua_StateRKN3phx8Vector2fE"]
    pub(super) fn line_segment_check_only_roof(state: *mut lua_State, line: *const phx::Vector2f) -> bool;

    #[link_name = "_ZN3app2ai29line_segment_check_only_floorEP9lua_StateRKN3phx8Vector2fE"]
    pub(super) fn line_segment_check_only_floor(state: *mut lua_State, line: *const phx::Vector2f) -> bool;

    #[link_name = "_ZN3app2ai28line_segment_check_only_wallEP9lua_StateRKN3phx8Vector2fE"]
    pub(super) fn line_segment_check_only_wall(state: *mut lua_State, line: *const phx::Vector2f) -> bool;

    #[link_name = "_ZN3app2ai12weapon_pos_xEP9lua_State"]
    pub(super) fn weapon_pos_x(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai12weapon_pos_yEP9lua_State"]
    pub(super) fn weapon_pos_y(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai14weapon_speed_xEP9lua_State"]
    pub(super) fn weapon_speed_x(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai14weapon_speed_yEP9lua_State"]
    pub(super) fn weapon_speed_y(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai19target_fighter_kindEP9lua_State"]
    pub(super) fn target_fighter_kind(state: *mut lua_State) -> app::FighterKind;

    #[link_name = "_ZN3app2ai24target_copy_fighter_kindEP9lua_State"]
    pub(super) fn target_copy_fighter_kind(state: *mut lua_State) -> app::FighterKind;

    #[link_name = "_ZN3app2ai33target_current_attack_start_frameEP9lua_State"]
    pub(super) fn target_current_attack_start_frame(state: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai37target_current_attack_combo_end_frameEP9lua_State"]
    pub(super) fn target_current_attack_combo_end_frame(state: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai34target_current_attack_cancel_frameEP9lua_State"]
    pub(super) fn target_current_attack_cancel_frame(state: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai21check_target_stat_airEP9lua_State"]
    pub(super) fn check_target_stat_air(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai26check_target_stat_build_upEP9lua_State"]
    pub(super) fn check_target_stat_build_up(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai27check_target_stat_attentionEP9lua_State"]
    pub(super) fn check_target_stat_attention(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_target_stat_finalEP9lua_State"]
    pub(super) fn check_target_stat_final(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_target_stat_deadEP9lua_State"]
    pub(super) fn check_target_stat_dead(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai28check_target_stat_invincibleEP9lua_State"]
    pub(super) fn check_target_stat_invincible(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai30check_target_stat_invincible_lEP9lua_State"]
    pub(super) fn check_target_stat_invincible_l(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai30check_target_stat_attack_catchEP9lua_State"]
    pub(super) fn check_target_stat_attack_catch(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_target_stat_reflectEP9lua_State"]
    pub(super) fn check_target_stat_reflect(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai32check_target_stat_unguarded_hindEP9lua_State"]
    pub(super) fn check_target_stat_unguarded_hind(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai27check_target_stat_unguardedEP9lua_State"]
    pub(super) fn check_target_stat_unguarded(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_target_stat_comboEP9lua_State"]
    pub(super) fn check_target_stat_combo(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai28check_target_stat_no_counterEP9lua_State"]
    pub(super) fn check_target_stat_no_counter(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_target_stat_downEP9lua_State"]
    pub(super) fn check_target_stat_down(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_target_stat_fall_spEP9lua_State"]
    pub(super) fn check_target_stat_fall_sp(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai22check_target_stat_piyoEP9lua_State"]
    pub(super) fn check_target_stat_piyo(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_target_stat_piyo_lEP9lua_State"]
    pub(super) fn check_target_stat_piyo_l(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_target_stat_dragoonEP9lua_State"]
    pub(super) fn check_target_stat_dragoon(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_target_stat_cliffEP9lua_State"]
    pub(super) fn check_target_stat_cliff(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai27check_target_stat_cliff_actEP9lua_State"]
    pub(super) fn check_target_stat_cliff_act(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_target_stat_catchEP9lua_State"]
    pub(super) fn check_target_stat_catch(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_target_stat_damageEP9lua_State"]
    pub(super) fn check_target_stat_damage(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_target_stat_guardEP9lua_State"]
    pub(super) fn check_target_stat_guard(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_target_stat_escapeEP9lua_State"]
    pub(super) fn check_target_stat_escape(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai25check_target_stat_rebirthEP9lua_State"]
    pub(super) fn check_target_stat_rebirth(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_target_stat_attackEP9lua_State"]
    pub(super) fn check_target_stat_attack(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai29check_target_stat_attack_holdEP9lua_State"]
    pub(super) fn check_target_stat_attack_hold(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_target_stat_squatEP9lua_State"]
    pub(super) fn check_target_stat_squat(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai31check_target_stat_unable_attackEP9lua_State"]
    pub(super) fn check_target_stat_unable_attack(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai32check_target_stat_unable_specialEP9lua_State"]
    pub(super) fn check_target_stat_unable_special(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai35check_target_stat_specialflag_hoistEP9lua_State"]
    pub(super) fn check_target_stat_specialflag_hoist(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai21check_target_chr_statEP9lua_StateNS_13FighterAIStat15CharacterStatusENS_11FighterKindE"]
    pub(super) fn check_target_chr_stat(state: *mut lua_State, stat: app::FighterAIStat::CharacterStatus, fighter_kind: app::FighterKind) -> bool;

    #[link_name = "_ZN3app2ai12target_widthEP9lua_State"]
    pub(super) fn target_width(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai13target_heightEP9lua_State"]
    pub(super) fn target_height(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai12target_pos_xEP9lua_State"]
    pub(super) fn target_pos_x(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai12target_pos_yEP9lua_State"]
    pub(super) fn target_pos_y(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai14target_speed_xEP9lua_State"]
    pub(super) fn target_speed_x(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai14target_speed_yEP9lua_State"]
    pub(super) fn target_speed_y(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai12target_scaleEP9lua_State"]
    pub(super) fn target_scale(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai26target_jump_rest_availableEP9lua_State"]
    pub(super) fn target_jump_rest_available(state: *mut lua_State) -> u16;

    #[link_name = "_ZN3app2ai13target_damageEP9lua_State"]
    pub(super) fn target_damage(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai9target_lrEP9lua_State"]
    pub(super) fn target_lr(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai18target_status_kindEP9lua_State"]
    pub(super) fn target_status_kind(state: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai18target_motion_kindEP9lua_State"]
    pub(super) fn target_motion_kind(state: *mut lua_State) -> phx::Hash40;

    #[link_name = "_ZN3app2ai19target_motion_frameEP9lua_State"]
    pub(super) fn target_motion_frame(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai25target_hit_collision_rectEP9lua_State"]
    pub(super) fn target_hit_collision_rect(state: *mut lua_State) -> cpp::simd::Vector4;

    #[link_name = "_ZN3app2ai12lr_to_targetEP9lua_State"]
    pub(super) fn lr_to_target(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai20is_looking_at_targetEP9lua_State"]
    pub(super) fn is_looking_at_target(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18distance_to_targetEP9lua_State"]
    pub(super) fn distance_to_target(state: *mut lua_State) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai20distance_x_to_targetEP9lua_State"]
    pub(super) fn distance_x_to_target(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai20distance_y_to_targetEP9lua_State"]
    pub(super) fn distance_y_to_target(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai23is_target_on_same_floorEP9lua_State"]
    pub(super) fn is_target_on_same_floor(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_any_danger_targetEP9lua_State"]
    pub(super) fn check_any_danger_target(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24check_parent_over_groundEP9lua_State"]
    pub(super) fn check_parent_over_ground(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai23check_parent_same_floorEP9lua_State"]
    pub(super) fn check_parent_same_floor(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai12parent_pos_yEP9lua_State"]
    pub(super) fn parent_pos_y(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai14parent_speed_yEP9lua_State"]
    pub(super) fn parent_speed_y(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai22floor_edge_distance_lrEP9lua_Statef"]
    pub(super) fn floor_edge_distance_lr(state: *mut lua_State, lr: f32) -> f32;

    #[link_name = "_ZN3app2ai21floor_edge_distance_fEP9lua_State"]
    pub(super) fn floor_edge_distance_f(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai21floor_edge_distance_bEP9lua_State"]
    pub(super) fn floor_edge_distance_b(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai28floor_edge_distance_floor_lrEP9lua_State"]
    pub(super) fn floor_edge_distance_floor_lr(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai34floor_edge_distance_floor_lr_movedEP9lua_Statef"]
    pub(super) fn floor_edge_distance_floor_lr_moved(state: *mut lua_State, move_amount: f32) -> f32;

    #[link_name = "_ZN3app2ai11floor_widthEP9lua_State"]
    pub(super) fn floor_width(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai12floor_centerEP9lua_State"]
    pub(super) fn floor_center(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai8floor_lrEP9lua_State"]
    pub(super) fn floor_lr(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai29target_floor_edge_distance_lrEP9lua_Statef"]
    pub(super) fn target_floor_edge_distance_lr(state: *mut lua_State, lr: f32) -> f32;

    #[link_name = "_ZN3app2ai17check_over_groundEP9lua_State"]
    pub(super) fn check_over_ground(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai37check_over_ground_distance_current_lrEP9lua_Statef"]
    pub(super) fn check_over_ground_distance_current_lr(state: *mut lua_State, lr: f32) -> bool;

    #[link_name = "_ZN3app2ai24check_target_over_groundEP9lua_State"]
    pub(super) fn check_target_over_ground(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai15check_over_goalEP9lua_State"]
    pub(super) fn check_over_goal(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai9floor_posEP9lua_Stateb"]
    pub(super) fn floor_pos(state: *mut lua_State, flag: bool) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai15floor_pos_movedEP9lua_Statefb"]
    pub(super) fn floor_pos_moved(state: *mut lua_State, move_amount: f32, flag: bool) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai18floor_pos_floor_lrEP9lua_Statefb"]
    pub(super) fn floor_pos_floor_lr(state: *mut lua_State, lr: f32, flag: bool) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai11floor_movesEP9lua_State"]
    pub(super) fn floor_moves(state: *mut lua_State) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai10return_posEP9lua_Stateb"]
    pub(super) fn return_pos(state: *mut lua_State, flag: bool) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai15safe_return_posEP9lua_Stateb"]
    pub(super) fn safe_return_pos(state: *mut lua_State, flag: bool) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai8goal_posEP9lua_State"]
    pub(super) fn goal_pos(state: *mut lua_State) -> cpp::simd::Vector2;

    #[link_name = "_ZN3app2ai16check_away_floorEP9lua_State"]
    pub(super) fn check_away_floor(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24is_offensive_on_strategyEP9lua_State"]
    pub(super) fn is_offensive_on_strategy(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai24is_defensive_on_strategyEP9lua_State"]
    pub(super) fn is_defensive_on_strategy(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai11press_frameEP9lua_State"]
    pub(super) fn press_frame(state: *mut lua_State) -> u8;

    #[link_name = "_ZN3app2ai9push_waitEP9lua_State"]
    pub(super) fn push_wait(state: *mut lua_State) -> u8;

    #[link_name = "_ZN3app2ai13change_actionEP9lua_Statet"]
    pub(super) fn change_action(state: *mut lua_State, command_id: app::FighterAIAct::CmdId);

    #[link_name = "_ZN3app2ai13set_auto_stopEP9lua_Statei"]
    pub(super) fn set_auto_stop(state: *mut lua_State, stop_frame: i32);

    #[link_name = "_ZN3app2ai12update_countEP9lua_State"]
    pub(super) fn update_count(state: *mut lua_State) -> i32;

    #[link_name = "_ZN3app2ai19is_update_count_oddEP9lua_State"]
    pub(super) fn is_update_count_odd(state: *mut lua_State) -> bool;

    #[link_name = "_ZN3app2ai18reset_return_countEP9lua_State"]
    pub(super) fn reset_return_count(state: *mut lua_State);

    #[link_name = "_ZN3app2ai19set_no_return_frameEP9lua_Statei"]
    pub(super) fn set_no_return_frame(state: *mut lua_State, frame: i32);

    #[link_name = "_ZN3app2ai22get_cmd_id_from_req_idEP9lua_StateNS_12FighterAIAct5ReqIdE"]
    pub(super) fn get_cmd_id_from_req_id(state: *mut lua_State, request_id: app::FighterAIAct::ReqId) -> app::FighterAIAct::CmdId;

    #[link_name = "_ZN3app2ai35get_cmd_id_from_req_id_with_predictEP9lua_StateNS_12FighterAIAct5ReqIdEfff"]
    pub(super) fn get_cmd_id_from_req_id_with_predict(state: *mut lua_State, request_id: app::FighterAIAct::ReqId, lr: f32, predict1: f32, predict2: f32) -> app::FighterAIAct::CmdId;

    #[link_name = "_ZN3app2ai31get_cmd_probability_from_req_idEP9lua_StateNS_12FighterAIAct5ReqIdENS3_5CmdIdE"]
    pub(super) fn get_cmd_probability_from_req_id(state: *mut lua_State, request_id: app::FighterAIAct::ReqId, command_id: app::FighterAIAct::CmdId) -> f32;

    #[link_name = "_ZN3app2ai14enable_commandEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn enable_command(state: *mut lua_State, command_id: app::FighterAIAct::CmdId);

    #[link_name = "_ZN3app2ai15disable_commandEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn disable_command(state: *mut lua_State, command_id: app::FighterAIAct::CmdId);

    #[link_name = "_ZN3app2ai26disable_command_ground_allEP9lua_State"]
    pub(super) fn disable_command_ground_all(state: *mut lua_State);

    #[link_name = "_ZN3app2ai23disable_command_air_allEP9lua_State"]
    pub(super) fn disable_command_air_all(state: *mut lua_State);

    #[link_name = "_ZN3app2ai33disable_command_attack_button_allEP9lua_State"]
    pub(super) fn disable_command_attack_button_all(state: *mut lua_State);

    #[link_name = "_ZN3app2ai34disable_command_special_button_allEP9lua_State"]
    pub(super) fn disable_command_special_button_all(state: *mut lua_State);

    #[link_name = "_ZN3app2ai32reset_cmd_id_probability_add_2ndEP9lua_State"]
    pub(super) fn reset_cmd_id_probability_add_2nd(state: *mut lua_State);

    #[link_name = "_ZN3app2ai30set_cmd_id_probability_add_2ndEP9lua_StateNS_12FighterAIAct5CmdIdEf"]
    pub(super) fn set_cmd_id_probability_add_2nd(state: *mut lua_State, command_id: app::FighterAIAct::CmdId, add_2nd: f32);

    #[link_name = "_ZN3app2ai32reset_cmd_id_probability_mul_2ndEP9lua_State"]
    pub(super) fn reset_cmd_id_probability_mul_2nd(state: *mut lua_State);

    #[link_name = "_ZN3app2ai30set_cmd_id_probability_mul_2ndEP9lua_StateNS_12FighterAIAct5CmdIdEf"]
    pub(super) fn set_cmd_id_probability_mul_2nd(state: *mut lua_State, command_id: app::FighterAIAct::CmdId, mul_2nd: f32);

    #[link_name = "_ZN3app2ai42set_cmd_id_probability_mul_for_specializerEP9lua_StateNS_12FighterAIAct5CmdIdEf"]
    pub(super) fn set_cmd_id_probability_mul_for_specializer(state: *mut lua_State, command_id: app::FighterAIAct::CmdId, mul: f32);

    #[link_name = "_ZN3app2ai26get_cmd_id_probability_mulEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn get_cmd_id_probability_mul(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> f32;

    #[link_name = "_ZN3app2ai21predict_landing_frameEP9lua_State"]
    pub(super) fn predict_landing_frame(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai28predict_target_landing_frameEP9lua_State"]
    pub(super) fn predict_target_landing_frame(state: *mut lua_State) -> f32;

    #[link_name = "_ZN3app2ai28predict_hit_in_target_attackEP9lua_StateNS_12FighterAIAct5CmdIdEff"]
    pub(super) fn predict_hit_in_target_attack(state: *mut lua_State, command_id: app::FighterAIAct::CmdId, predict1: f32, predict2: f32) -> bool;

    #[link_name = "_ZN3app2ai40predict_hit_in_target_attack_from_motionEP9lua_StateN3phx6Hash40Eff"]
    pub(super) fn predict_hit_in_target_attack_from_motion(state: *mut lua_State, motion: phx::Hash40, predict1: f32, predict2: f32) -> bool;

    #[link_name = "_ZN3app2ai28predict_target_hit_in_attackEP9lua_StateNS_12FighterAIAct5CmdIdEffff"]
    pub(super) fn predict_target_hit_in_attack(state: *mut lua_State, command_id: app::FighterAIAct::CmdId, start_frame: f32, mul_start_frame: f32, predict1: f32, predict2: f32) -> bool;

    #[link_name = "_ZN3app2ai35check_line_segment_vs_target_attackEP9lua_StateN3phx6Hash40ERKNS3_8Vector2fES7_RNS3_8Vector4fE"]
    pub(super) fn check_line_segment_vs_target_attack(state: *mut lua_State, motion: phx::Hash40, line_a: *const phx::Vector2f, line_b: *const phx::Vector2f, out: *mut phx::Vector4f) -> bool;

    #[link_name = "_ZN3app2ai18attack_start_frameEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_start_frame(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> i32;

    #[link_name = "_ZN3app2ai25target_attack_start_frameEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn target_attack_start_frame(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> i32;

    #[link_name = "_ZN3app2ai16attack_end_frameEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_end_frame(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> i32;

    #[link_name = "_ZN3app2ai19attack_cancel_frameEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_cancel_frame(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> i32;

    #[link_name = "_ZN3app2ai14attack_data_x0EP9lua_StateN3phx6Hash40E"]
    pub(super) fn attack_data_x0(state: *mut lua_State, motion: phx::Hash40) -> f32;

    #[link_name = "_ZN3app2ai14attack_data_x1EP9lua_StateN3phx6Hash40E"]
    pub(super) fn attack_data_x1(state: *mut lua_State, motion: phx::Hash40) -> f32;

    #[link_name = "_ZN3app2ai14attack_data_y0EP9lua_StateN3phx6Hash40E"]
    pub(super) fn attack_data_y0(state: *mut lua_State, motion: phx::Hash40) -> f32;

    #[link_name = "_ZN3app2ai14attack_data_y1EP9lua_StateN3phx6Hash40E"]
    pub(super) fn attack_data_y1(state: *mut lua_State, motion: phx::Hash40) -> f32;

    #[link_name = "_ZN3app2ai22attack_info_needs_turnEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_info_needs_turn(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> bool;

    #[link_name = "_ZN3app2ai20attack_info_reactionEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_info_reaction(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> f32;

    #[link_name = "_ZN3app2ai21attack_info_no_shieldEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_info_no_shield(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> bool;

    #[link_name = "_ZN3app2ai18attack_info_meteorEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_info_meteor(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> bool;

    #[link_name = "_ZN3app2ai23attack_info_reflectableEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_info_reflectable(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> bool;

    #[link_name = "_ZN3app2ai19attack_is_as_weaponEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_is_as_weapon(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> bool;

    #[link_name = "_ZN3app2ai20attack_info_distanceEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn attack_info_distance(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> f32;

    #[link_name = "_ZN3app2ai16motion_to_cmd_idEP9lua_StateN3phx6Hash40E"]
    pub(super) fn motion_to_cmd_id(state: *mut lua_State, motion: phx::Hash40) -> app::FighterAIAct::CmdId;

    #[link_name = "_ZN3app2ai29target_attack_info_needs_turnEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn target_attack_info_needs_turn(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> bool;

    #[link_name = "_ZN3app2ai28target_attack_info_no_shieldEP9lua_StateNS_12FighterAIAct5CmdIdE"]
    pub(super) fn target_attack_info_no_shield(state: *mut lua_State, command_id: app::FighterAIAct::CmdId) -> bool;

    #[link_name = "_ZN3app2ai23target_motion_to_cmd_idEP9lua_StateN3phx6Hash40E"]
    pub(super) fn target_motion_to_cmd_id(state: *mut lua_State, motion: phx::Hash40) -> app::FighterAIAct::CmdId;

    #[link_name = "_ZN3app2ai32current_attack_combo_next_motionEP9lua_State"]
    pub(super) fn current_attack_combo_next_motion(state: *mut lua_State) -> phx::Hash40;

    #[link_name = "_ZN3app2ai39target_current_attack_combo_next_motionEP9lua_State"]
    pub(super) fn target_current_attack_combo_next_motion(state: *mut lua_State) -> phx::Hash40;

    #[link_name = "_ZN3app2ai37target_attack_start_frame_from_motionEP9lua_StateN3phx6Hash40E"]
    pub(super) fn target_attack_start_frame_from_motion(state: *mut lua_State, motion: phx::Hash40) -> i32;
}