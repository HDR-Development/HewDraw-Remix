local value = {}

button:add_id(nil, "MELEE", 1)
button:add_id(nil, "SPIRITS", 2)
button:add_id(nil, "OTHER", 3)
button:add_id(nil, "ONLINE", 4)
button:add_id(nil, "COLLECTION", 5)
button:add_id(nil, "ESHOP", 6)

function value:initialize()
    self.layout_root = LayoutRootList.main_menu
    self.layout_view = self.layout_root:get_root_view()
    self.virtual_input = self.layout_root:get_virtual_input()
    self.in_anim = "in"
    self.out_anim = "out"

    self.selector = LayoutButtonSelector.new()
    self.selector_name = "selector_00"
    
    self.selector_config = LayoutButtonSelectorConfig.new()
    self.selector_config.directional_degree = 60
    self.selector_config.use_cursor_looping = false
    self.selector_config.is_unique_se = true
    self.selector_config.decide_se_label_code = "se_system_fixed_s"
    self.selector_config.cancel_se_label_code = "se_system_cancel"
    self.selector_config.cursor_se_label_code = "se_system_cursor_l"
    self.selector_initial_id = button.MELEE

    self.button_table = {}
    self.button_table[button.MELEE] = {
        parts = "set_parts_btn_melee",
        state = STATE_MELEE_IN
    }
    self.button_table[button.SPIRITS] = {
        parts = "set_parts_btn_spirits",
        state = STATE_SPIRITS_IN
    }
    self.button_table[button.OTHER] = {
        parts = "set_parts_btn_other",
        state = STATE_OTHER_IN
    }
    self.button_table[button.ONLINE] = {
        parts = "set_parts_btn_online",
        state = STATE_ONLINE_IN
    }
    self.button_table[button.COLLECTION] = {
        parts = "set_parts_btn_colle",
        state = STATE_COLLECTION_IN
    }
    self.button_table[button.ESHOP] = {
        parts = "set_parts_btn_eshop",
        func_decide = self.callback_eshop_decide
    }

    self.footer_table = {}
    self.footer_table[button.MELEE] = {
        label = "mnu_top_help_melee"
    }
    self.footer_table[button.SPIRITS] = {
        label = "mnu_top_help_spirits"
    }
    self.footer_table[button.OTHER] = {
        label = "mnu_top_help_other"
    }
    self.footer_table[button.ONLINE] = {
        label = "mnu_top_help_online"
    }
    self.footer_table[button.COLLECTION] = {
        label = "mnu_top_help_collection"
    }
    self.footer_table[button.ESHOP] = {
        label = "mnu_top_help_eshop"
    }

    self.preview_table = {}
    self.preview_table.layout_view = self.layout_view:get_parts("set_parts_center")
    self.preview_table[button.MELEE] = {
        tag = "select_melee"
    }
    self.preview_table[button.SPIRITS] = {
        tag = "select_spirits"
    }
    self.preview_table[button.OTHER] = {
        tag = "select_other"
    }
    self.preview_table[button.ONLINE] = {
        tag = "select_online"
    }
    self.preview_table[button.COLLECTION] = {
        tag = "select_collection"
    }
    self.preview_table[button.ESHOP] = {
        tag = "select_eshop"
    }
end

function value:callback_eshop_decide()
    common.wait(1)
    common.show_eshop()
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