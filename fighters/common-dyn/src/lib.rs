/// This crate is intended to be used during development to help speed up compile times
/// The `common` crate itself gets built and linked into the `fighters` crate, however 
/// all of the individual characters will depend on this crate, as changing the `common` crate can cause
/// lengthy compile times if we have to rebuild the entire plugin numerous times to test some changes

pub mod ext;
pub use ext::*;

// pub mod djc;

pub mod opff {
  pub struct FrameInfo {
    pub lua_state: u64,
    pub agent: *mut smash::lib::L2CAgent,
    pub boma: *mut smash::app::BattleObjectModuleAccessor,
    pub fighter_kind: i32,
    pub status_kind: i32,
    pub situation_kind: i32,
    pub motion_kind: smash::phx::Hash40,
    pub cur_frame: f32,
    pub frame: f32,
    pub cat: [i32; 4],
    pub facing: f32,
    pub stick_x: f32,
    pub stick_y: f32,
    pub id: usize
  }

  impl FrameInfo {
    pub fn update_and_get(fighter: &mut L2CFighterCommon) -> Option<Self> {
      unsafe {
        update_and_get_impl(fighter)
      }
    }
  }
  use smash::lua2cpp::L2CFighterCommon;
  extern "Rust" {
    #[link_name = "fighter_common_opff"]
    pub fn fighter_common_opff(fighter: &mut L2CFighterCommon);
    #[link_name = "update_and_get"]
    pub fn update_and_get_impl(fighter: &mut L2CFighterCommon) -> Option<FrameInfo>;
  }
}
