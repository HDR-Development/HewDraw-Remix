use crate::{
    phx::{Vec2, Vec3},
    *,
};

#[repr(C)]
pub struct FighterPostureInfo {
    vtable: *const *const (),
    padding: u64,
    pub current: Vec3,
    _current_padding: f32,
    pub prev: Vec3,
    _prev_padding: f32,
    pub slow: Vec3,
    _slow_padding: f32,
    pub slow_translation: Vec3,
    _slow_trans_padding: f32,
    pub update_model: bool,
    _update_model_padding: [u8; 15],
}

#[repr(C)]
#[vtable_impl(PostureModule)]
pub struct PostureModuleVTable {
    destructor: extern "C" fn(this: &mut PostureModule),
    deleter: extern "C" fn(this: &mut PostureModule),
    pub is_virtual: extern "C" fn(this: &PostureModule) -> bool,
    handle_int_msc_command:
        extern "C" fn(this: &mut PostureModule, command: &lib::MscCommand) -> lib::TValue,
    handle_float_msc_command:
        extern "C" fn(this: &mut PostureModule, command: &lib::MscCommand) -> lib::TValue,
    initialize: extern "C" fn(this: &mut PostureModule, args: *const ()),
    finalize: extern "C" fn(this: &mut PostureModule),
    pub initialize_settings:
        extern "C" fn(this: &mut PostureModule, pos: &Vec2, is_dead: bool, lr: f32),
    start_module: extern "C" fn(this: &mut PostureModule, pos: &Vec3, lr: f32),
    end_module: extern "C" fn(this: &mut PostureModule),
    pub reset_position: extern "C" fn(
        this: &mut PostureModule,
        prev_pos: &Vec2,
        pos: &Vec2,
        is_from_death: bool,
        is_link: bool,
    ),
    pub init_pos:
        extern "C" fn(this: &mut PostureModule, new_pos: &Vec3, is_dead: bool, is_link: bool),
    pub pos: extern "C" fn(this: &PostureModule) -> &Vec3,
    pub pos_2d: extern "C" fn(this: &PostureModule) -> cpp::simd::Vector2,
    pub set_pos: extern "C" fn(this: &mut PostureModule, pos: &Vec3),
    pub set_pos_2d: extern "C" fn(this: &mut PostureModule, pos: cpp::simd::Vector2),
    pub add_pos: extern "C" fn(this: &mut PostureModule, add: &Vec3),
    pub add_pos_2d: extern "C" fn(this: &mut PostureModule, add: cpp::simd::Vector2),
    pub prev_pos: extern "C" fn(this: &PostureModule) -> &Vec3,
    pub prev_pos_2d: extern "C" fn(this: &PostureModule) -> cpp::simd::Vector2,
    pub cursor_pos: extern "C" fn(this: &PostureModule) -> &Vec3,
    pub set_cursor_pos: extern "C" fn(this: &mut PostureModule, pos: &Vec3),
    pub lr: extern "C" fn(this: &PostureModule) -> f32,
    pub set_lr: extern "C" fn(this: &mut PostureModule, lr: f32),
    pub reverse_lr: extern "C" fn(this: &mut PostureModule),
    pub init_rot: extern "C" fn(this: &mut PostureModule),
    pub init_rot_with_index: extern "C" fn(this: &mut PostureModule, index: i32),
    pub rot: extern "C" fn(this: &PostureModule) -> &Vec3,
    pub set_rot: extern "C" fn(this: &mut PostureModule, rot: &Vec3, index: i32),
    pub init_rot_y_lr: extern "C" fn(this: &mut PostureModule),
    pub is_rot_y_lr_different_inner_lr: extern "C" fn(this: &PostureModule) -> bool,
    pub rot_y_lr: extern "C" fn(this: &PostureModule) -> f32,
    pub update_rot_y_lr: extern "C" fn(this: &mut PostureModule),
    pub reverse_rot_y_lr: extern "C" fn(this: &mut PostureModule),
    pub base_scale: extern "C" fn(this: &PostureModule) -> f32,
    pub scale: extern "C" fn(this: &PostureModule) -> f32,
    pub set_scale: extern "C" fn(this: &mut PostureModule, scale: f32),
    pub scale_status: extern "C" fn(this: &PostureModule) -> f32,
    pub set_scale_status: extern "C" fn(this: &mut PostureModule, scale: f32),
    pub init_scale: extern "C" fn(this: &mut PostureModule),
    pub owner_scale: extern "C" fn(this: &PostureModule) -> f32,
    pub set_owner_scale: extern "C" fn(this: &mut PostureModule, scale: f32),
    pub set_link_scale: extern "C" fn(this: &mut PostureModule, scale: f32, send_event: bool),
    pub update_vectors: extern "C" fn(this: &mut PostureModule),
    pub update_slow_speed: extern "C" fn(this: &mut PostureModule),
    pub finish_slow: extern "C" fn(this: &mut PostureModule),
    pub update_slow_pos: extern "C" fn(this: &mut PostureModule),
    pub set_stick_lr: extern "C" fn(this: &mut PostureModule, lr: f32),
    pub set_sync_constraint_joint: extern "C" fn(this: &mut PostureModule, joint: phx::Hash40),
    set_pos_from_joint: extern "C" fn(this: &mut PostureModule, joint: phx::Hash40, arg: *const ()), // Arg needs more research
    pub update_rotation_vectors: extern "C" fn(this: &mut PostureModule),
    pub on_change_status: extern "C" fn(this: &mut PostureModule, event: &lib::BasicEvent),
    pub set_posture_flag: extern "C" fn(this: &mut PostureModule, flag: i32),
    pub off_posture_flag: extern "C" fn(this: &mut PostureModule, flag: i32),
    pub clear_posture_flags: extern "C" fn(this: &mut PostureModule),
    pub is_posture_flag: extern "C" fn(this: &PostureModule, flag: i32) -> bool,
    pub has_link_owner: extern "C" fn(this: &PostureModule) -> bool,
    pub add_link_owner: extern "C" fn(this: &mut PostureModule),
    pub sub_link_owner: extern "C" fn(this: &mut PostureModule),
    pub set_is_linked: extern "C" fn(this: &mut PostureModule, is_linked: bool),
    pub set_pos_no_update: extern "C" fn(this: &mut PostureModule, pos: &Vec3),
}

#[repr(C)]
pub struct PostureModule {
    vtable: &'static PostureModuleVTable,
    owner: *mut app::BattleObjectModuleAccessor,
}

#[repr(C)]
#[virtual_implementor(PostureModule)]
pub struct FighterPostureModuleImpl {
    parent: PostureModule,
    _inner_vtable: *const *const (),
    pub posture_flags: i32,
    pub owners: i32,
    pub posture_info: FighterPostureInfo,
    pub cursor_pos: Vec3,
    _cursor_pos_padding: f32,
    pub rots: cpp::Array2<FighterPostureInfo>,
    pub base_scale: f32,
    pub scale_status: f32,
    pub user_scale: f32,
    pub lr: f32,
    pub rot_y_lr: f32,
    pub owner_scale: f32,
    pub link_scale: f32,
    pub initial_link_scale: f32,
    pub sync_constraint_joint: phx::Hash40,
    pub is_linked: bool,
    _padding: [u8; 7],
}
