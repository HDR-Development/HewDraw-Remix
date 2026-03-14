
use crate::*;

use super::class::*;

extern "C" {
    #[link_name = "_ZN7lua2cpp23L2CFighterAIAnalystBase35main_func__analyst_global_variablesEv"]
    pub(super) fn main_func__analyst_global_variables(this: *mut L2CFighterAIAnalystBase);

    #[link_name = "_ZN7lua2cpp23L2CFighterAIAnalystBase36main_func__analyst_private_variablesEv"]
    pub(super) fn main_func__analyst_private_variables(this: *mut L2CFighterAIAnalystBase);

    #[link_name = "_ZN7lua2cpp23L2CFighterAIAnalystBase13change_statusEN3lib8L2CValueE"]
    pub(super) fn change_status(this: *mut L2CFighterAIAnalystBase, arg1: lib::L2CValueHack);

    #[link_name = "_ZN7lua2cpp23L2CFighterAIAnalystBase13update_statusEv"]
    pub(super) fn update_status(this: *mut L2CFighterAIAnalystBase);

    #[link_name = "_ZN7lua2cpp23L2CFighterAIAnalystBase20update_chanced_frameEv"]
    pub(super) fn update_chanced_frame(this: *mut L2CFighterAIAnalystBase);

    #[link_name = "_ZN7lua2cpp23L2CFighterAIAnalystBase5ENTRYEv"]
    pub(super) fn ENTRY(this: *mut L2CFighterAIAnalystBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp23L2CFighterAIAnalystBase10init_linesEv"]
    pub(super) fn init_lines(this: *mut L2CFighterAIAnalystBase);

    #[link_name = "_ZN7lua2cpp23L2CFighterAIAnalystBase11init_eventsEv"]
    pub(super) fn init_events(this: *mut L2CFighterAIAnalystBase);

    #[link_name = "_ZN7lua2cpp23L2CFighterAIAnalystBase21init_global_variablesEv"]
    pub(super) fn init_global_variables(this: *mut L2CFighterAIAnalystBase);

    #[link_name = "_ZN7lua2cpp23L2CFighterAIAnalystBase22init_private_variablesEv"]
    pub(super) fn init_private_variables(this: *mut L2CFighterAIAnalystBase);

    #[link_name = "_ZN7lua2cpp23L2CFighterAIAnalystBase20call_function_updateEv"]
    pub(super) fn call_function_update(this: *mut L2CFighterAIAnalystBase) -> lib::L2CValueHack;

    #[link_name = "_ZN7lua2cpp23L2CFighterAIAnalystBase18call_event_on_deadEv"]
    pub(super) fn call_event_on_dead(this: *mut L2CFighterAIAnalystBase) -> lib::L2CValueHack;
}