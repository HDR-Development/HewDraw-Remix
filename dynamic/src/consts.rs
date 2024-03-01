pub mod globals {
    // 0x1
    pub const FIGHTER_KIND: i32 = 0x2;
    pub const OBJECT_ID: i32 = 0x3;
    // 0x4
    pub const MODULE_ACCESSOR: i32 = 0x5;
    // 0x6
    pub const INIT_STATUS_FUNC: i32 = 0x7;
    pub const IS_STOPPING: i32 = 0x8;
    pub const STATUS_KIND_INTERRUPT: i32 = 0x9;
    pub const PREV_STATUS_KIND: i32 = 0xA;
    pub const STATUS_KIND: i32 = 0xB;
    pub const STATUS_COUNT: i32 = 0xC;
    // 0xD
    pub const CURRENT_FRAME: i32 = 0xE;
    pub const CURRENT_FRAME2: i32 = 0xF;
    // 0x10
    // 0x11 func ptr
    // 0x12
    pub const SUB_STATUS3: i32 = 0x13;
    pub const SUB_STATUS2: i32 = 0x14;
    pub const SUB_STATUS: i32 = 0x15;
    pub const SITUATION_KIND: i32 = 0x16;
    pub const PREV_SITUATION_KIND: i32 = 0x17;
    pub const PREV_STATUS_FRAME: i32 = 0x18;
    // 0x19
    pub const STICK_X: i32 = 0x1A;
    pub const STICK_Y: i32 = 0x1B;
    pub const FLICK_X: i32 = 0x1C;
    pub const FLICK_Y: i32 = 0x1D;
    pub const FLICK_Y_DIR: i32 = 0x1E;
    pub const PAD_FLAG: i32 = 0x1F;
    pub const CMD_CAT1: i32 = 0x20;
    pub const CMD_CAT2: i32 = 0x21;
    pub const CMD_CAT3: i32 = 0x22;
    pub const CMD_CAT4: i32 = 0x23;
    // 0x24
    // 0x25
    // 0x26
    // 0x27
    // 0x28 some substatus
    pub const DASH_CALLBACK: i32 = 0x29;
    // 0x2A
    pub const CUSTOM_ROUTINE: i32 = 0x2B;
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
    pub const DAMAGE_MOTION_KIND_CALLBACK: i32 = 0x42;
    pub const DASH_POST_TRANSITION_CALLBACK: i32 = 0x57;
}

/*
WuBoy's VarModule Constant Overhaul!
The way our variable constants are labelled is changing.

Variables now have two categories:
INSTANCE, which persists until manually changed. Represented by 0x0XXX.
STATUS, which is automatically reset when the status changes. Represented by 0x1XXX.

In addition, there are two sub-categories.
Common, which is shared by every fighter. Represented by 0xX0XX.
Agent, which is specific to a certain fighter/agent. Represented by 0xX1XX.

This means for each combination, you have access to 256 common variables and 256 agent variables.
That's a LOT of space, and I don't anticipate it all gets used up with proper variable application.
*/

pub mod vars {
    pub mod common {
        pub mod instance {
            pub const HITSTUN_START: i32 = 0x0000;
            pub const IS_IN_HITSTUN: i32 = 0x0001;

            pub const CSTICK_OVERRIDE: i32 = 0x0002;
            pub const CSTICK_OVERRIDE_SECOND: i32 = 0x0003;

            pub const IS_TAP_JUMP: i32 = 0x0004;

            pub const OMNI_FLOAT: i32 = 0x0005;
            pub const AERIAL_NO_FLOAT: i32 = 0x0006;
            pub const FLOAT_PAUSE_AERIAL: i32 = 0x0007;

            pub const SIDE_SPECIAL_CANCEL: i32 = 0x0008;
            pub const UP_SPECIAL_CANCEL: i32 = 0x0009;

            pub const JAB_DA_CHECKS: i32 = 0x000A;
            pub const TILT_CHECKS: i32 = 0x000B;
            pub const AERIAL_CHECKS: i32 = 0x000C;
            pub const SMASH_CHECKS: i32 = 0x000D;

            pub const SPECIAL_STALL: i32 = 0x000E;
            pub const SPECIAL_STALL_USED: i32 = 0x000F;

            pub const ENABLE_AIR_ESCAPE_MAGNET: i32 = 0x0010;

            pub const DITCIT_SLIDING: i32 = 0x0011;

            pub const FOOTSTOOL_AIRDODGE_LOCKOUT: i32 = 0x0012;

            pub const CAN_ESCAPE_TUMBLE: i32 = 0x0013;

            pub const SPECIAL_WALL_JUMP: i32 = 0x0014;

            pub const TETHER_HOGGED: i32 = 0x0015;

            pub const B_REVERSED: i32 = 0x0016; // Converted for now, but will likely get removed when B Reverse Reimplementation happens

            pub const TUMBLE_KB: i32 = 0x0017;

            pub const CAN_GLIDE_TOSS: i32 = 0x0019;

            pub const IS_MOTION_BASED_ATTACK: i32 = 0x001A;

            pub const PREV_FLAG_DISABLE_ESCAPE_AIR: i32 = 0x001B;

            pub const ENABLE_WAVELAND_PLATDROP: i32 = 0x001C;

            pub const IS_DACUS: i32 = 0x001D;

            pub const IS_STICKY_WALK: i32 = 0x001E;
            pub const ENABLE_BOOST_RUN: i32 = 0x001F;

            pub const PERFECT_WAVEDASH: i32 = 0x0020;

            pub const JUMP_NEXT: i32 = 0x0021;

            pub const SHOULD_TRUMP_TETHER: i32 = 0x0022;

            pub const UP_SPECIAL_INTERRUPT: i32 = 0x0023; // Ness and Lucas use this
            pub const UP_SPECIAL_INTERRUPT_AIRTIME: i32 = 0x0024; // Ness and Lucas use this

            pub const SPECIAL_PROJECTILE_SPAWNED: i32 = 0x0025; // Luigi, Ivysaur, and Young Link use this
            pub const IS_TELEPORT_WALL_RIDE: i32 = 0x0026; // Mewtwo, Palutena, Sheik, and Zelda use this

            pub const STALL_PREVENTION: i32 = 0x0027; //Ness and Lucas down b stall prevention

            pub const SPIN_ATTACK_LAND_CANCEL: i32 = 0x003E; // Link and Mii Sword use this
            pub const SIDE_SPECIAL_CANCEL_NO_HIT: i32 = 0x004D; // Used by Kazuya and Sora

            pub const IS_LATE_PIVOT: i32 = 0x004E;
            pub const CAN_PERFECT_PIVOT: i32 = 0x004F;
            pub const IS_SMASH_TURN: i32 = 0x0050;

            pub const ENABLE_AIR_ESCAPE_JUMPSQUAT: i32 = 0x0051;

            pub const IS_KNOCKDOWN_THROW: i32 = 0x0052;

            pub const IS_HEAVY_ATTACK: i32 = 0x0053;

            pub const IS_CC_NON_TUMBLE: i32 = 0x0054;

            pub const IS_GETTING_POSITION_FOR_ECB: i32 = 0x0055;

            pub const CHECK_CHANGE_MOTION_ONLY: i32 = 0x0056;

            pub const BEFORE_GROUND_COLLISION: i32 = 0x0057;

            pub const IS_IGNORED_STATUS_FRAME_0: i32 = 0x0058;

            pub const FLUSH_EFFECT_ACMD: i32 = 0x0059;

            pub const IS_PARRY_FOR_GUARD_OFF: i32 = 0x0060;

            pub const TEMPORARY_CLIFF_STOP: i32 = 0x0061;

            pub const ENABLE_FRAME_DATA_DEBUG: i32 = 0x0062;

            pub const IS_ATTACK_CANCEL: i32 = 0x0063;

            pub const DISABLE_CSTICK_BUFFER_ROLL_OOS: i32 = 0x0064;

            // ints

            pub const LAST_ATTACK_RECEIVER_ENTRY_ID: i32 = 0x0000;

            pub const COSTUME_SLOT_NUMBER: i32 = 0x0001; // Unironically why does this need to exist? We have WorkModule.

            pub const FLOAT_TIMER: i32 = 0x0002;
            pub const FLOAT_DURATION: i32 = 0x0003;
            pub const FLOAT_STYLE: i32 = 0x0004;

            pub const HITFALL_BUFFER: i32 = 0x0005;

            pub const JUMP_SQUAT_FRAME: i32 = 0x0006;

            pub const GIMMICK_TIMER: i32 = 0x0007;

            pub const AIR_ESCAPE_MAGNET_FRAME: i32 = 0x0008;

            pub const CSTICK_LIFE: i32 = 0x0009;

            pub const AGT_USED_COUNTER: i32 = 0x000A;

            pub const CLIFF_XLU_FRAME: i32 = 0x000B;
            pub const LAST_ATTACK_HITBOX_ID: i32 = 0x000C;
            pub const SHIELD_EFFECT_HANDLE: i32 = 0x000D;

            pub const FRAME_COUNTER: i32 = 0x000E;

            pub const LEFT_STICK_FLICK_X: i32 = 0x000F;
            pub const LEFT_STICK_FLICK_Y: i32 = 0x0010;

            pub const LEDGE_ID: i32 = 0x0011;

            // floats

            pub const LAST_ATTACK_DAMAGE_DEALT: i32 = 0x0000;

            pub const CURRENT_MOMENTUM: i32 = 0x0001;
            pub const JUMPSQUAT_VELOCITY: i32 = 0x0002;
            /// This const is set in a fighter reset because the params used to calculate change depending on situation
            pub const JUMP_SPEED_RATIO: i32 = 0x0003;
            pub const DOUBLE_JUMP_FRAME: i32 = 0x0004;
            pub const GROUND_VEL: i32 = 0x0005; // Only ever gets set, goes effectively unused.
            pub const RAR_LENIENCY: i32 = 0x0006; // Only ever gets set, goes effectively unused.
            pub const CURRENT_MOMENTUM_SPECIALS: i32 = 0x0007;
            pub const DOUBLE_JUMP_TIMER: i32 = 0x0008; // Only used by Lucas, and it's commented out, goes unused.
            pub const ROLL_SPEED: i32 = 0x0009;
            // pub const LEDGE_POS: i32 = 0x000A;
            // pub const LEDGE_POS_X: i32 = 0x000A;
            // pub const LEDGE_POS_Y: i32 = 0x000B;
            // pub const LEDGE_POS_Z: i32 = 0x000C;
            pub const GET_DIST_TO_FLOOR: i32 = 0x000D;
            pub const ECB_BOTTOM_Y_OFFSET: i32 = 0x000E;
            pub const CURR_DASH_SPEED: i32 = 0x000F;
            pub const MOONWALK_SPEED: i32 = 0x0010;
            pub const ESCAPE_AIR_SLIDE_SPEED_X: i32 = 0x0011;
            pub const ESCAPE_AIR_SLIDE_SPEED_Y: i32 = 0x0012;
            pub const Y_POS: i32 = 0x0013;
            /// this multiplier can be set to a value between 0.1 and 3.0 to increase
            /// a character's jump speed max for momentum transfer (for meta quick, etc)
            pub const JUMP_SPEED_MAX_MUL: i32 = 0x0014;
            pub const ECB_TOP_Y_OFFSET: i32 = 0x0015;
            pub const LAST_ATTACK_HIT_LOCATION: i32 = 0x0016;
            pub const LAST_ATTACK_HIT_LOCATION_X: i32 = 0x0016;
            pub const LAST_ATTACK_HIT_LOCATION_Y: i32 = 0x0017;
            pub const LAST_ATTACK_HIT_LOCATION_Z: i32 = 0x0018;
            pub const ECB_CENTER_Y_OFFSET: i32 = 0x0019;
            pub const DASH_HIP_OFFSET_X: i32 = 0x0020;
            pub const RUN_HIP_OFFSET_X: i32 = 0x0021;
            pub const DACUS_TRANSITION_SPEED: i32 = 0x0022;
        }
        pub mod status {
            // flags
            pub const FAF_REACHED: i32 = 0x10FD;
            pub const PREV_AUTOCANCEL_FLAG: i32 = 0x10FE;
            pub const DISABLE_ECB_SHIFT: i32 = 0x10FF;

            pub const IS_DASH_TO_RUN_FRAME: i32 = 0x1000;
            pub const IS_AFTER_DASH_TO_RUN_FRAME: i32 = 0x1001;
            pub const APPLY_DASH_END_SPEED_MUL: i32 = 0x1002;

            pub const ATTACK_DASH_CANCEL_DISABLE: i32 = 0x1000;
            pub const ATTACK_DASH_ENABLE_AIR_FALL: i32 = 0x1001;
            pub const ATTACK_DASH_ENABLE_AIR_CONTINUE: i32 = 0x1002;
            pub const ATTACK_DASH_ENABLE_AIR_DRIFT: i32 = 0x1003;
            pub const ATTACK_DASH_AIR_DRIFT_ENABLED: i32 = 0x1004;
            pub const ATTACK_DASH_ENABLE_AIR_LANDING: i32 = 0x1005;

            pub const SHOULD_WAVELAND: i32 = 0x1000;

            pub const IS_JAB_LOCK_ROLL: i32 = 0x1000;
            pub const IS_SPIKE: i32 = 0x1001;
            pub const DAMAGE_FLY_RESET_TRIGGER: i32 = 0x1002;

            pub const SUICIDE_THROW_CAN_CLATTER: i32 = 0x1000;

            pub const ENABLE_UCF: i32 = 0x1000;

            pub const PUMMEL_OVERRIDE_GLOBAL_STATS: i32 = 0x1000;

            pub const CSTICK_IRAR: i32 = 0x1000;

            // ints

            pub const DOWN_STAND_FB_KIND: i32 = 0x1000;

            // floats

            pub const INITIAL_KNOCKBACK_VEL_X: i32 = 0x1000;
            pub const INITIAL_KNOCKBACK_VEL_Y: i32 = 0x1001;

            pub const RESTING_HIP_OFFSET_Y: i32 = 0x1000;

            pub const TELEPORT_INITIAL_SPEED_Y: i32 = 0x1000;
        }
    }

    pub mod bayonetta {
        pub mod instance {
            // flags
            pub const IS_HIT: i32 = 0x0100;

            // ints
            pub const NUM_RECOVERY_RESOURCE_USED: i32 = 0x0100;
            pub const FAIR_STATE: i32 = 0x0101;
            pub const SPECIAL_S_FRAME: i32 = 0x0102;
        }
        pub mod status {
            // floats
            pub const ABK_ANGLE: i32 = 0x1100;
        }
    }

    pub mod brave {
        pub mod instance {
            // flags
            pub const PERSIST_RNG: i32 = 0x0100;
            pub const PSYCHE_UP_ACTIVE: i32 = 0x0101;
            pub const UP_SPECIAL_FREEFALL: i32 = 0x0102;

            // ints
            pub const SPELL_SLOT_1: i32 = 0x0100;
            pub const SPELL_SLOT_2: i32 = 0x0101;
            pub const SPELL_SLOT_3: i32 = 0x0102;
            pub const SPELL_SLOT_4: i32 = 0x0103;
            pub const SPELL_SLOT_USED_1_1: i32 = 0x0104;
            pub const SPELL_SLOT_USED_1_2: i32 = 0x0105;
            pub const SPELL_SLOT_USED_1_3: i32 = 0x0106;
            pub const SPELL_SLOT_USED_1_4: i32 = 0x0107;
            pub const SPELL_SLOT_USED_2_1: i32 = 0x0108;
            pub const SPELL_SLOT_USED_2_2: i32 = 0x0109;
            pub const SPELL_SLOT_USED_2_3: i32 = 0x0110;
            pub const SPELL_SLOT_USED_2_4: i32 = 0x0111;
            pub const CURSOR_SLOT: i32 = 0x0112;
        }
    }

    pub mod buddy {
        pub mod instance {
            // flag
            pub const BEAKBOMB_ACTIVE: i32 = 0x0100;
            pub const BAYONET_ACTIVE: i32 = 0x0101;
            pub const FLUTTER_ENABLED: i32 = 0x0102;
            pub const UP_SPECIAL_FREEFALL: i32 = 0x0103;

            // int
            pub const HUD_DISPLAY_TIME: i32 = 0x0100;
            //Current frame of Beakbomb, used to detect mislanding
            pub const BEAKBOMB_FRAME: i32 = 0x0101;
            // 0: Normal Bounce (can be cancelled) 1: weak bounce 2: heavy bounce.
            pub const BEAKBOMB_BOUNCE: i32 = 0x0102;
            //Eggs fired gets reset when entering Bayonet, so we have to temporarily store current eggs fired
            pub const BAYONET_EGGS: i32 = 0x0103;

            // float
            pub const FEATHERS_RED_COOLDOWN: i32 = 0x0100;
            pub const BEAKBOMB_ANGLE: i32 = 0x0101;
        }
        pub mod status {
            // flags
            pub const IS_BURY_DTHROW: i32 = 0x1100;
        }
    }

    pub mod captain {
        pub mod status {
            pub const YES: i32 = 0x1100;
        }
    }

    pub mod chrom {
        pub mod instance {
            // flags
            pub use super::super::roy::instance::TRAIL_EFFECT;
            pub const SOARING_SLASH_HIT: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SOARING_SLASH_CANCEL: i32 = 0x1101;
        }
    }
    pub mod dedede{
        pub mod instance{
            //flags
            pub const CAN_WADDLE_DASH_FLAG: i32 = 0x0100;
            pub const IS_DASH_GORDO: i32 = 0x0101;
            pub const IS_ANGLED_FLAG: i32 = 0x0102;
            pub const IS_REMOVED_FLAG: i32 = 0x0103;

            //ints
            pub const RECATCH_COUNTER: i32 = 0x0104;

            //floats
            pub const INHALE_STICK_Y: i32 = 0x0105;
        }
    }
    pub mod demon {
        pub mod instance {
            // flags
            pub const SLAUGHTER_HIGH_KICK: i32 = 0x0100;
            pub const DEVASTATOR: i32 = 0x0101;
            pub const JAW_BREAKER: i32 = 0x0102;
            pub const SLICING_BLADE: i32 = 0x0103;
            pub const SPINNING_DEMON: i32 = 0x0104;
            pub const LIGHTNING_SCREW_UPPERCUT: i32 = 0x0105;
            pub const TWIN_FANG_DOUBLE_KICK: i32 = 0x0106;
            pub const UP_SPECIAL_FREEFALL: i32 = 0x0107;
        }
    }

    pub mod diddy {
        pub mod instance {
            pub const DISABLE_SPECIAL_S: i32 = 0x0100;
            pub const NO_CAP: i32 = 0x0101;
        }
        pub mod status {
            // ints
            pub const SPECIAL_N_CANCEL_TYPE: i32 = 0x1100;
        }

        pub const SPECIAL_N_CANCEL_TYPE_NONE: i32 = 0x0;
        pub const SPECIAL_N_CANCEL_TYPE_GROUND_JUMP: i32 = 0x1;
        pub const SPECIAL_N_CANCEL_TYPE_JUMP_AERIAL: i32 = 0x2;
        pub const SPECIAL_N_CANCEL_TYPE_GUARD: i32 = 0x3;
        pub const SPECIAL_N_CANCEL_TYPE_ESCAPE: i32 = 0x4;
        pub const SPECIAL_N_CANCEL_TYPE_ESCAPE_AIR: i32 = 0x5;
        pub const SPECIAL_N_CANCEL_TYPE_ESCAPE_F: i32 = 0x6;
        pub const SPECIAL_N_CANCEL_TYPE_ESCAPE_B: i32 = 0x7;
    }

    // Note: Terry starts his flags on 0xXX5X instead due to also using the shotos generic flags.
    pub mod dolly {
        pub mod instance {
            // ints
            pub const METER_STOCKS: i32 = 0x0150;
            pub const CURRENT_STOCKS: i32 = 0x0151;

            // flags
            pub const SUPER_CANCEL: i32 = 0x0150;
            pub const DISABLE_SPECIAL_S: i32 = 0x0151;
            pub const IS_INIT_METER: i32 = 0x0152;
            pub const INCREASE_METER_STOCKS: i32 = 0x0153;
        }
        pub mod status {
            // flags
            pub const IS_USE_FIRE_KICK: i32 = 0x1150;
            pub const UNABLE_CANCEL_S3_DASH: i32 = 0x1151;
            pub const IS_CHAIN_CANCEL: i32 = 0x1152;
            pub const IS_SHATTER_STRIKE: i32 = 0x1153;
            pub const AIR_SPECIAL_F: i32 = 0x1154;
        }
    }

    pub mod donkey {
        pub mod instance {
            // flags
            pub const SPECIAL_AIR_LW_USED_STALL: i32 = 0x0100;
            /// used to indicate that a new barrel was spawned
            pub const DID_SPAWN_BARREL: i32 = 0x0101;
        }
        pub mod status {
            // flags
            pub const SPECIAL_CHECKS: i32 = 0x1100;
            pub const IS_NEUTRAL_TOSS: i32 = 0x1101;

            // pub const SPECIAL_AIR_LW_STOP: i32 = 0x1100;
        }
    }

    pub mod duckhunt {
        pub mod instance {
            // int
            pub const GUNMAN_TIMER: i32 = 0x0100;
        }
        pub mod status {
            // flag
            pub const CLAY_SMASH_INPUT: i32 = 0x1100;
        }
    }
    pub mod peach {
        pub mod instance {
            // flag
            // Used to check if sideb wall bounce happens
            pub const IS_WALLBOUNCE: i32 = 0x0100;
            pub const DISABLE_SPECIAL_S: i32 = 0x0101;
        }
    }
    pub mod daisy {
        pub mod instance {
            // flag
            // Used to check if sideb wall bounce happens
            pub const IS_WALLBOUNCE: i32 = 0x0100;
            pub const DISABLE_SPECIAL_S: i32 = 0x0101;
        }
    }

    pub mod rosetta {
        pub mod instance {
            // ints
            pub const COOLDOWN: i32 = 0x0100;
            pub const ROSA_X: i32 = 0x0101;
            pub const ROSA_Y: i32 = 0x0102;
            pub const TICO_X: i32 = 0x0103;
            pub const TICO_Y: i32 = 0x0104;
            pub const TICO_RAYCAST: i32 = 0x0106;
            pub const TICO_X_DIST: i32 = 0x0107;
            pub const TICO_Y_DIST: i32 = 0x0108;

            // flags
            pub const IS_TICO_UNAVAILABLE: i32 = 0x0100;
        }
        pub mod status {
            // ints
            /// Used for determining what luma does
            pub const LUMA_STATE: i32 = 0x1100;

            // flags
            pub const IS_INVALID_TELEPORT: i32 = 0x1101;
            pub const GROUNDED_TELEPORT: i32 = 0x1102;
        }
    }

    pub mod edge {
        pub mod instance {
            // flags
            pub const SPECIAL_HI_BLADE_DASH_NO_HITBOX: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const FLASH_HOLD: i32 = 0x1101;
            pub const SPECIAL_N_FIRE: i32 = 0x1102;
        }
    }

    pub mod eflame {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_HI: i32 = 0x0104;
        }
    }

    pub mod elight {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_HI:             i32 = 0x0100;
            pub const DISABLE_SPECIAL_S:              i32 = 0x0101;
            pub const ENABLE_SPECIAL_S_ACTIONABILITY: i32 = 0x0102;
            pub const UP_SPECIAL_FREEFALL:            i32 = 0x0103;
        }
        pub mod status {
            // ints
            /// This is used to determine how to end the SpecialHiJump status script
            pub const SPECIAL_HI_JUMP_RESERVE_ACTION: i32 = 0x1100;
        }

        // not IDs but symbolic consts
        pub const SPECIAL_HI_JUMP_RESERVE_ACTION_ATTACK1: i32 = 0x0;
        pub const SPECIAL_HI_JUMP_RESERVE_ACTION_ATTACK2: i32 = 0x1;
        pub const SPECIAL_HI_JUMP_RESERVE_ACTION_FALL: i32 = 0x2;
    }

    pub mod falco {
        pub mod instance {
            // flags
            pub const SPECIAL_LW_DISABLE_STALL: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SET_ATTACK: i32 = 0x1100;
            pub const SET_EFFECT: i32 = 0x1101;

            // ints
            pub const SPECIAL_LW_STOP_Y_FRAME: i32 = 0x1100;
        }
    }

    pub mod gamewatch {
        pub mod instance {
            // flags
            pub const UP_SPECIAL_FREEFALL: i32 = 0x0100;
            pub const UP_SPECIAL_PARACHUTE: i32 = 0x0101;
        }
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
            pub const FLOAT_GROUND_DECIDE_ANGLE: i32 = 0x1103;
            pub const FLOAT_GROUND_CHANGE_KINETIC: i32 = 0x1104;
        }
    }

    pub mod gaogaen {
        pub mod instance {
            // flags
            pub const IS_SPECIAL_S_ALTERNATE_GRAB: i32 = 0x0100;
            pub const IS_SPECIAL_S_GROUND_GRAB: i32 = 0x0101;
            pub const IS_SPECIAL_S_AIR_GRAB: i32 = 0x0102;
        }
        pub mod status {
            // flags
            pub const IS_HIT_SPECIAL_HI_RISE: i32 = 0x1100;
            pub const IS_INPUT_CROSS_CHOP_CANCEL: i32 = 0x1101;
            pub const SHOULD_CROSS_CHOP_DIVE_EARLY: i32 = 0x1102;

            // ints
            pub const SPECIAL_N_STRENGTH_LEVEL: i32 = 0x1100;

            // floats
            pub const ANGLE_GRAB_STICK_Y: i32 = 0x1100;
        }
    }

    pub mod ike {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_S: i32 = 0x0100;

        }
        pub mod status {
            // flags
            pub const IS_QUICK_DRAW_INSTAKILL: i32 = 0x1100;
        }
    }

    pub mod inkling {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_S: i32 = 0x0100;
        }
        pub mod status {
            // flags

        }
    }

    pub mod jack {
        pub mod instance {
            // flags
            pub const GROUNDED_DOYLE_DASH: i32 = 0x0100;
        }
    }

    pub mod kamui {
        pub mod status {
            // floats
            pub const CURRENT_CHARGE: i32 = 0x1100;
        }
    }

    pub mod kirby {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_HI: i32 = 0x01FF; //Weird value to avoid conflicts with copy ability values
        
            // copy ability
            // flags
            pub const SHOULD_CYCLE_MATERIAL: i32 = 0x01F4;
            pub use super::super::ridley::instance::SPECIAL_N_EXPLODE;

            // ints 
            pub const MATERIAL_INDEX: i32 = 0x01F5;
        }
        pub mod status {
            // copy ability
            // flags
            pub use super::super::mario::status::FIREBRAND;
            pub use super::super::luigi::status::THUNDERHAND;
            pub use super::super::mariod::status::CHILL_PILL;
            pub const MINING_TIMER: i32 = 0x11F4;
            pub const KO_PUNCH_GRAVITY: i32 = 0x11F5;
            pub const KO_PUNCH_GRAVITY_END: i32 = 0x11F6;
        }
    }

    pub mod koopa {
        pub mod instance {
            // flags
            pub const IS_EXCELLENT_PUNCH: i32 = 0x0100; // determines if forward smash is an excellent punch

            // ints
            pub const FIREBALL_COOLDOWN_FRAME: i32 = 0x0100;
            pub const FIREBALL_EFFECT_ID: i32 = 0x0101;
            pub const CHARGE_EFFECT_HANDLER: i32 = 0x0102;
        }
        pub mod status {
            // flags
            pub const PUNCH_CAN_ZOOM: i32 = 0x1101; // flag for controlling the zoom opff so it only runs once on impact
        }
    }

    pub mod koopajr {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_S: i32 = 0x0100;
            pub const DISABLE_MECHAKOOPA: i32 = 0x0101;
            pub const MECHAKOOPA_IS_COOLDOWN: i32 = 0x0102;
        }
    }

    pub mod lucario {
        pub mod instance {
            // flags
            pub const METER_IS_BURNOUT: i32 = 0x0100;
            pub const IS_SPECIAL_LW_AIR: i32 = 0x0101;
            pub const DISABLE_SPECIAL_LW: i32 = 0x0102;
            pub const IS_POWERED_UP: i32 = 0x0103;
            pub const IS_USPECIAL_ATTACK_CANCEL: i32 = 0x0104;

            // ints
            pub const METER_PAUSE_REGEN_FRAME: i32 = 0x0100;

            // float
            pub const METER_PASSIVE_RATE: i32 = 0x0100;
        }
        pub mod status {
            // ints
            pub const FORCE_PALM_ROT_ANGLE: i32 = 0x1100;

            // floats
            pub const SPECIAL_N_ANGLE: i32 = 0x1100;
            pub const AURA_OVERRIDE: i32 = 0x1101;
        }
    }
    pub mod lucas {

        pub mod instance {
            // flag
            pub const SPECIAL_N_OFFENSE_UP_ACTIVE: i32 = 0x01EF;
            pub const SPECIAL_N_OFFENSE_UP_RELEASE_AFTER_WHIFF: i32 = 0x01EE;
            pub const SPECIAL_N_OFFENSE_UP_INIT: i32 = 0x01ED;
            pub const ATTACK_S4_ANGLE_DOWN: i32 = 0x0103;
            pub const ATTACK_S4_ANGLE_UP: i32 = 0x0104;

            // int
            pub const SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE1: i32 = 0x01DF;
            pub const SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE2: i32 = 0x01DE;
            pub const SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL: i32 = 0x01DD;
            pub const SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE3: i32 = 0x01DC;
        }

        pub mod status {
            // flag
            pub const SPECIAL_HI_ATTACK_IS_TOUCH_WALL: i32 = 0x1100;
            pub const SPECIAL_HI_ATTACK_IS_LEAVE_WALL: i32 = 0x1101;
            pub const SPECIAL_HI_ATTACK_IS_FLIPPED_MOMENTUM_AFTER_WALLTOUCH: i32 = 0x1102;
            pub const SPECIAL_HI_ATTACK_IS_SET_WALL_LEAVE_MOMENTUM: i32 = 0x1103;
            pub const THUNDER_LOOSE: i32 = 0x1104;

            // float
            /// Holds the initial y velocity you have during up b to compare once you've touched a wall to reflect your speed the other direction if it changes
            pub const SPECIAL_HI_ATTACK_Y_MOMENTUM: i32 = 0x1100;
            pub const SPECIAL_HI_ATTACK_Y_INIT_MOMENTUM: i32 = 0x1101;
            pub const SPECIAL_HI_ATTACK_Y_DIRECTION: i32 = 0x1102;
            pub const SPECIAL_HI_ATTACK_X_MOMENTUM: i32 = 0x1103;
            pub const SPECIAL_HI_ATTACK_X_INIT_MOMENTUM: i32 = 0x1104;
            pub const SPECIAL_HI_ATTACK_X_DIRECTION: i32 = 0x1105;
            pub const SPECIAL_HI_ATTACK_WALL_TOUCH_FRAME: i32 = 0x1106;
            pub const SPECIAL_HI_ATTACK_WALL_LEAVE_FRAME: i32 = 0x1107;
        }
    }

    pub mod lucina {
        pub mod instance {
            // flag
            pub const EQUIP_MASK: i32 = 0x0100;
            
            // int
            /// This int stores damage received from an attack during quick riposte
            pub const CURRENT_DAMAGE: i32 = 0x0100;
        }

        pub mod status {
            // int
            pub const SPECIAL_LW_MOTION: i32 = 0x1100;
            pub const SPECIAL_LW_MOTION_AIR: i32 = 0x1101;

            // flag
            pub const SPECIAL_LW_SPECIAL_CHECK: i32 = 0x1100;
        }
    }

    pub mod luigi {
        pub mod instance {
            // flag
            /// This flag stores whether or not Luigi currently has a misfire stored.
            pub const IS_MISFIRE_STORED: i32 = 0x0100;
            pub const TRAINING_ALWAYS_MISFIRES: i32 = 0x0101;
            // int
            /// This int stores the number of remaining green missile's luigi must do before getting a misfire
            pub const REMAINING_SPECIAL_S_UNTIL_MISFIRE: i32 = 0x0100;
            /// This int stores the handle of the charge smoke effect for killing it if we store misfire
            pub const CHARGE_SMOKE_EFFECT_HANDLE: i32 = 0x0101;
            /// This int stores the handle of the pulsing effect for killing it if we store misfire
            pub const CHARGE_PULSE_EFFECT_HANDLE: i32 = 0x0102;

            // float
            /// This float holds the current multiplier on damage for misfire
            pub const MISFIRE_DAMAGE_MULTIPLIER: i32 = 0x0100;
        }
        pub mod status {
            // flag
            pub const THUNDERHAND: i32 = 0x1100;
        }
    }

    pub mod mario {
        pub mod instance {
            // flags
            pub const DISABLE_DSPECIAL_STALL: i32 = 0x0100;
            pub const SPECIAL_S_DISABLE_STALL: i32 = 0x0101;
        }

        pub mod status {
            // flags
            pub const AERIAL_COMMAND_MOMENTUM_RESET: i32 = 0x1100;
            pub const AERIAL_COMMAND_RISING: i32 = 0x1101;
            pub const AERIAL_COMMAND_RISEN: i32 = 0x1102;

            pub const FIREBRAND: i32 = 0x1100;
        }
    }

    pub mod yoshi {
        pub mod status {
            pub use super::super::mario::status::{
                AERIAL_COMMAND_MOMENTUM_RESET, AERIAL_COMMAND_RISEN, AERIAL_COMMAND_RISING,
            };
        }
    }

    pub mod master {
        pub mod instance {
            // flags
            pub const SPECIAL_AIR_HI_CATCH: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const IS_ENABLE_SPECIAL_S_DASH_CANCEL: i32 = 0x1100;
            pub const AIR_SPECIAL_S_AUTOCANCEL: i32 = 0x1101;

            // ints
            pub const AYMR_CHARGE_LEVEL: i32 = 0x1100;
        }
    }

    pub mod mewtwo {
        pub mod instance {
            // flags
            pub const GROUNDED_TELEPORT: i32 = 0x0100;
            pub const UP_SPECIAL_JUMP_REFRESH: i32 = 0x0101;
            pub const UP_SPECIAL_FREEFALL: i32 = 0x0102;
        }
    }

    pub mod ness {
        pub mod status {
            // flags
            pub const THUNDER_LOOSE: i32 = 0x1100;
        }
    }

    pub mod pickel {
        pub mod instance {
            // flags
            pub const IS_CURRENT_ATTACK_LW3_SOUL_FIRE: i32 = 0x0100;
            pub const DISABLE_SPECIAL_S: i32 = 0x0103;
            pub const CAN_RESPAWN_TABLE: i32 = 0x0104;
            pub const SHOULD_CYCLE_MATERIAL: i32 = 0x0105;
            pub const SHOULD_RESET_ROT: i32 = 0x0106;

            // ints 
            pub const MATERIAL_INDEX: i32 = 0x0100;
            pub const MATERIAL_EFFECT_HANDLER: i32 = 0x0101;
            pub const HITSTUN_TIMER: i32 = 0x0102;

            //floats
            pub const FORGE_START_Y_POS: i32 = 0x0106;
            pub const DAMAGE_TRACKER: i32 = 0x0107;
            pub const TABLE_HP_TRACKER: i32 = 0x0108;
        }
        pub mod status {
            // ints
            pub const MINING_TIMER: i32 = 0x1100;
            
            // floats
            pub const GLIDE_TIMER: i32 = 0x1100;
        }
    }

    pub mod pikachu {
        pub mod instance {
            // flags
            pub const DISABLE_QA_JC: i32 = 0x0100;
            pub const QUICK_ATTACK_CANCEL: i32 = 0x0101;
        }
    }

    pub mod pikmin {
        pub mod instance {
            // flags
            pub const SPECIAL_HI_CANCEL_ESCAPE_AIR: i32 = 0x0100;
            pub const SPECIAL_S_PIKMIN_DETONATE_IS_DETACH_FOR_DETONATE: i32 = 0x0101;
        }
        pub mod status {
            // flags
            pub const SPECIAL_S_PIKMIN_DETONATE_IS_ATTACK_LAST_FRAME: i32 = 0x1100;
            // ints
            pub const SPECIAL_S_PIKMIN_DETONATE_TIMER: i32 = 0x1101;
        }
    }

    pub mod ptrainer {
        pub mod instance {
            pub const IS_SWITCH_BACKWARDS: i32 = 0x0100;
        }
    }

    pub mod plizardon {
        pub mod instance {
            pub const DISABLE_SPECIAL_S: i32 = 0x0100;
        }
    }

    pub mod pzenigame {
        pub mod instance {

        }
    }

    pub mod mariod {
        pub mod instance {
            // flags
            pub const UP_SPECIAL_CANCEL: i32 = 0x0100;
            pub const SPECIAL_S_DISABLE_STALL: i32 = 0x0101;
        }
        pub mod status {
            // flags
            pub const CHILL_PILL: i32 = 0x1100;
        }
    }

    pub mod reflet {
        pub mod status {
            // flags
            pub const ELWIND1_CANCEL: i32 = 0x1100;
        }
        pub mod instance {
            //flags
            pub const THUNDER_CHARGE: i32 = 0x0100;
            pub const UP_SPECIAL_FREEFALL: i32 = 0x0101;
        }
    }

    pub mod rockman {
        pub mod instance {
            // flags
            pub const CHARGE_SHOT_CHARGING : i32 = 0x0100;
            pub const CHARGE_SHOT_PLAYED_FX : i32 = 0x0101;
            pub const CHARGE_SHOT_RELEASE : i32 = 0x0102;
            pub const UP_SPECIAL_FREEFALL : i32 = 0x0103;
            
            // ints
            pub const CHARGE_SHOT_FRAME : i32 = 0x0100;
            pub const CHARGE_SHOT_EFF_HANDLE : i32 = 0x0101;
            pub const CHARGE_SHOT_SND_HANDLE : i32 = 0x0102;
            pub const CHARGE_SHOT_RELEASE_FRAME : i32 = 0x0103;
        }
        pub mod status {
            // flags
            pub const CHARGE_SHOT_KEEP_CHARGE : i32 = 0x1100;
        }
    }

    pub mod rockman_airshooter {
        pub mod status {
            // flags
            pub const MOVE : i32 = 0x1100;
        }
    }

    pub mod roy {
        pub mod instance {
            // flags
            pub const TRAIL_EFFECT: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SIDE_B_REVERSE : i32 = 0x1150;
        }
    }

    pub mod shotos {
        pub mod instance {
            // flags
            pub const IS_USE_EX_SPECIAL: i32 = 0x0100;
            pub const IS_MAGIC_SERIES_CANCEL: i32 = 0x0101;
            pub const IS_ENABLE_FADC:         i32 = 0x0102;
            pub const IS_TARGET_COMBO_1:      i32 = 0x0103;
            pub const IS_TARGET_COMBO_2:      i32 = 0x0104;
            pub const IS_CURRENT_HADOKEN_EX:  i32 = 0x0105;
            pub const DISABLE_SPECIAL_S:      i32 = 0x0106;
            pub const IS_CURRENT_HADOKEN_AIR: i32 = 0x0107;
            pub const DISABLE_SPECIAL_LW:     i32 = 0x0108;

            // ints
            pub const REPEAT_COUNT_LW: i32 = 0x0100;
            pub const REPEAT_COUNT_HI: i32 = 0x0101;
            pub const EX_SPECIAL_SCRIPTING: i32 = 0x0102;
            pub const AIR_CHAIN_COMBO_NUM: i32 = 0x0103;
        }
        pub mod status {
            // flags
            pub const SHOULD_COMBOS_SCALE: i32 = 0x1100;
            pub const REPEAT_INCREMENTED: i32 = 0x1101;
            pub const IS_ENABLE_MAGIC_SERIES_CANCEL: i32 = 0x1102;
        }
    }

    pub mod shizue {
        pub mod instance {
            // flags
            pub const LLOID_ASYNC: i32 = 0x0100;

            // floats
            pub const STORED_BALLOON_POWER: i32 = 0x0100;

            // ints
            pub const LLOID_TIMER: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const IS_NOT_QUICK_RELEASE: i32 = 0x1100;
            pub const IS_DETACH_BOOST: i32 = 0x1101;
        }
    }

    pub mod simon {
        pub mod status {
            
        }
    }

    pub mod sonic {
        pub mod instance {
            // flags
            pub const USED_AIR_ACTION: i32 = 0x0100;
            pub const UP_SPECIAL_FREEFALL: i32 = 0x0101;

            // ints
            pub const SPECIAL_N_POSE: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SPECIAL_N_BLAST_ATTACK: i32 = 0x1100;

            pub const PULSE_HITBOX: i32 = 0x1100;

            pub const SPECIAL_S_HOP: i32 = 0x1100;
            pub const SPECIAL_S_ENABLE_JUMP: i32 = 0x1101;
            pub const SPECIAL_S_ENABLE_CONTROL: i32 = 0x1102;

            // ints
            pub const SPECIAL_S_STEP: i32 = 0x1100;
        }
        pub const SPECIAL_S_STEP_START: i32 = 0x0;
        pub const SPECIAL_S_STEP_DASH: i32 = 0x1;
        pub const SPECIAL_S_STEP_END: i32 = 0x2;
    }

    pub mod snake {
        pub mod instance {
            // flags
            pub const SELF_STICK: i32 = 0x0100;
            pub const KNIFE_COMBO_ENABLE: i32 = 0x0101;
            pub const KNIFE_COMBO_IS_BUFFERED: i32 = 0x0102;
            pub const DTAUNT_C4_EXLPODE: i32 = 0x0103;
            pub const DTAUNT_GRENADE_WAIT_COUNT: i32 = 0x0104;
            pub const IS_GRAB_WALK: i32 = 0x0105;
            pub const TRANQ_RELOAD_VULNERABLE: i32 = 0x0106;
            pub const TRANQ_NEED_RELEOAD: i32 = 0x0107;

            // ints
            pub const SNAKE_GRENADE_COUNTER: i32 = 0x0100;
            pub const KNIFE_COMBO_COUNT: i32 = 0x0101;
            pub const TRANQ_AMMO_COUNT: i32 = 0x0102;
        }
    }

    pub mod szerosuit {
        pub mod status {
            // flags
            pub const ATTACK_AIR_LW_REBOUND: i32 = 0x1100;
            pub const SPECIAL_LW_MANUAL_FLIPSTOOL_ENABLE: i32 = 0x1101;
        }
    }

    pub mod trail {
        pub mod instance {
            // flags
            pub const ATTACK_12_INTO_S3: i32 = 0x0100;
            pub const COMBO_PLUS_GROUND: i32 = 0x0101;
            pub const COMBO_PLUS_AIR: i32 = 0x0102;
            pub const CYCLE_MAGIC: i32 = 0x0103; // used to properly cycle Sora's HUD to fire in training mode
            pub const ATTACK_LW4_REBOUND: i32 = 0x0104;

            // floats
            pub const JUMP_CANCEL_MOMENTUM_HANDLER: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SHOULD_PRORATE_ATTACK: i32 = 0x1100;

            pub const IS_LAND_CANCEL_THUNDER: i32 = 0x1100;
            pub const IS_GRAND_MAGIC: i32 = 0x1101;
            pub const IS_CURRENT_FIRAGA_GRAND_MAGIC: i32 = 0x1102;
            pub const IS_CURRENT_BLIZZAGA_GRAND_MAGIC: i32 = 0x1103;
            pub const IS_CURRENT_THUNDAGA_GRAND_MAGIC: i32 = 0x1104;

            pub const SIDE_SPECIAL_HIT: i32 = 0x1100;
            pub const IS_SIDE_SPECIAL_INPUT: i32 = 0x1101;
            pub const STOP_SIDE_SPECIAL: i32 = 0x1102;
            pub const UP_SPECIAL_TO_SIDE_SPECIAL: i32 = 0x1103;

            pub const UP_SPECIAL_HIT: i32 = 0x1100;

            // ints
            pub const ATTACK_LW4_TIMER: i32 = 0x1100;

            // floats
            pub const SONIC_BLADE_Y: i32 = 0x1100;
            pub const DACUS_SPEED_Y: i32 = 0x1101;
        }
    }

    pub mod samus {
        pub mod instance {
            // flags
            pub const SHINESPARK_USED: i32 = 0x0100;
            pub const SHINESPARK_READY: i32 = 0x0101;
        }
    }

    pub mod samusd {
        pub mod instance {
            // flags
            pub const MANUAL_DETONATE_READY: i32 = 0x0100;

            // ints
            pub const BOMB_OBJECT_ID: i32 = 0x0100;
        }
    }

    pub mod robot {
        pub mod instance {
            // flags
            pub const AIRTIME_BAIR: i32 = 0x0100;
            pub const AIRTIME_SIDEB: i32 = 0x0101;
            pub const IS_INIT_METER: i32 = 0x0102;
            pub const GROUNDED_UPB: i32 = 0x0103;
            pub const UPB_CANCEL: i32 = 0x0104;
            // ints
            pub const PASSIVE_FUEL_INDICATOR_EFFECT_HANDLE: i32 = 0x0100;
            pub const PREV_FUEL_THRESHOLD: i32 = 0x0101;
            // floats
            pub const STICK_ANGLE: i32 = 0x0100;
            pub const FRAMES_SINCE_UPB: i32 = 0x0101;
            pub const FRAMES_SINCE_UPB_RISE: i32 = 0x0102;
            pub const JOINT_ROT: i32 = 0x1103;
        }
        pub mod status {
            // flags
            pub const IS_CHARGE_STARTED: i32 = 0x1100;
            pub const IS_CHARGE_FINISHED: i32 = 0x1101;
            pub const IS_CHARGE_MAX: i32 = 0x1102;
            pub const CHARGE_ATTACK_LEVEL: i32 = 0x1103;
            pub const HELD_BUTTON: i32 = 0x1104;
        }
    }

    pub mod palutena {
        pub mod status {
            // flags
            pub const CAN_INCREASE_COLOR: i32 = 0x1100;

            // floats
            pub const ADD_DAMAGE: i32 = 0x1100;
        }
        pub mod instance {
            // flags
            pub const GROUNDED_TELEPORT: i32 = 0x0100;
            pub const UP_SPECIAL_JUMP_REFRESH: i32 = 0x0101;
            pub const FLUSH: i32 = 0x0102;
            pub const POWERED: i32 = 0x0103;
            pub const UP_SPECIAL_FREEFALL: i32 = 0x0104;

            // ints
            pub const POWER_BOARD_SLOT_1: i32 = 0x0100;
            pub const POWER_BOARD_SLOT_2: i32 = 0x0101;
            pub const SET_COLOR: i32 = 0x0102;
        }
    }

    pub mod miiswordsman {
        pub mod instance {
            // flags
            pub const CHAKRAM_STICK_ATTACK: i32 = 0x0100;
            pub const SKYWARD_SLASH_DASH_HIT: i32 = 0x0101;

            // ints
            pub const SPECIAL_LW1_CHARGE_LEVEL: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const WAVE_SPECIAL_N: i32 = 0x1100;
            pub const GALE_STAB_EDGE_CANCEL: i32 = 0x1101;
            pub const SHOCK_SPELL_HOLD: i32 = 0x1102;
            pub const SPECIAL_LW1_ATTACK_TRIGGER: i32 = 0x1103;
        }
    }

    pub mod miigunner {
        pub mod status {
            // flags
            pub const BOOSTED_AERIAL: i32 = 0x1100;

            // floats
            pub const CURRENT_CHARGE: i32 = 0x1100;
            pub const MISSILE_DETONATE: i32 = 0x1101;
        }
        pub mod instance {
            // flags
            pub const LUNAR_LAUNCH_AIR_USED: i32 = 0x0100;
            pub const DETONATE_READY: i32 = 0x0101;
            pub const BOOSTED_DAIR_AIRTIME: i32 = 0x0102;

            // ints
            pub const LUNAR_LAUNCH_EFF_HANDLER: i32 = 0x0100;
            pub const MISSILE_OBJECT_ID: i32 = 0x0101;
            pub const STEALTHBOMB_EFF_HANDLER: i32 = 0x0102;

            // floats
            pub const GRENADE_CHARGE: i32 = 0x0102;
        }
    }

    pub mod metaknight {
        pub mod instance {
            // flags
            pub const NEUTRAL_SPECIAL_HIT: i32 = 0x0103;
            pub const SIDE_SPECIAL_HIT: i32 = 0x0104;
            pub const UP_SPECIAL_HIT: i32 = 0x0105;
            pub const DOWN_SPECIAL_HIT: i32 = 0x0106;
        }
    }

    pub mod miifighter {
        pub mod instance {
            pub const QUAKE_EFFECT_HANDLER: i32 = 0x0100;
        }
        pub mod status {
            // ints
            pub const SPECIAL_LW1_CHARGE: i32 = 0x1100;

            // floats
            pub const SPECIAL_LW1_CHARGE_DISTANCE: i32 = 0x1101;
        }
    }

    pub mod littlemac {
        pub mod instance {
            // flags
            pub const IS_DREAMLAND_EXPRESS: i32 = 0x0100;
            pub const IS_LATE_DLE_INPUT: i32 = 0x0101;
            pub const KO_MOTION_AIRTIME: i32 = 0x0102;

            // floats
            pub const CURRENT_DAMAGE: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const LIMIT_METER_GAIN: i32 = 0x102;

            // ints
            pub const SPECIAL_LW_CANCEL_TYPE: i32 = 0x1100;
        }

        pub const SPECIAL_LW_CANCEL_TYPE_NONE: i32 = 0x0;
        pub const SPECIAL_LW_CANCEL_TYPE_GROUND_JUMP: i32 = 0x1;
        pub const SPECIAL_LW_CANCEL_TYPE_JUMP_AERIAL: i32 = 0x2;
        pub const SPECIAL_LW_CANCEL_TYPE_GUARD: i32 = 0x3;
        pub const SPECIAL_LW_CANCEL_TYPE_ESCAPE: i32 = 0x4;
        pub const SPECIAL_LW_CANCEL_TYPE_ESCAPE_AIR: i32 = 0x5;
        pub const SPECIAL_LW_CANCEL_TYPE_ESCAPE_F: i32 = 0x6;
        pub const SPECIAL_LW_CANCEL_TYPE_ESCAPE_B: i32 = 0x7;
    }

    pub mod pichu {
        pub mod instance {
            //flags
            pub const IS_CHARGE_ATTACK: i32 = 0x0102;

            //ints
            pub const CHARGE_LEVEL: i32 = 0x0100;
            pub const CHARGE_EFFECT_HANDLER: i32 = 0x0101;

            // floats
            pub const CHARGE_DAMAGE_MUL: i32 = 0x0100;
            pub const CHARGE_RECOIL_MUL: i32 = 0x0101;
            pub const DISCHARGE_POWER_MUL: i32 = 0x0102;
            pub const DISCHARGE_SIZE_MUL: i32 = 0x0103;
        }
    }

    pub mod zelda {
        pub mod instance {
            // flags
            pub const DEIN_ACTIVE: i32 = 0x0100;
            pub const PHANTOM_HIT: i32 = 0x0101;
            pub const HIT_CANCEL_PHANTOM: i32 = 0x0102;

            // ints
            pub const DEIN_OBJECT_ID: i32 = 0x0100;
            pub const DEIN_EFF_HANDLER_FLASH: i32 = 0x0101;
            pub const DEIN_EFF_HANDLER_FIRE: i32 = 0x0102;
            pub const PHANTOM_EFF_HANDLER: i32 = 0x0103;
            pub const EFF_COOLDOWN_HANDLER: i32 = 0x0104;
        }
    }

    pub mod murabito {
        pub mod instance {
            // flags
            pub const IS_TILT_LW_SAPLING_PULL: i32 = 0x0100;
            pub const DISABLE_SPECIAL_S: i32 = 0x0101;

            // floats
            pub const SAPLING_PULL_SAPLING_POS_X: i32 = 0x0101;
            pub const SAPLING_PULL_SAPLING_POS_Y: i32 = 0x0102;
            pub const SAPLING_PULL_SAPLING_POS_Z: i32 = 0x0103;

            // ints
            pub const TURNIP_NUM_HI: i32 = 0x0104;
            pub const TURNIP_NUM_LW: i32 = 0x0105;
        }
    }

    pub mod ridley {
        pub mod instance {
            // flags
            pub const SPECIAL_LW_IS_GRAB: i32 = 0x0100;
            pub const SPECIAL_LW_IS_THROW: i32 = 0x101;
            pub const SPECIAL_LW_ENABLE_LANDING: i32 = 0x0102;
            pub const SPECIAL_LW_IS_LANDING: i32 = 0x0103;
            pub const SPECIAL_LW_ENABLE_BOUNCE: i32 = 0x0104;
            pub const SPECIAL_N_EXPLODE: i32 = 0x0105;

            // floats
            pub const SPECIAL_LW_BOUNCE_PREV_POS: i32 = 0x0100; //vector, requires two indexes
            pub const SPECIAL_S_FAILURE_CANCEL_FRAME: i32 = 0x0102;

            // ints
            pub const SPECIAL_LW_CATCH_ID: i32 = 0x0100;
        }
        pub mod status {
            // floats
            pub const SKEWER_STICK_Y: i32 = 0x1100;
        }
    }

    pub mod iceclimbers {
        pub mod instance {
            //flags
            pub const IS_VOLUNTARY_SOPO: i32 = 0x0100;
            pub const IS_SEPARATED: i32 = 0x0101;

            //ints
            pub const SEPARATED_EFFECT: i32 = 0x0110;
        }
    }

    pub mod packun {
        pub mod instance {
            // flags
            pub const STANCE_NEED_SET_SPEEDS: i32 = 0x0100;
            pub const STANCE_INIT: i32 = 0x0101;
            pub const PTOOIE_SHOULD_EXPLODE: i32 = 0x0102;
            pub const STANCE_REVERSE: i32 = 0x0103;

            // floats
            pub const PTOOIE_SCALE: i32 = 0x01BF;

            // ints
            pub const CURRENT_STANCE: i32 = 0x01BE; // 0 = Normal, 1 = Putrid, 2 = Prickly
            pub const STANCE_STATUS: i32 = 0x0101;
        }
        pub mod status {
            // flags
            pub const FLAME_ACTIVE: i32 = 0x1100;
            pub const CLOUD_COVER: i32 = 0x1101;
            pub const BITE_START: i32 = 0x1102;
            pub const BURST: i32 = 0x1103;
        }
    }

    pub mod wolf {
        pub mod status {
            // flags
            pub const SPECIAL_S_RESERVE_FALL: i32 = 0x1100;
        }
    }

    pub mod tantan {
        pub mod instance {
            //ints
            pub const DRAGONIZE_R_EFFECT_HANDLE: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const ARMS_ATTACK_CANCEL: i32 = 0x1100;
        }
    }

    pub mod krool {
        pub mod instance {
            //ints
            pub const SPECIAL_HI_FUEL: i32 = 0x0100;
            pub const FUEL_EFFECT_HANDLER: i32 = 0x0101;

            // floats
            pub const STORED_DAMAGE: i32 = 0x0100;

            // flags
            pub const BLUNDERBUSS_GRAB: i32 = 0x01CF;
        }
        pub mod status {
            // ints
            pub const CURRENT_CHARGE: i32 = 0x1100;

            // flags
            pub const GUT_CHECK_CHARGED: i32 = 0x1100;
        }
    }

    pub mod wiifit {
        pub mod instance {
            // flags
            pub const DEEP_BREATHING_COOLDOWN: i32 = 0x0100;
            pub const IS_RING_VISIBLE: i32 = 0x0101;
            pub const SPECIAL_S_DISABLE_STALL: i32 = 0x0102;

            // ints
            pub const SHOW_RING_MOTION: i32 = 0x0100;
            pub const RING_EFF_HANDLE: i32 = 0x0101;
            pub const RING_SECOND_EFF_HANDLE: i32 = 0x0102;
            pub const RING_THIRD_EFF_HANDLE: i32 = 0x0103;

            // floats
            pub const RING_START_FRAME: i32 = 0x0100;
            pub const RING_CURRENT_FRAME: i32 = 0x0101;
            pub const RING_END_FRAME: i32 = 0x0102;
            pub const RING_START_SIZE: i32 = 0x0103;
            pub const RING_END_SIZE: i32 = 0x0104;
            pub const RING_COLOR: i32 = 0x0105;     // this is a vector, so it needs three values (next value starts at 0x0108)
            pub const RING_SECOND_COLOR: i32 = 0x108;
        }
    }
}

pub mod statuses {
    pub mod elight {
        pub const SPECIAL_HI_FINISH2: i32 = 0x1F1;
    }

    pub mod falco {
        pub const SPECIAL_LW_LOOP: i32 = 0x1e8;
        pub const SPECIAL_LW_END: i32 = 0x1e9;
    }

    pub mod gamewatch {
        pub const SPECIAL_HI_OPEN: i32 = 0x205;
    }

    pub mod ganon {
        pub const SPECIAL_N_FLOAT: i32 = 0x1EC;
    }

    pub mod ryu {
        pub const AIR_DASH: i32 = 0x202;
    }

    pub mod ken {
        pub const ATTACK_COMMAND_4: i32 = 0x202;
    }

    pub mod buddy {
        pub const BUDDY_BAYONET_END: i32 = 0x1FB;
    }

    pub mod littlemac {
        pub const SPECIAL_LW_CANCEL: i32 = 0x1F4;
        pub const SPECIAL_LW_CANCEL_JUMP: i32 = 0x1F5;
    }

    pub mod wolf {
        pub const SPECIAL_S_RUSH: i32 = 0x1EA;
        pub const SPECIAL_S_END: i32 = 0x1EB;
    }

    pub mod diddy {
        pub const SPECIAL_N_CANCEL: i32 = 0x1FF;
        pub const SPECIAL_N_CANCEL_JUMP: i32 = 0x200;
    }

    pub mod kirby {
        pub const SPECIAL_HI_H: i32 = 0x3E6; //Weird value to avoid conflicts with copy ability values
        pub const GANON_SPECIAL_N_FLOAT: i32 = 0x3E7; 
        pub const LITTLEMAC_SPECIAL_N_CANCEL: i32 = 0x3E8; 
        pub const LITTLEMAC_SPECIAL_N_CANCEL_JUMP: i32 = 0x3E9; 
        pub const DIDDY_SPECIAL_N_CANCEL: i32 = 0x3EA; 
        pub const DIDDY_SPECIAL_N_CANCEL_JUMP: i32 = 0x3EB; 
    }

    pub mod krool {
        pub const SPECIAL_LW_GUT: i32 = 0x1F9;
    }

    pub mod palutena {
        pub const SPECIAL_N_R: i32 = 0x1E9;
        pub const SPECIAL_N_B: i32 = 0x1EA;
        pub const SPECIAL_N_Y: i32 = 0x1EB;
        pub const SPECIAL_N_P: i32 = 0x1EC;
        pub const SPECIAL_N_O: i32 = 0x1ED;
        pub const SPECIAL_N_G: i32 = 0x1EE;
    }
    
    pub mod bayonetta {
        pub const SPECIAL_S_KICK: i32 = 0x1F2;
        pub const SPECIAL_S_EDGE: i32 = 0x1F3;
    }
}
