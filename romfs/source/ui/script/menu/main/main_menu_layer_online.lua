local value = {}

button:add_id("ONLINE", "MELEE", 1)
button:add_id("ONLINE_MELEE", "ROOM", 1)
button:add_id("ONLINE", "WATCH", 2)
button:add_id("ONLINE", "OPTION", 3)
button:add_id("ONLINE", "SHARE", 4)
button:add_id("ONLINE", "TOURNAMENT", 5)
button:add_id("ONLINE", "BANNER", 6)

function value:initialize()
    self.layout_root = LayoutRootList.main_menu
    self.layout_view = self.layout_root:get_root_view()
    self.virtual_input = self.layout_root:get_virtual_input()
    self.in_anim = "appear_online_top"
    self.out_anim = "disappear_online_top"
    self.header = {}
    self.header.layout_view = self.layout_view:get_parts("set_header_online")
    self.selector = LayoutButtonSelector.new()
    self.selector_parts = "set_menu_online"
    self.selector_name = "selector_0"
    self.selector_config = LayoutButtonSelectorConfig.new()
    self.selector_config.bring_front_on_select = true
    self.selector_config.directional_degree = 60
    self.selector_config.use_cursor_looping = false
    self.selector_config.is_unique_se = true
    self.selector_config.decide_se_label_code = "se_system_fixed_s"
    self.selector_config.cancel_se_label_code = "se_system_cancel"
    self.selector_config.cursor_se_label_code = "se_system_cursor_l"
    self.selector_initial_id = button.ONLINE_MELEE_ROOM
    self.button_table = {}
    self.button_table[button.ONLINE_MELEE_ROOM] = {
        parts = "set_parts_btn_00",
        sequence = SEQUENCE_ONLINE_MELEE_ROOM
    }
    self.button_table[button.ONLINE_WATCH] = {
        parts = "set_parts_btn_01",
        sequence = SEQUENCE_ONLINE_WATCH
    }
    self.button_table[button.ONLINE_OPTION] = {
        parts = "set_parts_btn_02",
        sequence = SEQUENCE_OPTION_ONLINE
    }
    self.button_table[button.ONLINE_SHARE] = {
        parts = "set_parts_btn_03",
        sequence = SEQUENCE_ONLINE_SHARE
    }
    self.button_table[button.ONLINE_TOURNAMENT] = {
        parts = "set_parts_btn_04",
        sequence = SEQUENCE_ONLINE_MELEE_TOURNAMENT
    }
    self.button_table[button.ONLINE_BANNER] = {
        parts = "set_parts_btn_05",
        func_decide = self.callback_banner_decide,
        func_lr = self.callback_banner_lr
    }

    self.footer_table = {}
    self.footer_table[button.ONLINE_MELEE_ROOM] = {
        label = "mnu_onl_mel_top_help_room"
    }
    self.footer_table[button.ONLINE_WATCH] = {
        label = "mnu_onl_top_help_spectate"
    }
    self.footer_table[button.ONLINE_OPTION] = {
        label = "mnu_onl_top_help_opt_online"
    }
    self.footer_table[button.ONLINE_SHARE] = {
        label = "mnu_onl_top_help_contribution"
    }
    self.footer_table[button.ONLINE_TOURNAMENT] = {
        label = "mnu_onl_top_help_tournament"
    }
    self.footer_table[button.ONLINE_BANNER] = {
        label = "mnu_onl_top_event_help_banner"
    }

    self.preview_table = {}
    self.preview_table.layout_view = self.layout_view:get_parts("set_menu_preview_online")

    self.preview_table[button.ONLINE_MELEE] = {
        tag = "select_melee"
    }
    self.preview_table[button.ONLINE_WATCH] = {
        tag = "select_spectate"
    }
    self.preview_table[button.ONLINE_OPTION] = {
        tag = "select_option"
    }
    self.preview_table[button.ONLINE_SHARE] = {
        tag = "select_post"
    }
    self.preview_table[button.ONLINE_TOURNAMENT] = {
        tag = "select_tournament"
    }
    self.preview_table[button.ONLINE_BANNER] = {
        tag = "select_event"
    }

    self.howto_table = {}
    self.howto_table[button.ONLINE_WATCH] = {
        id = "UI_HOWTOPLAY_SPECTATE_TOP"
    }
    self.howto_table[button.ONLINE_OPTION] = {
        id = "UI_HOWTOPLAY_ONLINE_OPTION_TOP"
    }
    self.howto_table[button.ONLINE_SHARE] = {
        id = "UI_HOWTOPLAY_EVERYONES_POST_TOP"
    }
    self.howto_table[button.ONLINE_TOURNAMENT] = {
        id = "UI_HOWTOPLAY_ONLINE_TOURNAMENT_TOP"
    }

    self.banner_table = actor:get_online_banner_data()
    local selector = self.layout_view:get_parts(self.selector_parts)
    if 0 < #self.banner_table then
        local banner_parts = selector:get_parts(self.button_table[button.ONLINE_BANNER].parts)
        self.banner = UiScriptPlayer.require("menu/main/main_menu_banner")
        self.banner:initialize(banner_parts, self.banner_table)
        common.play_animation(selector, "show_banner")
    else
        common.play_animation(selector, "hide_banner")
    end
end

function value:setup_selector()
    local is_allowed_to_share = prg_func.is_free_communication_permission()
    local selector = self.layout_view:get_parts(self.selector_parts)
    local share_button = selector:get_parts(self.button_table[button.ONLINE_SHARE].parts)
    common.play_animation(share_button, is_allowed_to_share and "hide_watch" or "show_watch")
    local is_tourney = actor:is_in_period_online_tournament()
    local tourney_button = selector:get_parts(self.button_table[button.ONLINE_TOURNAMENT].parts)
    common.play_animation(tourney_button, is_tourney and "event_on" or "event_off")
    self.selector:select_button(self.selector_initial_id, true)
end

function value:callback_banner_decide()
    if self.banner then
        common.wait(1)
        self.banner:decide()
    end
end

function value:callback_banner_lr(lr)
    if self.banner then
        self.banner:move_lr(lr, true)
    end
end

function value:update_state_in()
    if state:is_first() and not resume then
        if Network:connect() == true then
            AppPopupManager:open_database("ID_POPUP_ONLINE_CONNECTING_CAN")
            repeat
                coroutine.yield()
            until AppPopupManager:is_idle() == true
        end
        while Network:connecting() == true do
            if AppPopupManager:is_busy() == false then
                Network:cancel()
            end
            coroutine.yield()
        end
        if AppPopupManager:is_busy() then
            AppPopupManager:close()
        end
        repeat
            coroutine.yield()
        until AppPopupManager:is_busy() == false
        if Network:is_connected() == false then
            state:pop()
            Network:sleep_error()
            return
        end
        if prg_func.is_first_online() then
            prg_func.show_online_terms()
            while prg_func.is_showing_online_terms() do
                coroutine.yield()
            end
        end
        if not prg_func.is_valid_profile() then
            root_type = SEQUENCE_OPTION_ONLINE
            prg_func.set_resume_button_id(button.ONLINE_MELEE_ROOM)
            state:set(STATE_TERM)
            return
        end
    end
    prg_func.set_sequence_online(true)
    common.update_state_in(self)
end

function value:update_state_main()
    if Network:is_connected() == false then
        state:advance()
    end
    common.update_state_main(self)
end

function value:update_state_out()
    prg_func.set_sequence_online(false)
    Network:sleep_error()
    common.update_state_out(self)
end

return value