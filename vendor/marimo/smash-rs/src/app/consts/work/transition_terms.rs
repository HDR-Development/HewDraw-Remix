use crate::*;

macro_rules! make_work_ids {
    ($ty:ident, $kind:ident, $(($name:ident, $index:literal))*) => {
        $(
            pub const $name: app::WorkId = app::WorkId::from_parts(app::WorkType::$ty, app::WorkKind::$kind, $index);
        )*
    }
}

make_work_ids!(
    Int,
    Transition,
    (WAIT,                          0x00)
    (WAIT_FROM_RUN_BRAKE,           0x01)
    (FALL,                          0x02)
    (FALL_AERIAL,                   0x03)
    (FALL_SPECIAL,                  0x04)
    (DAMAGE_FALL,                   0x05)
    (DAMAGE_FLY_REFLECT_L,          0x06)
    (DAMAGE_FLY_REFLECT_R,          0x07)
    (DAMAGE_FLY_REFLECT_U,          0x08)
    (DAMAGE_FLY_REFLECT_D,          0x09)
    (DASH_TO_RUN,                   0x0A)
    (SLIP,                          0x0B)
    (RUN,                           0x0C)
    (DOWN,                          0x0D)
    (DOWN_WAIT,                     0x0E)
    (DOWN_STAND,                    0x0F)
    (DOWN_DAMAGE_FALL,              0x10)
    (DOWN_DAMAGE_REFLECT_L,         0x11)
    (DOWN_DAMAGE_REFLECT_R,         0x12)
    (PASSIVE,                       0x13)
    (PASSIVE_FB,                    0x14)
    (PASSIVE_WALL,                  0x15)
    (PASSIVE_WALL_JUMP,             0x16)
    (PASSIVE_WALL_JUMP_BUTTON,      0x17)
    (PASSIVE_CEIL,                  0x18)
    (STOP_WALL,                     0x19)
    (STOP_CEIL,                     0x1A)
    (LANDING,                       0x1B)
    (LANDING_LIGHT,                 0x1C)
    (LANDING_ATTACK_AIR,            0x1D)
    (LANDING_FALL_SPECIAL,          0x1E)
    (CLIFF_CATCH,                   0x1F)
    (GLIDE,                         0x20)
    (GLIDE_LANDING,                 0x21)
    (SQUAT_WAIT,                    0x22)
    (CATCH_WAIT,                    0x23)
    (CAPTURE_WAIT,                  0x24)
    (CAPTURE_CUT,                   0x25)
    (CONT_ESCAPE,                   0x26)
    (CONT_ESCAPE_F,                 0x27)
    (CONT_ESCAPE_B,                 0x28)
    (CONT_GUARD_ON,                 0x29)
    (CONT_GUARD,                    0x2A)
    (CONT_JUMP_SQUAT,               0x2B)
    (CONT_JUMP_SQUAT_BUTTON,        0x2C)
    (CONT_DASH,                     0x2D)
    (CONT_RUN_BRAKE,                0x2E)
    (CONT_TURN,                     0x2F)
    (CONT_TURN_RUN,                 0x30)
    (CONT_TURN_DASH,                0x31)
    (CONT_TURN_DASH_DASH,           0x32)
    (CONT_SLIP_TURN,                0x33)
    (CONT_SQUAT,                    0x34)
    (CONT_SQUAT_RV,                 0x35)
    (CONT_SQUAT_WAIT,               0x36)
    (CONT_SQUAT_F,                  0x37)
    (CONT_SQUAT_B,                  0x38)
    (CONT_APPEAL_U,                 0x39)
    (CONT_APPEAL_S,                 0x3A)
    (CONT_APPEAL_LW,                0x3B)
    (CONT_WALK,                     0x3C)
    (CONT_ESCAPE_AIR,               0x3D)
    (CONT_JUMP_AERIAL,              0x3E)
    (CONT_JUMP_AERIAL_BUTTON,       0x3F)
    (CONT_TREAD_JUMP,               0x40)
    (CONT_TREAD_JUMP_BUTTON,        0x41)
    (CONT_TREAD_JUMP_NO_TRIGGER,    0x42)
    (CONT_FLY,                      0x43)
    (CONT_FLY_BUTTON,               0x44)
    (CONT_FLY_NEXT,                 0x45)
    (CONT_CLIFF_ATTACK,             0x46)
    (CONT_CLIFF_SPEICAL,            0x47)
    (CONT_CLIFF_ESCAPE,             0x48)
    (CONT_CLIFF_CLIMB,              0x49)
    (CONT_CLIFF_FALL,               0x4A)
    (CONT_CLIFF_JUMP,               0x4B)
    (CONT_CLIFF_JUMP_BUTTON,        0x4C)
    (CONT_GLIDE_START,              0x4D)
    (CONT_GLIDE_END,                0x4E)
    (CONT_GLIDE_ATTACK,             0x4F)
    (CONT_PASS,                     0x50)
    (CONT_AIR_LASSO,                0x51)
    (CONT_ATTACK,                   0x52)
    (CONT_ATTACK_100,               0x53)
    (CONT_ATTACK_DASH,              0x54)
    (CONT_ATTACK_S3,                0x55)
    (CONT_ATTACK_HI3,               0x56)
    (CONT_ATTACK_LW3,               0x57)
    (CONT_ATTACK_S4,                0x58)
    (CONT_ATTACK_S4_START,          0x59)
    (CONT_ATTACK_S4_HOLD,           0x5A)
    (CONT_ATTACK_HI4,               0x5B)
    (CONT_ATTACK_HI4_START,         0x5C)
    (CONT_ATTACK_HI4_HOLD,          0x5D)
    (CONT_ATTACK_LW4,               0x5E)
    (CONT_ATTACK_LW4_START,         0x5F)
    (CONT_ATTACK_LW4_HOLD,          0x60)
    (CONT_ATTACK_COMMAND1,          0x61)
    (CONT_ATTACK_AIR,               0x62)
    (CONT_SPECIAL_N,                0x63)
    (CONT_SPECIAL_S,                0x64)
    (CONT_SPECIAL_HI,               0x65)
    (CONT_SPECIAL_LW,               0x66)
    (CONT_SPECIAL_N_COMMAND,        0x67)
    (CONT_SPECIAL_N2_COMMAND,       0x68)
    (CONT_SPECIAL_S_COMMAND,        0x69)
    (CONT_SPECIAL_HI_COMMAND,       0x6A)
    (CONT_THROW_F,                  0x6B)
    (CONT_THROW_B,                  0x6C)
    (CONT_THROW_HI,                 0x6D)
    (CONT_THROW_LW,                 0x6E)
    (CONT_CATCH,                    0x6F)
    (CONT_CATCH_DASH,               0x70)
    (CONT_CATCH_TURN,               0x71)
    (CONT_CAPTURE_JUMP_BUTTON,      0x72)
    (CONT_CAPTURE_JUMP,             0x73)
    (CONT_ITEM_PICKUP_LIGHT,        0x74)
    (CONT_ITEM_PICKUP_HEAVY,        0x75)
    (CONT_ITEM_THROW,               0x76)
    (CONT_ITEM_THROW_GUARD,         0x77)
    (CONT_ITEM_THROW_FORCE,         0x78)
    (CONT_ITEM_THROW_FORCE_DASH,    0x79)
    (CONT_ITEM_THROW_DASH,          0x7A)
    (CONT_ITEM_THROW_DASH_CLACKER,  0x7B)
    (CONT_ITEM_SWING_BAT_4,         0x7C)
    (CONT_ITEM_SWING_4,             0x7D)
    (CONT_ITEM_SWING_4_HOLD,        0x7E)
    (CONT_ITEM_SWING_3,             0x7F)
    (CONT_ITEM_SWING,               0x80)
    (CONT_ITEM_SWING_DASH,          0x81)
    (CONT_ITEM_SHOOT,               0x82)
    (CONT_ITEM_SHOOT_S3,            0x83)
    (CONT_ITEM_SHOOT_S4,            0x84)
    (CONT_ITEM_SHOOT_WAIT,          0x85)
    (CONT_ITEM_SHOOT_WALK_F,        0x86)
    (CONT_ITEM_SHOOT_WALK_F_BRAKE,  0x87)
    (CONT_ITEM_SHOOT_WALK_B,        0x88)
    (CONT_ITEM_SHOOT_WALK_B_BRAKE,  0x89)
    (CONT_ITEM_SHOOT_DASH_F,        0x8A)
    (CONT_ITEM_SHOOT_DASH_B,        0x8B)
    (CONT_ITEM_SHOOT_AIR,           0x8C)
    (CONT_ITEM_SHOOT_AIR_PASS,      0x8D)
    (CONT_ITEM_SHOOT_JUMP_BUTTON,   0x8E)
    (CONT_ITEM_SHOOT_JUMP,          0x8F)
    (CONT_ITEM_SHOOT_TURN_DASH,     0x90)
    (CONT_ITEM_SHOOT_TURN,          0x91)
    (CONT_ITEM_SHOOT_LANDING,       0x92)
    (ITEM_GEKIKARA_WAIT,            0x93)
    (ITEM_GEKIKARA_RUN,             0x94)
    (CONT_ITEM_GEKIKARA_RUN_BRAKE,  0x95)
    (THROW_KIRBY_GROUND,            0x96)
    (FINAL,                         0x97)
    (THROW_ENEMY,                   0x98)
    (SMASH_APPEAL,                  0x99)
    (ATTACH_WALL,                   0x9A)
    (WALL_JUMP,                     0x9B)
    (JUMP_START,                    0x9C)
    (CONT_SPECIAL_LW_COMMAND,       0x9D)
    (CONT_SUPER_SPECIAL,            0x9E)
    (CONT_SUPER_SPECIAL2,           0x9F)
    (CONT_ITEM_THROW_RUN_BRAKE_HI4, 0xA0)
    (CONT_LADDER_ATTACK,            0xA1)
    (CONT_ITEM_PICKUP_LIGHT_DASH,   0xA2)
    (CONT_ITEM_PICKUP_HEAVY_DASH,   0xA3)
    (CONT_COMMAND_623NB,            0xA4)
    (CONT_ATTACK_STAND,             0xA5)
    (CONT_ATTACK_SQUAT,             0xA6)
);

pub mod mario {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (PUMP_WAIT, 0xa7)
        (PUMP_FALL, 0xa8)
    );
}
pub mod link {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (RSLASH_HOLD, 0xa7)
        (RSLASH_END, 0xa8)
        (BOOMERANG_WAIT, 0xa7)
        (BOOMERANG_FALL, 0xa8)
        (BOOMERANG_CATCH_WAIT, 0xa7)
        (BOOMERANG_CATCH_FALL, 0xa8)
        (BOW_WAIT, 0xa7)
        (BOW_FALL, 0xa8)
        (BOW_HOLD, 0xa9)
        (BOW_END, 0xaa)
        (BOMB_WAIT, 0xa7)
        (BOMB_FALL, 0xa8)
        (BOMB, 0xa9)
        (FINAL_WAIT, 0xa7)
        (FINAL_FALL, 0xa8)
        (FINAL_DASH, 0xa9)
        (FINAL, 0xaa)
    );
}
pub mod samus {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (SCREW_ATTACK_START, 0xa6)
        (SCREW_ATTACK_ENABLE_FALL_SPECIAL, 0xa7)
    );
}
pub mod yoshi {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (TRANS_ID_START, 0xa6)
        (TRANS_ID_END, 0xa7)
    );
}
pub mod kirby {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (SPECIAL_N_GROUND, 0xa7)
        (SPECIAL_N_AIR, 0xa8)
        (SPECIAL_N, 0xa9)
        (HAMMER_SQUAT, 0xa7)
        (HAMMER_JUMP, 0xa8)
        (HAMMER_AIR, 0xa9)
        (HAMMER_GROUND, 0xaa)
        (HAMMER, 0xab)
    );
}
pub mod fox {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (BLASTER_WAIT, 0xa7)
        (BLASTER_FALL, 0xa8)
        (BLASTER_LANDING, 0xa9)
        (BLASTER, 0xaa)
        (ILLUSION_WAIT, 0xa7)
        (ILLUSION_FALL, 0xa8)
        (FIRE_WAIT, 0xa7)
        (FIRE, 0xa8)
        (REFLECTOR_WAIT, 0xa7)
        (REFLECTOR_FALL, 0xa8)
        (LANDMASTER_WAIT, 0xa7)
        (LANDMASTER_FALL, 0xa8)
    );
}
pub mod pikachu {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (SKULL_BASH_WAIT, 0xa7)
        (SKULL_BASH_FALL, 0xa8)
        (KAMINARI_WAIT, 0xa7)
        (KAMINARI_FALL, 0xa8)
        (VORTEX_WAIT, 0xa7)
        (VORTEX_FALL, 0xa8)
    );
}
pub mod luigi {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (SPECIAL_S_RAM_GROUND, 0xa7)
        (SPECIAL_S_RAM_HIT, 0xa8)
    );
}
pub mod ness {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (START, 0xa6)
        (ATTACK_HI4_MAX, 0xa7)
        (ATTACK_LW4_MAX, 0xa8)
        (_0, 0xa9)
        (_1, 0xaa)
        (END, 0xab)
    );
}
pub mod captain {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (FALCON_KICK_AA_START, 0xa6)
        (FALCON_KICK_AA_INHERIT_FALL, 0xa7)
        (FALCON_KICK_AA_LANDING_WAIT, 0xa8)
        (FALCON_KICK_AA_LANDING_FALL, 0xa9)
        (FINAL_SITUATION_GROUND, 0xa6)
    );
}
pub mod peach {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (TRANS_ID_START, 0xa6)
        (TRANS_ID_SPECIAL_LW_ITEM_THROW, 0xa7)
        (TRANS_ID_END, 0xa8)
        (SPECIAL_S_JUMP_TRANS_ID_START, 0xa7)
        (SPECIAL_S_JUMP_ID_TIME_OUT, 0xa8)
        (UNIQ_FLOAT_TRANS_ID_START, 0xa7)
        (UNIQ_FLOAT_ID_STICK_Y_UNDER, 0xa8)
        (UNIQ_FLOAT_TRANS_ID_FALL_CONTROL, 0xa9)
        (UNIQ_FLOAT_TRANS_ID_FALL_TIME, 0xaa)
        (UNIQ_FLOAT_TRANS_ID_ATTACK_AIR_CONTROL, 0xab)
        (UNIQ_FLOAT_TRANS_ID_ATTACK_AIR_TIME, 0xac)
    );
}
pub mod koopa {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (BREATH_WAIT, 0xa7)
        (BREATH_FALL, 0xa8)
        (BREATH_END, 0xa9)
    );
}
pub mod sheik {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (START, 0xa6)
        (_0, 0xa7)
        (_1, 0xa8)
        (END, 0xa9)
    );
}
pub mod falco {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (BLASTER_WAIT, 0xa7)
        (BLASTER_FALL, 0xa8)
        (BLASTER_LANDING, 0xa9)
        (BLASTER, 0xaa)
        (ILLUSION_WAIT, 0xa7)
        (ILLUSION_FALL, 0xa8)
        (FIRE_WAIT, 0xa7)
        (FIRE_, 0xa8)
        (REFLECTOR_WAIT, 0xa7)
        (REFLECTOR_FALL, 0xa8)
        (LANDMASTER_WAIT, 0xa7)
        (LANDMASTER_FALL, 0xa8)
    );
}
pub mod ganon {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (PUNCH_START, 0xa6)
        (PUNCH_ENABLE_GROUND_END, 0xa7)
        (PUNCH_ENABLE_AIR_END, 0xa8)
        (KICK_AA_START, 0xa6)
        (KICK_AA_INHERIT_FALL, 0xa7)
        (KICK_AA_LANDING, 0xa8)
        (KICK_AA_LANDING_WAIT, 0xa9)
        (KICK_AA_LANDING_FALL, 0xaa)
        (BEAST_START, 0xa6)
        (BEAST_WAIT, 0xa7)
        (BEAST_FALL, 0xa8)
        (BEAST, 0xa9)
    );
}
pub mod mewtwo {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (SHADOWBALL_START_HOLD, 0xa7)
        (SHADOWBALL_START_SHOOT, 0xa8)
    );
}
pub mod metaknight {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (TRANS_ID_START, 0xa6)
        (TRANS_ID_END, 0xa7)
    );
}
pub mod snake {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (TRANS_ID_START, 0xa6)
        (TRANS_ID_END, 0xa7)
        (CYPHER_HANG_TRANS_ID_START, 0xa6)
        (CYPHER_HANG_TRANS_ID_CUT_STICK, 0xa7)
        (CYPHER_HANG_TRANS_ID_CUT_TIME_OUT, 0xa8)
    );
}
pub mod pokemon {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (PRE_START, 0xa6)
        (CONT_SPECIAL_LW, 0xa7)
    );
}
pub mod plizardon {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (BREATH_WAIT, 0xa7)
        (BREATH_FALL, 0xa8)
        (BREATH_END, 0xa9)
    );
}
pub mod diddy {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (SPECIAL_S_GROUND, 0xa7)
        (SPECIAL_S_AIR, 0xa8)
        (SPECIAL_S, 0xa9)
        (CONT_SPECIAL_HI_SQUAT, 0xa7)
        (FINAL_GROUND, 0xa7)
        (FINAL_AIR, 0xa8)
        (FINAL, 0xa9)
    );
}
pub mod lucas {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (START, 0xa6)
        (ATTACK_HI4_MAX, 0xa7)
        (ATTACK_LW4_MAX, 0xa8)
        (_0, 0xa9)
        (_1, 0xaa)
        (END, 0xab)
    );
}
pub mod sonic {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (TRANS_ID_START, 0xa6)
        (TRANS_ID_END, 0xa7)
    );
}
pub mod dedede {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (SPECIAL_N_GROUND, 0xa7)
        (SPECIAL_N_AIR, 0xa8)
        (SPECIAL_N, 0xa9)
        (SUPER_JUMP_TURN, 0xa7)
        (SUPER_JUM, 0xa8)
        (JET_HAMMER_SQUAT, 0xa7)
        (JET_HAMMER_JUMP, 0xa8)
        (JET_HAMMER_AIR, 0xa9)
        (JET_HAMMER_GROUND, 0xaa)
        (JET_HAMMER, 0xab)
    );
}
pub mod lucario {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (AURABALL_START_HOLD, 0xa7)
        (AURABALL_START_SHOOT, 0xa8)
        (MACH_WAIT, 0xa7)
        (MACH_, 0xa8)
    );
}
pub mod robot {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (BEAM_WAIT, 0xa7)
        (BEAM_FALL, 0xa8)
        (BEAM, 0xa9)
    );
}
pub mod shulk {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (FINAL_SITUATION_GROUND, 0xa6)
    );
}
pub mod duckhunt {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (FINAL_SITUATION_GROUND, 0xa6)
    );
}
pub mod miistatus {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (SYOTEN_KICK_HOLD, 0xa7)
        (SYOTEN_KICK_END, 0xa8)
        (NENSYO_KICK_WAIT, 0xa7)
        (NENSYO_KICK_FALL, 0xa8)
    );
}
pub mod miiswordsman {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (SPECIAL_S1_WAIT, 0xa7)
        (SPECIAL_S1_FALL, 0xa8)
        (RSLASH_HOLD, 0xa7)
        (RSLASH_END, 0xa8)
        (SDUSH_WAIT, 0xa7)
        (SDUSH_, 0xa8)
        (JET_STUB_AA_START, 0xa6)
        (JET_STUB_AA_INHERIT_FALL, 0xa7)
        (JET_STUB_AA_LANDING_WAIT, 0xa8)
        (JET_STUB_AA_LANDING_FALL, 0xa9)
    );
}
pub mod miigunner {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (GUNNER_CHARGE_START, 0xa6)
        (GUNNER_CHARGE_ENABLE_AIR, 0xa7)
        (GUNNER_CHARGE_ENABLE_GROUND, 0xa8)
        (RAPID_SHOT_START, 0xa6)
        (RAPID_SHOT_WAIT, 0xa7)
        (RAPID_SHOT_FALL, 0xa8)
        (RAPID_SHOT_LANDING, 0xa9)
        (REFLECTOR_WAIT, 0xa7)
        (REFLECTOR_FALL, 0xa8)
        (ABSORBER_0, 0xa7)
        (ABSORBER_1, 0xa8)
    );
}
pub mod popo {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (TRANS_ID_START, 0xa6)
        (TRANS_ID_END, 0xa7)
        (SPECIAL_HI_TRANS_ID_START, 0xa6)
    );
}
pub mod miienemyf {
    use super::*;
    make_work_ids!(
        Int,
        Transition,
        (SYOTEN_KICK_HOLD, 0xa7)
        (SYOTEN_KICK_END, 0xa8)
        (_100RUSH_AA_START, 0xa6)
        (_100RUSH_AA_INHERIT_FALL, 0xa7)
        (_100RUSH_AA_LANDING_WAIT, 0xa8)
        (_100RUSH_AA_LANDING_FALL, 0xa9)
        (NENSYO_KICK_WAIT, 0xa7)
        (NENSYO_KICK_FALL, 0xa8)
    );
}
