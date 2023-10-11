--[[
FILE: stage_select_actor3.lua
Reference Code: stage_select_actor3.lc
Author: blujay
Notes: The following code has been decompiled with the assistance of a modified
binary of the DSLuaDecompiler to produce *some* output, while not very meaningful it helps
outline the structure of some functions so that the disassembled lua could be used
as a reference instead of the guiding light
The following code was all written by hand, and was compiled via https://github.com/ultimate-research/smash-lua
and disassembled using https://github.com/jam1garner/smash-luadec
The output disassembly of this file is guaranteed to match exactly that of stage_select_actor3.lc
when unmodified.
]]--

--[[
    this file was created from 6000 lines of disassembly that the DSLuaDecompiler couldn't decompile

    i am very tired
]]--

-- R0
local ui_common        = UiScriptPlayer.require("common/ui_common")

-- R1
local stage_select_bgm = UiScriptPlayer.require2("common/stage_select_bgm3")

-- R2
local layout_root_index = 1

-- R3
local blink_counter = 30

-- this is the default scaling of the stage tiles
-- when they are not currently selected
-- R4
local unselected_button_scale = 1.0

-- this is the scaling of the stage tiles
-- when they are currently selected
-- R5
local selected_button_scale = 1.3

-- this is the number of frames for how long an (un)select animation
-- should take
-- R6
local select_button_frames = 4

-- this appears to be related to the scroll bar on custom stages
-- R7
local scroll_amount_max = 150.0

-- medal states
local MEDAL_STATE_UNINITIALIZED = 0        -- R8
local MEDAL_STATE_ACTIVE        = 1        -- R9
local MEDAL_STATE_WAITING       = 2        -- R10
local MEDAL_STATE_PLACED        = 3        -- R11
local MEDAL_STATE_TERM          = 4        -- R12

-- Button IDs for the main buttons (unused)
local BUTTON_ID_BACK    = 0       -- R13
local BUTTON_ID_NORMAL  = 1       -- R14
local BUTTON_ID_MAKE    = 2       -- R15
local MAIN_BUTTON_COUNT = 3       -- R16

-- Buttons IDs for the up/down buttons on the custom stage tab
local BUTTON_ID_SUB_STAGE_UP   = 0       -- R17
local BUTTON_ID_SUB_STAGE_DOWN = 1       -- R18

-- Button ids for the stage select previews
local BUTTON_ID_FORM_TYPE   = 0       -- R19
local BUTTON_ID_MUSC_SELECT = 1       -- R20
local PREVIEW_BUTTON_COUNT  = 2       -- R21

--
local SCENE_STATE_REGULAR     = 0 -- R22
local SCENE_STATE_SHOULD_EXIT = 1 -- R23
local SCENE_STATE_EXITING     = 2 -- R24
local SCENE_STATE_EXITED      = 3 -- R25

--
local INPUT_STRIKE = VI_BUTTON_EXTRA29
local INPUT_ALT_R = VI_BUTTON_EXTRA28
local INPUT_ALT_L = VI_BUTTON_EXTRA27

exit_code_ = nil

-- The layout root, gotten from the LayoutRootList with the index specified
-- by `layout_root_index`
local layout_root = nil     -- R26

-- The root view of the layout root stored in `layout_root`
local root_view = nil         -- R27

-- VirtualInput
local virtual_input = nil         -- R28

-- LayoutButtonSelector
-- This button selector contains the back button, and the
-- taps for both the main stages and the custom stages
local root_button_selector = nil         -- R29

-- LayoutButtonSelector
-- Owns `back_button`, below, which is the virtual back button
local back_button_selector = nil         -- R30

-- LayoutButtonSelector
-- Owns the up/down buttons for navigating the custom stage tab
local navigation_button_selector = nil         -- R31

-- array of LayoutButtonSelector for stage previews
-- handles selecting things like the music, form, etc.
local preview_button_selectors = { }         -- R32

-- Data class that holds the selected/decided buttons selectors
local SelectedButton = {           -- R33
    new = function ()
        return {
            selected_button_id_ = UI_INVALID_INDEX,
            decided_button_id_ = UI_INVALID_INDEX
        }
    end
}

-- Instance of above class
local root_selected_button = nil         -- R34

-- a virtual, invisible back button
-- this button is triggered by a unique layout for only if you are pressing the B
-- button, at which point it will update the root selector's back button
local back_button = nil         -- R35

-- selected button of the two buttons in `navigation_button_selector`
local sub_stage_nav_button = nil         -- R36

-- selected buttons on each of the previews
local preview_selected_buttons = { }         -- R37
local current_selected_preview   = -1          -- R38
local highlighed_preview = -1          -- R39
local prev_highlighed_preview = -1          -- R40
local current_selected_panel = -1            -- R41

--[[
Data Type: StagePreview
Description: Holds information about a stage preview on the left side (vanilla) of the SSS layout
Members:
    * `enable_`: true if this preview is enabled (there can be multiple previews)
    * `panel_id_`: the pane ID of this preview
    * `form_type_`: the form (normal, battlefield, or omega) of the preview
    * `is_sub_stage_`: if this is a sub stage
    * `music_parts_`: the music note icon/name on the preview
    * `on_button_form_type_`: if the form info (icon + name) is extended out more than the icon
    * `on_button_music_`: if the music info is extended out more than the icon
    * `form_type_blink_counter_`: the number of frames that the form info is extended (swich from battlefield -> normal and you'll see what this is)
]]--
local StagePreview = {           -- R42
    new = function()
        return  {
            selected_alt_ = 0,
            enable_ = false,
            panel_id_ = UI_INVALID_INDEX,
            form_type_ = 0,
            is_sub_stage_ = false,
            form_type_parts_ = nil,
            music_parts_ = nil,
            on_button_form_type_ = false,
            on_button_music_ = false,
            form_type_blink_counter_ = 0
        }
    end
}

-- Array of stage previews with length 3 (value of USE_STAGE_MAX)
local stage_previews = {}          -- R43

local is_invalid_stage_2 = false           -- R44
local prev_invalid_stage_2 = false           -- R45

--[[
Data Type: StagePanel
Description: Holds UI/animation information about each stage panel. A stage panel
                is one of the icons for selecting a stage
Members:
    * `frame_`: the current frame of animation
    * `target_scale_`: the end scale of the animation
    * `scale_value_`: the amount to scale by (?)
]]--
local StagePanel = {           -- R46
    new = function()
        return {
            frame_ = 0,
            target_scale_ = 0.0,
            scale_value_ = 0.0,
            is_striked_ = false
        }
    end
}

-- Array of stage panels with length STAGE_PANEL_LIST_NUM
local stage_panels = {}          -- R47

-- Information regarding the state of picking a custom stage
local stage_sub_selector_info = {          -- R48
    scroll_value_ = 0.0,
    scroll_end_ = false,
    is_play_se_ = true,
    select_sub_id_ = UI_INVALID_INDEX,
    waiting_in_ = false,
}

stage_sub_scroll_button_operatable_ = false

--[[
Data Type: Medal
Description: Holds information about the medals that are used to pick stages. There
                are the same number of these as there are stage previews
Members:
    * `state_`: the state of the medal
    * `put_index_`: the index of the panel which the medal is put on
    * `take_animation_`: animation to pickup the medal
    * `is_sub_stage_`: if it is on a custom stage
]]--
local Medal = {           -- R49
    new = function ()
        return {
            state_ = MEDAL_STATE_WAITING,
            put_index_ = UI_INVALID_INDEX,
            take_animation_ = nil,
            is_sub_stage_ = false
        }
    end
}

-- An array of Medal classes, with a length of 3 (same length as USE_STAGE_MAX)
local medals = {}          -- R50

local tab_index = 0           -- R51
local allow_sub_stage = true        -- R52

-- The layout view for the selection tab
local tab_layout = nil                   -- R53

-- The pane for the text describing which form is currently selected
local tab_form_button_pane = nil         -- R54

-- Array of buttons in the root selector
local main_buttons = {}          -- R55
local is_canceling = false       -- R56
local ignore_cancel_input = false       -- R57
local is_hand_interpolated_moving = false       -- R58
local should_play_cursor_sound = true        -- R59
local is_page_changing = false       -- R60
local long_cancel_se = nil         -- R61

-- The animation to transition to the next scene, gotten from `root_view`
local next_scene_animation = nil         -- R62
local scene_state = SCENE_STATE_REGULAR     -- R63

-- The strike cancel state
local strike_cancel = {
    current_ = 0,
    start_se_ = 30,
    total_ = 120,
    is_canceling_ = false
}

-- Stage paging information
local PANELS_PER_PAGE = 14
local total_pages = 0 -- to be filled in during setup
local current_page = 0
local Page = {
    new = function(start, length)
        local ret = {}
        for i=1,length,1 do
            ret[i] = root_view:get_parts(string.format("set_parts_n_stage_%03d", start + i - 1))
        end
        return ret
    end
}
local pages = {} -- to be filled in during setup

-- custom for us
local print_error_handler = function(err)
    print("Error caught: " .. tostring(err))
    
end

-- all of the (1 indexed) IDs which are actually Training (for tourney mode, to ignore them)
-- this is basically a map<id, is_training>
local training_stages = {}

-- Performs interpolation of a value using a sin wave for a more natural curve than just linear
-- CLOSURE_4, R64
local sin_interpolate = function(progress, total)
    local angle = 0.0
    if progress ~= 0 then
        angle = 1.5708 * (progress / total)
    end
    return math.sin(angle)
end

-- Gets the name of the stage preview part in the layout file
-- CLOSURE_5, R65
local get_stage_preview_name = function(preview_index)
    return string.format("set_preview_st_0%d", preview_index)
end

-- Gets the name of the stage icon part in the layout file
-- CLOSURE_6, R66
local get_stage_panel_name = function(stage_index)
    return string.format("set_parts_n_stage_%03d", stage_index)
end

-- Gets the name of the medal part in the layout file
-- CLOSURE_7, R67
local get_medal_name = function(medal_index)
    return string.format("set_medal_0%d", medal_index)
end

-- Gets the index of the next enabled stage preview
-- CLOSURE_8, R68
local get_next_enabled_preview = function()
    for i=1, USE_STAGE_NUM, 1 do
        if stage_previews[i].enable_ == true then
            return i - 1
        end
    end
    return UI_INVALID_INDEX
end

-- Gets the index of the last enabled stage preview
-- CLOSURE_9, R69
local get_last_enabled_preview = function()
    for i=USE_STAGE_NUM, 1, -1 do
        if stage_previews[i].enable_ == true then
            return i - 1
        end
    end
    return UI_INVALID_INDEX
end

-- Gets the index of the next disabled stage preview
-- CLOSURE_10, R70
local get_next_disabled_preview = function()
    for i=1, USE_STAGE_NUM, 1 do
        if stage_previews[i].enable_ == false then
            return i - 1
        end
    end
    return UI_INVALID_INDEX
end

-- Gets the index of the last disabled stage preview
-- CLOSURE_11, R71
local get_last_disabled_preview = function()
    for i=USE_STAGE_NUM, 1, -1 do
        if stage_previews[i].enable_ == false then
            return i - 1
        end
    end
    return UI_INVALID_INDEX
end

-- Checks if all stage previews are enabled or not
-- CLOSURE_12, R72
local check_all_previews_enabled = function()
    if get_next_disabled_preview() == UI_INVALID_INDEX then
        return true
    end
    return false
end

-- Checks if all stage previews are disabled or not
-- CLOSURE_13, R73
local check_all_previews_disabled = function()
    if get_next_enabled_preview() == UI_INVALID_INDEX then
        return true
    end
    return false
end

-- Gets the next stage from (usually triggered by pressing X)
-- CLOSURE_14, R74
local get_next_stage_form = function(current_form)
    current_form = current_form + 1
    if STAGE_FORM_TYPE_NUM <= current_form then
        current_form = 0
    end
    return current_form
end

-- Updates the text on the tab at the top of the SSS
-- CLOSURE_15, R75
local set_tab_form_text = function(stage_form)
    local form_names = {
        "normal",
        "battlefield",
        "end"
    }

    tab_layout:play_animation(string.format("stage_%s", form_names[stage_form + 1]), 1.0)
    tab_form_button_pane:set_text_message(string.format("mel_stage_select_%s", form_names[stage_form + 1]))
end

-- Plays the looping long-cancel sound effect (when you are holding B to exit)
-- CLOSURE_16, R76
local play_long_cancel_se = function()
    if long_cancel_se == nil then
        long_cancel_se = UiSoundManager.play_se_loop("se_system_cancel_longpress")
    end
end

-- Stops playing the looping long-cancel sound effect if it is playing
-- CLOSURE_17, R77
local stop_long_cancel_se = function()
    if long_cancel_se ~= nil then
        long_cancel_se:keyoff(0)
        long_cancel_se = nil
    end
end

-- Shows or hides the scene
-- CLOSURE_18, R78
local set_scene_enable = function(enable)
    virtual_input:set_enable(enable)
    root_button_selector:set_enable(enable)
    root_button_selector:set_focus(enable)
    local show_back_button = enable
    if IS_SIMPLE_CANCEL == false then
        show_back_button = false
    end

    back_button_selector:set_enable(show_back_button)
    back_button_selector:set_focus(show_back_button)

    local stage_preview_button = nil
    for i=1, USE_STAGE_NUM, 1 do
        stage_preview_button = preview_button_selectors[i]
        stage_preview_button:set_enable(enable)
        stage_preview_button:set_focus(enable)
    end
end

-- Shows or hides the scene/hand
-- CLOSURE_19, R79
local show_scene_and_hand = function(show_scene, enable_hand)
    set_scene_enable(show_scene)
    local enable_hand = enable_hand
    if enable_hand == false and UiScriptPlayer.invoke("get_hand_on_stage_preview_id") ~= UI_INVALID_INDEX then
        enable_hand = true
    end

    if enable_hand == true then
        UiScriptPlayer.invoke("set_hand_enable", show_scene)
    end
end

-- Gets the specified main button
-- CLOSURE_20, R80
local get_main_button = function(button_id)
    return main_buttons[button_id + 1]
end

-- Sets the active state of the specified button
-- CLOSURE_21, R81
local set_main_button_active = function(button_id, active)
    root_button_selector:set_selectable(button_id, active)
    root_button_selector:set_decidable(button_id, active)
end

-- Enables the specified stage preview
-- CLOSURE_22, R82
local enable_stage_preview = function(preview_index, panel_id, is_sub_stage)
    local preview = stage_previews[preview_index + 1]
    preview.enable_ = true
    preview.panel_id_ = panel_id
    preview.is_sub_stage_ = is_sub_stage
end

-- Disables the specified stage preview
-- CLOSURE_23, R83
local disable_stage_preview = function(preview_index)
    local preview = stage_previews[preview_index + 1]
    preview.enable_ = false
    preview.panel_id_ = UI_INVALID_INDEX
    preview.is_sub_stage_ = false
end

-- Sets the stage form of the specified preview
-- CLOSURE_24, R84
local set_stage_preview_form = function(preview_index, stage_form)
    if preview_index ~= UI_INVALID_INDEX then
        if UiScriptPlayer.invoke("is_fixed_form_type_stage_preview", preview_index) == true then
            local training_form = UiScriptPlayer.invoke("get_stage_fixed_form_type", preview_index)
            UiScriptPlayer.invoke("set_stage_form_type_stage_preview", preview_index, training_form)
            if preview_index == current_selected_preview then
                set_tab_form_text(training_form)
            end
            return
        end
        stage_previews[preview_index + 1].form_type_ = stage_form
        UiScriptPlayer.invoke("set_stage_form_type_stage_preview", preview_index, stage_form)
        if preview_index == current_selected_preview then
            set_tab_form_text(stage_form)
        end
    end
end

local set_alt_texture = function(left, texture_index, preview_idx)
  local base_alt_name = left and "alt_l" or "alt_r"
  local texture_name = left and "set_rep_alt_l" or "set_rep_alt_r"
    local parts_name = get_stage_preview_name(preview_idx)
    local parts = root_view:get_parts(parts_name)

  local base_pane = parts:get_pane(base_alt_name)
  local texture_pane = parts:get_pane(texture_name)

  if texture_index == nil then
    base_pane:set_visible(false)
  else
    base_pane:set_visible(true)
    texture_pane:replace_texture(texture_index)
  end
end

local set_alt_panel_textures = function(is_forward)
    if current_selected_preview == UI_INVALID_INDEX then
        Alts.send_message("Can't change alt on invalid preview")
        return
    end

    local preview = stage_previews[current_selected_preview + 1]
    if current_selected_panel == UI_INVALID_INDEX then
        set_alt_texture(true, nil, current_selected_preview)
        set_alt_texture(false, nil, current_selected_preview)
        Alts.send_message("Preview's panel id is invalid")
        return
    end

    local count = Alts.get_panel_alt_count(current_selected_panel, preview.form_type_)

    if is_forward == true then
        if count == preview.selected_alt_ then
            preview.selected_alt_ = 0
        else
            preview.selected_alt_ = preview.selected_alt_ + 1
        end
    elseif is_forward == false then
        if preview.selected_alt_ == 0 then
            preview.selected_alt_ = count
        else
            preview.selected_alt_ = preview.selected_alt_ - 1
        end
    end

    local texture_idx = Alts.get_alt_texture_index(current_selected_panel, preview.form_type_, preview.selected_alt_)

    local parts_name = get_stage_preview_name(current_selected_preview)
    local parts = root_view:get_parts(parts_name)

    if count == 0 then
      set_alt_texture(true, nil, current_selected_preview)
      set_alt_texture(false, nil, current_selected_preview)
    elseif count == 1 then
      set_alt_texture(true, nil, current_selected_preview)

      local idx = preview.selected_alt_ == 0 and 1 or 0
      set_alt_texture(false, Alts.get_alt_texture_index(current_selected_panel, preview.form_type_, idx), current_selected_preview)
    else
      local left_idx = preview.selected_alt_ == 0 and count or preview.selected_alt_ - 1
      local right_idx = preview.selected_alt_ == count and 0 or preview.selected_alt_ + 1

      set_alt_texture(
        true,
        Alts.get_alt_texture_index(current_selected_panel, preview.form_type_, left_idx),
        current_selected_preview
      )

      set_alt_texture(
        false,
        Alts.get_alt_texture_index(current_selected_panel, preview.form_type_, right_idx),
        current_selected_preview
      )
    end

    if texture_idx < 0 then
        return
    end
    local pane_name = "set_rep_stage"
    if preview.form_type_ == STAGE_FORM_TYPE_BATTLE then
        pane_name = "set_rep_stage_battle"
    elseif preview.form_type_ == STAGE_FORM_TYPE_END then
        pane_name = "set_rep_stage_end"
    end

    local texture_pane = parts:get_pane(pane_name)
    texture_pane:replace_texture(texture_idx)
end



-- Enables/disables the stage form/music buttons for the specified preview
-- CLOSURE_25, R85
local set_stage_preview_buttons_enable = function(preview_index, enable)
    -- the BUTTON_FORM_TYPE_0 is the index of the first
    -- stage preview's form button, and to get to the next one
    -- you add 2, which is why this adds preview_index << 1 (shifting to the left by 
    -- one is the same as multiplying by 2)
    local button_id = BUTTON_FORM_TYPE_0 + (preview_index << 1)
    local preview = preview_button_selectors[preview_index + 1]

    preview:set_selectable(button_id, enable)
    preview:set_decidable(button_id, enable)

    -- the music button is at button_id + 1
    preview:set_selectable(button_id + 1, enable)
    preview:set_decidable(button_id + 1, enable)
end

-- Sets the stage preview based on the selected stage panel
-- CLOSURE_26, R86
local set_stage_preview_from_stage_panel = function(preview_index, panel_index)
    if allow_sub_stage == true and panel_index ~= UI_INVALID_INDEX then
        set_stage_preview_buttons_enable(preview_index, true)
    end

    UiScriptPlayer.invoke("set_stage_preview_from_panel", preview_index, panel_index)
    set_stage_preview_form(preview_index, stage_previews[preview_index + 1].form_type_)
end

-- Sets the stage preview based on the selected custom stage panel
-- CLOSURE_27, R87
local set_stage_preview_from_sub_stage_panel = function(preview_index, panel_index)
    if allow_sub_stage == true and panel_index ~= UI_INVALID_INDEX then
        set_stage_preview_buttons_enable(preview_index, false)
    end

    UiScriptPlayer.invoke("set_stage_preview_from_sub_panel", preview_index, panel_index)
end

-- Plays the stage form switch animation
-- CLOSURE_28, R88
local switch_stage_form = function(preview_index, stage_form)
    local anims = {
        "anim_type_normal",
        "anim_type_battlefield",
        "anim_type_end"
    }

    root_view:play_animation_parts(get_stage_preview_name(preview_index), anims[stage_form + 1])
    UiSoundManager.play_se_label("se_system_switch")
end

-- Plays the stage decide animation if we aren't in my music
-- CLOSURE_29, R89
local play_decide_stage_animation = function(preview_index)
    if IS_MY_MUSIC == false then
        root_view:play_animation_parts(get_stage_preview_name(preview_index), "decide")
    end
end

-- Plays the stage undecide animation
-- CLOSURE_30, R90
local play_un_decide_animation = function(preview_index)
    root_view:play_animation_parts(get_stage_preview_name(preview_index), "un_decide")
end

-- Plays the selection animation on the stage preview
-- CLOSURE_31, R91
local play_select_animation = function(preview_index)
    local name = get_stage_preview_name(preview_index)
    root_view:play_animation_parts(name, "select")
    root_view:play_animation_parts(name, "on_crs_preview_anime")
end

-- Plays the off_crs animation on the stage preview
-- CLOSURE_32, R92
local play_off_preview_animation = function(preview_index)
    root_view:play_animation_parts(get_stage_preview_name(preview_index), "off_crs_preview_anime")
end

-- Plays the unselect animation on the stage preview
-- CLOSURE_33, R93
local play_un_select_animation = function(preview_index)
    root_view:play_animation_parts(get_stage_preview_name(preview_index), "un_select")
end

-- Moves to the next stage preview in a multi-select stage preview
-- CLOSURE_34, R94
local advance_stage_preview = function(next_preview, current_preview)
    play_un_select_animation(current_preview)
    play_off_preview_animation(current_preview)
    UiScriptPlayer.invoke("set_enable_shortcut_button_stage_preview", current_preview, false)
    play_select_animation(next_preview)
    UiScriptPlayer.invoke("set_enable_shortcut_button_stage_preview", next_preview, true)
end

-- Plays the cursor sound
-- CLOSURE_35, R95
local play_cursor_sound = function()
    if should_play_cursor_sound == true and is_page_changing == false then
        UiSoundManager.play_se(UI_SE_ID_CURSOR)
    end
    should_play_cursor_sound = true
end

-- Sets up the target scaling for the specified panel
-- CLOSURE_36, R96
local setup_scale_anim = function(panel_id, target_scale)
    local x_scale, y_scale = nil
    x_scale, y_scale = root_view:get_scale_parts(get_stage_panel_name(panel_id))
    local panel = stage_panels[panel_id + 1]
    panel.target_scale_ = target_scale
    panel.scale_value_ = panel.target_scale_ - x_scale
end

-- Selects the specified panel if it is allowed
-- CLOSURE_37, R97
local select_panel = function(panel_id)
    if panel_id ~= UI_INVALID_INDEX then
        if UiScriptPlayer.invoke("is_lock_stage_panel", panel_id) == true then
            return 
        end
        local panel = root_view:get_button(get_stage_panel_name(panel_id))
        panel:select(false, true)
        stage_panels[panel_id + 1].frame_ = select_button_frames
        setup_scale_anim(panel_id, selected_button_scale)
    end
end

-- Unselects the specified panel
-- CLOSURE_38, R98
local unselect_panel = function(panel_id)
    if panel_id ~= UI_INVALID_INDEX then
        local panel = root_view:get_button(get_stage_panel_name(panel_id))
        panel:unselect()
        stage_panels[panel_id + 1].frame_ = select_button_frames
        setup_scale_anim(panel_id, unselected_button_scale)
    end
end

-- attempts to find the real desired panel using the given panel ID
local find_proper_panel = function(panel_id)
    if panel_id == UI_INVALID_INDEX then
        return panel_id
    end

    -- in tourney mode, dont allow selection of training stage.
    -- They are used as a buffer to put the starters/counterpicks
    -- in the right locations.
    if training_stages[panel_id + 1] == true then
        return UI_INVALID_INDEX
    end

    local panel_min = current_page * PANELS_PER_PAGE
    local panel_max = (current_page + 1) * PANELS_PER_PAGE
    if panel_min <= panel_id and panel_id < panel_max then
        return panel_id
    end

    if (panel_min + panel_id) >= STAGE_PANEL_LIST_NUM then
        return UI_INVALID_INDEX
    else
        return panel_min + panel_id
    end
end

local find_page_for_panel = function(panel_id)
    if panel_id == UI_INVALID_INDEX then
        return 0, panel_id
    end

    local page = panel_id // PANELS_PER_PAGE
    local front_panel = panel_id % PANELS_PER_PAGE
    return page, front_panel
end
-- Changes the currently selected
-- CLOSURE_39, R99
local change_panel = function(new_panel_id)
    if current_selected_panel == new_panel_id then
        return false
    end

    new_panel_id = find_proper_panel(new_panel_id)

    unselect_panel(current_selected_panel)
    select_panel(new_panel_id)
    current_selected_panel = new_panel_id

    if current_selected_panel ~= UI_INVALID_INDEX then
        play_cursor_sound()
    end
    return true
end

-- Updates the scaling of the specified panel
-- CLOSURE_40, R100
local update_panel_scaling = function(panel_id)
    local name = get_stage_panel_name(panel_id)
    local panel = stage_panels[panel_id + 1]
    if panel.frame_ <= 0 then
        return
    end

    panel.frame_ = ui_common.sub_time_counter(panel.frame_)
    local scale = 1.0
    if panel.frame_ <= 0 then
        scale = panel.target_scale_
    else
        local interpolation = sin_interpolate(panel.frame_, select_button_frames)
        scale = panel.target_scale_ - panel.scale_value_ * interpolation
    end

    root_view:set_scale_parts(name, scale, scale)
end

-- Advances the stage preview and then sets up the from
-- CLOSURE_41, R101
local switch_stage_preview = function(preview_id)
    if preview_id < 0 then
        preview_id = 0
    elseif USE_STAGE_NUM <= preview_id then
        preview_id = USE_STAGE_NUM - 1
    end

    if current_selected_preview == preview_id then
        return
    end

    play_un_select_animation(current_selected_preview)
    play_off_preview_animation(current_selected_preview)
    play_select_animation(preview_id)

    UiScriptPlayer.invoke("set_enable_shortcut_button_stage_preview", current_selected_preview, false)
    UiScriptPlayer.invoke("set_enable_shortcut_button_stage_preview", preview_id, true)

    current_selected_preview = preview_id

    local preview = stage_previews[current_selected_preview + 1]
    if preview.is_sub_stage_ == false then
        if preview.panel_id_ ~= UI_INVALID_INDEX then
            should_play_cursor_sound = false
            change_panel(preview.panel_id_)
            should_play_cursor_sound = true
        end
        set_stage_preview_form(current_selected_preview, preview.form_type_)
    else
        set_tab_form_text(preview.form_type_)
    end

end

-- Sets the information for the specified medal
-- CLOSURE_42, R102
local set_medal_info = function(medal_id, medal_state, is_sub)
    local medal = medals[medal_id + 1]
    medal.state_ = medal_state
    medal.is_sub_stage_ = is_sub
end

-- Gets the current medal index from the hand
-- CLOSURE_43, R103
local get_current_medal_index = function()
    if tab_index == TAB_SWITCH_SUB then
        return UiScriptPlayer.invoke("get_hand_on_medal_index_sub_list")
    end

    return UiScriptPlayer.invoke("get_hand_on_medal_index")
end

-- Checks whether any medal is currently grabbed
-- CLOSURE_44, R104
local any_medal_grabbed = function()
    for i=0, USE_STAGE_NUM - 1, 1 do
        if UiScriptPlayer.invoke("is_medal_grabbed", i) == true then
            return true
        end
    end

    return false
end

-- Grabs the specified medal
-- CLOSURE_45, R105
local grab_medal = function(medal_id, update_hand)
    UiScriptPlayer.invoke("set_medal_grabbed", medal_id, update_hand, false)
    if update_hand == true then
        set_medal_info(medal_id, MEDAL_STATE_ACTIVE, false)
        UiScriptPlayer.invoke("set_hand_grabbed", true)
        medals[medal_id + 1].take_animation_:stop_at_end()
    end
end

-- Updates the visibility of all medals
-- CLOSURE_46, R106
local update_medal_visibility = function()
    if USE_STAGE_NUM < 2 then
        return
    end

    for i=0, USE_STAGE_NUM - 1, 1 do
        if UiScriptPlayer.invoke("is_medal_grabbed", i) == true then
            if i ~= current_selected_preview then
                UiScriptPlayer.invoke("set_medal_visible", i, false)
                set_medal_info(i, MEDAL_STATE_WAITING, false)
            end
        end
    end
end

-- CLOSURE_47, R107
local update_medal = function()
    local index = get_current_medal_index()
    if index ~= UI_INVALID_INDEX and stage_previews[index + 1].enable_ == true then
        if index == current_selected_preview then
            if check_all_previews_enabled() == true then
                play_select_animation(index)
            end
        else
            switch_stage_preview(index)
        end

        grab_medal(current_selected_preview, true)
        UiScriptPlayer.invoke("cancel_medal_sub_list", current_selected_preview)
        disable_stage_preview(current_selected_preview)
        play_un_decide_animation(current_selected_preview)
        UiScriptPlayer.invoke("move_medal_interpolated_from_hand", current_selected_preview)
        return
    end
end

-- CLOSURE_48, R108
local un_decide_medal = function()
    local medal = medals[current_selected_preview + 1]
    if medal.state_ ~= MEDAL_STATE_ACTIVE then
        grab_medal(current_selected_preview, true)
        UiScriptPlayer.invoke("cancel_medal_sub_list", current_selected_preview)
        disable_stage_preview(current_selected_preview)
        play_un_decide_animation(current_selected_preview)
        UiScriptPlayer.invoke("move_hand_interpolated_from_medal", current_selected_preview)
        update_medal_visibility()
    end
end

-- Plays either the select or unselect animation
-- CLOSURE_49, R109
local play_tab_animation = function(current_tab, is_select)
    local button_name = nil
    if current_tab == TAB_SWITCH_SUB then
        button_name = "set_parts_btn_make"
    else
        button_name = "set_parts_btn_normal"
    end
    
    if is_select == true then
        root_view:play_animation_parts(button_name, "tab_select")
    else
        root_view:play_animation_parts(button_name, "tab_un_select")
    end
end

-- Changes the current page
-- CLOSURE_50, R110
local change_page = function(should_play_page_change)
    local tab_anim = "tab_stage_normal"
    local part_name = "set_parts_btn_normal"
    local normal_select = true
    local make_select = false

    if tab_index == TAB_SWITCH_SUB then
        tab_anim = "tab_stage_make"
        part_name = "set_parts_btn_make"
        normal_select = false
        make_select = true
    end

    is_page_changing = true

    UiScriptPlayer.invoke("set_se_unique_counter", "se_system_cursor", 2)
    UiScriptPlayer.invoke("set_hand_medal_range", tab_index)

    play_tab_animation(TAB_SWITCH_NORMAL, normal_select)
    play_tab_animation(TAB_SWITCH_SUB, make_select)

    root_view:play_animation(tab_anim, 1.0)
    root_view:bring_to_front_parts(part_name)

    -- sets the button as active, does not set it as the active tab
    set_main_button_active(BUTTON_TAB_SUB, normal_select)
    for i=0, USE_STAGE_NUM - 1, 1 do
        local medal = medals[i + 1]
        if medal.state_ == MEDAL_STATE_PLACED then
            if medal.is_sub_stage_ == false then
                UiScriptPlayer.invoke("set_medal_visible", i, normal_select)
            else
                UiScriptPlayer.invoke("set_medal_visible", i, make_select)
            end
        end
    end

    if DO_NOT_SCROLL_STAGE_SUB_LIST == false then
        navigation_button_selector:set_enable(make_select)
        navigation_button_selector:set_focus(make_select)
    end

    if stage_sub_list_selector_ ~= nil then
        stage_sub_list_selector_:set_enable(make_select)
        stage_sub_list_selector_:set_focus(make_select)
    end

    if make_select == true then
        change_panel(UI_INVALID_INDEX)
        if UiScriptPlayer.invoke("get_hand_on_stage_sub_id") == UI_INVALID_INDEX then
            stage_sub_selector_info.select_sub_id_ = UI_INVALID_INDEX
            stage_sub_list_selector_:select_item(stage_sub_selector_info.select_sub_id_, true)
            set_stage_preview_from_sub_stage_panel(current_selected_preview, stage_sub_selector_info.select_sub_id_)
        end
    end

    if check_all_previews_enabled() == false then
        local medal_anim = "stage_make"
        if normal_select == true then
            current_selected_panel = UI_INVALID_INDEX
            local form = stage_previews[current_selected_preview + 1].form_type_
            if form == STAGE_FORM_TYPE_BATTLE then
                medal_anim = "stage_battlefield"
            elseif form == STAGE_FORM_TYPE_END then
                medal_anim = "stage_end"
            else
                medal_anim = "stage_normal"
            end
        else
            stage_sub_selector_info.select_sub_id_ = UI_INVALID_INDEX
        end
        local parts = root_view:get_parts(get_medal_name(current_selected_preview))
        parts:play_animation(medal_anim, 1.0)
    end

    if should_play_page_change == true then
        UiSoundManager.play_se_label("se_system_page_change")
    end
end

local change_sub_page = function(target_page)
    -- Page is actually all the parts of the set_parts_n_stage_XXX
    local current_page_ = pages[current_page + 1]
    local target_page_ = pages[target_page + 1]

    local positions = {}

    -- load the positions of the first set of panels
    for i=1,PANELS_PER_PAGE,1 do
        local parts = root_view:get_parts(get_stage_panel_name(i - 1))
        local px, py = parts:get_root_pane():get_position()
        positions[i] = {
            x = px,
            y = py
        }
    end

    -- hide all of the panels on the current page
    for i, panel in ipairs(current_page_) do
        panel:set_visible(false)
    end      

    -- reposition and show the new page's panels
    for i, panel in ipairs(target_page_) do 
        -- dont reposition or show any invalid stages
        if i > STAGE_PANEL_LIST_NUM then
            break
        end
        
        -- set the preview    
        UiScriptPlayer.invoke("set_stage_preview_from_panel", 0 , i - 1)
        -- use the set preview to check if this preview is the training stage
        local is_random = UiScriptPlayer.invoke("is_training_stage_preview",  0)

        -- if its the training stage (which we are using as a buffer to align the stages),
        -- then hide the stage, and record whether its the training stage or not
        print("is random: " .. tostring(is_random))   
        if is_random == true then
            -- save that this is the training stage
            training_stages[i] = true
            print("found the random stage: " .. i)
            panel:set_visible(false)

            --[[ for getting reflective data about the stages
            for key,value in pairs(getmetatable(panel)) do
                print("panel member " .. key .. ", value: " .. tostring(value) );
            end

            for key,value in pairs(getmetatable(panel:get_root_pane())) do
                print("root pane member " .. key .. ", value: " .. tostring(value) );
            end
            ]]--
        else
            training_stages[i] = false
            
            -- print("setting root pane parent pane position")
            -- panel:get_root_pane():get_parent_pane():set_position(positions[offset].x, positions[offset].y)  
            print("bringing to front panel")
            panel:set_visible(true)
        end
    end
    current_page = target_page
end
-- Sets up the SSS layout
-- CLOSURE_51, R111
local setup = function()
    layout_root = LayoutRootList[layout_root_index]
    root_view = layout_root:get_root_view()
    virtual_input = layout_root:get_virtual_input()
    next_scene_animation = root_view:get_animation("anim_next_scene")

    -- set up the previews (these are the 1-3 big previews on SSS)
    for i=1, USE_STAGE_MAX, 1 do
        stage_previews[i] = StagePreview.new()
        medals[i] = Medal.new()
    end

    for i=1, STAGE_PANEL_LIST_NUM, 1 do
        stage_panels[i] = StagePanel.new()

        local strike_panel = root_view:get_parts(get_stage_panel_name(i - 1)):get_pane("set_rep_strike")
        strike_panel:set_visible(false)
    end

    for i=0, USE_STAGE_NUM - 1, 1 do
        local stage_parts = root_view:get_parts(get_stage_preview_name(i))
        stage_previews[i + 1].form_type_parts_ = stage_parts:get_parts("set_parts_btn_stage")
        stage_previews[i + 1].music_parts_ = stage_parts:get_parts("set_parts_btn_music")

        local medal_parts = root_view:get_parts(get_medal_name(i))
        medals[i + 1].take_animation_ = medal_parts:get_animation("take")
    end

    local config = LayoutButtonSelectorConfig.new()
    config.selection_type = LAYOUTBUTTONSELECTOR_SELECTION_TYPE_POINTER
    config.use_only_pointer_input = true
    config.is_unique_se = true
    config.cursor_se_label_code = "se_system_cursor"
    
    root_button_selector = LayoutButtonSelector.new()
    root_button_selector:setup(root_view, "selector_0", config)

    root_selected_button = SelectedButton.new()

    back_button_selector = LayoutButtonSelector.new()
    back_button_selector:setup(root_view, "selector_2", config)

    back_button_selector:setup_button(BUTTON_BACK, "btn_back")

    back_button = SelectedButton.new()

    navigation_button_selector = LayoutButtonSelector.new()
    navigation_button_selector:setup(root_view, "selector_1", config)

    navigation_button_selector:setup_button(BUTTON_ID_SUB_STAGE_UP, "set_parts_btn_csr_t")
    navigation_button_selector:setup_button(BUTTON_ID_SUB_STAGE_DOWN, "set_parts_btn_csr_b")

    sub_stage_nav_button = SelectedButton.new()

    local parts, selector = nil
    local current_button = BUTTON_FORM_TYPE_0
    for i=0, USE_STAGE_NUM - 1, 1 do
        selector = LayoutButtonSelector.new()
        parts = root_view:get_parts(get_stage_preview_name(i))

        selector:setup(parts, "selector_0", config)
        selector:setup_button(current_button, "set_parts_btn_stage")
        current_button = current_button + 1
        selector:setup_button(current_button, "set_parts_btn_music")
        current_button = current_button + 1
        preview_button_selectors[i + 1] = selector
        preview_selected_buttons[i + 1] = SelectedButton.new()
    end

    local main_button_names = {
        "set_parts_btn_back",
        "set_parts_btn_normal",
        "set_parts_btn_make",
    }

    local name = nil
    for i=1, BUTTON_KIND_MAIN_NUM, 1 do
        name = main_button_names[i]
        root_button_selector:setup_button(i - 1, name)
        main_buttons[i] = root_view:get_button(name)
        main_buttons[i]:set_decide_on_trigger(true, false)
    end

    total_pages = STAGE_PANEL_LIST_NUM // PANELS_PER_PAGE
    if STAGE_PANEL_LIST_NUM % PANELS_PER_PAGE ~= 0 then
        total_pages = total_pages + 1
    end
    for i=0,total_pages-1,1 do
        pages[i + 1] = Page.new(i * PANELS_PER_PAGE, math.min(PANELS_PER_PAGE, STAGE_PANEL_LIST_NUM - i * PANELS_PER_PAGE))
    end

    local back_button = root_view:get_parts(main_button_names[BUTTON_BACK + 1])
    local text_00 = back_button:get_pane("set_txt_back_00")
    local text_01 = back_button:get_pane("set_txt_back_01")

    text_00:set_text_message(string.format("mel_over_return_0%d", RETURN_TEXT_ID))
    text_01:set_text_message(string.format("mel_return_0%d", RETURN_TEXT_ID))
    if IS_INVISIBLE_CANCEL == true then
        back_button:set_visible(false)
    end

    INPUT_STRIKE = VI_BUTTON_EXTRA29
    INPUT_ALT_R = VI_BUTTON_EXTRA28
    INPUT_ALT_L = VI_BUTTON_EXTRA27
    virtual_input:set_assign(INPUT_STRIKE, LIB_BUTTON_ZR, nil)
    virtual_input:set_assign(INPUT_ALT_R, LIB_BUTTON_R, nil)
    virtual_input:set_assign(INPUT_ALT_L, LIB_BUTTON_L, nil)
end

-- Initializes the medal position, presumably at the beginning of the SSS load
-- CLOSURE_52, R112
local init_medal = function(medal_id)
    UiScriptPlayer.invoke("set_hand_position_from_panel", 0)
    UiScriptPlayer.invoke("set_medal_collect_range_from_panel", medal_id, 0)
    UiScriptPlayer.invoke("set_medal_position_from_panel", medal_id, 0)
    UiScriptPlayer.invoke("set_medal_visible", medal_id, true)
end

-- Sets up the stage select screen from the environment
-- CLOSURE_53, R113
local setup_from_environment = function()
    local last_enabled_preview = 0
    local current_id, unused_variable = nil -- unused variable to match register usage, not sure why

    highlighed_preview = UI_INVALID_INDEX
    prev_highlighed_preview = UI_INVALID_INDEX
    current_selected_panel = UI_INVALID_INDEX
    tab_index = TAB_SWITCH_NORMAL

    is_hand_interpolated_moving = false
    should_play_cursor_sound = false
    tab_layout = root_view:get_parts("set_parts_btn_normal")
    tab_form_button_pane = tab_layout:get_pane("set_txt_normal")
    set_tab_form_text(STAGE_FORM_TYPE_NORMAL)

    for i=1, USE_STAGE_NUM, 1 do
        if IS_SELECT_FROM_FIRST == true and 1 < i then
            break
        end
        current_id = i - 1
        local preview = stage_previews[i]
        preview.enable_ = UiScriptPlayer.invoke("is_valid_entrance_param", current_id)
        if preview.enable_ == true then
            last_enabled_preview = i
            preview.form_type_ = UiScriptPlayer.invoke("get_stage_form_type_entrance_param", current_id)
            preview.is_sub_stage_ = UiScriptPlayer.invoke("is_sub_entrance_param", current_id)
            if preview.is_sub_stage_ == false then
                preview.panel_id_ = UiScriptPlayer.invoke("get_panel_id_entrance_param", current_id)
                local page, front_panel = find_page_for_panel(preview.panel_id_)
                change_sub_page(page)
                set_stage_preview_from_stage_panel(current_id, preview.panel_id_)
                preview.panel_id_ = front_panel
                UiScriptPlayer.invoke("set_enable_shortcut_button_stage_preview", current_id, false)
                tab_index = TAB_SWITCH_NORMAL
            else
                preview.panel_id_ = UiScriptPlayer.invoke("get_stage_sub_id_entrance_param", current_id)
                set_stage_preview_from_sub_stage_panel(current_id, preview.panel_id_)
                tab_index = TAB_SWITCH_SUB
            end
            play_decide_stage_animation(current_id)
            local is_fixed = UiScriptPlayer.invoke("is_fixed_form_type_stage_preview", current_id)
            if preview.form_type_ ~= STAGE_FORM_TYPE_NORMAL and is_fixed == false then
                preview.form_type_parts_:stop_animation_at_end("blink_on")
            else
                preview.form_type_parts_:stop_animation_at_end("blink_off")
                preview.form_type_blink_counter_ = 0
            end
            if is_fixed == true then
                preview.form_type_parts_:play_animation("unable_shortcut", 1.0)
            end
            local medal = medals[i]
            medal.put_index_ = preview.panel_id_
            if preview.is_sub_stage_ == false then
                UiScriptPlayer.invoke("set_medal_collect_range_from_panel", current_id, preview.panel_id_)
                UiScriptPlayer.invoke("set_medal_position_from_panel", current_id, preview.panel_id_)
                UiScriptPlayer.invoke("set_medal_visible", current_id, true)
            else
                local medal_parts = root_view:get_parts(get_medal_name(current_id))
                medal_parts:play_animation("stage_make", 1.0)
                UiScriptPlayer.invoke("put_medal_sub_list", current_id, preview.panel_id_, true)
            end
            UiScriptPlayer.invoke("set_medal_grabbed", current_id, false, true)
            set_medal_info(current_id, MEDAL_STATE_PLACED, preview.is_sub_stage_)
        else
            preview.panel_id_ = UI_INVALID_INDEX
            preview.form_type_ = DEFAULT_STAGE_FORM_TYPE
        end
        play_un_select_animation(current_id)
        play_off_preview_animation(current_id)
    end

    if 0 < last_enabled_preview then
        local dont_set_hand_by_panel = false
        if IS_SELECT_FROM_FIRST == true then
            switch_stage_preview(0)
        elseif check_all_previews_enabled() == true then
            switch_stage_preview(USE_STAGE_NUM - 1)
        else
            switch_stage_preview(last_enabled_preview)
            dont_set_hand_by_panel = true
        end
        if dont_set_hand_by_panel == false then
            local preview = stage_previews[current_selected_preview + 1]
            if preview.is_sub_stage_ == false then
                UiScriptPlayer.invoke("set_hand_position_from_panel", preview.panel_id_)
                preview.panel_id_ = find_proper_panel(preview.panel_id_)
            else
                UiScriptPlayer.invoke("set_hand_position_from_stage_sub_panel", preview.panel_id_)
                stage_sub_selector_info.waiting_in_ = true
            end
        else
            init_medal(current_selected_preview)
        end
        grab_medal(current_selected_preview, true)
        UiScriptPlayer.invoke("cancel_medal_sub_list", current_selected_preview)
        disable_stage_preview(current_selected_preview)
        play_un_decide_animation(current_selected_preview)
    else
        switch_stage_preview(0)
        init_medal(current_selected_preview)
        set_stage_preview_from_stage_panel(0, 0)
        grab_medal(current_selected_preview, true)
        UiScriptPlayer.invoke("cancel_medal_sub_list", current_selected_preview)
    end

    if IS_MODE_STAGE_2_CHANGE == true or IS_MODE_KUMITE == true or IS_MENU_FILTER == true or IS_MY_MUSIC == true or IS_EMPTY_STAGE_SUB_LIST == true or USABLE_SUB_STAGE == false then
        allow_sub_stage = false
    end

    if allow_sub_stage == true then
        root_view:play_animation("tab_position_00", 1.0)
        change_page(false)
        if tab_index == TAB_SWITCH_SUB then
            local medal = medals[current_selected_preview + 1]
            local is_visible = UiScriptPlayer.invoke("ensure_visible_sub_list", medal.put_index_, false)
            if is_visible == false then
                stage_sub_list_selector_:redraw_all_items()
            end
        end
    else
        root_view:play_animation("tab_position_01", 1.0)
        set_main_button_active(BUTTON_TAB_SUB, false)
    end

    if IS_MENU_FILTER == true or IS_MY_MUSIC == true then
        root_view:visible_parts("set_parts_btn_normal", false)
    end

    if IS_SIMPLE_CANCEL == true or IS_INVISIBLE_CANCEL == true then
        set_main_button_active(BUTTON_BACK, false)
    end

    if IS_MODE_KUMITE == true then
        set_stage_preview_form(0, DEFAULT_STAGE_FORM_TYPE)
        set_main_button_active(BUTTON_FORM_TYPE_0, false)
    end

    if DO_NOT_SCROLL_STAGE_SUB_LIST == true then
        navigation_button_selector:set_enable(false)
        navigation_button_selector:set_focus(false)
        root_view:visible_parts("set_parts_btn_csr_t", false)
        root_view:visible_parts("set_parts_btn_csr_b", false)
    end

    if IS_MY_MUSIC == true then
        UiScriptPlayer.invoke("not_use_medal")
    end

    UiScriptPlayer.invoke("setup_bgm")

    change_sub_page(0)
end

-- Cancels, presumably a part of the exit sequence
-- CLOSURE_54, R114
local cancel = function()
    exit_code_ = SCENE_EXIT_CODE_CANCEL
    UiSoundManager.play_se(UI_SE_ID_CANCEL)
    if UiScriptPlayer.invoke("is_fixed_form_type_stage_preview", current_selected_preview) == true then
        -- pretty sure this is a bug lol
        -- this variable is mentioned nowhere else (preview_id)
        local form = UiScriptPlayer.invoke("get_stage_fixed_form_type", preview_id)
        local preview = stage_previews[current_selected_preview + 1]
        if preview.form_type_ ~= form then
            preview.form_type_ = form
        end
    end

    set_stage_preview_from_stage_panel(current_selected_preview, UI_INVALID_INDEX)
    if tab_index == TAB_SWITCH_SUB then
        local medal_parts = root_view:get_parts(get_medal_name(current_selected_preview))
        medal_parts:play_animation("stage_make", 1.0)
    end
end

-- Sets the exit code for a normal exit
-- CLOSURE_55, R115
local exit_normal = function()
    exit_code_ = SCENE_EXIT_CODE_NORMAL
end

-- CLOSURE_56, R116
local prepare_scene_exit = function()
    if scene_state == SCENE_STATE_REGULAR then
        scene_state = SCENE_STATE_SHOULD_EXIT
        return true
    end
    return false
end

-- CLOSURE_57, R117
local un_decide = function(ignore_cancel_input, play_catch_se)
    if (ignore_cancel_input == true or virtual_input:is_cancel() == true) and check_all_previews_disabled() == false then
        local prev_selected_index = current_selected_preview
        local last_enabled = get_last_enabled_preview()
        local preview = stage_previews[last_enabled + 1]
        if preview.is_sub_stage_ == false then
            if tab_index == TAB_SWITCH_SUB then
                tab_index = TAB_SWITCH_NORMAL
                change_page(true)
            end
        else
            if tab_index == TAB_SWITCH_NORMAL then
                tab_index = TAB_SWITCH_SUB
                change_page(true)
            end
            local medal = UiScriptPlayer.invoke("get_medal_on_stage_sub_id", last_enabled)
            if medal ~= UI_INVALID_INDEX then
                UiScriptPlayer.invoke("ensure_visible_sub_list", medal, false)
                stage_sub_selector_info.select_sub_id_ = UI_INVALID_INDEX
            end
        end

        switch_stage_preview(last_enabled)
        if check_all_previews_enabled() == false then
            UiScriptPlayer.invoke("set_enable_shortcut_button_stage_preview", prev_selected_index, true)
            set_stage_preview_from_stage_panel(prev_selected_index, UI_INVALID_INDEX)
            if IS_MODE_STAGE_2_CHANGE == true and is_invalid_stage_2 == true then
                root_view:play_animation_parts(get_stage_preview_name(prev_selected_index), "off_atteintion")
                is_invalid_stage_2 = false
                prev_invalid_stage_2 = false
            end
        elseif current_selected_preview ~= prev_selected_index then
            play_select_animation(current_selected_preview)
        end

        un_decide_medal()
        if play_catch_se == true then
            UiSoundManager.play_se_label("se_system_plate_catch")
        end
        return true
    end
    return false
end

-- Stops canceling the scene
-- CLOSURE_58, R118
local stop_canceling = function() 
    if is_canceling == true then
        is_canceling = false
        local button = get_main_button(BUTTON_BACK)
        button:stop_pressed(false)
        stop_long_cancel_se()
    end
end

-- Tries to cancel with the back button
-- CLOSURE_59, R119
local try_back_cancel = function()
    if BACK_POPUP_ID ~= nil then
        AppPopupManager.open_database(BACK_POPUP_ID)
        coroutine.yield()
        while AppPopupManager.is_busy() == true do
            coroutine.yield()
        end
        if AppPopupManager.get_result() == POPUP_RESULT_NO then
            stop_canceling()
            return false
        end
    end
    return true
end

-- Checks for and tries to cancel
-- CLOSURE_60, R120
local check_for_cancel = function()
    if IS_INVISIBLE_CANCEL == true then
        return false
    elseif IS_SIMPLE_CANCEL == true then
        if virtual_input:is_cancel() == true then
            back_button_selector:decide_button(BUTTON_BACK)
            cancel()
            return true
        end
        return false
    end

    local back_button = get_main_button(BUTTON_BACK)
    local is_cancel_input = false
    if check_all_previews_disabled() == true then
        is_cancel_input = virtual_input:is_pressing(INPUT_CANCEL)
    end
    if ignore_cancel_input == true then
        if is_cancel_input == false then
            ignore_cancel_input = false
        end
        return false
    end
    if is_canceling ~= is_cancel_input then
        is_canceling = is_cancel_input
        if is_canceling == true then
            back_button:play_pressed(true)
            play_long_cancel_se()
        else
            back_button:stop_pressed(false)
            stop_long_cancel_se()
        end
    end

    if is_canceling == true and back_button:is_pressed_finished() == true then
        stop_long_cancel_se()
        if try_back_cancel() == false then
            return false
        end
        cancel()
        return true
    end
    return false
end

local strike_stage = function(panel_id, is_strike)
    if panel_id ~= UI_INVALID_INDEX and stage_panels[panel_id + 1].is_striked_ ~= is_strike then
        stage_panels[panel_id + 1].is_striked_ = is_strike
        local parts = root_view:get_parts(get_stage_panel_name(panel_id))
        local strike_pane = parts:get_pane("set_rep_strike")
        strike_pane:set_visible(is_strike)
    end
end


local check_for_strike_cancel = function()
    strike_cancel.is_canceling_ = virtual_input:is_pressing(INPUT_STRIKE)
    
    if strike_cancel.is_canceling_ == true then
        strike_cancel.current_ = strike_cancel.current_ + 1
        if strike_cancel.current_ == strike_cancel.start_se_ then
            play_long_cancel_se()
        end
    else
        strike_cancel.current_ = 0
        stop_long_cancel_se()
    end

    if strike_cancel.current_ == strike_cancel.total_ then
        for i=1, STAGE_PANEL_LIST_NUM, 1 do
            strike_stage(i - 1, false)
        end
        stop_long_cancel_se()
    end
end

-- Cycles the stage form
-- CLOSURE_61, R121
local cycle_stage_form = function()
    if tab_index == TAB_SWITCH_NORMAL then
        if ENABLE_STAGE_FORM_TYPE == true then
            local next_form = get_next_stage_form(stage_previews[current_selected_preview + 1].form_type_)
            set_stage_preview_form(current_selected_preview, next_form)
            switch_stage_form(current_selected_preview, next_form)
            for i=1, USE_STAGE_NUM, 1 do
                local preview = stage_previews[i]
                if preview.enable_ == false then
                    if i - 1 ~= current_selected_preview then
                        preview.form_type_ = next_form
                    end
                    local is_fixed = UiScriptPlayer.invoke("is_fixed_form_type_stage_preview", i - 1)
                    if preview.form_type_ ~= STAGE_FORM_TYPE_NORMAL and is_fixed == false then
                        preview.form_type_parts_:stop_animation_at_end("blink_on")
                    else
                        preview.form_type_parts_:play_animation("blink_off", 1.0)
                        preview.form_type_blink_counter_ = 0
                    end
                end
            end
        end
        return
    end

    if allow_sub_stage == true then
        tab_index = TAB_SWITCH_NORMAL
        change_page(true)
    end
end

-- CHanges to the sub/custom stage tab
-- CLOSURE_62, R122
local change_to_sub_tab = function()
    if allow_sub_stage == true and tab_index ~= TAB_SWITCH_SUB then
        tab_index = TAB_SWITCH_SUB
        change_page(true)
    end
end

-- Advances the stage form for the specified id
-- CLOSURE_63, R123
local advance_stage_form = function(preview_id)
    if UiScriptPlayer.invoke("is_empty_stage_preview", preview_id) == false then
        local form = get_next_stage_form(stage_previews[preview_id + 1].form_type_)
        set_stage_preview_form(preview_id, form)
        switch_stage_form(preview_id, form)
    end
end

-- Advanced the stage form for the specified button
-- CLOSURE_64, R124
local advance_stage_form_by_button = function(button_id)
    if IS_MODE_KUMITE == false then
        if button_id == BUTTON_FORM_TYPE_0 or button_id == BUTTON_FORM_TYPE_1 or button_id == BUTTON_FORM_TYPE_2 then
            local preview_id = (button_id - BUTTON_FORM_TYPE_0) >> 1
            advance_stage_form(preview_id)
            return true
        end
    end
    return false
end

-- Opens the bgm selection menu
-- CLOSURE_65, R125
local open_bgm_select = function(preview_id, show_hand)
    if ENABLE_SELECT_BGM == false or is_invalid_stage_2 == true then
        return
    end

    if UiScriptPlayer.invoke("is_random_stage_preview", preview_id) == true then
        return
    end

    if highlighed_preview ~= UI_INVALID_INDEX then
        if stage_previews[highlighed_preview + 1].is_sub_stage_ == true then
            return
        end
    elseif tab_index == TAB_SWITCH_SUB then
        return
    end

    if UiScriptPlayer.invoke("is_empty_stage_preview", preview_id) == false then
        stop_canceling()
        local bgm_arg = false
        if preview_id == current_selected_preview and check_all_previews_enabled() == false then
            bgm_arg = true
        end
        stage_select_bgm:activate(preview_id, bgm_arg, true)
        stage_select_bgm.layout_update_function = update_stage_preview_bgm_select
        show_scene_and_hand(false, show_hand)
    end
end

-- Opens the bgm selection menu from a button id
-- CLOSURE_66, R126
local open_bgm_select_by_button = function(button_id)
    if button_id == BUTTON_MUSIC_0 or button_id == BUTTON_MUSIC_1 or button_id == BUTTON_MUSIC_2 then
        local preview_id = (button_id - BUTTON_MUSIC_0) >> 1
        open_bgm_select(preview_id, true)
        return true
    end

    return false
end

-- CLOSURE_67, R127
local decide_normal_stage = function()
    if check_all_previews_enabled() == true then
        if any_medal_grabbed() == false then
            update_medal()
        end
        return false
    end

    if is_invalid_stage_2 == true then
        return false
    end
    
    if current_selected_panel == UI_INVALID_INDEX then
        return false
    end

    if stage_panels[current_selected_panel + 1].is_striked_ then
        UiSoundManager.play_se(UI_SE_ID_ERROR)
        return false
    end

    UiScriptPlayer.invoke("set_medal_visible", current_selected_preview, true)
    UiScriptPlayer.invoke("set_medal_collect_range_from_panel", current_selected_preview, current_selected_panel)
    if UiScriptPlayer.invoke("is_hand_interpolated_moving") == false then
        UiScriptPlayer.invoke("set_medal_position_from_hand", current_selected_preview)
    end

    grab_medal(current_selected_preview, false)

    set_medal_info(current_selected_preview, MEDAL_STATE_PLACED, false)

    medals[current_selected_preview + 1].put_index_ = current_selected_panel

    enable_stage_preview(current_selected_preview, current_selected_panel, false)
    UiScriptPlayer.invoke("set_extra_command_stage_preview", current_selected_preview)

    play_decide_stage_animation(current_selected_preview)

    return true
end

-- CLOSURE_68, R128
local decide_sub_stage = function()
    local scroll_value = math.abs(stage_sub_selector_info.scroll_value_)
    if 0.0 < scroll_value then
        return false
    end

    if check_all_previews_enabled() == true then
        if any_medal_grabbed() == false then
            update_medal()
        end
        return false
    end

    if is_invalid_stage_2 == true then
        return false
    end

    if stage_sub_selector_info.select_sub_id_ == UI_INVALID_INDEX then
        return false
    end

    if IS_ONLINE == true then
        if UiScriptPlayer.invoke("is_enable_made_stage_sub_list") == false then
            local db = UiScriptPlayer.invoke("get_disable_made_stage_popup_id_sub_list")
            AppPopupManager.open_database(db)
            repeat
                coroutine.yield()
            until AppPopupManager.is_busy() == false
            return false
        end
        if UiScriptPlayer.invoke("is_member_is_four_sub_list", stage_sub_selector_info.select_sub_id_) == true then
            AppPopupManager.open_database("ID_POPUP_STAGE_SELECT_STGCREATE_LIMIT_RANDOM")
            repeat
                coroutine.yield()
            until AppPopupManager.is_busy() == false
        end
    elseif UiScriptPlayer.invoke("is_random_sub_list", stage_sub_selector_info.select_sub_id_) == true then
        if UiScriptPlayer.invoke("is_all_panel_exceeded_num_player_sub_list") == true then
            AppPopupManager.open_database("ID_POPUP_STAGE_SELECT_STGCREATE_NO_STAGE")
            repeat
                coroutine.yield()
            until AppPopupManager.is_busy() == false
            return false
        end
    elseif UiScriptPlayer.invoke("is_exceeded_num_player_sub_list", stage_sub_selector_info.select_sub_id_) == true then
        AppPopupManager.open_database("ID_POPUP_STAGE_SELECT_STGCREATE_LIMIT")
        repeat
            coroutine.yield()
        until AppPopupManager.is_busy() == false
        return false
    end

    UiScriptPlayer.invoke("put_medal_sub_list", current_selected_preview, stage_sub_selector_info.select_sub_id_, false)
    grab_medal(current_selected_preview, false)
    set_medal_info(current_selected_preview, MEDAL_STATE_PLACED, true)
    medals[current_selected_preview + 1].put_index_ = stage_sub_selector_info.select_sub_id_
    enable_stage_preview(current_selected_preview, stage_sub_selector_info.select_sub_id_, true)
    play_decide_stage_animation(current_selected_preview)
    stage_sub_selector_info.select_sub_id_ = UI_INVALID_INDEX
    return true
end

-- CLOSURE_69, R129
local handle_panel_decide = function()
    local decided = false
    if tab_index == TAB_SWITCH_NORMAL then
        decided = decide_normal_stage()
    else
        decided = decide_sub_stage()
    end

    if decided == false then
        return
    end

    if IS_MENU_FILTER == false and IS_MY_MUSIC == false then
        UiScriptPlayer.invoke("play_rumble_input_device")
    end

    UiSoundManager.play_se_label("se_system_plate_off_stageselect")
    if IS_DECIDE_SE_AUDIENCE == true then
       UiSoundManager.play_se_label("se_audience_suddendeath")
    end

    if check_all_previews_enabled() == true then
        UiScriptPlayer.invoke("set_hand_grabbed", false)
        play_off_preview_animation(current_selected_preview)
        if tab_index == TAB_SWITCH_NORMAL then
            change_panel(UI_INVALID_INDEX)
        elseif stage_sub_list_selector_ ~= nil then
            stage_sub_list_selector_:set_focus(false)
        end
        prepare_scene_exit()
        UiScriptPlayer.invoke("set_hand_grabable", false)
        UiScriptPlayer.invoke("set_hand_operatable", false)
    else
        UiScriptPlayer.invoke("play_hand_released")
        switch_stage_preview(get_next_disabled_preview())
        UiScriptPlayer.invoke("set_medal_visible", current_selected_preview, true)
        set_medal_info(current_selected_preview, MEDAL_STATE_ACTIVE, false)
        if IS_MODE_STAGE_2_CHANGE == true and UiScriptPlayer.invoke("is_random_stage_preview", 0) == false then
            UiScriptPlayer.invoke("move_hand_interpolated", 50.0, -50.0)
        else
            set_stage_preview_from_stage_panel(current_selected_preview, current_selected_panel)
            if tab_index == TAB_SWITCH_NORMAL then
                UiScriptPlayer.invoke("set_enable_shortcut_button_stage_preview", current_selected_preview, true)
            end
        end

        if tab_index == TAB_SWITCH_SUB then
            local medal_parts = root_view:get_parts(get_medal_name(current_selected_preview))
            medal_parts:play_animation("stage_make", 1.0)
        end
    end
end

-- CLOSURE_70, R130
local handle_button_decide = function()
    local decided = root_selected_button.decided_button_id_
    if decided == BUTTON_BACK then
        stop_canceling()
        if try_back_cancel() == false then
            return false
        end
        cancel()
        return true
    end

    if decided == BUTTON_TAB_NORMAL then
        cycle_stage_form()
        return true
    end

    if decided == BUTTON_TAB_SUB then
        change_to_sub_tab()
        return true
    end

    if IS_SIMPLE_CANCEL == true and back_button_selector:get_decided_button_id() == BUTTON_BACK then
        cancel()
        return true
    end

    decided = UI_INVALID_INDEX
    if ENABLE_STAGE_FORM_TYPE == true then
        for i=1, USE_STAGE_NUM, 1 do
            decided = preview_selected_buttons[i].decided_button_id_
            if BUTTON_FORM_TYPE_0 <= decided then
                break
            end
        end

        if advance_stage_form_by_button(decided) == true then
            return true
        end

        if open_bgm_select_by_button(decided) == true then
            return true
        end
    end

    if tab_index ~= TAB_SWITCH_SUB then
        return false
    end

    local nav_decided = sub_stage_nav_button.decided_button_id_
    if nav_decided ~= UI_INVALID_INDEX then
        local scroll_amount = nil
        if nav_decided == BUTTON_ID_SUB_STAGE_UP then
            scroll_amount = -STAGE_SUB_LIST_PAGE_SCROLL_VALUE
        elseif nav_decided == BUTTON_ID_SUB_STAGE_DOWN then
            scroll_amount = STAGE_SUB_LIST_PAGE_SCROLL_VALUE
        end
        stage_sub_selector_info.scroll_value_ = stage_sub_selector_info.scroll_value_ + scroll_amount
        return true
    else
        if stage_sub_list_selector_ ~= nil then
            stage_sub_list_selector_:set_auto_scroll(0.0, false)
        end
    end
    return false
end

-- CLOSURE_71, R131
local normal_tab_main_update = function()
    if check_all_previews_enabled() == false then
        if UiScriptPlayer.invoke("is_hand_interpolated_moving") == false then
            local panel_id = UiScriptPlayer.invoke("get_hand_on_stage_panel_id")
            panel_id = find_proper_panel(panel_id)
            if change_panel(panel_id) == true or is_hand_interpolated_moving == true then
                stage_previews[current_selected_preview + 1].selected_alt_ = 0
                set_alt_panel_textures(nil)
                if UiScriptPlayer.invoke("is_training_stage_preview", current_selected_preview) == true then
                    local preview = stage_previews[current_selected_preview + 1]
                    if preview.form_type_ ~= STAGE_FORM_TYPE_NORMAL and preview.form_type_blink_counter_ <= 0 then
                        preview.form_type_parts_:play_animation("blink_on", 1.0)
                    end
                    preview.form_type_parts_:play_animation("able_shortcut", 1.0)
                end
                set_stage_preview_from_stage_panel(current_selected_preview, current_selected_panel)

                if tab_index == TAB_SWITCH_NORMAL then
                    UiScriptPlayer.invoke("set_enable_shortcut_button_stage_preview", current_selected_preview, true)
                end

                if UiScriptPlayer.invoke("is_training_stage_preview", current_selected_preview) == true then
                    local preview = stage_previews[current_selected_preview + 1]
                    preview.form_type_parts_:play_animation("unable_shortcut", 1.0)
                end
            end
            is_hand_interpolated_moving = false
        else
            is_hand_interpolated_moving = true
        end
    end
end

-- CLOSURE_72, R132
local sub_tab_main_update = function()
    if stage_sub_list_selector_ == nil then
        return
    end

    local will_play_cursor_sound = true
    local scroll_value = math.abs(stage_sub_selector_info.scroll_value_)
    if 0.0 < scroll_value then
        if stage_sub_selector_info.is_play_se_ == true then
            stage_sub_selector_info.is_play_se_ = false
            UiSoundManager.play_se_label("se_system_scrollbar")
        end

        local scroll_amount = 0.0
        if scroll_amount_max <= scroll_value then
            if stage_sub_selector_info.scroll_value_ < 0.0 then
                stage_sub_selector_info.scroll_value_ = stage_sub_selector_info.scroll_value_ + scroll_amount_max
                scroll_amount = -scroll_amount_max
            else
                stage_sub_selector_info.scroll_value_ = stage_sub_selector_info.scroll_value_ - scroll_amount_max
                scroll_amount = scroll_amount_max
            end
        else
            scroll_amount = stage_sub_selector_info.scroll_value_
            stage_sub_selector_info.scroll_value_ = 0.0
            stage_sub_selector_info.scroll_end_ = true
            stage_sub_selector_info.is_play_se_ = true
        end

        stage_sub_list_selector_:set_auto_scroll(scroll_amount, false)
        return
    elseif stage_sub_selector_info.scroll_end_ == true then
        stage_sub_selector_info.scroll_end_ = false
        stage_sub_selector_info.is_play_se_ = true
        stage_sub_list_selector_:set_auto_scroll(0.0, false)
        will_play_cursor_sound = false
    end

    if UiScriptPlayer.invoke("is_top_sub_list") == true then
        if navigation_button_selector:is_selectable(BUTTON_ID_SUB_STAGE_UP) == true then
            navigation_button_selector:set_selectable(BUTTON_ID_SUB_STAGE_UP, false)
        end
    elseif navigation_button_selector:is_selectable(BUTTON_ID_SUB_STAGE_UP) == false then
        navigation_button_selector:set_selectable(BUTTON_ID_SUB_STAGE_UP, true)
    end

    if UiScriptPlayer.invoke("is_lowest_sub_list") == true then
        if navigation_button_selector:is_selectable(BUTTON_ID_SUB_STAGE_DOWN) == true then
            navigation_button_selector:set_selectable(BUTTON_ID_SUB_STAGE_DOWN, false)
        end
    elseif navigation_button_selector:is_selectable(BUTTON_ID_SUB_STAGE_DOWN) == false then
        navigation_button_selector:set_selectable(BUTTON_ID_SUB_STAGE_DOWN, true)
    end

    if stage_sub_selector_info.scroll_value_ == 0.0 then
        local scroll_difference = 0.0
        if virtual_input:is_pressing(INPUT_SUB_PAGE_UP) == true then
            if navigation_button_selector:is_selectable(BUTTON_ID_SUB_STAGE_UP) == true then
                scroll_difference = -STAGE_SUB_LIST_PAGE_SCROLL_VALUE
            end
        elseif virtual_input:is_pressing(INPUT_SUB_PAGE_DOWN) == true then
            if navigation_button_selector:is_selectable(BUTTON_ID_SUB_STAGE_DOWN) == true then
                scroll_difference = STAGE_SUB_LIST_PAGE_SCROLL_VALUE
            end
        end
        stage_sub_selector_info.scroll_value_ = stage_sub_selector_info.scroll_value_ + scroll_difference
    end

    if check_all_previews_enabled() == true then
        return
    end

    if UiScriptPlayer.invoke("is_hand_interpolated_moving") == false then
        local hand_id = UiScriptPlayer.invoke("get_hand_on_stage_sub_id")
        if hand_id ~= stage_sub_selector_info.select_sub_id_ then
            stage_sub_selector_info.select_sub_id_ = hand_id
            stage_sub_list_selector_:select_item(stage_sub_selector_info.select_sub_id_, true)
            if stage_sub_selector_info.select_sub_id_ ~= UI_INVALID_INDEX then
                if stage_sub_list_selector_:has_focus() == false then
                    stage_sub_list_selector_:set_focus(true)
                end
                if stage_sub_selector_info.waiting_in_ == true then
                    if root_view:is_animation_finished("in") == true then
                        stage_sub_selector_info.waiting_in_ = false
                        should_play_cursor_sound = true
                    else
                        will_play_cursor_sound = false
                    end
                end
                if will_play_cursor_sound == true then
                    play_cursor_sound()
                end
            end
            set_stage_preview_from_sub_stage_panel(current_selected_preview, stage_sub_selector_info.select_sub_id_)
        end
    end
end

-- CLOSURE_73, R133
local update_preview_blink = function(preview_id, preview)
    if preview.form_type_ == STAGE_FORM_TYPE_NORMAL or UiScriptPlayer.invoke("is_fixed_form_type_stage_preview", preview_id - 1) == true then
        if preview.on_button_form_type_ == false and preview.form_type_blink_counter_ > 0 then
            preview.form_type_blink_counter_ = preview.form_type_blink_counter_ - 1
            if preview.form_type_blink_counter_ <= 0 then
                preview.form_type_parts_:play_animation("blink_off", 1.0)
                preview.form_type_blink_counter_ = 0
            end
        end
    else
        preview.form_type_parts_:play_animation_properly("blink_on", 1.0)
        preview.form_type_blink_counter_ = blink_counter
    end
end

-- CLOSURE_74, R134
local update_stage_previews = function()
    prev_highlighed_preview = highlighed_preview

    highlighed_preview =  UiScriptPlayer.invoke("get_hand_on_stage_preview_id")

    if highlighed_preview ~= UI_INVALID_INDEX then
        if stage_previews[highlighed_preview + 1].enable_ == false then
            if prev_highlighed_preview ~= UI_INVALID_INDEX then
                advance_stage_preview(highlighed_preview, prev_highlighed_preview)
            end
            highlighed_preview = UI_INVALID_INDEX
        elseif highlighed_preview ~= prev_highlighed_preview then
            if prev_highlighed_preview ~= UI_INVALID_INDEX then
                advance_stage_preview(highlighed_preview, prev_highlighed_preview)
            elseif highlighed_preview ~= current_selected_preview then
                advance_stage_preview(highlighed_preview, current_selected_preview)
            end
        end
    elseif prev_highlighed_preview ~= UI_INVALID_INDEX then
        advance_stage_preview(current_selected_preview, prev_highlighed_preview)
        prev_highlighed_preview = UI_INVALID_INDEX
    end

    if ENABLE_STAGE_FORM_TYPE == true and virtual_input:is_pressed(INPUT_STAGE_TYPE) == true then
        local index = UI_INVALID_INDEX
        if highlighed_preview ~= UI_INVALID_INDEX then
            if UiScriptPlayer.invoke("is_fixed_form_type_stage_preview", highlighed_preview) == false then
                local preview = stage_previews[highlighed_preview + 1]
                if preview.enable_ == true and preview.is_sub_stage_ == false then
                    local next_form = get_next_stage_form(preview.form_type_)
                    set_stage_preview_form(highlighed_preview, next_form)
                    switch_stage_form(highlighed_preview, next_form)
                    index = highlighed_preview
                end
            end
        elseif check_all_previews_enabled() == true or get_panel_id ~= UI_INVALID_INDEX then
            if UiScriptPlayer.invoke("is_fixed_form_type_stage_preview", current_selected_preview) == false and tab_index == TAB_SWITCH_NORMAL then
                local next_form = get_next_stage_form(stage_previews[current_selected_preview + 1].form_type_)
                set_stage_preview_form(current_selected_preview, next_form)
                switch_stage_form(current_selected_preview, next_form)
                index = current_selected_preview
            end
        end
        
        if index ~= UI_INVALID_INDEX then
            local preview = stage_previews[index + 1]
            if preview.on_button_form_type_ == false then
                if preview.form_type_blink_counter_ <= 0 then
                    preview.form_type_parts_:play_animation("blink_on", 1.0)
                end
                preview.form_type_blink_counter_ = blink_counter
            end
        end
    end

    if virtual_input:is_pressed(INPUT_STRIKE) == true then
        strike_stage(current_selected_panel, true)
    end
    
    local is_striked = current_selected_panel ~= UI_INVALID_INDEX and stage_panels[current_selected_panel + 1].is_striked_
    if is_striked ~= prev_invalid_stage_2 then
        local anim = "on_atteintion"
        if not is_striked then
            anim = "off_atteintion"
        end
        root_view:play_animation_parts(get_stage_preview_name(current_selected_preview), anim)
        local preview_parts = root_view:get_parts(get_stage_preview_name(current_selected_preview))
        preview_parts:get_pane("txt_attention"):set_text_string("This stage has been striked")
    end
    prev_invalid_stage_2 = is_striked

    for i=1, USE_STAGE_NUM, 1 do
        local button_id = BUTTON_FORM_TYPE_0 + (i - 1) * PREVIEW_BUTTON_COUNT
        local selected = preview_selected_buttons[i].selected_button_id_ - button_id
        local is_selected_preview = false
        if highlighed_preview == i - 1 then
            is_selected_preview = true
        end
        local preview = stage_previews[i]
        if preview.on_button_form_type_ == false and selected == BUTTON_ID_FORM_TYPE then
            if preview.form_type_ == STAGE_FORM_TYPE_NORMAL then
                preview.form_type_parts_:play_animation("blink_on", 1.0)
            end
            preview.form_type_parts_:play_animation("color_blink_on", 1.0)
            preview.on_button_form_type_ = true
        elseif preview.on_button_form_type_ == true and selected ~= BUTTON_ID_FORM_TYPE then
            if preview.form_type_parts_:is_animation_finished("blink_on") == true then
                if preview.form_type_ == STAGE_FORM_TYPE_NORMAL or UiScriptPlayer.invoke("is_fixed_form_type_stage_preview", i - 1) == true then
                    preview.form_type_parts_:play_animation("blink_off", 1.0)
                    preview.form_type_blink_counter_ = 0
                end
                if is_selected_preview == true then
                    preview.form_type_parts_:play_animation("color_blink_off", 1.0)
                end
                preview.on_button_form_type_ = false
            else
                preview_selected_buttons[i].selected_button_id_ = BUTTON_ID_FORM_TYPE + button_id
            end
        end

        update_preview_blink(i, preview)
        if preview.on_button_music_ == false and selected == BUTTON_ID_MUSC_SELECT then
            preview.music_parts_:play_animation("color_blink_on", 1.0)
            preview.on_button_music_ = true
        elseif preview.on_button_music_ == true and selected ~= BUTTON_ID_MUSC_SELECT then
            if is_selected_preview == true then
                preview.music_parts_:play_animation("color_blink_off", 1.0)
            end
            preview.on_button_music_ = false
        end
    end

    if IS_MODE_STAGE_2_CHANGE == true and UiScriptPlayer.invoke("is_hand_interpolated_moving") == false then
        if is_invalid_stage_2 ~= prev_invalid_stage_2 then
            local anim = "on_atteintion"
            if is_invalid_stage_2 == false then
                anim = "off_atteintion"
            end
            root_view:play_animation_parts(get_stage_preview_name(current_selected_preview), anim)
        end
        prev_invalid_stage_2 = is_invalid_stage_2
    end
        
end

-- CLOSURE_75
update_stage_preview_bgm_select = function()
    for i=1, USE_STAGE_NUM, 1 do
        local preview = stage_previews[i]
        if preview.on_button_form_type_ == true then
            if preview.form_type_ == STAGE_FORM_TYPE_NORMAL or UiScriptPlayer.invoke("is_fixed_form_type_stage_preview", i - 1) == true then
                preview.form_type_parts_:play_animation("blink_off", 1.0)
                preview.form_type_blink_counter_ = 0
            end         
            preview.form_type_parts_:play_animation("color_blink_off", 1.0)
            preview.on_button_form_type_ = false
        end
        update_preview_blink(i, preview)
    end
end

-- Updates all the panel scalings
-- CLOSURE_76, R135
local update_panel_scalings = function()
    for i=0, STAGE_PANEL_LIST_NUM - 1, 1 do
        update_panel_scaling(i)
    end
end


local handle_change_page = function(dir)
    if dir == 0 then
        return
    end

    -- if there is only one page, do nothing
    if total_pages == 1 then
        return
    end

    -- Target page is going to be the page that we are transitioning to
    -- Since there is a max amount of pages (value of `target_pages`), we can use
    -- that value as the indicator of when we are going to be seeing the custom stages screen
    local target_page = nil
    -- Check if we are in the custom stages screen
    if tab_index == TAB_SWITCH_SUB then
        -- If we are going to the left, then set our page at the last page
        if dir < 0 then
            target_page = total_pages - 1
        else -- Otherwise, restart at the beginning
            target_page = 0
        end
    else
        -- If we are going to the left and we are on the normal stages screen, then
        -- we need to do some extra checks
        if dir < 0 then
            -- If we are on the first page, then going to the left puts us on the custom stages screen
            if current_page == 0 then
                target_page = total_pages
            else -- Otherwise just go left a page
                target_page = current_page - 1
            end
        else -- Increase our page by one if going to the right, will naturally take us to the custom stages screen
            target_page = current_page + 1
        end
    end

    -- Check if we are on the custom stages screen, ensure that we are able to see custom stages
    if target_page == total_pages and allow_sub_stage == false then
        -- If our direction is left then go to the final page, if our direction is right then go to the first page
        target_page = dir < 0 and (total_pages - 1) or 0
    end

    -- At this point, if our target_page is total_pages, we go to custom stages
    if target_page == total_pages then
        tab_index = TAB_SWITCH_SUB
        change_page(true)
    else -- Otherwise, see if we need to switch to the normal stage screen first
        if tab_index == TAB_SWITCH_SUB then
            tab_index = TAB_SWITCH_NORMAL
            change_page(true)
        end
        -- Set our sub page
        change_sub_page(target_page)
        UiSoundManager.play_se_label("se_system_page_change")
    end

end

local get_page_button_dir = function()
    local PaneBoundingBox = {
        new = function(pane)
            local w, h = pane:get_size()
            local x, y = pane:get_position()
            local bbox = {
                top = y + h / 2,
                bottom = y - h / 2,
                left = x - w / 2,
                right = x + w / 2
            }
            function bbox:contains(x, y)
                return self.left <= x and self.right >= x and self.bottom <= y and self.top >= y
            end
            return bbox
        end
    }
    -- Handles the page changing
    local move_dir = 0
    -- Check for the hit on left or right
    local x = UiScriptPlayer.invoke("get_hand_position_x")
    local y = UiScriptPlayer.invoke("get_hand_position_y")
    
    local hit_left_pane = root_view:get_pane("hit_backward")
    local hit_right_pane = root_view:get_pane("hit_forward")

    local left_box = PaneBoundingBox.new(hit_left_pane)
    local right_box = PaneBoundingBox.new(hit_right_pane)

    local is_decide = virtual_input:is_decide()

    if left_box:contains(x, y) and is_decide then
        move_dir = -1
    elseif right_box:contains(x, y) and is_decide then
        move_dir = 1
    end
    return move_dir
end

-- CLOSURE_77, R136
local update_both_tabs = function()
    local PaneBoundingBox = {
        new = function(pane)
            local w, h = pane:get_size()
            local x, y = pane:get_position()
            local bbox = {
                top = y + h / 2,
                bottom = y - h / 2,
                left = x - w / 2,
                right = x + w / 2
            }
            function bbox:contains(x, y)
                return self.left <= x and self.right >= x and self.bottom <= y and self.top >= y
            end
            return bbox
        end
    }

    -- Handles the page changing
    local move_dir = get_page_button_dir()
    
    if virtual_input:is_pressed(INPUT_TAB_CHANGE_L) == true then
        move_dir = -1
    elseif virtual_input:is_pressed(INPUT_TAB_CHANGE_R) == true then
        move_dir = 1
    end

    handle_change_page(move_dir)

    if tab_index == TAB_SWITCH_NORMAL then
        normal_tab_main_update()
    else
        sub_tab_main_update()
    end

    update_stage_previews()
    update_panel_scalings()
    local is_medal_display = false
    if UiScriptPlayer.invoke("is_medal_display_from_hand") == true and get_current_medal_index() ~= UI_INVALID_INDEX then
        is_medal_display = true
    end

    UiScriptPlayer.invoke("set_hand_grabable", is_medal_display)

    local is_sub_stage_button = false
    local is_button_selected = false
    if root_selected_button.selected_button_id_ >= 0 then
        is_button_selected = true
    elseif back_button.selected_button_id_ >= 0 then
        is_button_selected = true
    elseif sub_stage_nav_button.selected_button_id_ >= 0 then
        is_button_selected = true
        is_sub_stage_button = true
    else
        for i=1, USE_STAGE_NUM, 1 do
            if preview_selected_buttons[i].selected_button_id_ >= 0 then
                is_button_selected = true
                break
            end
        end
    end

    UiScriptPlayer.invoke("set_hand_operatable", is_button_selected)
    if is_sub_stage_button == true then
        if check_all_previews_enabled() == false then
            stage_sub_scroll_button_operatable_ = true
            UiScriptPlayer.invoke("set_hand_grabbed", false)
            UiScriptPlayer.invoke("set_medal_visible", current_selected_preview, false)
        end
    elseif stage_sub_scroll_button_operatable_ == true then
        stage_sub_scroll_button_operatable_ = false
        UiScriptPlayer.invoke("set_hand_grabbed", true)
        UiScriptPlayer.invoke("set_medal_visible", current_selected_preview, true)
    end
end

-- CLOSURE_78, R137
local update_from_pointer = function()
    local x = UiScriptPlayer.invoke("get_hand_position_x")
    local y = UiScriptPlayer.invoke("get_hand_position_y")

    x, y = layout_root:layout_to_screen_position(x, y)

    local selected, decided = nil

    selected, decided = root_button_selector:update_pointer_input(x, y, virtual_input, true)
    
    root_selected_button.selected_button_id_ = selected
    root_selected_button.decided_button_id_ = decided

    if IS_SIMPLE_CANCEL == true then
        selected, decided = back_button_selector:update_pointer_input(x, y, virtual_input, true)
        back_button.selected_button_id_ = selected
        back_button.decided_button_id_ = decided
    end

    if tab_index == TAB_SWITCH_SUB then
        selected, decided = navigation_button_selector:update_pointer_input(x, y, virtual_input, true)
        sub_stage_nav_button.selected_button_id_ = selected
        sub_stage_nav_button.decided_button_id_ = decided
    else
        sub_stage_nav_button.selected_button_id_ = UI_INVALID_INDEX
        sub_stage_nav_button.decided_button_id_ = UI_INVALID_INDEX
    end

    for i=1, USE_STAGE_NUM, 1 do
        selected, decided = preview_button_selectors[i]:update_pointer_input(x, y, virtual_input, true)
        preview_selected_buttons[i].selected_button_id_ = selected
        preview_selected_buttons[i].decided_button_id_ = decided
    end
end

-- CLOSURE_79, R138
local try_handle_exiting_scene = function()
    if scene_state == SCENE_STATE_REGULAR or scene_state == SCENE_STATE_EXITED then
        return false
    end

    update_panel_scalings()

    if scene_state == SCENE_STATE_SHOULD_EXIT then
        next_scene_animation:play(1.0)
        scene_state = SCENE_STATE_EXITING
    elseif scene_state == SCENE_STATE_EXITING then
        if next_scene_animation:is_finished() == true then
            scene_state = SCENE_STATE_EXITED
            return false
        else

            local was_invalid_sub_stage = false
            for i=1, USE_STAGE_NUM, 1 do
                if stage_previews[i].is_sub_stage_ == true then
                    if UiScriptPlayer.invoke("is_enable_made_stage_sub_list") ~= false then
                        break
                    end
                    local db = UiScriptPlayer.invoke("get_disable_made_stage_popup_id_sub_list")
                    AppPopupManager.open_database(db)
                    repeat
                        coroutine.yield()
                    until AppPopupManager.is_busy() == false
                    was_invalid_sub_stage = true
                    break
                end
            end

            local is_bgm_pressed = virtual_input:is_pressed(INPUT_BGM_SELECT)

            if is_bgm_pressed == true or was_invalid_sub_stage == true or virtual_input:is_cancel() == true or virtual_input:is_pressed(INPUT_STAGE_TYPE) == true then
                local panel_id = stage_previews[current_selected_preview + 1].panel_id_
                next_scene_animation:stop_at_start()
                scene_state = SCENE_STATE_REGULAR
                local should_play_se = true
                if is_bgm_pressed == true then
                    should_play_se = false
                else
                    root_view:play_animation_parts(get_stage_preview_name(current_selected_preview), "on_crs_preview_anime")
                end

                un_decide(true, should_play_se)
                if tab_index == TAB_SWITCH_NORMAL then
                    should_play_cursor_sound = false
                    change_panel(panel_id)
                else
                    stage_sub_selector_info.select_sub_id_ = panel_id
                    stage_sub_list_selector_:select_item(stage_sub_selector_info.select_sub_id_, true)
                    if stage_sub_list_selector_:has_focus() == false then
                        stage_sub_list_selector_:set_focus(true)
                    end
                end
                if is_bgm_pressed == true then
                    open_bgm_select(current_selected_preview, false)
                elseif check_all_previews_disabled() == true then
                    ignore_cancel_input = true
                end
            end
        end
    end
    return true
end

local change_selected_alt = function(is_forward)
    if current_selected_preview == UI_INVALID_INDEX then
        Alts.send_message("Can't change alt on invalid preview")
        return
    end

    local preview = stage_previews[current_selected_preview + 1]
    if current_selected_panel == UI_INVALID_INDEX then
        Alts.send_message("Preview's panel id is invalid")
        return
    end

    local count = Alts.get_panel_alt_count(current_selected_panel, preview.form_type_)

    if is_forward then
        if count == preview.selected_alt_ then
            preview.selected_alt_ = 0
        else
            preview.selected_alt_ = preview.selected_alt_ + 1
        end
    else
        if preview.selected_alt_ == 0 then
            preview.selected_alt_ = count
        else
            preview.selected_alt_ = preview.selected_alt_ - 1
        end
    end

    local texture_idx = Alts.get_alt_texture_index(current_selected_panel, preview.form_type_, preview.selected_alt_)
    if texture_idx < 0 then
        return
    end
    local parts_name = get_stage_preview_name(current_selected_preview)
    local parts = root_view:get_parts(parts_name)
    local pane_name = "set_rep_stage"
    if preview.form_type_ == STAGE_FORM_TYPE_BATTLE then
        pane_name = "set_rep_stage_battle"
    elseif preview.form_type_ == STAGE_FORM_TYPE_END then
        pane_name = "set_rep_stage_end"
    end

    local texture_pane = parts:get_pane(pane_name)
    texture_pane:replace_texture(texture_idx)
end



-- CLOSURE_80, R139
local regular_main_update = function()
    set_scene_enable(true)
    repeat
        is_page_changing = false
        while try_handle_exiting_scene() == true do
            coroutine.yield()
        end
        
        if scene_state == SCENE_STATE_EXITED then
            exit_normal()
            break
        end
        if stage_select_bgm:update() == false then
            update_from_pointer()
            if handle_button_decide() == false then

                if IS_MODE_STAGE_2_CHANGE == true then
                    is_invalid_stage_2 = false
                    local current_preview_index = current_selected_preview
                    local current_panel = stage_previews[current_preview_index + 1].panel_id_

                    local other_preview_index = current_selected_preview ~ 1
                    local other_panel = stage_previews[other_preview_index + 1].panel_id_

                    if 2 < other_panel then
                        if other_panel == current_selected_panel or other_panel == current_panel then
                            if stage_previews[1].form_type_ == stage_previews[2].form_type_ then
                                is_invalid_stage_2 = true
                            elseif UiScriptPlayer.invoke("is_battle_field_stage_preview", other_preview_index) == true then
                                if stage_previews[other_preview_index + 1].form_type_ ~= STAGE_FORM_TYPE_END and stage_previews[current_preview_index + 1].form_type_ ~= STAGE_FORM_TYPE_END then
                                    is_invalid_stage_2 = true
                                end
                            elseif UiScriptPlayer.invoke("is_end_stage_preview", other_preview_index) == true then
                                if stage_previews[other_preview_index + 1].form_type_ ~= STAGE_FORM_TYPE_BATTLE and stage_previews[current_preview_index + 1].form_type_ ~= STAGE_FORM_TYPE_BATTLE then
                                    is_invalid_stage_2 = true
                                end
                            end
                        else
                            local current_form = stage_previews[current_preview_index + 1].form_type_
                            local other_form = stage_previews[other_preview_index + 1].form_type_

                            if UiScriptPlayer.invoke("is_battle_field_stage_preview", other_preview_index) == true then
                                if UiScriptPlayer.invoke("is_battle_field_ls_stage_preview", current_preview_index) == true then
                                    if other_form == STAGE_FORM_TYPE_NORMAL or other_form == STAGE_FORM_TYPE_BATTLE then
                                        if current_form == STAGE_FORM_TYPE_BATTLE then
                                            is_invalid_stage_2 = true
                                        end
                                    elseif current_form == STAGE_FORM_TYPE_END then
                                        is_invalid_stage_2 = true
                                    end
                                end
                            elseif UiScriptPlayer.invoke("is_battle_field_ls_stage_preview", other_preview_index) == true then
                                if UiScriptPlayer.invoke("is_battle_field_stage_preview", current_preview_index) == true then
                                    if other_form == STAGE_FORM_TYPE_BATTLE then
                                        if current_form == STAGE_FORM_TYPE_NORMAL or current_form == STAGE_FORM_TYPE_BATTLE then
                                            is_invalid_stage_2 = true
                                        end
                                    elseif other_form == STAGE_FORM_TYPE_END and current_form == STAGE_FORM_TYPE_END then
                                        is_invalid_stage_2 = true
                                    end
                                end
                                if UiScriptPlayer.invoke("is_battle_field_ls_stage_preview", current_preview_index) == true then
                                    if other_form == STAGE_FORM_TYPE_BATTLE and current_form == STAGE_FORM_TYPE_BATTLE then
                                        is_invalid_stage_2 = true
                                    elseif other_form == STAGE_FORM_TYPE_END and current_form == STAGE_FORM_TYPE_END then
                                        is_invalid_stage_2 = true
                                    end
                                end
                            end
                        end

                    end
                end

                if check_for_cancel() == true then
                    break
                end
                check_for_strike_cancel()
                if un_decide(false, true) == false then
                    if virtual_input:is_pressed(INPUT_NEXT_SCENE) == true then
                        local is_valid_stage = true
                        if tab_index == TAB_SWITCH_SUB then
                            if IS_ONLINE == true and UiScriptPlayer.invoke("is_enable_made_stage_sub_list") == false then
                                local db = UiScriptPlayer.invoke("get_disable_made_stage_popup_id_sub_list")
                                AppPopupManager.open_database(db)
                                repeat
                                    coroutine.yield()
                                until AppPopupManager.is_busy() == false
                                is_valid_stage = false
                            end
                            if is_valid_stage == true and UiScriptPlayer.invoke("is_all_panel_exceeded_num_player_sub_list") == true then
                                AppPopupManager.open_database("ID_POPUP_STAGE_SELECT_STGCREATE_NO_STAGE")
                                repeat
                                    coroutine.yield()
                                until AppPopupManager.is_busy() == false
                                is_valid_stage = false
                            end
                        end
                        if is_valid_stage == true and prepare_scene_exit() == true then
                            UiSoundManager.play_se_label("se_system_fixed_s")
                            local off_preview_index = -1
                            if check_all_previews_enabled() == false then
                                play_off_preview_animation(current_selected_preview)
                                off_preview_index = current_selected_preview
                            end
                            if tab_index == TAB_SWITCH_SUB then
                                UiScriptPlayer.invoke("set_stage_preview_empty_exit_sub", off_preview_index)
                            else
                                local form = stage_previews[current_selected_preview + 1].form_type_
                                UiScriptPlayer.invoke("set_stage_preview_empty_exit", off_preview_index, form)
                            end
                        end
                    elseif virtual_input:is_decide() == true and get_page_button_dir() == 0 then
                        -- possible
                        handle_panel_decide()
                    elseif virtual_input:is_pressed(INPUT_BGM_SELECT) == true then
                        local index = current_selected_preview
                        if highlighed_preview ~= UI_INVALID_INDEX then
                            index = highlighed_preview
                        end
                        open_bgm_select(index, false)
                    elseif virtual_input:is_pressed(INPUT_ALT_L) then
                        set_alt_panel_textures(false)
                    elseif virtual_input:is_pressed(INPUT_ALT_R) then
                        set_alt_panel_textures(true)
                    else
                        -- possible
                        update_both_tabs()
                    end
                end
            else
                UiScriptPlayer.invoke("play_hand_operated")
            end
            if exit_code_ ~= nil then
                break
            end
        else
            show_scene_and_hand(true, true)
            stage_select_bgm.is_decided = false
        end

        coroutine.yield()
    until false

    virtual_input:set_enable(false)
    if exit_code_ ~= SCENE_EXIT_CODE_CANCEL then
        stop_canceling()
    end
end

-- CLOSURE_81, R140
local my_music_main_update = function()
    set_scene_enable(true)
    repeat
        local x = UiScriptPlayer.invoke("get_hand_position_x")
        local y = UiScriptPlayer.invoke("get_hand_position_y")

        x, y = layout_root:layout_to_screen_position(x, y)

        local selected, decided = back_button_selector:update_pointer_input(x, y, virtual_input, true)
        back_button.selected_button_id_ = selected
        back_button.decided_button_id_ = decided

        if stage_select_bgm:update() == false then
            if virtual_input:is_cancel() == true then
                back_button_selector:decide_button(BUTTON_BACK)
            elseif virtual_input:is_decide() == true and get_page_button_dir() == 0 then
                if 0 <= current_selected_panel and UiScriptPlayer.invoke("is_empty_stage_preview", current_selected_preview) == false then
                    stage_select_bgm:activate(0, true, false)
                    show_scene_and_hand(false, false)
                end
            else
                -- Handles the page changing
                local move_dir = get_page_button_dir()
                
                if virtual_input:is_pressed(INPUT_TAB_CHANGE_L) == true then
                    move_dir = -1
                elseif virtual_input:is_pressed(INPUT_TAB_CHANGE_R) == true then
                    move_dir = 1
                end
            
                handle_change_page(move_dir)
                normal_tab_main_update()
                update_panel_scalings()
                local should_be_operatable = false
                if current_selected_panel ~= UI_INVALID_INDEX then
                    if UiScriptPlayer.invoke("is_lock_stage_panel", current_selected_panel) == false then
                        should_be_operatable = true
                    end
                elseif 0 <= back_button.selected_button_id_ then
                    should_be_operatable = true
                end
                UiScriptPlayer.invoke("set_hand_operatable", should_be_operatable)
            end
        else
            show_scene_and_hand(true, false)
        end

        if back_button_selector:get_decided_button_id() == BUTTON_BACK then
            cancel()
            break
        end

        coroutine.yield()

    until false

    virtual_input:set_enable(false)
end

main = function()
    setup()
    stage_select_bgm:setup()
    xpcall(setup_from_environment, print_error_handler)
    root_view:play_animation("in", 1.0)
    if IS_SIMPLE_CANCEL == true then
        local parts = root_view:get_parts("set_parts_txt_head_00")
        if IS_RETURN_MENU == false then
            parts:play_animation("in", 1.0)
        else
            parts:stop_animation_at_end("in")
        end
    end
    repeat
        coroutine.yield()
    until root_view:is_animation_finished("in")
    virtual_input:set_enable(true)
    if IS_MY_MUSIC == true then
        xpcall(my_music_main_update, print_error_handler)
    else
        xpcall(regular_main_update, print_error_handler)
    end

    local first = {
        alt_ = -1,
        panel_ = -1,
        form_ = -1
    }

    local second = {
        alt_ = -1,
        panel_ = -1,
        form_ = -1
    }

    local third = {
        alt_ = -1,
        panel_ = -1,
        form_ = -1
    }

    if USE_STAGE_NUM > 0 then
        first.alt_ = stage_previews[1].selected_alt_
        first.panel_ = stage_previews[1].panel_id_
        first.form_ = stage_previews[1].form_type_
    end

    if USE_STAGE_NUM > 1 then
        second.alt_ = stage_previews[2].selected_alt_
        second.panel_ = stage_previews[2].panel_id_
        second.form_ = stage_previews[2].form_type_
    end

    if USE_STAGE_NUM > 2 then
        third.alt_ = stage_previews[3].selected_alt_
        third.panel_ = stage_previews[3].panel_id_
        third.form_ = stage_previews[3].form_type_
    end

    Alts.set_alts(
        first.alt_,
        first.panel_,
        first.form_,
        second.alt_,
        second.panel_,
        second.form_,
        third.alt_,
        third.panel_,
        third.form_
    )

    stop_long_cancel_se()
    UiScriptPlayer.invoke("finalize_bgm")
end

get_tab_switch = function()
    return tab_index
end
 
