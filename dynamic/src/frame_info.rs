use smash::{
  app::{
      self,
      sv_animcmd::{
          frame,
          wait
      },
      lua_bind::*
  },
  lib::lua_const::*,
  lib::L2CAgent,
  lua2cpp::*,
  phx::*
};
use smash_script::{
  *,
  macros::*
};
use crate::{
  *,
  consts::*,
  ext::*
};


#[repr(C)]
pub struct FrameInfo {
  pub lua_state: u64,
  pub agent: *mut L2CAgent,
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
  pub unsafe fn update_and_get(fighter: &mut L2CFighterCommon) -> Option<Self> {
      let lua_state = fighter.lua_state_agent;
      let boma = fighter.boma();
      let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
      if !(0..8).contains(&id) {
          return None;
      }
      let cat1 = ControlModule::get_command_flag_cat(boma, 0);
      let cat2 = ControlModule::get_command_flag_cat(boma, 1);
      let cat3 = ControlModule::get_command_flag_cat(boma, 2);
      let cat4 = ControlModule::get_command_flag_cat(boma, 3);
      let cur_frame = MotionModule::frame(boma);
      VarModule::set_int(fighter.battle_object, vars::common::instance::COSTUME_SLOT_NUMBER,WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR));
      Some(Self {
          lua_state: lua_state,
          agent: fighter as *mut L2CFighterCommon as *mut L2CAgent,
          boma: boma as *mut smash::app::BattleObjectModuleAccessor,
          fighter_kind: boma.kind(),
          status_kind: StatusModule::status_kind(boma),
          situation_kind: StatusModule::situation_kind(boma),
          motion_kind: Hash40::new_raw(MotionModule::motion_kind(boma)),
          cur_frame: cur_frame,
          frame: cur_frame + 1.0,
          cat: [cat1, cat2, cat3, cat4],
          facing: PostureModule::lr(boma),
          stick_x: ControlModule::get_stick_x(boma),
          stick_y: ControlModule::get_stick_y(boma),
          id: id
      })
  }
}
