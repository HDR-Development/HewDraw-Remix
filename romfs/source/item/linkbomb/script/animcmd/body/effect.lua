effect_Appear = function ()
    sv_animcmd.frame(1)
    if sv_animcmd.is_excute() then
        sv_animcmd.EFFECT_FOLLOW(69147576730, 13402447818, 0, 0.5, 0, 0, 0, 0, 1, true)
    end
    sv_animcmd.frame(3)
    if sv_animcmd.is_excute() then
        sv_animcmd.EFFECT_FOLLOW(62748914918, 13402447818, 0, 0.5, 0, 0, 0, 0, 0.8, false)
        EffectModule.enable_sync_init_pos_last()
    end
    return 
end

effect_Born = function ()
    if sv_animcmd.is_excute() then
        sv_animcmd.EFFECT(39481568190, 13402447818, 0, 0, 0, 0, 0, 0, 0.74, 0, 0, 0, 0, 0, 0, false)
        sv_animcmd.LAST_EFFECT_SET_COLOR(1.0, 0.0, 0.0)
        sv_animcmd.LANDING_EFFECT(17813994575, 13402447818, 10, 18446744073709551614, 0, 0, 0, 0, 0.699999988079071, 0, 0, 0, 0, 0, 0, false)
        sv_animcmd.LAST_EFFECT_SET_COLOR(1.0, 0.0, 0.0)
        sv_animcmd.LANDING_EFFECT(17813994575, 13402447818, 0, 18446744073709551614, 5, 0, 0, 0, 0.800000011920929, 0, 0, 0, 0, 0, 0, false)
        sv_animcmd.LAST_EFFECT_SET_COLOR(1.0, 0.0, 0.0)
        sv_animcmd.LANDING_EFFECT(17813994575, 13402447818, 18446744073709551606, 18446744073709551614, 0, 0, 0, 0, 0.699999988079071, 0, 0, 0, 0, 0, 0, false)
        sv_animcmd.LAST_EFFECT_SET_COLOR(1.0, 0.0, 0.0)
    end
    return 
end

effect_HitstopBeforeBorn = function ()
    while true do
        if sv_animcmd.is_excute() then
            sv_animcmd.FLASH_NO_STOP(1, 1, 1, 0.699999988079071)
        end
        sv_animcmd.wait(1)
        if sv_animcmd.is_excute() then
            sv_animcmd.FLASH_FRM(2, 0, 0, 0, 0)
        end
        sv_animcmd.wait(1)
        if sv_animcmd.is_excute() then
            sv_animcmd.COL_NORMAL()
        end
        sv_animcmd.wait(1)
    end
end

effect_Flash = function ()
    while true do
        if sv_animcmd.is_excute() then
            sv_animcmd.FLASH(1, 1, 1, 0.699999988079071)
        end
        sv_animcmd.wait(1)
        if sv_animcmd.is_excute() then
            sv_animcmd.FLASH_FRM(2, 0, 0.5, 1, 0)
        end
        sv_animcmd.wait(2)
        if sv_animcmd.is_excute() then
            sv_animcmd.COL_NORMAL()
        end
        sv_animcmd.wait(2)
    end
end

effect_Bound = function ()
    if sv_animcmd.is_excute() then
        sv_animcmd.FOOT_EFFECT(66964071076, 13402447818, 0, 0, 0, 0, 0, 0, 0.6000000238418579, 0, 0, 0, 0, 0, 0, false)
        sv_animcmd.LAST_EFFECT_SET_ALPHA(0.699999988079071)
        sv_animcmd.EFFECT(65162439316, 13402447818, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
    end
    return 
end

return 