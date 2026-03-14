use crate::*;

#[repr(C)]
pub struct AttackAbsoluteData {
    pub power: f32,
    pub vector: i32,
    pub r_eff: i32,
    pub r_fix: i32,
    pub r_add: i32,
    pub slip: f32,
    pub stop_frame: f32,
    pub stop_delay: f32,
    pub attr: phx::Hash40,
    pub sound_level: app::CollisionSoundLevel,
    pub sound_attr: app::CollisionSoundAttr,
    pub lr_check: app::AttackLRCheck,
    pub no_stop: bool,
    pub no_effect: bool,
    unused: u8,
    pub region: app::AttackRegion,
    pub catch: bool,
}

#[cfg(feature = "type_assert")]
impl AttackAbsoluteData {
    pub fn assert() {
        assert_eq!(size_of!(AttackAbsoluteData), 0x30);
        assert_eq!(offset_of!(AttackAbsoluteData, power), 0x0);
        assert_eq!(offset_of!(AttackAbsoluteData, vector), 0x4);
        assert_eq!(offset_of!(AttackAbsoluteData, r_eff), 0x8);
        assert_eq!(offset_of!(AttackAbsoluteData, r_fix), 0xC);
        assert_eq!(offset_of!(AttackAbsoluteData, r_add), 0x10);
        assert_eq!(offset_of!(AttackAbsoluteData, slip), 0x14);
        assert_eq!(offset_of!(AttackAbsoluteData, stop_frame), 0x18);
        assert_eq!(offset_of!(AttackAbsoluteData, stop_delay), 0x1C);
        assert_eq!(offset_of!(AttackAbsoluteData, attr), 0x20);
        assert_eq!(offset_of!(AttackAbsoluteData, sound_level), 0x28);
        assert_eq!(offset_of!(AttackAbsoluteData, sound_attr), 0x29);
        assert_eq!(offset_of!(AttackAbsoluteData, lr_check), 0x2A);
        assert_eq!(offset_of!(AttackAbsoluteData, no_stop), 0x2B);
        assert_eq!(offset_of!(AttackAbsoluteData, no_effect), 0x2C);
        assert_eq!(offset_of!(AttackAbsoluteData, region), 0x2E);
        assert_eq!(offset_of!(AttackAbsoluteData, catch), 0x2F);
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct AttackData {
    pub offset: phx::Vector3f,
    padding1: f32,
    pub offset2: phx::Vector3f,
    padding2: f32,
    pub power: f32,
    pub size: f32,
    pub vector: i32,
    pub r_eff: i32,
    pub r_fix: i32,
    pub r_add: i32,
    pub slip: f32,
    pub stop_frame: f32,
    pub stop_delay: f32,
    pub node: phx::Hash40,
    pub target_category: app::CollisionCategoryMask,
    pub target_situation: app::CollisionSituationMask,
    pub target_lr: bool,
    pub target_part: app::CollisionPartMask,
    pub attr: phx::Hash40,
    pub sound_level: app::CollisionSoundLevel,
    pub sound_attr: app::CollisionSoundAttr,
    pub set_off: app::AttackSetOffKind,
    pub no_scale: bool,
    pub shield: bool,
    pub reflector: bool,
    pub absorber: bool,
    pub direct: bool,
    pub no_invincible: bool,
    pub no_xlu: bool,
    pub lr_check: app::AttackLRCheck,
    pub catch: bool,
    pub no_team: bool,
    pub no_stop: bool,
    pub no_effect: bool,
    unused1: u8,
    pub speed: bool,
    pub region: app::AttackRegion,
    pub ignore_down: bool,
    pub check_type: app::CollisionShapeType,
    pub sub_shield: u16,
    pub camera_quake: app::CameraQuakeKind,
    pub serial_hit_frame: u32,
    pub force_reaction: bool,
    pub no_attacker_log: bool,
    pub no_weight_reaction: u8,
    pub no_reaction_search: u8,
    pub keep_rumble: bool,
    pub composition_speed: bool,
    pub target_pos_node: phx::Hash40,
    pub target_pos_offset: phx::Vector2f,
    padding3: u64,
    pub target_pos_frame: i32,
    pub r_fix_damage_speed_up: bool,
    unused2: u8,
    pub captured_same_time_attack: bool,
    unknown: [u8; 0x59], // 0xA7
}

#[cfg(feature = "type_assert")]
impl AttackData {
    pub fn assert() {
        assert_eq!(size_of!(AttackData), 0x100);
        assert_eq!(offset_of!(AttackData, offset), 0x0);
        assert_eq!(offset_of!(AttackData, offset2), 0x10);
        assert_eq!(offset_of!(AttackData, power), 0x20);
        assert_eq!(offset_of!(AttackData, size), 0x24);
        assert_eq!(offset_of!(AttackData, vector), 0x28);
        assert_eq!(offset_of!(AttackData, r_eff), 0x2C);
        assert_eq!(offset_of!(AttackData, r_fix), 0x30);
        assert_eq!(offset_of!(AttackData, r_add), 0x34);
        assert_eq!(offset_of!(AttackData, slip), 0x38);
        assert_eq!(offset_of!(AttackData, stop_frame), 0x3C);
        assert_eq!(offset_of!(AttackData, stop_delay), 0x40);
        assert_eq!(offset_of!(AttackData, node), 0x48);
        assert_eq!(offset_of!(AttackData, target_category), 0x50);
        assert_eq!(offset_of!(AttackData, target_situation), 0x52);
        assert_eq!(offset_of!(AttackData, target_lr), 0x53);
        assert_eq!(offset_of!(AttackData, target_part), 0x54);
        assert_eq!(offset_of!(AttackData, attr), 0x58);
        assert_eq!(offset_of!(AttackData, sound_level), 0x60);
        assert_eq!(offset_of!(AttackData, sound_attr), 0x61);
        assert_eq!(offset_of!(AttackData, set_off), 0x62);
        assert_eq!(offset_of!(AttackData, no_scale), 0x63);
        assert_eq!(offset_of!(AttackData, shield), 0x64);
        assert_eq!(offset_of!(AttackData, reflector), 0x65);
        assert_eq!(offset_of!(AttackData, absorber), 0x66);
        assert_eq!(offset_of!(AttackData, direct), 0x67);
        assert_eq!(offset_of!(AttackData, no_invincible), 0x68);
        assert_eq!(offset_of!(AttackData, no_xlu), 0x69);
        assert_eq!(offset_of!(AttackData, lr_check), 0x6A);
        assert_eq!(offset_of!(AttackData, catch), 0x6B);
        assert_eq!(offset_of!(AttackData, no_team), 0x6C);
        assert_eq!(offset_of!(AttackData, no_stop), 0x6D);
        assert_eq!(offset_of!(AttackData, no_effect), 0x6E);
        assert_eq!(offset_of!(AttackData, speed), 0x70);
        assert_eq!(offset_of!(AttackData, region), 0x71);
        assert_eq!(offset_of!(AttackData, ignore_down), 0x72);
        assert_eq!(offset_of!(AttackData, check_type), 0x73);
        assert_eq!(offset_of!(AttackData, sub_shield), 0x74);
        assert_eq!(offset_of!(AttackData, camera_quake), 0x76);
        assert_eq!(offset_of!(AttackData, serial_hit_frame), 0x78);
        assert_eq!(offset_of!(AttackData, force_reaction), 0x7C);
        assert_eq!(offset_of!(AttackData, no_attacker_log), 0x7D);
        assert_eq!(offset_of!(AttackData, no_weight_reaction), 0x7E);
        assert_eq!(offset_of!(AttackData, no_reaction_search), 0x7F);
        assert_eq!(offset_of!(AttackData, keep_rumble), 0x80);
        assert_eq!(offset_of!(AttackData, composition_speed), 0x81);
        assert_eq!(offset_of!(AttackData, target_pos_node), 0x88);
        assert_eq!(offset_of!(AttackData, target_pos_offset), 0x90);
        assert_eq!(offset_of!(AttackData, target_pos_frame), 0xA0);
        assert_eq!(offset_of!(AttackData, r_fix_damage_speed_up), 0xA4);
        assert_eq!(offset_of!(AttackData, captured_same_time_attack), 0xA6);
    }
}
