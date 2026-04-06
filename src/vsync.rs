// pub use ssbusync as SsbuSync;
// use SsbuSync::*;

// pub fn setup_ssbu_sync() {
//     println!("[HDR] installing custom ssbusync path via Main \n");
//     let mut sync_config = SsbuSyncConfig::default();
//     sync_config.disable_vsync = true;
//     sync_config.disable_pacer = false;
//     sync_config.slow_pacer_bias = false;
//     sync_config.enable_triple_buffer = false;
//     sync_config.allow_buffer_swap = false;
//     sync_config.smooth_ffa = false;
//     sync_config.online_only = false;

//     // create a profile with ssbusync config
//     // but we don't actually use the options that are stored in that profile if they are changed by the user
//     SsbuSync::Get_Init_SsbuSync_Profile("HDR", &sync_config, 1.0);
//     ssbusync::Install_SSBU_Sync(sync_config);

//     // if ssbusync::render::buffer_swap::subscribe_buffer_mode_change(on_buffer_switch) {
//     //     println!("[HDR] Subscribed to buffer switch \n");
//     // } else {
//     //     println!("[HDR] Failed to subscribe to buffer switch \n");
//     // }
// }

// Work-around for setting input delay during doubles/FFAs
// pub fn set_doubles_delay(playercount: i32) -> bool {
//     if (playercount > 2) {
//         ssbusync::Enable_Triple_Buffer();
//         return true;
//     } else {
//         ssbusync::Enable_Double_Buffer();
//         return false;
//     }
// }

// fn on_buffer_switch(mode: ssbusync::render::buffer_swap::BufferMode) {
//     println!("Buffer Successfully Switched: {:?} \n", mode)
// }

// if ssbusync::SyncEnv::ALLOW_BUFFER_SWAP() {
//     let player_count = count_active_players(instance);
//     crate::set_doubles_delay(player_count);
//     ssbusync::Check_Buffer_Swap();
// }

// if  ssbusync::SyncEnv::online_only() {
//     EmuNetplay::check_online_fix_emu();
// }