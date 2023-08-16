local value = {}

button:add_id("OTHER", "STANDARD", 1)
button:add_id("OTHER", "TRAINING", 2)
button:add_id("OTHER", "KUMITE", 3)
button:add_id("OTHER", "CHARAMAKE", 4)
button:add_id("OTHER", "AMIIBO", 5)
button:add_id("OTHER", "CHALLENGER", 6)
button:add_id("OTHER", "STAGEMAKE", 7)
button:add_id("OTHER", "VR", 8)
button:add_id("OTHER", "HOMERUN", 9)

function value:initialize()
    self.layout_root = LayoutRootList.main_menu
    self.layout_view = self.layout_root:get_root_view()
    self.virtual_input = self.layout_root:get_virtual_input()
    self.in_anim = "appear_other_top"
    self.out_anim = "disappear_other_top"
    self.header = {}
    self.header.layout_view = self.layout_view:get_parts("set_header_other")
    self.selector = LayoutButtonSelector.new()
    self.selector_parts = "set_menu_other"
    self.selector_name = "selector_0"
    self.selector_config = LayoutButtonSelectorConfig.new()
    self.selector_config.bring_front_on_select = true
    self.selector_config.directional_degree = 60
    self.selector_config.use_cursor_looping = false
    self.selector_config.is_unique_se = true
    self.selector_config.decide_se_label_code = "se_system_fixed_s"
    self.selector_config.cancel_se_label_code = "se_system_cancel"
    self.selector_config.cursor_se_label_code = "se_system_cursor_l"
    self.selector_initial_id = button.OTHER_TRAINING
    self.button_table = {}
    self.button_table[button.OTHER_STANDARD] = {
        parts = "set_parts_btn_00",
        sequence = SEQUENCE_OTHER_STANDARD
    }
    self.button_table[button.OTHER_TRAINING] = {
        parts = "set_parts_btn_01",
        sequence = SEQUENCE_OTHER_TRAINING
    }
    self.button_table[button.OTHER_KUMITE] = {
        parts = "set_parts_btn_02",
        state = STATE_KUMITE_IN
    }
    self.button_table[button.OTHER_CHARAMAKE] = {
        parts = "set_parts_btn_03",
        sequence = SEQUENCE_OTHER_CHARA_MAKE
    }
    self.button_table[button.OTHER_AMIIBO] = {
        parts = "set_parts_btn_04",
        sequence = SEQUENCE_OTHER_AMIIBO
    }
    self.button_table[button.OTHER_CHALLENGER] = {
        parts = "set_parts_btn_05",
        sequence = SEQUENCE_OTHER_CHALLENGER
    }
    self.button_table[button.OTHER_STAGEMAKE] = {
        parts = "set_parts_btn_06",
        sequence = SEQUENCE_OTHER_STAGE_MAKE
    }
    self.button_table[button.OTHER_VR] = {
        parts = "set_parts_btn_07",
        sequence = SEQUENCE_OTHER_VR
    }
    self.button_table[button.OTHER_HOMERUN] = {
        parts = "set_parts_btn_08",
        state = STATE_HOMERUN_IN
    }

    self.footer_table = {}
    self.footer_table[button.OTHER_STANDARD] = {
        label = "mnu_oth_top_help_standard"
    }
    self.footer_table[button.OTHER_TRAINING] = {
        label = "mnu_oth_top_help_training"
    }
    self.footer_table[button.OTHER_KUMITE] = {
        label = "mnu_oth_top_help_sparring"
    }
    self.footer_table[button.OTHER_CHARAMAKE] = {
        label = "mnu_oth_top_help_make_miifighter"
    }
    self.footer_table[button.OTHER_AMIIBO] = {
        label = "mnu_oth_top_help_amiibo"
    }
    self.footer_table[button.OTHER_CHALLENGER] = {
        label = "mnu_oth_top_help_challengers"
    }
    self.footer_table[button.OTHER_STAGEMAKE] = {
        label = "mnu_oth_top_help_build_stage"
    }
    self.footer_table[button.OTHER_VR] = {
        label = "mnu_oth_top_help_vr"
    }
    self.footer_table[button.OTHER_HOMERUN] = {
        label = "mnu_oth_top_help_homerun"
    }

    self.preview_table = {}
    self.preview_table.layout_view = self.layout_view:get_parts("set_menu_preview_other")
    self.preview_table[button.OTHER_STANDARD] = {
        tag = "select_standard"
    }
    self.preview_table[button.OTHER_TRAINING] = {
        tag = "select_training"
    }
    self.preview_table[button.OTHER_KUMITE] = {
        tag = "select_sparring"
    }
    self.preview_table[button.OTHER_CHARAMAKE] = {
        tag = "select_make_miifighter"
    }
    self.preview_table[button.OTHER_AMIIBO] = {
        tag = "select_amiibo"
    }
    self.preview_table[button.OTHER_CHALLENGER] = {
        tag = "select_challengers"
    }
    self.preview_table[button.OTHER_STAGEMAKE] = {
        tag = "select_edit_stage"
    }
    self.preview_table[button.OTHER_VR] = {
        tag = "select_vr"
    }
    self.preview_table[button.OTHER_HOMERUN] = {
        tag = "select_homerun"
    }

    self.howto_table = {}
    self.howto_table[button.OTHER_STANDARD] = {
        id = "UI_HOWTOPLAY_STANDARD_TOP"
    }
    self.howto_table[button.OTHER_KUMITE] = {
        id = "UI_HOWTOPLAY_KUMITE_TOP"
    }
    self.howto_table[button.OTHER_CHARAMAKE] = {
        id = "UI_HOWTOPLAY_MIIFIGHTER_TOP"
    }
    self.howto_table[button.OTHER_AMIIBO] = {
        id = "UI_HOWTOPLAY_AMIIBO_TOP"
    }
    self.howto_table[button.OTHER_CHALLENGER] = {
        id = "UI_HOWTOPLAY_CHALLENGER_TOP"
    }
    self.howto_table[button.OTHER_STAGEMAKE] = {
        id = "UI_HOWTOPLAY_STAGE_BUILDER_TOP"
    }
    self.howto_table[button.OTHER_VR] = {
        id = "UI_HOWTOPLAY_VR_MELEE_TOP"
    }
    self.howto_table[button.OTHER_HOMERUN] = {
        id = "UI_HOWTOPLAY_HOMERUN_TOP"
    }
end

function value:setup_selector()
    local is_challenger = prg_func.get_challenger_state() == CHALLENGER_OPEN
    local selector = self.layout_view:get_parts(self.selector_parts)
    local challenger_parts = selector:get_parts(self.button_table[button.OTHER_CHALLENGER].parts)
    local challenger_btn = self.selector:get_button(button.OTHER_CHALLENGER)
    common.play_animation(challenger_parts, is_challenger and "show" or "hide")
    challenger_btn:set_selectable(is_challenger)
    challenger_btn:set_decidable(is_challenger)
    local view_anim = prg_func.is_locale_japan() and "view_jp" or "view_other"
    local charamake_parts = selector:get_parts(self.button_table[button.OTHER_CHARAMAKE].parts)
    common.play_animation(charamake_parts, view_anim)
    local stagemake_parts = selector:get_parts(self.button_table[button.OTHER_STAGEMAKE].parts)
    common.play_animation(stagemake_parts, view_anim)
end

function value:update_state_in()
    common.update_state_in(self)
end

function value:update_state_main()
    common.update_state_main(self)
end

function value:update_state_out()
    common.update_state_out(self)
end

return value