--[[
FILE: option_button_controller.lua
Reference Code: option_button_controller.lc

Author: blujay

Notes: The following code has been decompiled with the assistance of a modified
binary of the DSLuaDecompiler to produce *some* output, while not very meaningful it helps
outline the structure of some functions so that the disassembled lua could be used
as a reference instead of the guiding light

The following code was all written by hand, and was compiled via https://github.com/ultimate-research/smash-lua
and disassembled using https://github.com/jam1garner/smash-luadec

The output disassembly of this file is guaranteed to match exactly that of option_button_controller.lc
when unmodified.
]]--

local ui_common = UiScriptPlayer.require("common/ui_common")
layout_view = nil
exit_code = SCENE_EXIT_CODE_NONE
button_is_edited = nil
updated_button_id = nil

local button_selector = nil
local button_selector_config = nil
local guide_layout = nil
local parts_guide = nil
local can_test_play = false

local play_select_full_key_animation = function (button_id)
    local selection = "ctr_normal_sel_"
    if IS_PRO == true then
        selection = "ctr_pro_sel_"
    end

    layout_view = layout_root:get_root_view()
    if button_id == BUTTON_ID_FK_TRIGGER_L then
        layout_view:play_animation(selection.."l", 1.0)
    elseif button_id == BUTTON_ID_FK_TRIGGER_R then
        layout_view:play_animation(selection.."r", 1.0)
    elseif button_id == BUTTON_ID_FK_TRIGGER_ZL then
        layout_view:play_animation(selection.."zl", 1.0)
    elseif button_id == BUTTON_ID_FK_TRIGGER_ZR then
        layout_view:play_animation(selection.."zr", 1.0)
    elseif button_id == BUTTON_ID_FK_UP then
        layout_view:play_animation(selection.."ct", 1.0)
    elseif button_id == BUTTON_ID_FK_SIDE then
        layout_view:play_animation(selection.."clr", 1.0)
    elseif button_id == BUTTON_ID_FK_DOWN then
        layout_view:play_animation(selection.."cb", 1.0)
    elseif button_id == BUTTON_ID_FK_BUTTON_A then
        layout_view:play_animation(selection.."a", 1.0)
    elseif button_id == BUTTON_ID_FK_BUTTON_B then
        layout_view:play_animation(selection.."b", 1.0)
    elseif button_id == BUTTON_ID_FK_STICK_R then
        layout_view:play_animation(selection.."sr", 1.0)
    elseif button_id == BUTTON_ID_FK_BUTTON_X then
        layout_view:play_animation(selection.."x", 1.0)
    elseif button_id == BUTTON_ID_FK_BUTTON_Y then
        layout_view:play_animation(selection.."y", 1.0)
    else
        layout_view:play_animation(selection.."off", 1.0)
    end
end

local play_select_joy_key_animation = function (button_id)
    local selection = "ctr_joy_sel_"

    layout_view = layout_root:get_root_view()

    if button_id == BUTTON_ID_JC_TRIGGER_LR then
        layout_view:play_animation(selection.."l", 1.0)
        layout_view:play_animation(selection.."r", 1.0)
    elseif button_id == BUTTON_ID_JC_TRIGGER_ZLR then
        layout_view:play_animation(selection.."zl", 1.0)
        layout_view:play_animation(selection.."zr", 1.0)
    elseif button_id == BUTTON_ID_JC_TRIGGER_SL then
        layout_view:play_animation(selection.."sl", 1.0)
    elseif button_id == BUTTON_ID_JC_TRIGGER_SR then
        layout_view:play_animation(selection.."sr", 1.0)
    elseif button_id == BUTTON_ID_JC_UP then
        layout_view:play_animation(selection.."ct", 1.0)
    elseif button_id == BUTTON_ID_JC_RIGHT then
        layout_view:play_animation(selection.."cr", 1.0)
    elseif button_id == BUTTON_ID_JC_LEFT then
        layout_view:play_animation(selection.."cl", 1.0)
    elseif button_id == BUTTON_ID_JC_DOWN then
        layout_view:play_animation(selection.."cb", 1.0)
    else
        layout_view:play_animation(selection.."off", 1.0)
    end
end

local play_select_gc_key_animation = function (button_id)
    local selection = "ctr_gc_sel_"

    layout_view = layout_root:get_root_view()

    if button_id == BUTTON_ID_GC_TRIGGER_L then
        layout_view:play_animation(selection.."l", 1.0)
    elseif button_id == BUTTON_ID_GC_TRIGGER_R then
        layout_view:play_animation(selection.."r", 1.0)
    elseif button_id == BUTTON_ID_GC_TRIGGER_Z then
        layout_view:play_animation(selection.."z", 1.0)
    elseif button_id == BUTTON_ID_GC_UP then
        layout_view:play_animation(selection.."ct", 1.0)
    elseif button_id == BUTTON_ID_GC_SIDE then
        layout_view:play_animation(selection.."clr", 1.0)
    elseif button_id == BUTTON_ID_GC_DOWN then
        layout_view:play_animation(selection.."cb", 1.0)
    elseif button_id == BUTTON_ID_GC_BUTTON_A then
        layout_view:play_animation(selection.."a", 1.0)
    elseif button_id == BUTTON_ID_GC_BUTTON_B then
        layout_view:play_animation(selection.."b", 1.0)
    elseif button_id == BUTTON_ID_GC_STICK_C then
        layout_view:play_animation(selection.."c", 1.0)
    elseif button_id == BUTTON_ID_GC_BUTTON_X then
        layout_view:play_animation(selection.."x", 1.0)
    elseif button_id == BUTTON_ID_GC_BUTTON_Y then
        layout_view:play_animation(selection.."y", 1.0)
    else
        layout_view:play_animation(selection.."off", 1.0)
    end
end

local play_select_key_animation = function (controller_id, button_id)
    if controller_id == CONTROLLER_FULL_KEY then
        play_select_full_key_animation(button_id)
    elseif controller_id == CONTROLLER_JOY_CON then
        play_select_joy_key_animation(button_id)
    elseif controller_id == CONTROLLER_GC_CON then
        play_select_gc_key_animation(button_id)
    end
end

local is_button_joycon_trigger = function (button_id)
    return button_id == BUTTON_ID_JC_TRIGGER_LR or button_id == BUTTON_ID_JC_TRIGGER_ZLR
end

local is_button_right_stick = function (controller_id, button_id)
    if controller_id == CONTROLLER_FULL_KEY then
        return button_id == BUTTON_ID_FK_STICK_R
    elseif controller_id == CONTROLLER_JOY_CON then
        return false
    elseif controller_id == CONTROLLER_GC_CON then
        return button_id == BUTTON_ID_GC_STICK_C
    end
end

local set_button_text = function (controller_id, button_id, layout)
    local text_pane = layout:get_pane("set_txt_name_00")

    local operation = UiScriptPlayer.invoke("get_operation", controller_id, button_id)
    if operation == OPERATE_ATTACK then
        if is_button_right_stick(controller_id, button_id) == true then
           text_pane:set_text_message(MSG_LABEL_ATTACK_STRONG) 
        else
            text_pane:set_text_message(MSG_LABEL_ATTACK)
        end
    elseif operation == OPERATE_SPECIAL then
        text_pane:set_text_message(MSG_LABEL_SPECIAL)
    elseif operation == OPERATE_JUMP then
        text_pane:set_text_message(MSG_LABEL_JUMP)
    elseif operation == OPERATE_GUARD then
        text_pane:set_text_message(MSG_LABEL_GUARD)
    elseif operation == OPERATE_CATCH then
        text_pane:set_text_message(MSG_LABEL_CATCH)
    elseif operation == OPERATE_SMASH then
        text_pane:set_text_message(MSG_LABEL_SMASH)
    elseif operation == OPERATE_SMASH_UP then
        text_pane:set_text_message(MSG_LABEL_SMASH_UP)
    elseif operation == OPERATE_SMASH_SIDE then
        text_pane:set_text_message(MSG_LABEL_SMASH_SIDE)
    elseif operation == OPERATE_SMASH_DOWN then
        text_pane:set_text_message(MSG_LABEL_SMASH_DOWN)
    elseif operation == OPERATE_APPEAL then
        text_pane:set_text_message(MSG_LABEL_APPEAL)
    elseif operation == OPERATE_APPEAL_UP then
        text_pane:set_text_message(MSG_LABEL_APPEAL_UP)
    elseif operation == OPERATE_APPEAL_SIDE then
        text_pane:set_text_message(MSG_LABEL_APPEAL_SIDE)
    elseif operation == OPERATE_APPEAL_DOWN then
        text_pane:set_text_message(MSG_LABEL_APPEAL_DOWN)
    elseif operation == 0x12 then -- HDR exclusive, operate ID for Shorthop
        text_pane:set_text_message("mnu_opt_btn_key_short_hop")
    elseif operation == 0x13 then -- HDR exclusive, operate ID for Tilt Attack
        text_pane:set_text_message("mnu_opt_btn_key_tilt_attack")
    elseif operation == 0x14 then -- HDR exclusive, operate ID for Parry
        text_pane:set_text_string("parry")
    elseif operation == OPERATE_NONE then
        text_pane:set_text_message(MSG_LABEL_NONE)
    end
end

local play_check_animation = function (controller_id, button_id, on_off)
    layout_view = layout_root:get_root_view()

    local button_layout = layout_view:get_parts(button_selector:get_button(button_id):get_name())
    set_button_text(controller_id, button_id, button_layout)
    
    local check_on = true

    if on_off == true then
    else
        if on_off == false then
            check_on = false
        else
            check_on = UiScriptPlayer.invoke("is_button_setting_changed", controller_id, button_id)
        end
    end

    if check_on == true then
        button_layout:play_animation("check_on", 1.0)
    else
        button_layout:play_animation("check_off", 1.0)
    end
end

local display_option_icons = function (controller_id)
    layout_view = layout_root:get_root_view()

    local option_button_layout = layout_view:get_parts(button_selector:get_button(BUTTON_ID_OPTION):get_name())
    local rumble_icon = option_button_layout:get_parts("option_com_lct_icn_00")
    local ab_smash_icon = option_button_layout:get_parts("option_com_lct_icn_01")
    local tap_jump_icon = option_button_layout:get_parts("option_com_lct_icn_02")
    local sensitivity_icon = option_button_layout:get_parts("option_com_lct_icn_03")
    if UiScriptPlayer.invoke("get_option_vibration", controller_id) == true then
        rumble_icon:play_animation("icn_col_on", 1.0)
    else
        rumble_icon:play_animation("icn_col_off", 1.0)
    end

    if UiScriptPlayer.invoke("get_option_smash_attack", controller_id) == true then
        ab_smash_icon:play_animation("icn_col_on", 1.0)
    else
        ab_smash_icon:play_animation("icn_col_off", 1.0)
    end

    if UiScriptPlayer.invoke("get_option_flick_up_jump", controller_id) == true then
        tap_jump_icon:play_animation("icn_col_on", 1.0)
    else
        tap_jump_icon:play_animation("icn_col_off", 1.0)
    end

    if UiScriptPlayer.invoke("get_option_flick_sensor", controller_id) == FLICK_SENSOR_NORMAL then
        sensitivity_icon:play_animation("icn_col_off", 1.0)
    else
        sensitivity_icon:play_animation("icn_col_on", 1.0)
    end
end

local restore_settings = function (controller_id)
    UiScriptPlayer.invoke("restore_settings", controller_id)
    if controller_id == CONTROLLER_FULL_KEY then
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_TRIGGER_L, nil)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_TRIGGER_R, nil)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_TRIGGER_ZL, nil)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_TRIGGER_ZR, nil)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_UP, nil)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_SIDE, nil)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_DOWN, nil)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_BUTTON_A, nil)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_BUTTON_B, nil)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_STICK_R, nil)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_BUTTON_X, nil)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_BUTTON_Y, nil)
    elseif controller_id == CONTROLLER_JOY_CON then
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_TRIGGER_LR, nil)
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_TRIGGER_ZLR, nil)
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_TRIGGER_SL, nil)
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_TRIGGER_SR, nil)
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_UP, nil)
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_RIGHT, nil)
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_LEFT, nil)
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_DOWN, nil)
    elseif controller_id == CONTROLLER_GC_CON then
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_TRIGGER_L, nil)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_TRIGGER_R, nil)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_TRIGGER_Z, nil)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_UP, nil)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_SIDE, nil)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_DOWN, nil)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_BUTTON_A, nil)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_BUTTON_B, nil)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_STICK_C, nil)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_BUTTON_X, nil)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_BUTTON_Y, nil)
    end

    display_option_icons(controller_id)
end

local setup = function (controller_id) 
    layout_view = layout_root:get_root_view()

    layout_view:stop_animation_frame("in", 0.0)
    if button_selector == nil then
        button_selector = LayoutButtonSelector:new()
    end

    if button_selector_config == nil then
        button_selector_config = LayoutButtonSelectorConfig:new()
        button_selector_config.is_unique_se = true
        button_selector_config.cursor_se_label_code = "se_system_cursor"
    end

    if controller_id == CONTROLLER_FULL_KEY then
        local fk_kind = "normal"
        if IS_PRO == true then
            fk_kind = "pro"
        end

        layout_view:play_animation(string.format("ctr_set_%s", fk_kind), 1.0)
        if IS_PRO == true then
            option_button_ctrl_func:selector_setup_bind_func(button_selector, layout_view, "selector_3", button_selector_config)
        else
            option_button_ctrl_func:selector_setup_bind_func(button_selector, layout_view, "selector_0", button_selector_config)
        end

        button_selector:setup_button(BUTTON_ID_FK_TRIGGER_L, string.format("set_parts_ctr_%s_l", fk_kind))
        button_selector:setup_button(BUTTON_ID_FK_TRIGGER_R, string.format("set_parts_ctr_%s_r", fk_kind))
        button_selector:setup_button(BUTTON_ID_FK_TRIGGER_ZL, string.format("set_parts_ctr_%s_zl", fk_kind))
        button_selector:setup_button(BUTTON_ID_FK_TRIGGER_ZR, string.format("set_parts_ctr_%s_zr", fk_kind))
        button_selector:setup_button(BUTTON_ID_FK_UP, string.format("set_parts_ctr_%s_ct", fk_kind))
        button_selector:setup_button(BUTTON_ID_FK_SIDE, string.format("set_parts_ctr_%s_clr", fk_kind))
        button_selector:setup_button(BUTTON_ID_FK_DOWN, string.format("set_parts_ctr_%s_cb", fk_kind))
        button_selector:setup_button(BUTTON_ID_FK_BUTTON_A, string.format("set_parts_ctr_%s_a", fk_kind))
        button_selector:setup_button(BUTTON_ID_FK_BUTTON_B, string.format("set_parts_ctr_%s_b", fk_kind))
        button_selector:setup_button(BUTTON_ID_FK_STICK_R, string.format("set_parts_ctr_%s_st", fk_kind))
        button_selector:setup_button(BUTTON_ID_FK_BUTTON_X, string.format("set_parts_ctr_%s_x", fk_kind))
        button_selector:setup_button(BUTTON_ID_FK_BUTTON_Y, string.format("set_parts_ctr_%s_y", fk_kind))
    elseif controller_id == CONTROLLER_JOY_CON then
        layout_view:play_animation("ctr_set_joy", 1.0)
        option_button_ctrl_func:selector_setup_bind_func(button_selector, layout_view, "selector_1", button_selector_config)

        button_selector:setup_button(BUTTON_ID_JC_TRIGGER_LR, "set_parts_ctr_joy_l")
        button_selector:setup_button(BUTTON_ID_JC_TRIGGER_ZLR, "set_parts_ctr_joy_zl")
        button_selector:setup_button(BUTTON_ID_JC_TRIGGER_SL, "set_parts_ctr_joy_sl")
        button_selector:setup_button(BUTTON_ID_JC_TRIGGER_SR, "set_parts_ctr_joy_sr")
        button_selector:setup_button(BUTTON_ID_JC_UP, "set_parts_ctr_joy_ct")
        button_selector:setup_button(BUTTON_ID_JC_RIGHT, "set_parts_ctr_joy_cr")
        button_selector:setup_button(BUTTON_ID_JC_LEFT, "set_parts_ctr_joy_cl")
        button_selector:setup_button(BUTTON_ID_JC_DOWN, "set_parts_ctr_joy_cb")
    elseif controller_id == CONTROLLER_GC_CON then
        layout_view:play_animation("ctr_set_gc", 1.0)
        option_button_ctrl_func:selector_setup_bind_func(button_selector, layout_view, "selector_2", button_selector_config)

        button_selector:setup_button(BUTTON_ID_GC_TRIGGER_L, "set_parts_ctr_gc_l")
        button_selector:setup_button(BUTTON_ID_GC_TRIGGER_R, "set_parts_ctr_gc_r")
        button_selector:setup_button(BUTTON_ID_GC_TRIGGER_Z, "set_parts_ctr_gc_z")
        button_selector:setup_button(BUTTON_ID_GC_UP, "set_parts_ctr_gc_ct")
        button_selector:setup_button(BUTTON_ID_GC_SIDE, "set_parts_ctr_gc_clr")
        button_selector:setup_button(BUTTON_ID_GC_DOWN, "set_parts_ctr_gc_cb")
        button_selector:setup_button(BUTTON_ID_GC_BUTTON_A, "set_parts_ctr_gc_a")
        button_selector:setup_button(BUTTON_ID_GC_BUTTON_B, "set_parts_ctr_gc_b")
        button_selector:setup_button(BUTTON_ID_GC_STICK_C, "set_parts_ctr_gc_c")
        button_selector:setup_button(BUTTON_ID_GC_BUTTON_X, "set_parts_ctr_gc_x")
        button_selector:setup_button(BUTTON_ID_GC_BUTTON_Y, "set_parts_ctr_gc_y")
    else
        return
    end

    button_selector:setup_button(BUTTON_ID_OPTION, "set_btn_another_00")
    button_selector:setup_button(BUTTON_ID_TEST, "set_btn_test_00")
    button_selector:setup_button(BUTTON_ID_SAVE, "set_footer_area")
    button_selector:get_button(BUTTON_ID_SAVE):set_decidable(false)

    if guide_layout == nil then
        guide_layout = LayoutButtonSelector:new()
    end

    local config = LayoutButtonSelectorConfig:new()
    config.use_only_shortcut = true
    config.is_unique_se = true

    guide_layout:setup(layout_view, "selector_00", config)

    guide_layout:setup_button(BUTTON_ID_RESET, "set_parts_guide")
    guide_layout:set_shortcut(BUTTON_ID_RESET, BUTTON_RESET, false)
    guide_layout:clear_selection()
    guide_layout:set_focus(true)
    parts_guide = layout_view:get_pane("set_parts_guide")
    local test_button = layout_view:get_parts(button_selector:get_button(BUTTON_ID_TEST):get_name())
    local icn_00 = test_button:get_pane("set_txt_icn_on_00")
    local icn_01 = test_button:get_pane("set_txt_icn_on_01")
    if controller_id == CONTROLLER_JOY_CON then
        icn_00:set_text_message("mnu_opt_joycon_side_left")
        icn_01:set_text_message("mnu_opt_joycon_side_left")
    elseif controller_id == CONTROLLER_GC_CON then
        icn_00:set_text_message("mnu_opt_gccon")
        icn_01:set_text_message("mnu_opt_gccon")
    else
        icn_00:set_text_message("mnu_opt_joycon_both")
        icn_01:set_text_message("mnu_opt_joycon_both")
    end

    can_test_play = UiScriptPlayer.invoke("can_test_play")
    button_selector:get_button(BUTTON_ID_TEST):set_selectable(can_test_play)
    button_selector:get_button(BUTTON_ID_TEST):set_decidable(can_test_play)

    local option_button = layout_view:get_parts(button_selector:get_button(BUTTON_ID_OPTION):get_name())
    local rumble_icon = option_button:get_parts("option_com_lct_icn_00")
    local ab_smash_icon = option_button:get_parts("option_com_lct_icn_01")
    local tap_jump_icon = option_button:get_parts("option_com_lct_icn_02")
    local sensitivity_icon = option_button:get_parts("option_com_lct_icn_03")

    ab_smash_icon:play_animation("icn_smash_ab", 1.0)
    rumble_icon:play_animation("icn_con_normal", 1.0)
    tap_jump_icon:play_animation("icn_jump_normal", 1.0)
    sensitivity_icon:play_animation("icn_sens_normal", 1.0)

    if controller_id == CONTROLLER_FULL_KEY then
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_TRIGGER_L, false)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_TRIGGER_R, false)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_TRIGGER_ZL, false)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_TRIGGER_ZR, false)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_UP, false)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_SIDE, false)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_DOWN, false)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_BUTTON_A, false)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_BUTTON_B, false)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_STICK_R, false)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_BUTTON_X, false)
        play_check_animation(CONTROLLER_FULL_KEY, BUTTON_ID_FK_BUTTON_Y, false)
    elseif controller_id == CONTROLLER_JOY_CON then
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_TRIGGER_LR, false)
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_TRIGGER_ZLR, false)
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_TRIGGER_SL, false)
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_TRIGGER_SR, false)
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_UP, false)
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_RIGHT, false)
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_LEFT, false)
        play_check_animation(CONTROLLER_JOY_CON, BUTTON_ID_JC_DOWN, false)
    elseif controller_id == CONTROLLER_GC_CON then
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_TRIGGER_L, false)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_TRIGGER_R, false)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_TRIGGER_Z, false)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_UP, false)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_SIDE, false)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_DOWN, false)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_BUTTON_A, false)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_BUTTON_B, false)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_STICK_C, false)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_BUTTON_X, false)
        play_check_animation(CONTROLLER_GC_CON, BUTTON_ID_GC_BUTTON_Y, false)
    end

    display_option_icons(controller_id)
end

local should_exit = function (controller_id)
    if UiScriptPlayer.invoke("is_setting_changed", controller_id) == true then
        AppPopupManager.open_database("ID_POPUP_OPT_BTN_KEY_RETURN_NO_SAVE")
        repeat
            if AppPopupManager.is_busy() == false then
                local result = AppPopupManager.get_result()
                return result == POPUP_RESULT_YES
            end
            coroutine.yield()
        until false
    else
        UiSoundManager:play_se_label("se_system_cancel")
        return true
    end
end

local wait_for_save = function (controller_id) 
    local not_assigned = UiScriptPlayer.invoke("get_not_assigned_operation_num", controller_id)
    if 0 == not_assigned then
        UiSoundManager:play_se_label("se_system_save")
        return true
    elseif not_assigned > 0 then
        repeat
            if AppPopupManager.is_busy() == false then
                local result = AppPopupManager.get_result()
                return result == POPUP_RESULT_YES
            end
            coroutine.yield()
        until false
    else
        repeat
            if AppPopupManager.is_busy() == false then
                return false
            end
            coroutine.yield()
        until false
    end
end

local get_controller_label = function (controller_id)
    if controller_id == CONTROLLER_GC_CON then
        return MSG_LABEL_GC_CON
    elseif controller_id == CONTROLLER_JOY_CON then
        return MSG_LABEL_JOY_CON
    end
    return MSG_LABEL_FULL_KEY
end

local try_test_play = function (controller_id)
    if UiScriptPlayer.invoke("can_test_play") == false then
        AppPopupManager.open_database_args("ID_POPUP_OPT_BTN_KEY_CANNOT_TRY", get_controller_label(controller_id), "s")
        repeat
            if AppPopupManager.is_busy() == false then
                return false
            end
            coroutine.yield()
        until false
    else
        return true
    end
end

local test_play = function ()
    button_selector:set_enable(false)
    ComFrameActor.hide_frame(true)
    ui_common.play_animation_and_wait(layout_view, "out")
    UiScriptPlayer.invoke("start_test_play")
    repeat
        if UiScriptPlayer.invoke("is_playing") == false then
            ComFrameActor.unselect()
            ui_common.play_animation_and_wait(layout_view, "r_in")
            button_selector:set_focus(true)
            button_selector:set_enable(true)
            return
        end
        coroutine.yield()
    until false
end

local set_guide_visibility = function (enabled)
    parts_guide:set_visible(enabled)
    guide_layout:set_enable(enabled)
end

--[[
UPVALUES:
- 0: _G
- 1: setup
- 2: button_selector
- 3: guide_layout
- 4: play_select_key_animation
- 5: is_button_joycon_trigger
- 6: ui_common
- 7: set_guide_visibility
- 8: get_controller_label
- 9: restore_settings
- 10: display_option_icons
- 11: play_check_animation
- 12: should_exit
- 13: wait_for_save
- 14: try_test_play
- 15: test_play
]]--
main = function ()
    layout_view = layout_root:get_root_view()
    exit_code = SCENE_EXIT_CODE_NONE
    setup(CONTROLLER_TYPE)
    button_selector:clear_selection()
    button_selector:select_button(BUTTON_ID_INIT)
    button_selector:set_focus(true)
    button_selector:set_enable(false)
    guide_layout:clear_selection()
    guide_layout:set_focus(true)
    guide_layout:set_enable(false)
    ComFrameActor.set_back_enable(true)
    ComFrameActor.set_save_button(COMFRAME_FOOTERBUTTON_LARGE_100)
    ComFrameActor.set_footer_area(COMFRAME_FOOTERMESSAGEAREA_NONE)
    ComFrameActor.unselect()
    play_select_key_animation(CONTROLLER_TYPE, BUTTON_ID_INIT)
    if CONTROLLER_TYPE == CONTROLLER_JOY_CON then
        if is_button_joycon_trigger(BUTTON_ID_INIT) == true then
            layout_view:play_animation("ctr_joy_chg_a", 1.0)
        else
            layout_view:play_animation("ctr_joy_chg_b", 1.0)
        end
    end

    if IS_BACK == true then
        layout_view:play_animation("r_in", 1.0)
    else
        layout_view:play_animation("in", 1.0)
    end

    ui_common.fade_in(false)

    layout_root:set_enable_input(true)
    button_selector:set_enable(true)

    local is_default = UiScriptPlayer.invoke("is_default_settings", CONTROLLER_TYPE)
    set_guide_visibility(not is_default)

    local unk = nil
    button_is_edited = false
    updated_button_id = BUTTON_ID_NONE

    repeat

        is_default = UiScriptPlayer.invoke("is_default_settings", CONTROLLER_TYPE)
        local is_enable = guide_layout:is_enable()
        if (is_enable and is_default) or (not is_enable and not is_default) then
            set_guide_visibility(not is_default)
        end

        if guide_layout:get_decided_button_id() == BUTTON_ID_RESET then
            AppPopupManager.open_database_args("ID_POPUP_OPT_BTN_CTRL_INIT", get_controller_label(CONTROLLER_TYPE), "s")
            repeat
                if AppPopupManager.is_busy() == false then
                    local result = AppPopupManager.get_result()
                    if result == POPUP_RESULT_YES then
                        restore_settings(CONTROLLER_TYPE)
                    end
                    break
                end
                coroutine.yield()
            until false
        end

        if updated_button_id == BUTTON_ID_OPTION then
            display_option_icons(CONTROLLER_TYPE)
            button_selector:set_focus(true)
            button_selector:set_enable(true)
            ComFrameActor.set_save_button(COMFRAME_FOOTERBUTTON_LARGE_100)
            updated_button_id = BUTTON_ID_NONE
        elseif updated_button_id ~= BUTTON_ID_NONE then
            play_check_animation(CONTROLLER_TYPE, updated_button_id, button_is_edited)
            layout_view:get_pane("cursor"):set_visible(true)
            button_selector:set_enable(true)
            button_is_edited = false
            updated_button_id = BUTTON_ID_NONE
        end

        if button_selector:is_cancelled() == true then
            ComFrameActor.press_cancel()
        end

        if ComFrameActor.is_cancelled() == true then
            if should_exit(CONTROLLER_TYPE) == true then
                exit_code = SCENE_EXIT_CODE_CANCEL
            end
        elseif ComFrameActor.is_decided() == true then
            if wait_for_save(CONTROLLER_TYPE) == true then
                exit_code = SCENE_EXIT_CODE_NORMAL
            end
        else
            local selected = button_selector:get_selected_button_id()

            if selected ~= unk then
                if CONTROLLER_TYPE == CONTROLLER_JOY_CON then
                    if is_button_joycon_trigger(selected) == true and is_button_joycon_trigger(unk) == false then
                        layout_view:play_animation("ctr_joy_chg_a", 1.0)
                    elseif is_button_joycon_trigger(selected) == false and is_button_joycon_trigger(unk) == true then
                        layout_view:play_animation("ctr_joy_chg_b", 1.0)
                    end
                end
                if unk == BUTTON_ID_SAVE then
                    ComFrameActor.unselect()
                end

                unk = selected

                play_select_key_animation(CONTROLLER_TYPE, unk)
                if unk == BUTTON_ID_SAVE then
                    ComFrameActor.set_focus(true)
                    ComFrameActor.select()
                    UiSoundManager:play_se_label("se_system_cursor")
                end
            end

            if unk ~= BUTTON_ID_SAVE and ComFrameActor.is_selected() == true then
                button_selector:select_button(BUTTON_ID_SAVE)
                unk = BUTTON_ID_SAVE
                play_select_key_animation(CONTROLLER_TYPE, unk)
            end

            decided_button_id = button_selector:get_decided_button_id()
            if decided_button_id == BUTTON_ID_TEST then
                if try_test_play(CONTROLLER_TYPE) == true then
                    UiSoundManager:play_se_label("se_system_fixed")
                    test_play()
                end
            elseif decided_button_id ~= BUTTON_ID_NONE then
                if decided_button_id == BUTTON_ID_OPTION then
                    button_selector:set_focus(false)
                end
                button_selector:set_enable(false)
                layout_view:get_pane("cursor"):set_visible(false)
                if decided_button_id == BUTTON_ID_OPTION then
                    ComFrameActor.set_button(COMFRAME_FOOTERBUTTON_NONE)
                end

                UiScriptPlayer.invoke("open_sub_window", CONTROLLER_TYPE, decided_button_id)
            end
        end

        if exit_code ~= SCENE_EXIT_CODE_NONE then
            break
        end

        coroutine.yield()

    until false
        
    layout_root:set_enable_input(false)
    if exit_code == SCENE_EXIT_CODE_CANCEL then
        ui_common.play_animation_and_wait(layout_view, "r_out")
    else
        ui_common.play_animation_and_wait(layout_view, "out")
    end
end

local last_button = nil

select_button_func = function (current_button, stick_x, stick_y)
    local new_button = current_button
    if CONTROLLER_TYPE == CONTROLLER_FULL_KEY then
        if current_button == BUTTON_ID_FK_BUTTON_X then
            if 0.0 < stick_y then
                if 0.0 < stick_x then
                    new_button = BUTTON_ID_FK_BUTTON_A
                elseif stick_x < 0.0 then
                    new_button = BUTTON_ID_FK_BUTTON_Y
                else
                    if last_button == BUTTON_ID_FK_BUTTON_A or last_button == BUTTON_ID_FK_BUTTON_Y then
                        new_button = last_button
                    else
                        new_button = BUTTON_ID_FK_BUTTON_Y
                    end
                end
            end
        elseif current_button == BUTTON_ID_FK_BUTTON_Y or current_button == BUTTON_ID_FK_BUTTON_A then
            if 0.0 < stick_y then
                new_button = BUTTON_ID_FK_BUTTON_B
            elseif stick_y < 0.0 then
                new_button = BUTTON_ID_FK_BUTTON_X
            end
        elseif current_button == BUTTON_ID_FK_BUTTON_B then
            if stick_y < 0.0 then
                if 0.0 < stick_x then
                    new_button = BUTTON_ID_FK_BUTTON_A
                elseif stick_x < 0.0 then
                    new_button = BUTTON_ID_FK_BUTTON_Y
                else
                    if last_button == BUTTON_ID_FK_BUTTON_A or last_button == BUTTON_ID_FK_BUTTON_Y then
                        new_button = last_button
                    else
                        new_button = BUTTON_ID_FK_BUTTON_Y
                    end
                end
            elseif stick_x < 0.0 then
                new_button = BUTTON_ID_FK_STICK_R
            end
        elseif current_button == BUTTON_ID_FK_STICK_R then
            if stick_y < 0.0 then
                new_button = BUTTON_ID_FK_BUTTON_Y
            elseif stick_y > 0.0 then
                if can_test_play then
                    new_button = BUTTON_ID_TEST
                end
            elseif stick_x > 0.0 then
                new_button = BUTTON_ID_FK_BUTTON_B
            elseif stick_x < 0.0 then
                new_button = BUTTON_ID_FK_DOWN
            end
        elseif current_button == BUTTON_ID_FK_DOWN then
            if 0.0 < stick_x then
                new_button = BUTTON_ID_FK_STICK_R
            end
        elseif current_button == BUTTON_ID_TEST then
            if stick_y < 0.0 then
                new_button = BUTTON_ID_FK_STICK_R
            end
        end
    elseif CONTROLLER_TYPE == CONTROLLER_JOY_CON then
        if current_button == BUTTON_ID_JC_UP then
            if 0.0 < stick_x then
                new_button = BUTTON_ID_JC_RIGHT
            elseif stick_x < 0.0 then
                new_button = BUTTON_ID_JC_LEFT
            elseif 0.0 < stick_y then
                if last_button == BUTTON_ID_JC_RIGHT or last_button == BUTTON_ID_JC_LEFT then
                    new_button = last_button
                else
                    new_button = BUTTON_ID_JC_LEFT
                end
            end
        elseif current_button == BUTTON_ID_JC_RIGHT or current_button == BUTTON_ID_JC_LEFT then
            if 0.0 < stick_y then
                new_button = BUTTON_ID_JC_DOWN
            elseif stick_y < 0.0 then
                new_button = BUTTON_ID_JC_UP
            end
        elseif current_button == BUTTON_ID_JC_DOWN then
            if 0.0 < stick_x then
                new_button = BUTTON_ID_JC_RIGHT
            elseif stick_x < 0.0 then
                new_button = BUTTON_ID_JC_LEFT
            elseif stick_y < 0.0 then
                if last_button == BUTTON_ID_JC_RIGHT or last_button == BUTTON_ID_JC_LEFT then
                    new_button = last_button
                else
                    new_button = BUTTON_ID_JC_LEFT
                end
            end
        end
    end

    if current_button == BUTTON_ID_SAVE then
        if stick_x < 0.0 and can_test_play then
            new_button = BUTTON_ID_TEST
        end
    elseif current_button == BUTTON_ID_TEST then
        if 0.0 < stick_x then 
            new_button = BUTTON_ID_SAVE
        end
    end

    last_button = current_button

    return new_button
end
