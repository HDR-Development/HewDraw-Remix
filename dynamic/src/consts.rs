pub mod globals {
    // 0x1
    pub const FIGHTER_KIND:          i32 = 0x2;
    pub const OBJECT_ID:             i32 = 0x3;
    // 0x4
    pub const MODULE_ACCESSOR:       i32 = 0x5;
    // 0x6
    pub const INIT_STATUS_FUNC:      i32 = 0x7;
    pub const IS_STOPPING:           i32 = 0x8;
    pub const STATUS_KIND_INTERRUPT: i32 = 0x9;
    pub const PREV_STATUS_KIND:      i32 = 0xA;
    pub const STATUS_KIND:           i32 = 0xB;
    pub const STATUS_COUNT:          i32 = 0xC;
    // 0xD
    pub const CURRENT_FRAME:         i32 = 0xE;
    pub const CURRENT_FRAME2:        i32 = 0xF;
    // 0x10
    // 0x11 func ptr
    // 0x12
    pub const SUB_STATUS3:           i32 = 0x13;
    pub const SUB_STATUS2:           i32 = 0x14;
    pub const SUB_STATUS:            i32 = 0x15;
    pub const SITUATION_KIND:        i32 = 0x16;
    pub const PREV_SITUATION_KIND:   i32 = 0x17;
    pub const PREV_STATUS_FRAME:     i32 = 0x18;
    // 0x19
    pub const STICK_X:               i32 = 0x1A;
    pub const STICK_Y:               i32 = 0x1B;
    pub const FLICK_X:               i32 = 0x1C;
    pub const FLICK_Y:               i32 = 0x1D;
    pub const FLICK_Y_DIR:           i32 = 0x1E;
    pub const PAD_FLAG:              i32 = 0x1F;
    pub const CMD_CAT1:              i32 = 0x20;
    pub const CMD_CAT2:              i32 = 0x21;
    pub const CMD_CAT3:              i32 = 0x22;
    pub const CMD_CAT4:              i32 = 0x23;
    // 0x24
    // 0x25
    // 0x26
    // 0x27
    // 0x28 some substatus
    pub const DASH_CALLBACK:         i32 = 0x29;
    // 0x2A
    pub const CUSTOM_ROUTINE:        i32 = 0x2B;
    // 0x2C
    // 0x2D
    // 0x2E
    // 0x2F
    // 0x30
    // 0x31
    // 0x32 some substatus
    pub const STATUS_CHANGE_CALLBACK: i32 = 0x3E;
    pub const DASH_POST_TRANSITION_CALLBACK: i32 = 0x57;
}

pub mod vars {
    pub mod common {
        // flag
        pub const IS_HEAVY_ATTACK: i32 = 0x0;
        pub const FIREBRAND_ACTIVATED: i32 = 0x1;
        pub const DOUBLE_FIREBALL: i32 = 0x2;
        pub const NOKNOK_SHELL: i32 = 0x3;
        pub const IS_IN_HITSTUN: i32 = 0x4;
        pub const CSTICK_OVERRIDE: i32 = 0x5;
        pub const CSTICK_OVERRIDE_SECOND: i32 = 0x6;
        pub const IS_TAP_JUMP: i32 = 0x7;
        pub const OMNI_FLOAT: i32 = 0x8;
        pub const SIDE_SPECIAL_CANCEL: i32 = 0x9;
        pub const DISABLE_UP_SPECIAL_JUMP_REFRESH: i32 = 0xA;
        pub const HITSTUN_START: i32 = 0xB;
        pub const AERIAL_NO_FLOAT: i32 = 0xC;
        pub const FLOAT_PAUSE_AERIAL: i32 = 0xD;
        pub const SMASH_CHECKS: i32 = 0xE;
        pub const TILT_CHECKS: i32 = 0xF;
        pub const JAB_DA_CHECKS: i32 = 0x10;
        pub const AERIAL_CHECKS: i32 = 0x11;
        pub const SPECIAL_STALL_USED: i32 = 0x12;
        pub const SPECIAL_STALL: i32 = 0x13;
        pub const UP_SPECIAL_INTERRUPT: i32 = 0x14;
        pub const ENABLE_AIR_ESCAPE_MAGNET: i32 = 0x15;
        pub const UP_SPECIAL_INTERRUPT_AIRTIME: i32 = 0x16;
        pub const DITCIT_SLIDING: i32 = 0x17;
        pub const AIR_CROSS: i32 = 0x18;
        pub const FINAL_CUTTER_HIT: i32 = 25;
        pub const SPECIAL_CHECKS: i32 = 26;
        pub const DISABLE_AIRDODGE: i32 = 27;
        pub const NEUTRAL_SPECIAL_HIT: i32 = 28;
        pub const ILLUSION_SHORTENED: i32 = 30;
        pub const FOOTSTOOL_AIRDODGE_LOCKOUT: i32 = 31;
        pub const CAN_ESCAPE_TUMBLE: i32 = 32;
        pub const SUPER_CANCEL: i32 = 33;
        pub const SPECIAL_AUTOCANCEL: i32 = 35;
        pub const ILLUSION_SHORTEN: i32 = 37;
        pub const SOARING_SLASH_HIT: i32 = 38;
        pub const DOUBLE_JUMP_STOP: i32 = 39;
        pub const KIRBY_STAR_ROD: i32 = 40;
        pub const IS_IN_TUMBLE: i32 = 41;
        pub const SPECIAL_WALL_JUMP: i32 = 42;
        pub const DOWN_SPECIAL_HIT: i32 = 43;
        pub const MAGIC_CANCEL_ADDITIONAL: i32 = 44;
        pub const TETHER_HOGGED: i32 = 45;
        pub const AERIAL_COMMAND_MOMENTUM_RESET: i32 = 46;
        pub const TUMBLE_START: i32 = 47;
        pub const B_REVERSED: i32 = 48;
        pub const AERIAL_COMMAND_RISING: i32 = 49;
        pub const SIDE_SPECIAL_HIT: i32 = 50;
        pub const TUMBLE_KB: i32 = 51;
        pub const UP_SPECIAL_HIT: i32 = 52;
        pub const AIR_SPECIAL_USED: i32 = 54;
        pub const LEDGE_OCCUPYING: i32 = 55;
        pub const DOUBLE_JUMP_CANCELED: i32 = 56;
        pub const IS_MOONWALK: i32 = 57;
        pub const CAN_GLIDE_TOSS: i32 = 58;
        pub const IS_MOONWALK_JUMP: i32 = 59;
        pub const IS_MOTION_BASED_ATTACK: i32 = 60;
        pub const PREV_FLAG_DISABLE_ESCAPE_AIR: i32 = 61;
        pub const ENABLE_WAVELAND_PLATDROP: i32 = 62;
        pub const SPECIAL_PROJECTILE_SPAWNED: i32 = 63;
        pub const UP_SPECIAL_CANCEL: i32 = 64;
        pub const IS_TELEPORT_WALL_RIDE: i32 = 65;
        pub const SPIN_ATTACK_LAND_CANCEL: i32 = 66;
        pub const AERIAL_COMMAND_RISEN: i32 = 67;
        pub const DISABLE_SPECIAL_JC: i32 = 68;
        pub const IS_DACUS: i32 = 69;
        pub const ATTACK_DASH_CANCEL_DISABLE: i32 = 70;
        pub const DISABLE_BACKDASH: i32 = 71;
        pub const IS_LATE_PIVOT: i32 = 72;
        pub const IS_TURNDASH_INPUT: i32 = 73;
        pub const IS_BACKDASH: i32 = 74;
        pub const IS_STICKY_WALK: i32 = 75;
        pub const ENABLE_BOOST_RUN: i32 = 76;
        pub const UP_SPECIAL_JUMP_REFRESH_WINDOW: i32 = 77;
        pub const PERFECT_WAVEDASH: i32 = 78;
        pub const ENABLE_AIR_ESCAPE_JUMPSQUAT: i32 = 79;
        pub const SHOULD_WAVELAND: i32 = 80;
        pub const SIDE_SPECIAL_CANCEL_NO_HIT: i32 = 81;
        pub const JUMP_NEXT: i32 = 82;

        

        // int
        pub const LAST_ATTACK_RECEIVER_ENTRY_ID: i32 = 0x0;
        pub const COSTUME_SLOT_NUMBER: i32 = 0x1;
        pub const FLOAT_TIMER: i32 = 0x2;
        pub const FLOAT_DURATION: i32 = 0x3;
        pub const FLOAT_STYLE: i32 = 0x4;
        pub const GIMMICK_READY_GLOW_TIMER: i32 = 0x5;
        pub const HITFALL_BUFFER: i32 = 0x7;
        pub const JUMP_SQUAT_FRAME: i32 = 0x8;
        pub const GIMMICK_TIMER: i32 = 0x9;
        pub const ATTACK_DASH_CANCEL_FRAME: i32 = 0xA;
        pub const AIR_ESCAPE_MAGNET_FRAME: i32 = 0xB;
        pub const TURN_DASH_FRAME: i32 = 0xC;

        // float
        pub const LAST_ATTACK_DAMAGE_DEALT: i32 = 0x0;
        pub const CURRENT_MOMENTUM: i32 = 0x1;
        pub const JUMPSQUAT_VELOCITY: i32 = 0x2;
        /// This const is set in a fighter reset because the params used to calculate change depending on situation
        pub const JUMP_SPEED_RATIO: i32 = 0x3;
        pub const DOUBLE_JUMP_FRAME: i32 = 0x4;
        pub const GROUND_VEL: i32 = 0x5;
        pub const RAR_LENIENCY: i32 = 0x6;
        pub const CURRENT_MOMENTUM_SPECIALS: i32 = 0x7;
        pub const DOUBLE_JUMP_TIMER: i32 = 0x8;
        pub const GLIDE_TIMER: i32 = 0x9;
        pub const BASE_RUN_SPEED_MAX: i32 = 0xA;
        pub const SONIC_LIGHTSPEED_DASH_FRAME_COUNTER: i32 = 0xB;
        pub const BASE_DASH_SPEED: i32 = 0xC;
        pub const WITHDRAW_FRAME: i32 = 0xD;
        pub const ROLL_DIR: i32 = 0xE;
        pub const LEDGE_POS: i32 = 0xF;
        pub const LEDGE_POS_X: i32 = 0xF;
        pub const LEDGE_POS_Y: i32 = 0x10;
        pub const LEDGE_POS_Z: i32 = 0x11;
        pub const MP_SPEED_RATIO: i32 = 0x12;
        pub const GET_DIST_TO_FLOOR: i32 = 0x13;
        pub const ECB_Y_OFFSETS: i32 = 0x14;
        pub const CURR_DASH_SPEED: i32 = 0x15;
        pub const MOONWALK_SPEED: i32 = 0x16;
        pub const ESCAPE_AIR_SLIDE_SPEED_X: i32 = 0x17;
        pub const ESCAPE_AIR_SLIDE_SPEED_Y: i32 = 0x18;
        pub const Y_POS: i32 = 0x19;
       
    }

    pub mod brave {
        // flags
        pub const IS_CRITICAL_HIT: i32 = 0x1000;
    }

    pub mod chrom {
        // flags
        pub use super::roy::TRAIL_EFFECT;
    }

    pub mod demon {
        // flags
        pub const SLAUGHTER_HIGH_KICK: i32 = 0x1000;
    }

    pub mod dolly {
        pub use super::shotos::*;

        // flags
        pub const IS_USE_FIRE_KICK: i32 = 0x1050;
        pub const UNABLE_CANCEL_S3_DASH: i32 = 0x1051;
    }

    pub mod duckhunt {
        // int
        pub const GUNMAN_TIMER: i32 = 0x1000;
    }

    pub mod gaogaen {
        // floats
        pub const ANGLE_GRAB_STICK_Y: i32 = 0x1000;
    }

    pub mod kamui {
        // flag
        pub const BAIR_BOOST: i32 = 0x1000;
    }

    pub mod ken {
        pub use super::shotos::*;
    }

    pub mod luigi {
        // flag
        /// This flag stores whether or not Luigi currently has a misfire stored.
        pub const IS_MISFIRE_STORED: i32 = 0x1000;
        
        // int
        /// This int stores the number of remaining green missile's luigi must do before getting a misfire
        pub const REMAINING_SPECIAL_S_UNTIL_MISFIRE: i32 = 0x1000;
        /// This int stores the handle of the charge smoke effect for killing it if we store misfire
        pub const CHARGE_SMOKE_EFFECT_HANDLE: i32 = 0x1001;
        /// This int stores the handle of the pulsing effect for killing it if we store misfire
        pub const CHARGE_PULSE_EFFECT_HANDLE: i32 = 0x1002;

        // float
        /// This float holds the current multiplier on damage for misfire
        pub const MISFIRE_DAMAGE_MULTIPLIER: i32 = 0x1000;
    }

    pub mod mario {
        // flags
        pub const FIREBRAND_SPAWNED: i32 = 0x1000;
    }

    pub mod roy {
        // flags
        pub const TRAIL_EFFECT: i32 = 0x1000;
    }

    pub mod shotos {
        // flags
        pub const IS_USE_EX_SPECIAL: i32 = 0x1000;
        pub const IS_MAGIC_SERIES_CANCEL: i32 = 0x1001;
        pub const SHOULD_COMBOS_SCALE: i32 = 0x1002;
        pub const REPEAT_INCREMENTED: i32 = 0x1003;

        // ints
        pub const REPEAT_COUNT_LW: i32 = 0x1000;
        pub const REPEAT_COUNT_HI: i32 = 0x1001;
        pub const EX_SPECIAL_SCRIPTING: i32 = 0x1002;
    }

    pub mod ryu {
        pub use super::shotos::*;
    }
    
    pub mod sonic {
        // flags
        pub const PULSE_HITBOX: i32 = 0x1000;
    }
    
    pub mod snake {
        // ints
        pub const SNAKE_GRENADE_COUNTER: i32 = 0x1000;
    }
    
    pub mod trail {
        // flags
        pub const ATTACK_12_INTO_S3: i32 = 0x1000;
        pub const UP_SPECIAL_TO_SIDE_SPECIAL: i32 = 0x1001;
        pub const SIDE_SPECIAL_HIT: i32 = 0x1002;
        pub const IS_SIDE_SPECIAL_INPUT: i32 = 0x1003;
        pub const STOP_SIDE_SPECIAL: i32 = 0x1004;
    }

    pub mod samus {
        // flags
        pub const SHINESPARK_USED: i32 = 0x1000;
        pub const SHINESPARK_READY: i32 = 0x1001;
    }

    pub mod robot {
        // flags
        pub const BOOST_ATTACK: i32 = 0x1000;
    }

    pub mod palutena {
        // floats
        pub const SPECIAL_LW_LR: i32 = 0x1000;
    }

    pub mod miiswordsman {
        // ints
        pub const SPECIAL_LW1_CHARGE_LEVEL: i32 = 0x1000;

        // flags
        pub const SPECIAL_LW1_ATTACK_TRIGGER: i32 = 0x1000;
        pub const SKYWARD_SLASH_DASH_HIT: i32 = 0x1001;
        pub const WAVE_SPECIAL_N: i32 = 0x1002;
        pub const CHAKRAM_STICK_ATTACK: i32 = 0x1003;
        pub const GALE_STAB_EDGE_CANCEL: i32 = 0x1004;

    }

    pub mod miigunner {
        // floats
        pub const CHARGE_ATTACK_LEVEL: i32 = 0x1000;
        
        // flags
        pub const IS_CHARGE_FINISHED: i32 = 0x1000;
    }


}
