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
    pub const USE_SPECIAL_N_CALLBACK: i32 = 0x38;
    pub const USE_SPECIAL_S_CALLBACK: i32 = 0x39;
    pub const USE_SPECIAL_HI_CALLBACK: i32 = 0x3A;
    pub const USE_SPECIAL_LW_CALLBACK: i32 = 0x3B;
    pub const CHECK_SPECIAL_COMMAND: i32 = 0x3C;
    pub const WAZA_CUSTOMIZE_CONTROL: i32 = 0x3D;
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
        pub const IS_SMASH_TURN: i32 = 74;
        pub const IS_STICKY_WALK: i32 = 75;
        pub const ENABLE_BOOST_RUN: i32 = 76;
        pub const UP_SPECIAL_JUMP_REFRESH_WINDOW: i32 = 77;
        pub const PERFECT_WAVEDASH: i32 = 78;
        pub const ENABLE_AIR_ESCAPE_JUMPSQUAT: i32 = 79;
        pub const SHOULD_WAVELAND: i32 = 80;
        pub const SIDE_SPECIAL_CANCEL_NO_HIT: i32 = 81;
        pub const JUMP_NEXT: i32 = 82;
        pub const IS_JAB_LOCK_ROLL: i32 = 83;
        pub const SHOULD_TRUMP_TETHER: i32 = 84;
        pub const CAN_PERFECT_PIVOT: i32 = 85;
        

        // int
        pub const LAST_ATTACK_RECEIVER_ENTRY_ID: i32 = 0x0;
        pub const COSTUME_SLOT_NUMBER: i32 = 0x1;
        pub const FLOAT_TIMER: i32 = 0x2;
        pub const FLOAT_DURATION: i32 = 0x3;
        pub const FLOAT_STYLE: i32 = 0x4;
        pub const HITFALL_BUFFER: i32 = 0x5;
        pub const JUMP_SQUAT_FRAME: i32 = 0x6;
        pub const GIMMICK_TIMER: i32 = 0x7;
        pub const ATTACK_DASH_CANCEL_FRAME: i32 = 0x8;
        pub const AIR_ESCAPE_MAGNET_FRAME: i32 = 0x9;
        pub const TURN_DASH_FRAME: i32 = 0xA;
        pub const DOWN_STAND_FB_KIND: i32 = 0xB;
        pub const CSTICK_LIFE: i32 = 0xC;
        pub const AGT_USED_COUNTER: i32 = 0xD;

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
        /// this multiplier can be set to a value between 0.1 and 3.0 to increase
        /// a character's jump speed max for momentum transfer (for meta quick, etc)
        pub const JUMP_SPEED_MAX_MUL: i32 = 0x1A;
    }

    pub mod bayonetta {
        // flags
        pub const IS_ENABLE_SPECIAL_CANCEL: i32 = 0x1000;
        pub const SHOULD_PRORATE_DAMAGE: i32 = 0x1001;
        pub const IS_SPECIAL_S_CANCELED_INTO: i32 = 0x1002;
        pub const IS_SPECIAL_HI_CANCELED_INTO: i32 = 0x1003;

        //ints
        pub const NUM_RECOVERY_RESOURCE_USED:         i32 = 0x1000;
        pub const NUM_SPECIAL_S_CANCEL_THIS_AIRTIME:  i32 = 0x1001;
        pub const NUM_SPECIAL_HI_CANCEL_THIS_AIRTIME: i32 = 0x1002;
    }

    pub mod brave {
        // flags
        pub const IS_CRITICAL_HIT: i32 = 0x1000;
    }

    pub mod buddy {
        // flags
        pub const IS_BURY_DTHROW: i32 = 0x1000;
    }

    pub mod chrom {
        // flags
        pub use super::roy::TRAIL_EFFECT;
    }

    pub mod demon {
        // flags
        pub const SLAUGHTER_HIGH_KICK: i32 = 0x1000;
        pub const DEVASTATOR: i32 = 0x1001;
        pub const JAW_BREAKER: i32 = 0x1002;
        pub const SLICING_BLADE: i32 = 0x1003;
        pub const SPINNING_DEMON: i32 = 0x1004;
        pub const LIGHTNING_SCREW_UPPERCUT: i32 = 0x1005;
        pub const TWIN_FANG_DOUBLE_KICK: i32 = 0x1006;
    }

    pub mod dolly {
        pub use super::shotos::*;

        // flags
        pub const IS_USE_FIRE_KICK: i32 = 0x1050;
        pub const UNABLE_CANCEL_S3_DASH: i32 = 0x1051;
        pub const IS_CHAIN_CANCEL: i32 = 0x1052;
        pub const IS_SHATTER_STRIKE: i32 = 0x1053;
        pub const IS_STARTED_SPECIAL_B_GROUNDED: i32 = 0x1054;
    }

    pub mod duckhunt {
        // int
        pub const GUNMAN_TIMER: i32 = 0x1000;
    }

    pub mod elight {
        // int

        /// This is used to determine how to end the SpecialHiJump status script
        pub const SPECIAL_HI_JUMP_RESERVE_ACTION: i32 = 0x1000;

        // flags
        pub const DISABLE_SPECIAL_HI_JUMP: i32 = 0x1000;
        pub const DISABLE_SPECIAL_S:       i32 = 0x1001;

        // not IDs but symbolic consts
        pub const SPECIAL_HI_JUMP_RESERVE_ACTION_ATTACK1: i32 = 0x0;
        pub const SPECIAL_HI_JUMP_RESERVE_ACTION_ATTACK2: i32 = 0x1;
        pub const SPECIAL_HI_JUMP_RESERVE_ACTION_FALL:    i32 = 0x2;
    }

    pub mod ganon {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_N: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const FLOAT_ENABLE_ACTIONS: i32 = 0x1100;
            pub const FLOAT_FALL_SPEED_Y_INCREASE: i32 = 0x1101;
            pub const FLOAT_CANCEL: i32 = 0x1102;
            pub const FLOAT_GROUND_CHANGE_KINETIC: i32 = 0x1103;
        }

    }

    pub mod gaogaen {
        // flags
        pub const IS_SPECIAL_S_ALTERNATE_GRAB:             i32 = 0x1000;
        pub const IS_SPECIAL_S_GROUND_GRAB:                i32 = 0x1001;
        pub const IS_SPECIAL_S_AIR_GRAB:                   i32 = 0x1002;
        pub const IS_HIT_SPECIAL_HI_RISE:                  i32 = 0x1003;
        pub const IS_INPUT_CROSS_CHOP_CANCEL:              i32 = 0x1004;
        pub const SHOULD_CROSS_CHOP_DIVE_EARLY:            i32 = 0x1005;
        pub const IS_SPECIAL_LW_COMMAND_DASH:              i32 = 0x1006;
        pub const IS_ENABLE_SPECIAL_LW_LARIAT_INPUT:       i32 = 0x1007;
        pub const IS_INPUT_SPECIAL_LW_LARIAT:              i32 = 0x1008;
        pub const DID_SPECIAL_LW_COMMAND_DASH_TANK_DAMAGE: i32 = 0x1009;
        pub const SHOULD_APPLY_REVENGE_BUFFS:              i32 = 0x100A;
        

        // ints
        pub const SPECIAL_N_STRENGTH_LEVEL: i32 = 0x1000;

        // floats
        pub const ANGLE_GRAB_STICK_Y: i32 = 0x1000;
    }

    pub mod ike {
        // flags
        pub const IS_QUICK_DRAW_INSTAKILL: i32 = 0x1000;
    }

    pub mod inkling {
        // flag
        pub const IS_ENABLE_SPECIAL_S_JUMP_EARLY_CANCEL: i32 = 0x1000;
    }

    pub mod kamui {
        // flag
        pub const BAIR_BOOST: i32 = 0x1000;
    }

    pub mod ken {
        pub use super::shotos::*;
    }

    pub mod lucas {
        // flag
        pub const SPECIAL_HI_ATTACK_IS_TOUCH_WALL: i32 = 0x1000;
        pub const SPECIAL_HI_ATTACK_IS_LEAVE_WALL: i32 = 0x1001;
        pub const SPECIAL_HI_ATTACK_IS_FLIPPED_MOMENTUM_AFTER_WALLTOUCH: i32 = 0x1002;
        pub const SPECIAL_HI_ATTACK_IS_SET_WALL_LEAVE_MOMENTUM: i32 = 0x1003;

        // float
        /// Holds the initial y velocity you have during up b to compare once you've touched a wall to reflect your speed the other direction if it changes
        pub const SPECIAL_HI_ATTACK_Y_MOMENTUM: i32 = 0x1000;
        pub const SPECIAL_HI_ATTACK_Y_INIT_MOMENTUM: i32 = 0x1001;
        pub const SPECIAL_HI_ATTACK_Y_DIRECTION: i32 = 0x1002;
        pub const SPECIAL_HI_ATTACK_X_MOMENTUM: i32 = 0x1003;
        pub const SPECIAL_HI_ATTACK_X_INIT_MOMENTUM: i32 = 0x1004;
        pub const SPECIAL_HI_ATTACK_X_DIRECTION: i32 = 0x1005;
        pub const SPECIAL_HI_ATTACK_WALL_TOUCH_FRAME: i32 = 0x1006;
        pub const SPECIAL_HI_ATTACK_WALL_LEAVE_FRAME: i32 = 0x1007;
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
        pub const IS_SPECIAL_N_FIREBRAND:                i32 = 0x1000;
        pub const SPECIAL_N_DOUBLE_FIREBALL_NOTIFY_FLAG: i32 = 0x1001;
        pub const IS_SPECIAL_N_DOUBLE_FIREBALL:          i32 = 0x1002;
        pub const CAN_INPUT_SPECIAL_N_DOUBLE_FIREBALL:   i32 = 0x1003;
    }

    pub mod master {
        // ints
        pub const IS_ENABLE_SPECIAL_S_DASH_CANCEL: i32 = 0x1000;

        // ints
        pub const AYMR_CHARGE_LEVEL: i32 = 0x1000;
    }
    
    pub mod pickel {
        // flags
        pub const IS_CURRENT_ATTACK_LW3_SOUL_FIRE: i32 = 0x1000;
    }

    pub mod mariod {
        // flags
        pub const IS_SPECIAL_N_CHILL_PILL:       i32 = 0x1000;
        pub const IS_SPECIAL_S_ELECTRIC_BLANKET: i32 = 0x1001;
        pub const IS_SPECIAL_HI_UNABLE_CANCEL: i32 = 0x1002;
        pub const IS_SPECIAL_HI_SWEETSPOT_HIT: i32 = 0x1003;
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
        pub const IS_ENABLE_MAGIC_SERIES_CANCEL: i32 = 0x1004;
        pub const IS_ENABLE_FADC: i32 = 0x1005;
        pub const IS_TARGET_COMBO_1: i32 = 0x1006;
        pub const IS_TARGET_COMBO_2: i32 = 0x1007;
        pub const IS_CURRENT_HADOKEN_EX: i32 = 0x1008;
        pub const IS_ENABLE_AIRDASH_CANCEL: i32 = 0x1009;

        // ints
        pub const REPEAT_COUNT_LW: i32 = 0x1000;
        pub const REPEAT_COUNT_HI: i32 = 0x1001;
        pub const EX_SPECIAL_SCRIPTING: i32 = 0x1002;
        pub const AIR_CHAIN_COMBO_NUM: i32 = 0x1003;
    }

    pub mod shizue {
        // flags
        pub const IS_NOT_QUICK_RELEASE: i32 = 0x1000;

        // floats
        pub const STORED_BALLOON_POWER: i32 = 0x1000;
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
        pub const UP_SPECIAL_HIT: i32 = 0x1005;
        pub const COMBO_PLUS_GROUND: i32 = 0x1006;
        pub const COMBO_PLUS_AIR: i32 = 0x1007;
        pub const SHOULD_PRORATE_ATTACK: i32 = 0x1008;
        pub const IS_LAND_CANCEL_THUNDER: i32 = 0x1009;
        pub const IS_GRAND_MAGIC: i32 = 0x100A;
        pub const IS_CURRENT_FIRAGA_GRAND_MAGIC: i32 = 0x100B;
        pub const IS_CURRENT_BLIZZAGA_GRAND_MAGIC: i32 = 0x100C;
        pub const IS_CURRENT_THUNDAGA_GRAND_MAGIC: i32 = 0x100D;

        // floats
        pub const SONIC_BLADE_Y: i32 = 0x1000;
    }

    pub mod samus {
        // flags
        pub const SHINESPARK_USED: i32 = 0x1000;
        pub const SHINESPARK_READY: i32 = 0x1001;
    }

    pub mod robot {
        // ints
        pub const PASSIVE_FUEL_INDICATOR_EFFECT_HANDLE: i32 = 0x1000;
        pub const PREV_FUEL_THRESHOLD:                  i32 = 0x1001;

        // flags
        pub const BOOST_ATTACK: i32 = 0x1000;
    }

    pub mod palutena {
        // floats
        pub const SPECIAL_LW_LR: i32 = 0x1000;

        // flags
        pub const SPECIAL_LW_AEGIS_REFLECTOR: i32 = 0x1000;
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

    pub mod metaknight {
        /// i32 timer for color flashing when meta quick is ready
        pub const META_QUICK_READY_FLASH_TIMER: i32 = 0x1000;
        pub const META_QUICK_STATUS: i32 = 0x1001;

        /// u32 effect handle, 0xFFFF_FFFF for invalid
        pub const META_QUICK_EFFECT_HANDLE: i32 = 0x1002;
        
        /// u32 effect handle for the charge, 0xFFFF_FFFF for invalid
        pub const META_QUICK_CHARGE_EFFECT_HANDLE: i32 = 0x1003;
        
        /// u32 effect handle, 0xFFFF_FFFF for invalid
        pub const META_QUICK_EFFECT_HANDLE2: i32 = 0x1004;

        /// flag whether we need to set metaknight speed values next frame
        pub const COMPLETED_SET_SPEEDS: i32 = 0x1000;
        pub const META_QUICK_NEED_SET_SPEEDS: i32 = 0x1001;
        pub const META_QUICK_PLAY_VC: i32 = 0x1002;
    }
    
    pub mod miifighter {
        // floats
        pub const CHARGE_ATTACK_LEVEL: i32 = 0x1000;
        
        // flags
        pub const IS_COUNTER_THROW_PARRIED_ATTACK: i32 = 0x1000;
        pub const IS_CURRENT_IRONBALL_HEAVY:       i32 = 0x1001;

    }
    
    pub mod littlemac {
        // flags
        pub const IS_DREAMLAND_EXPRESS: i32 = 0x1000;
    }


}

pub mod statuses {
    pub mod elight {
        pub const SPECIAL_HI_FINISH2: i32 = 0;
    }

    pub mod metaknight {
        pub const METAQUICK_SUMMON: i32 = 0;
    }

    pub mod ganon {
        pub const SPECIAL_N_FLOAT: i32 = 0;
    }

}