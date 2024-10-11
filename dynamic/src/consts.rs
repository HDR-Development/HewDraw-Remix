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

            pub const IS_INIT: i32 = 0x0065;

            pub const IS_FLOAT: i32 = 0x0066;

            pub const WEIRD_ASS_TURN_RUN_ANIMATION: i32 = 0x0067;

            // ints

            pub const LAST_ATTACK_RECEIVER_ENTRY_ID: i32 = 0x0000;

            pub const COSTUME_SLOT_NUMBER: i32 = 0x0001; // Unironically why does this need to exist? We have WorkModule.

            pub const FLOAT_DURATION: i32 = 0x0002;
            pub const FLOAT_STATUS_KIND: i32 = 0x0003;

            pub const HITFALL_BUFFER: i32 = 0x0004;

            pub const JUMP_SQUAT_FRAME: i32 = 0x0005;

            pub const GIMMICK_TIMER: i32 = 0x0006;

            pub const AIR_ESCAPE_MAGNET_FRAME: i32 = 0x0007;

            pub const CSTICK_LIFE: i32 = 0x0008;

            pub const AGT_USED_COUNTER: i32 = 0x0009;

            pub const CLIFF_XLU_FRAME: i32 = 0x000A;
            pub const LAST_ATTACK_HITBOX_ID: i32 = 0x000B;
            pub const SHIELD_EFFECT_HANDLE: i32 = 0x000C;

            pub const FRAME_COUNTER: i32 = 0x000D;

            pub const LEFT_STICK_FLICK_X: i32 = 0x000E;
            pub const LEFT_STICK_FLICK_Y: i32 = 0x000F;

            pub const LEDGE_ID: i32 = 0x0010;

            pub const RIGHT_STICK_FLICK_X: i32 = 0x0011;
            pub const RIGHT_STICK_FLICK_Y: i32 = 0x0012;

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
            pub const ATTACK_S3_CSTICK_X: i32 = 0x0023;
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

            pub const FLOAT_INHERIT_AERIAL: i32 = 0x1000;

            pub const IS_TELEPORT_WALL_RIDE: i32 = 0x1000; // Mewtwo, Palutena, Sheik, and Zelda use this

            pub const ENABLE_SPECIAL_WALLJUMP: i32 = 0x1050;

            pub const HIT_EFFECT_DROP_ITEM: i32 = 0x1051;

            pub const SHOULD_HITFALL: i32 = 0x1006;

            pub const NO_POCKET: i32 = 0x1052;

            // ints

            pub const DOWN_STAND_FB_KIND: i32 = 0x1000;

            pub const FLOAT_FRAME: i32 = 0x1000;
            pub const FLOAT_ENABLE_UNIQ: i32 = 0x1001;
            pub const FLOAT_MTRANS: i32 = 0x1002;

            pub const WARP_EFF_HANDLER: i32 = 0x1000;

            // floats

            pub const INITIAL_KNOCKBACK_VEL_X: i32 = 0x1000;
            pub const INITIAL_KNOCKBACK_VEL_Y: i32 = 0x1001;

            pub const RESTING_HIP_OFFSET_Y: i32 = 0x1000;

            pub const TELEPORT_INITIAL_SPEED_X: i32 = 0x1000;
            pub const TELEPORT_INITIAL_SPEED_Y: i32 = 0x1001;
        }
    }

    pub mod bayonetta {
        pub mod instance {
            // ints
            pub const RECOVERY_RESOURCE_COUNT: i32 = 0x0100;
            pub const ATTACK_AIR_F_COUNT: i32 = 0x0101;
            pub const SPECIAL_N_CANCEL_TYPE: i32 = 0x0102;
            pub const SPECIAL_S_DABK_COUNT: i32 = 0x0103;
        }
        pub mod status {
            // floats
            pub const SPECIAL_S_ABK_ANGLE: i32 = 0x1100;
        }
    }

    pub mod brave {
        pub mod instance {
            // flags
            pub const PERSIST_RNG: i32 = 0x0100;
            pub const PSYCHE_UP_ACTIVE: i32 = 0x0101;
            pub const MENU_TRAINING_MODE_LOCK: i32 = 0x0102;

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
            pub const MENU_TRAINING_MODE_INDEX: i32 = 0x0113;
        }
    }

    pub mod buddy {
        pub mod instance {
            // flag
            pub const SPECIAL_S_BEAKBOMB_ACTIVE: i32 = 0x0100;
            pub const SPECIAL_N_BAYONET_ACTIVE: i32 = 0x0101;
            pub const SPECIAL_S_FAIL_ENABLE: i32 = 0x0102;
            pub const SPECIAL_HI_ENABLE_FREEFALL: i32 = 0x0103;
            pub const SPECIAL_N_LAND_CANCEL: i32 = 0x0104;

            // int
            pub const HUD_DISPLAY_TIME: i32 = 0x0100;
            //Current frame of Beakbomb, used to detect mislanding
            pub const SPECIAL_S_BEAKBOMB_FRAME: i32 = 0x0101;
            // 0: Normal Bounce (can be cancelled) 1: weak bounce 2: heavy bounce.
            pub const SPECIAL_S_BEAKBOMB_BOUNCE_TYPE: i32 = 0x0102;
            //Eggs fired gets reset when entering Bayonet, so we have to temporarily store current eggs fired
            pub const SPECIAL_N_BAYONET_EGGS_FIRED: i32 = 0x0103;

            // float
            pub const SPECIAL_S_RED_FEATHER_COOLDOWN: i32 = 0x0100;
            pub const SPECIAL_S_BEAKBOMB_ANGLE: i32 = 0x0101;
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
        }
        pub mod status {
            // flags
            pub const SPECIAL_HI_DIVE_ENABLE: i32 = 0x1101;
            pub const SPECIAL_HI_DIVE_START: i32 = 0x1102;
        }
    }

    pub mod cloud {

    }

    pub mod daisy {
        pub mod instance {   
            // flag
            pub const DISABLE_SPECIAL_S: i32 = 0x0100;
            pub const SPECIAL_S_GROUND_START: i32 = 0x0101;
        }
        pub mod status {
            // flags
            pub const SPECIAL_N_CRYSTAL_ACTIVE: i32 = 0x1100;
            pub const SPECIAL_N_AIR_START: i32 = 0x1101;
            pub const SPECIAL_N_DIVE: i32 = 0x1102;
            pub const SPECIAL_N_AUTOCANCEL: i32 = 0x1103;
            pub const GUARD_OFF_YAP: i32 = 0x1104;
        }
    }

    pub mod daisy_kinopio {
        pub mod status {
            // flags
            pub const YAP_ON: i32 = 0x1100;
            pub const YAP_OFF: i32 = 0x1101;
            pub const PARRY_YAP: i32 = 0x1102;

            // ints
            pub const IDLE_FRAMES: i32 = 0x1100;
            pub const YAPPING_TIMER: i32 = 0x1101;
            pub const FLOWER_EFFECT_ID: i32 = 0x1102;
            pub const FLOWER_EFFECT_FRAMES: i32 = 0x1103;
        }
    }

    pub mod dedede {
        pub mod instance{
            //flags
            pub const APPEAL_EQUIP_MASK: i32 = 0x0100;
            pub const SPECIAL_LW_CONTINUE_JET_SPIN: i32 = 0x0101;
            pub const SPECIAL_S_GORDO_DASH_DISABLE: i32 = 0x0102;

            //ints
            pub const SPECIAL_S_RECATCH_COUNT: i32 = 0x0103;
            pub const SPECIAL_LW_CHARGE_FRAME: i32 = 0x0104;

            //floats
            pub const SPECIAL_N_STICK_Y: i32 = 0x0105;
            pub const SPECIAL_S_TOSS_LR: i32 = 0x0106;
        }
        pub mod status{
            //flags
            pub const SPECIAL_LW_CONTINUE_SPIN: i32 = 0x1100;
        }
    }

    pub mod demon {
        pub mod instance {
            // flags
            pub const ATTACK_HI3_SLAUGHTER_HIGH_KICK: i32 = 0x0100;
            pub const ATTACK_HI3_DEVASTATOR: i32 = 0x0101;
            pub const ATTACK_STEP2S_SPINNING_DEMON: i32 = 0x0102;
            pub const ATTACK_STAND2_LIGHTNING_SCREW_UPPERCUT: i32 = 0x0103;
            pub const SPECIAL_HI_ENABLE_FREEFALL: i32 = 0x0104;
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

            // flags
            pub const SPECIAL_S_ENABLE_ATTACK: i32 = 0x1100;
            pub const SPECIAL_S_ENABLE_JUMP: i32 = 0x1101;

            // floats
            pub const SPECIAL_HI_INITIAL_POWER: i32 = 0x1100;
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
            /// used to indicate that a new barrel was spawned
            pub const SPECIAL_LW_BARREL_GENERATED: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SUPERLIFT_NEUTRAL_TOSS: i32 = 0x1100;
        }
    }

    pub mod duckhunt {
        pub mod instance {
            // flag
            pub const SPECIAL_HI2_ENABLE: i32 = 0x0100;

            // int
            pub const SPECIAL_LW_GUNMAN_TIMER: i32 = 0x0100;
        }
        pub mod status {
            // flag
            pub const SPECIAL_S_SMASH_INPUT: i32 = 0x1100;
            pub const SPECIAL_HI_JUMP: i32 = 0x0101;
            pub const SPECIAL_HI_ENABLE_SHOT: i32 = 0x1102;
            pub const SPECIAL_HI2_KILLSHOT_BUFFERED: i32 = 0x1103;
        }
    }

    pub mod edge {
        pub mod instance {
            // flags
            pub const FLASH_REFINE: i32 = 0x0100;
            pub const FLASH_REFRACT: i32 = 0x0101;

            // ints
            pub const FIRE_ID: i32 = 0x0100;
            pub const FLARE1_ID: i32 = 0x0101;

            // floats
            pub const FIRE_POS_X: i32 = 0x0100;
            pub const FIRE_POS_Y: i32 = 0x0101;
            pub const FLARE1_POS_X: i32 = 0x102;
            pub const FLARE2_POS_Y: i32 = 0x0103;
        }
        pub mod status {
            // flags
            pub const FLASH_HOLD: i32 = 0x1100;
        }
    }

    pub mod edge_fire {
        pub mod instance {
            //flags
            pub const REFLECT: i32 = 0x0100;
        }
        pub mod status {
            // floats
            pub const STICK_Y: i32 = 0x1100;
        }
    }

    pub mod edge_flare1 {
        pub mod status {
            // flags
            pub const REFRACTED: i32 = 0x1100;
        }
    }

    pub mod edge_flash {
        pub mod status {
            // ints
            pub const REFINE_COOLDOWN: i32 = 0x1100;
            pub const REFRACT_COOLDOWN: i32 = 0x1101;
            pub const LIFE: i32 = 0x1102;
            pub const EFFECT_HANDLE: i32 = 0x1103;
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
            pub const DISABLE_SPECIAL_HI: i32 = 0x0100;
            pub const DISABLE_SPECIAL_S: i32 = 0x0101;
            pub const SPECIAL_S_ENABLE_ACTION: i32 = 0x0102;
            pub const SPECIAL_HI_ENABLE_FREEFALL: i32 = 0x0103;
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
            pub const SPECIAL_LW_SET_ATTACK: i32 = 0x1100;
            pub const SPECIAL_LW_SET_EFFECT: i32 = 0x1101;
            pub const SPECIAL_LW_CONTINUE_MOTION: i32 = 0x1102;

            // ints
            pub const SPECIAL_LW_STOP_Y_FRAME: i32 = 0x1100;
        }
    }

    pub mod fox {

    }

    pub mod gamewatch {
        pub mod instance {
            // flags
            pub const SPECIAL_HI_ENABLE_FREEFALL: i32 = 0x0100;
            pub const SPECIAL_HI_ENABLE_PARACHUTE: i32 = 0x0101;

            // ints
            pub const SPECIAL_S_MATH_STATE: i32 = 0x0100;
            pub const SPECIAL_S_MATH_RESULT: i32 = 0x0101;
        }
    }

    pub mod ganon {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_N: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SPECIAL_N_ENABLE_ACTION: i32 = 0x1100;
            pub const SPECIAL_N_CHANGE_KINETIC_GROUND: i32 = 0x1101;
            pub const SPECIAL_N_CHANGE_FALL_SPEED: i32 = 0x1102;
            pub const SPECIAL_N_DECIDE_ANGLE: i32 = 0x1103;
            pub const SPECIAL_N_END: i32 = 0x1104;
        }
    }

    pub mod gaogaen {
        pub mod instance {
            // flags
            pub const SPECIAL_S_ALTERNATE_GRAB: i32 = 0x0100;
            pub const SPECIAL_S_LOW_GRAB: i32 = 0x0101;
            pub const SPECIAL_S_HIGH_GRAB: i32 = 0x0102;
        }
        pub mod status {
            // flags
            pub const SPECIAL_HI_RISE_END: i32 = 0x1100;

            // floats
            pub const GRAB_STICK_Y: i32 = 0x1100;
        }
    }

    pub mod gekkouga {
        pub mod instance {
            // flags
            pub const SPECIAL_LW_IS_DOLL: i32 = 0x0100;
            pub const SPECIAL_LW_CAN_TELEPORT: i32 = 0x0101;

            // ints
            pub const SPECIAL_LW_SUMMON_SUB_COOLDOWN: i32 = 0x0100;
            pub const SPECIAL_LW_MARKER_EFF_HANDLE: i32 = 0x0101;
        }
        pub mod status {
            // flags
            pub const SPECIAL_LW_SPAWN_SUB: i32 = 0x1150;
            pub const SPECIAL_LW_VANISH: i32 = 0x1151;
            pub const SPECIAL_LW_TELEPORT_OK: i32 = 0x1152;

            // ints
            pub const SPECIAL_LW_VANISH_TIMER: i32 = 0x1150;
        }
    }

    pub mod iceclimbers {
        pub mod instance {
            //flags
            pub const IS_VOLUNTARY_SOPO: i32 = 0x0100;
        }
    }

    pub mod ike {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_S: i32 = 0x0100;

        }
        pub mod status {
            // flags
            pub const SPECIAL_S_INSTAKILL: i32 = 0x1100;
            pub const SPECIAL_S_GROUND_START: i32 = 0x1101;
        }
    }

    pub mod inkling {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_S: i32 = 0x0100;
        }
    }

    pub mod jack {
        pub mod instance {
            // flags
            pub const SPECIAL_HI_GROUND_START: i32 = 0x0100;
        }
    }

    pub mod kamui {
        pub mod status {
            // floats
            pub const ATTACK_AIR_B_CHARGE: i32 = 0x1100;
        }
    }

    pub mod kirby {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_HI: i32 = 0x01FF; //Weird value to avoid conflicts with copy ability values
            pub const SPECIAL_N_PICKEL_CYCLE_MATERIAL: i32 = 0x01F4;
            pub use super::super::ridley::instance::SPECIAL_N_EXPLODE;

            // ints
            pub const SPECIAL_N_PICKEL_MATERIAL_INDEX: i32 = 0x01F5;
            pub use super::super::bayonetta::instance::SPECIAL_N_CANCEL_TYPE;

            // floats
            pub use super::super::reflet::instance::SPECIAL_N_CHARGE;
        }
        pub mod status {
            // ints
            pub const PURIN_SPECIAL_N_ENABLE_HIT_CANCEL_FRAME: i32 = 0x11F9;

            // flags
            pub const SPECIAL_N_PICKEL_MINING_TIMER: i32 = 0x11F4;
            pub const SPECIAL_N_LITTLEMAC_GRAVITY_ENABLE: i32 = 0x11F5;
            pub const SPECIAL_N_LITTLEMAC_GRAVITY_END: i32 = 0x11F6;
            pub const SPECIAL_N_LITTLEMAC_CLEAR_CRIT: i32 = 0x11F7;
            pub const PURIN_SPECIAL_N_HIT: i32 = 0x11F8;
            pub const PURIN_SPECIAL_N_HIT_CANCEL_OK: i32 = 0x11F9;

            pub use super::super::mario::status::SPECIAL_N_FIREBRAND;
            pub use super::super::luigi::status::SPECIAL_N_THUNDERHAND;
            pub use super::super::mariod::status::SPECIAL_N_CHILL_PILL;
            pub use super::super::daisy::status::SPECIAL_N_CRYSTAL_ACTIVE;
            pub use super::super::daisy::status::SPECIAL_N_AIR_START;
            pub use super::super::daisy::status::SPECIAL_N_DIVE;
            pub use super::super::daisy::status::SPECIAL_N_AUTOCANCEL;
        }
    }

    pub mod koopa {
        pub mod instance {
            // flags
            pub const ATTACK_S4_EXCELLENT_PUNCH: i32 = 0x0100; // determines if forward smash is an excellent punch

            // ints
            pub const SPECIAL_N_FIREBALL_COOLDOWN: i32 = 0x0100;
            pub const SPECIAL_N_FIREBALL_EFFECT_ID: i32 = 0x0101;
            pub const ATTACK_S4_EFFECT_HANDLE: i32 = 0x0102;
            pub const SPECIAL_S_THROW_TYPE: i32 = 0x0103;
        }
        pub mod status {
            // flags
            pub const SPECIAL_S_ABOVE_BLASTZONE: i32 = 0x1100;
        }
    }

    pub mod koopajr {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_S: i32 = 0x0100;
            pub const DISABLE_MECHAKOOPA: i32 = 0x0101;
            pub const MECHAKOOPA_COOLDOWN_ACTIVE: i32 = 0x0102;
        }
    }

    pub mod krool {
        pub mod instance {
            //ints
            pub const SPECIAL_HI_FUEL: i32 = 0x0100;
            pub const FUEL_EFFECT_HANDLE: i32 = 0x0101;

            // floats
            pub const SPECIAL_LW_STORED_DAMAGE: i32 = 0x0100;

            // flags
            pub const SPECIAL_N_GRAB: i32 = 0x01CF;
        }
        pub mod status {
            // ints
            pub const ATTACK_CHARGE: i32 = 0x1100;

            // flags
            pub const SPECIAL_LW_GUT_CHARGED: i32 = 0x1100;
        }
    }

    pub mod link {

    }

    pub mod littlemac {
        pub mod instance {
            // flags
            pub const ATTACK_13_DREAMLAND_EXPRESS: i32 = 0x0100;
            pub const ATTACK_13_LATE_DLE_INPUT: i32 = 0x0101;
            pub const SPECIAL_N_MOTION_AIR: i32 = 0x0102;

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

    pub mod lucario {
        pub mod instance {
            // flags
            pub const METER_BURNOUT: i32 = 0x0100;
            pub const SPECIAL_LW_AIR: i32 = 0x0101;
            pub const DISABLE_SPECIAL_LW: i32 = 0x0102;
            pub const IS_POWERED_UP: i32 = 0x0103;
            pub const SPECIAL_HI_ATTACK_CANCEL: i32 = 0x0104;

            // ints
            pub const METER_PAUSE_REGEN_FRAME: i32 = 0x0100;

            // float
            pub const METER_PASSIVE_RATE: i32 = 0x0100;
        }
        pub mod status {
            // ints
            pub const SPECIAL_S_ROT_ANGLE: i32 = 0x1100;

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
            pub const SPECIAL_N_OFFENSE_UP_EFFECT_HANDLE3: i32 = 0x01DD;
            pub const SPECIAL_N_OFFENSE_UP_CHARGE_LEVEL: i32 = 0x01DC;
        }
        pub mod status {
            // flag
            pub const SPECIAL_HI_ATTACK_TOUCH_WALL: i32 = 0x1100;
            pub const SPECIAL_HI_ATTACK_LEAVE_WALL: i32 = 0x1101;
            pub const SPECIAL_HI_ATTACK_WALL_TOUCH_REVERSE_MOMENTUM: i32 = 0x1102;
            pub const SPECIAL_HI_ATTACK_SET_WALL_LEAVE_MOMENTUM: i32 = 0x1103;
            pub const SPECIAL_HI_THUNDER_LOOSE: i32 = 0x1104;

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
            pub const APPEAL_EQUIP_MASK: i32 = 0x0100;
            
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
            pub const SPECIAL_S_MISFIRE_STORED: i32 = 0x0100;
            pub const SPECIAL_S_TRAINING_MISFIRE: i32 = 0x0101;
            pub const SPECIAL_S_MISFIRE_INIT: i32 = 0x0102;
            // int
            /// This int stores the handle of the charge smoke effect for killing it if we store misfire
            pub const SPECIAL_S_SMOKE_EFFECT_HANDLE: i32 = 0x0101;
            /// This int stores the handle of the pulsing effect for killing it if we store misfire
            pub const SPECIAL_S_PULSE_EFFECT_HANDLE: i32 = 0x0102;

            // float
            /// This float holds the current multiplier on damage for misfire
            pub const SPECIAL_S_MISFIRE_DAMAGE_MUL: i32 = 0x0100;
        }
        pub mod status {
            // flag
            pub const SPECIAL_N_THUNDERHAND: i32 = 0x1100;
        }
    }

    pub mod mario {
        pub mod instance {
            // flags
            pub const SPECIAL_LW_DISABLE_STALL: i32 = 0x0100;
            pub const SPECIAL_S_DISABLE_STALL: i32 = 0x0101;
        }
        pub mod status {
            // flags
            pub const SPECIAL_N_FIREBRAND: i32 = 0x1100;
            pub const SPECIAL_LW_GROUND_START: i32 = 0x0101;
        }
    }

    pub mod mariod {
        pub mod instance {
            // flags
            pub const SPECIAL_HI_GROUND_INTERRUPT: i32 = 0x0100;
            pub const SPECIAL_S_DISABLE_STALL: i32 = 0x0101;
        }
        pub mod status {
            // flags
            pub const SPECIAL_N_CHILL_PILL: i32 = 0x1100;
        }
    }

    pub mod marth {

    }

    pub mod master {
        pub mod instance {
            // flags
            pub const ATTACK_S4_SPECIAL: i32 = 0x0100;
            pub const SPECIAL_HI_CATCH_USED: i32 = 0x0101;
        }
        pub mod status {
            // flags
            pub const SPECIAL_LW_HOLD: i32 = 0x1100;
        }
    }

    pub mod master_axe {
        pub mod status {
            // ints
            pub const LIFE: i32 = 0x1100;
        }
    }

    pub mod metaknight {
        pub mod instance {
            // flags
            pub const SPECIAL_S_HIT: i32 = 0x0100;
        }
    }

    pub mod mewtwo {
        pub mod instance {
            // flags
            pub const SPECIAL_HI_TELEPORT_CANCEL: i32 = 0x0100;
            pub const SPECIAL_HI_ENABLE_FREEFALL: i32 = 0x0101;
            pub const SPECIAL_HI_GROUNDED_TELEPORT: i32 = 0x0102;
        }
    }

    pub mod miifighter {
        pub mod instance {
            // ints
            pub const SPECIAL_LW1_QUAKE_EFFECT_HANDLE: i32 = 0x0100;

            // flags
            pub const SPECIAL_LW3_STALL: i32 = 0x0100;
        }
        pub mod status {
            // ints
            pub const SPECIAL_LW1_CHARGE: i32 = 0x1100;

            // floats
            pub const SPECIAL_LW1_CHARGE_DISTANCE: i32 = 0x1101;
        }
    }

    pub mod miigunner {
        pub mod status {
            // flags
            pub const BOOSTED_AERIAL: i32 = 0x1100;

            // floats
            pub const ATTACK_CHARGE: i32 = 0x1100;
        }
        pub mod instance {
            // flags
            pub const SPECIAL_HI1_LAUNCH_AIR_USED: i32 = 0x0100;
            pub const BOOSTED_ATTACK_AIR_LW_AIRTIME: i32 = 0x0101;

            // ints
            pub const SPECIAL_HI1_LAUNCH_EFFECT_HANDLE: i32 = 0x0100;
            pub const SPECIAL_S3_MISSILE_OBJECT_ID: i32 = 0x0101;
            pub const SPECIAL_S2_STEALTHBOMB_EFFECT_HANDLE: i32 = 0x0102;

            // floats
            pub const SPECIAL_N3_CHARGE: i32 = 0x0102;
        }
    }

    pub mod miigunner_supermissile {
        pub mod instance {
            // flags
            pub const PULSE_DETONATE: i32 = 0x0100;
        }
    }

    pub mod miiswordsman {
        pub mod instance {
            // flags
            pub const SPECIAL_S3_CHAKRAM_STICK: i32 = 0x0100;
            pub const SPECIAL_HI2_DASH_HIT: i32 = 0x0101;
        }
        pub mod status {
            // flags
            pub const SPECIAL_N1_WAVE: i32 = 0x1100;
            pub const SPECIAL_S2_EDGE_CANCEL: i32 = 0x1101;
            pub const SPECIAL_LW2_SHOCK_SPELL_HOLD: i32 = 0x1102;
            pub const SPECIAL_S2_GROUND_START: i32 = 0x1103;
        }
    }

    pub mod murabito {
        pub mod instance {
            // flags
            pub const ATTACK_LW3_SAPLING_PULL: i32 = 0x0100;
            pub const DISABLE_SPECIAL_S: i32 = 0x0101;

            // floats
            pub const SAPLING_PULL_SAPLING_POS_X: i32 = 0x0101;
            pub const SAPLING_PULL_SAPLING_POS_Y: i32 = 0x0102;
            pub const SAPLING_PULL_SAPLING_POS_Z: i32 = 0x0103;

            // ints
            pub const ATTACK_AIR_HI_TURNIP_COUNT: i32 = 0x0104;
            pub const ATTACK_AIR_LW_TURNIP_COUNT: i32 = 0x0105;
        }
    }

    pub mod ness {
        pub mod instance {
            //flags
            pub const DISABLE_SPECIAL_HI: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SPECIAL_HI_THUNDER_LOOSE: i32 = 0x1100;
        }
    }

    pub mod packun {
        pub mod instance {
            // flags
            pub const STANCE_ENABLE_CHANGE_SPEED: i32 = 0x0100;
            pub const APPEAL_STANCE_INIT: i32 = 0x0101;
            pub const APPEAL_STANCE_REVERSE: i32 = 0x0102;

            // floats
            pub const SPECIAL_N_PTOOIE_SCALE: i32 = 0x01BF;
            pub const FIRE_POS_X: i32 = 0x0101;
            pub const FIRE_POS_Y: i32 = 0x0102;

            // ints
            pub const CURRENT_STANCE: i32 = 0x01BE; // 0 = Normal, 1 = Putrid, 2 = Prickly
            pub const STANCE_STATUS: i32 = 0x0101;
        }
        pub mod status {
            // flags
            pub const POSION_BREATH_ENABLE_STANDARD_FLAME: i32 = 0x1100;
            pub const APPEAL_CLOUD_COVER: i32 = 0x1101;
            pub const POISON_BREATH_ENABLE_PRICKLY_BITE: i32 = 0x1102;
            pub const POISON_BREATH_BURST: i32 = 0x1103;
        }
    }

    pub mod packun_spikeball {
        pub mod instance {
            // flags
            pub const ENABLE_EXPLODE: i32 = 0x0100;
        }
    }

    pub mod pacman {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_HI: i32 = 0x0100;
            pub const SPECIAL_HI_GROUND_START: i32 = 0x0101;
            pub const SPECIAL_HI_AERIAL_USED: i32 = 0x0102;
            pub const SPECIAL_S_GROUND_START: i32 = 0x0103;
        }
        pub mod status {
            // flags
            pub const SPECIAL_HI_AERIAL: i32 = 0x1100;
        }
    }

    pub mod palutena {
        pub mod status {
            // flags
            pub const ENABLE_COLOR_INCREMENT: i32 = 0x1100;
            pub const SPECIAL_HI_TELEPORT_AIR_START: i32 = 0x1101;

            // floats
            pub const SPECIAL_LW_STORED_DAMAGE: i32 = 0x1100;
        }
        pub mod instance {
            // flags
            pub const SPECIAL_HI_TELEPORT_GROUND_START: i32 = 0x0100;
            pub const SPECIAL_HI_JUMP_REFRESH: i32 = 0x0101;
            pub const SPECIAL_N_FLUSH_BOARD: i32 = 0x0102;
            pub const SPECIAL_N_PRIMARY_POWERED: i32 = 0x0103;
            pub const SPECIAL_HI_ENABLE_FREEFALL: i32 = 0x0104;

            // ints
            pub const POWER_BOARD_SLOT_1: i32 = 0x0100;
            pub const POWER_BOARD_SLOT_2: i32 = 0x0101;
            pub const SPECIAL_N_GAINED_COLOR: i32 = 0x0102;
            // kirby specific
            pub const SPECIAL_N_PALUTENA_COLOR_COUNT: i32 = 0x0103;
        }
    }

    pub mod peach {
        pub mod instance {
            // flag
            // Used to check if sideb wall bounce happens
            pub const SPECIAL_S_WALL_BOUNCE: i32 = 0x0100;
            pub const DISABLE_SPECIAL_S: i32 = 0x0101;
        }
    }

    pub mod pfushigisou {

    }

    pub mod pichu {
        pub mod instance {
            //flags
            pub const CHARGE_STATE_ATTACK: i32 = 0x0102;

            //ints
            pub const CHARGE_STATE_ENABLED: i32 = 0x0100;
            pub const CHARGE_EFFECT_HANDLER: i32 = 0x0101;

            // floats
            pub const CHARGE_STATE_DAMAGE_MUL: i32 = 0x0100;
            pub const CHARGE_STATE_RECOIL_MUL: i32 = 0x0101;
            pub const SPECIAL_LW_DISCHARGE_DAMAGE_MUL: i32 = 0x0102;
            pub const SPECIAL_LW_DISCHARGE_SIZE_MUL: i32 = 0x0103;
        }
    }

    pub mod pickel {
        pub mod instance {
            // flags
            pub const TABLE_ENABLE_RESPAWN: i32 = 0x0100;
            pub const CYCLE_MATERIAL: i32 = 0x0101;
            pub const DAMAGE_FLY_RESET_ROT: i32 = 0x0102;

            // ints 
            pub const MATERIAL_INDEX: i32 = 0x0100;
            pub const MATERIAL_EFFECT_HANDLE: i32 = 0x0101;
            pub const DAMAGE_RED_EFFECT_TIMER: i32 = 0x0102;
            pub const PEARL_COOLDOWN: i32 = 0x103;

            // floats
            pub const DAMAGE_RED_STORED_DAMAGE: i32 = 0x0100;
            pub const TABLE_CURRENT_LIFE: i32 = 0x0101;
        }
        pub mod status {
            // ints
            pub const MINING_TIMER: i32 = 0x1100;
            
            // floats
            pub const SPECIAL_HI_GLIDE_TIMER: i32 = 0x1100;

            // flags
            pub const ATTACK_LW3_SOUL_FIRE: i32 = 0x1100;
            pub const SPECIAL_S_THROW_PEARL: i32 = 0x1101;
        }
    }

    pub mod pickel_forge {
        pub mod instance {
            // floats 
            pub const START_Y_POS: i32 = 0x0100;
        }
    }

    pub mod pickel_trolley {
        pub mod instance {
            // ints 
            pub const PEARL_OWNER_ID: i32 = 0x0100;
        }
        pub mod status {
            // ints
            pub const REFLECT_COUNT: i32 = 0x1100;
            pub const TRAVEL_FRAMES: i32 = 0x1101;
            
            // floats
            pub const PREV_LR: i32 = 0x1100;
        }
    }

    pub mod pikachu {
        pub mod instance {
            // flags
            pub const SPECIAL_HI_DISABLE_JUMP_CANCEL: i32 = 0x0100;
            pub const SPECIAL_HI_QUICK_ATTACK_CANCEL: i32 = 0x0101;
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

    pub mod pit {

    }

    pub mod pitb {

    }

    pub mod plizardon {
        pub mod instance {
            pub const DISABLE_SPECIAL_S: i32 = 0x0100;
        }
    }

    pub mod ptrainer {
        pub mod instance {
            pub const SPECIAL_LW_BACKWARDS_SWITCH: i32 = 0x0100;
        }
    }

    pub mod pzenigame {

    }

    pub mod reflet {
        pub mod status {
            // flags
            pub const SPECIAL_HI_ELWIND1_CANCEL: i32 = 0x1100;
        }
        pub mod instance {
            // flags
            pub const SPECIAL_HI_ENABLE_FREEFALL: i32 = 0x0100;

            // ints
            pub const ATTACK_AIR_LEVIN_LENIENCY: i32 = 0x0100;

            // floats
            pub const SPECIAL_N_CHARGE: i32 = 0x0100;
        }
    }

    pub mod richter {
        pub mod instance {
            // flags
            pub const SPECIAL_HI_ENABLE_FREEFALL: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const ATTACK_LW3_JUMP_BUFFER: i32 = 0x1150;
            pub const SPECIAL_S_CHANGE_KINETIC: i32 = 0x1151;
            pub const SPECIAL_S_ENABLE_GRAVITY: i32 = 0x1152;
        }
    }

    pub mod ridley {
        pub mod instance {
            // flags
            pub const SPECIAL_LW_GROUND_START: i32 = 0x0100;
            pub const SPECIAL_LW_ENABLE_LANDING: i32 = 0x0101;
            pub const SPECIAL_LW_LANDING: i32 = 0x0102;
            pub const SPECIAL_LW_ENABLE_BOUNCE: i32 = 0x0103;
            pub const SPECIAL_N_EXPLODE: i32 = 0x0104;

            // floats
            pub const SPECIAL_LW_BOUNCE_PREV_POS: i32 = 0x0100; //vector, requires two indexes
            pub const SPECIAL_S_FAILURE_CANCEL_FRAME: i32 = 0x0102;

            // ints
            pub const SPECIAL_LW_CATCH_ID: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SPECIAL_HI_HOVER_DECIDE_STICK: i32 = 0x1100;

            // floats
            pub const SPECIAL_LW_STICK_Y: i32 = 0x1100;
            pub const SPECIAL_HI_HOVER_DECIDE_STICK_X: i32 = 0x1101;
            pub const SPECIAL_HI_HOVER_DECIDE_STICK_Y: i32 = 0x1102;
            pub const SPECIAL_HI_CHARGE_DIR: i32 = 0x1104;
        }
    }

    pub mod robot {
        pub mod instance {
            // flags
            pub const ATTACK_AIR_B_USED: i32 = 0x0100;
            pub const SPECIAL_S_AIR_USED: i32 = 0x0101;
            pub const IS_INIT_METER: i32 = 0x0102;
            pub const SPECIAL_HI_GROUND_START: i32 = 0x0103;
            pub const SPECIAL_HI_MARKER_EFFECT_HANDLE: i32 = 0x0104;

            // ints
            pub const SPECIAL_HI_CHARGE_FRAME: i32 = 0x0100;

            // floats
            pub const SPECIAL_HI_ROT_X: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const ATTACK_AIR_B_CHARGE: i32 = 0x1100;
            pub const IS_CHARGE_FINISHED: i32 = 0x1101;
            pub const ATTACK_AIR_B_MAX_CHARGE: i32 = 0x1102;
            pub const ATTACK_AIR_B_CHARGE_LEVEL: i32 = 0x1103;
        }
    }

    pub mod rockman {
        pub mod instance {
            // flags
            pub const SPECIAL_N_CHARGE_SHOT_CHARGING: i32 = 0x0100;
            pub const SPECIAL_N_CHARGE_SHOT_PLAYED_SFX: i32 = 0x0101;
            pub const SPECIAL_N_CHARGE_SHOT_RELEASE: i32 = 0x0102;
            pub const SPECIAL_HI_ENABLE_FREEFALL: i32 = 0x0103;
            
            // ints
            pub const SPECIAL_N_CHARGE_SHOT_FRAME: i32 = 0x0100;
            pub const SPECIAL_N_CHARGE_SHOT_EFFECT_HANDLE: i32 = 0x0101;
            pub const SPECIAL_N_CHARGE_SHOT_SOUND_HANDLE: i32 = 0x0102;
            pub const SPECIAL_N_CHARGE_SHOT_RELEASE_FRAME: i32 = 0x0103;
        }
        pub mod status {
            // flags
            pub const SPECIAL_N_CHARGE_SHOT_KEEP_CHARGE: i32 = 0x1100;
        }
    }

    pub mod rockman_airshooter {
        pub mod status {
            // flags
            pub const MOVE: i32 = 0x1100;
        }
    }

    pub mod rosetta {
        pub mod instance {
            // ints
            pub const SPECIAL_LW_WARP_EFFECT_HANDLE: i32 = 0x0100;
            pub const ROSA_X: i32 = 0x0101;
            pub const ROSA_Y: i32 = 0x0102;
            pub const TICO_X: i32 = 0x0103;
            pub const TICO_Y: i32 = 0x0104;
            pub const TICO_RAYCAST: i32 = 0x0106;
            pub const TICO_X_DIST: i32 = 0x0107;
            pub const TICO_Y_DIST: i32 = 0x0108;

            // flags
            pub const SPECIAL_LW_TICO_UNAVAILABLE: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SPECIAL_LW_INVALID_WARP: i32 = 0x1100;
            pub const SPECIAL_LW_WARP_GROUND_START: i32 = 0x1101;
        }
    }

    pub mod roy {
        pub mod instance {
            // flags
            pub const TRAIL_EFFECT: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SPECIAL_S4_REVERSE: i32 = 0x1150;
        }
    }

    pub mod samus {
        pub mod instance {
            // flags
            pub const ATTACK_DASH_SHINESPARK: i32 = 0x0100;
            pub const ATTACK_DASH_ENABLE_SHINESPARK: i32 = 0x0101;
        }
    }

    pub mod samusd {
        pub mod instance {
            // flags
            pub const SPECIAL_LW_ENABLE_DETONATE: i32 = 0x0100;

            // ints
            pub const SPECIAL_LW_BOMB_OBJECT_ID: i32 = 0x0100;
        }
    }

    pub mod sheik {
        pub mod instance {
            // flags
            pub const SPECIAL_LW_HIT: i32 = 0x0100;
        }
    }

    pub mod shizue {
        pub mod instance {
            // flags
            pub const SPECIAL_LW_LLOID_ASYNC: i32 = 0x0100;

            // ints
            pub const SPECIAL_LW_LLOID_TIMER: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SPECIAL_HI_EARLY_RELEASE: i32 = 0x1100;
            pub const SPECIAL_HI_LATE_RELEASE: i32 = 0x1101;
        }
    }

    pub mod shotos {
        pub mod instance {
            // flags
            pub const EX_SPECIAL_USED: i32 = 0x0100;
            pub const MAGIC_SERIES_CANCEL: i32 = 0x0101;
            pub const SPECIAL_LW_ENABLE_FADC: i32 = 0x0102;
            pub const ENABLE_TARGET_COMBO_1: i32 = 0x0103;
            pub const ENABLE_TARGET_COMBO_2: i32 = 0x0104;
            pub const DISABLE_SPECIAL_S: i32 = 0x0106;
            pub const SPECIAL_N_HADOKEN_AIR: i32 = 0x0107;
            pub const DISABLE_SPECIAL_LW: i32 = 0x0108;
            pub const SPECIAL_LW_ENABLE_INSTALL: i32 = 0x0109;

            // ints
            pub const SPECIAL_HI_FIRE_EFFECT_HANDLE: i32 = 0x0100;
            pub const SPECIAL_LW_FIRE_EFF_ID_0: i32 = 0x0101;
            pub const SPECIAL_LW_FIRE_EFF_ID_1: i32 = 0x0102;
            pub const SPECIAL_N_EX_NUM: i32 = 0x0103;
        }
        pub mod status {
            // flags
            pub const SCALE_COMBO_DAMAGE: i32 = 0x1100;
            pub const MAGIC_SERIES_CANCEL_ENABLED: i32 = 0x1101;
        }
    }

    pub mod shulk {
        pub mod instance {
            // flags
            pub const DISABLE_SPECIAL_S: i32 = 0x0100;

            // ints
            pub const SPECIAL_S_STEP: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const MONADO_BEAT: i32 = 0x1100;
        }
    }

    pub mod simon {
        pub mod status {
            //flags
            pub const SPECIAL_S_LAND_CANCEL: i32 = 0x1100;
        }
    }

    pub mod snake {
        pub mod instance {
            // flags
            pub const SPECIAL_LW_SELF_STICK: i32 = 0x0100;
            pub const ATTACK_S4_ENABLE_COMBO: i32 = 0x0101;
            pub const ATTACK_S4_COMBO_BUFFER: i32 = 0x0102;
            pub const APPEAL_LW_C4_EXPLODE: i32 = 0x0103;
            pub const APPEAL_LW_GRENADE_WAIT_COUNT: i32 = 0x0104;
            pub const CATCH_ENABLE_WALK: i32 = 0x0105;
            pub const SPECIAL_S_RELOAD_VULNERABLE: i32 = 0x0106;
            pub const SPECIAL_S_FORCE_RELOAD: i32 = 0x0107;

            // ints
            pub const ATTACK_S4_COMBO_COUNT: i32 = 0x0100;
            pub const SPECIAL_S_AMMO_COUNT: i32 = 0x0101;
        }
    }

    pub mod sonic {
        pub mod instance {
            // flags
            pub const SPECIAL_AIR_ACTION_USED: i32 = 0x0100;
            pub const SPECIAL_HI_ENABLE_FREEFALL: i32 = 0x0101;

            // ints
            pub const SPECIAL_N_POSE: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SPECIAL_N_BLAST_ATTACK: i32 = 0x1100;

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

    pub mod szerosuit {
        pub mod status {
            // flags
            pub const ATTACK_AIR_LW_REBOUND: i32 = 0x1100;
            pub const SPECIAL_LW_ENABLE_MANUAL_FOOTSTOOL: i32 = 0x1101;
        }
    }

    pub mod tantan {
        pub mod instance {
            //ints
            pub const ARMR_DRAGONIZE_EFFECT_HANDLE: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const ARMS_ATTACK_CANCEL: i32 = 0x1100;
        }
    }

    pub mod toonlink {

    }

    pub mod trail {
        pub mod instance {
            // flags
            pub const ATTACK_12_ENABLE_S3_COMBO: i32 = 0x0100;
            pub const ATTACK_LW4_REBOUND: i32 = 0x0101;
            pub const DISABLE_SPECIAL_N: i32 = 0x102;

            // ints
            pub const SPECIAL_N_MAGIC_TIMER: i32 = 0x100;

            // floats
            pub const SPECIAL_S_JUMP_SPEED_X: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SPECIAL_N_THUNDER_LAND_CANCEL: i32 = 0x1100;
            pub const SPECIAL_S_HIT: i32 = 0x1101;
            pub const SPECIAL_S_INPUT_CHECK: i32 = 0x1102;
            pub const SPECIAL_S_STOP: i32 = 0x1103;

            // ints
            pub const ATTACK_LW4_TIMER: i32 = 0x1100;

            // floats
            pub const ATTACK_LW4_DACUS_SPEED_Y: i32 = 0x1100;
        }
    }

    pub mod wario {

    }

    pub mod wiifit {
        pub mod instance {
            // flags
            pub const SPECIAL_LW_RESPAWN_COOLDOWN: i32 = 0x0100;
            pub const RING_EFFECT_VISIBLE: i32 = 0x0101;
            pub const SPECIAL_S_DISABLE_STALL: i32 = 0x0102;

            // ints
            pub const RING_SHOW_MOTION: i32 = 0x0100;
            pub const RING_EFFECT_HANDLE: i32 = 0x0101;
            pub const RING_SECOND_EFFECT_HANDLE: i32 = 0x0102;
            pub const RING_THIRD_EFFECT_HANDLE: i32 = 0x0103;

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

    pub mod wolf {
        pub mod status {
            // flags
            pub const SPECIAL_S_RESERVE_FALL: i32 = 0x1100;
            pub const AWOO: i32 = 0x1101;
        }
    }

    pub mod yoshi {
        pub mod status {
            
        }
    }

    pub mod younglink {

    }

    pub mod zelda {
        pub mod instance {
            // flags
            pub const SPECIAL_HI_GROUNDED_TELEPORT: i32 = 0x0100;
            pub const SPECIAL_LW_PHANTOM_HIT: i32 = 0x0101;
            pub const SPECIAL_LW_FORWARD_PHANTOM: i32 = 0x0102;
            pub const SPECIAL_LW_DISABLE_PHANTOM: i32 = 0x0103;

            // ints
            pub const SPECIAL_S_DEIN_OBJECT_ID: i32 = 0x0100;
            pub const SPECIAL_S_DEIN_OBJECT_ID_2: i32 = 0x0101;
            pub const SPECIAL_S_CURRENT_DEIN_MOVE_OBJECT_ID: i32 = 0x0102;
            pub const SPECIAL_S_DEIN_FLASH_EFFECT_HANDLE: i32 = 0x0103;
            pub const SPECIAL_S_DEIN_FIRE_EFFECT_HANDLE: i32 = 0x0104;
            pub const SPECIAL_S_COOLDOWN_EFFECT_HANDLE: i32 = 0x0105;
            pub const SPECIAL_LW_PHANTOM_OBJECT_ID: i32 = 0x0106;
            pub const SPECIAL_LW_PHANTOM_EFFECT_HANDLE: i32 = 0x0107;
            pub const SPECIAL_LW_COOLDOWN_EFFECT_HANDLE: i32 = 0x0108;

            // floats
            pub const SPECIAL_HI_TELEPORT_END_SPEED_X: i32 = 0x0100;
        }
        pub mod status {
            // flags
            pub const SPECIAL_S_DINS_REFRESH: i32 = 0x1100;
            pub const SPECIAL_LW_PHANTOM_NO_BUILD: i32 = 0x1101;
            pub const SPECIAL_LW_PHANTOM_CANCEL_FRAME: i32 = 0x1102;
        }
    }
}

pub mod statuses {
    pub mod bayonetta {
        pub const SPECIAL_S_KICK: i32 = 0x1F2;
        pub const SPECIAL_N_CANCEL: i32 = 0x1F3;
    }

    pub mod chrom {
        pub const SPECIAL_HI_FLIP: i32 = 0x1ea;
        pub const SPECIAL_HI_DIVE: i32 = 0x1eb;
    }

    pub mod buddy {
        pub const SPECIAL_N_BAYONET_END: i32 = 0x1FB;
    }

    pub mod daisy_kinopio {
        pub const YAP: i32 = 0x1;
    }

    pub mod diddy {
        pub const SPECIAL_N_CANCEL: i32 = 0x1FF;
        pub const SPECIAL_N_CANCEL_JUMP: i32 = 0x200;
    }

    pub mod edge_flash {
        pub const BURST: i32 = 0x2;
        pub const VANISH: i32 = 0x3;
    }

    pub mod elight {
        pub const SPECIAL_HI_FINISH2: i32 = 0x1F1;
    }

    pub mod falco {
        pub const SPECIAL_LW_LOOP: i32 = 0x1e8;
        pub const SPECIAL_LW_END: i32 = 0x1e9;
        pub const SPECIAL_LW_HIT: i32 = 0x1ea;
    }

    pub mod gamewatch {
        pub const SPECIAL_HI_OPEN: i32 = 0x205;
    }

    pub mod ganon {
        pub const SPECIAL_N_FLOAT: i32 = 0x1EC;
    }

    pub mod gekkouga {
        pub const SPECIAL_LW_JUMP: i32 = 0x1F1;
    }

    pub mod ken {
        pub const SPECIAL_LW_INSTALL: i32 = 0x202;
        pub const ATTACK_COMMAND_4: i32 = 0x203;
    }

    pub mod kirby {
        pub const SPECIAL_HI_H: i32 = 0x3E6; //Weird value to avoid conflicts with copy ability values
        pub const GANON_SPECIAL_N_FLOAT: i32 = 0x3E7; 
        pub const LITTLEMAC_SPECIAL_N_CANCEL: i32 = 0x3E8; 
        pub const LITTLEMAC_SPECIAL_N_CANCEL_JUMP: i32 = 0x3E9; 
        pub const DIDDY_SPECIAL_N_CANCEL: i32 = 0x3EA; 
        pub const DIDDY_SPECIAL_N_CANCEL_JUMP: i32 = 0x3EB;
        pub const BAYONETTA_SPECIAL_N_CANCEL: i32 = 0x3EC;
        pub const BUDDY_SPECIAL_N_BAYONET_END: i32 = 0x3ED;
    }

    pub mod krool {
        pub const SPECIAL_LW_GUT: i32 = 0x1F9;
        pub const SPECIAL_N_FIRE_HI: i32 = 0x1FA;
    }

    pub mod littlemac {
        pub const SPECIAL_LW_CANCEL: i32 = 0x1F4;
        pub const SPECIAL_LW_CANCEL_JUMP: i32 = 0x1F5;
    }

    pub mod master_axe {
        pub const SPECIAL_LW_YEET: i32 = 0x7;
    }

    pub mod mewtwo {
        pub const FLOAT: i32 = 0x1E9;
    }

    pub mod packun_firebreath {
        pub const REGULAR: i32 = 0x0;
    }

    pub mod palutena {
        pub const SPECIAL_N_R: i32 = 0x1E9;
        pub const SPECIAL_N_B: i32 = 0x1EA;
        pub const SPECIAL_N_Y: i32 = 0x1EB;
        pub const SPECIAL_N_P: i32 = 0x1EC;
        pub const SPECIAL_N_O: i32 = 0x1ED;
        pub const SPECIAL_N_G: i32 = 0x1EE;
    }

    pub mod pickel_trolley {
        pub const PEARL_FLY: i32 = 0x2;
    }

    pub mod purin_disarming_voice {
        pub const SHOOT: i32 = 0x0;
    }

    pub mod reflet {
        pub const FLOAT: i32 = 0x1FD;
    }

    pub mod ryu {
        pub const SPECIAL_LW_INSTALL: i32 = 0x202;
        pub const ATTACK_COMMAND_4: i32 = 0x203;
    }

    pub mod ryu_shinkuhadoken {
        pub const FINISH: i32 = 0x2;
    }

    pub mod samusd {
        pub const FLOAT: i32 = 0x1F6;
    }

    pub mod wolf {
        pub const SPECIAL_S_RUSH: i32 = 0x1EA;
        pub const SPECIAL_S_END: i32 = 0x1EB;
    }
}

pub mod articles {
    pub mod packun {
        pub const FIREBREATH: i32 = 0x4;
    }
    
    pub mod purin {
        pub const DISARMING_VOICE: i32 = 0x2;
    }
}

pub mod melee_mode {
    pub const SMASH: i32 = 0x0;

    pub const CUSTOM_SMASH: i32 = 0x3;
    pub const SUPER_SUDDEN_DEATH: i32 = 0x4;
    pub const SMASHDOWN: i32 = 0x5;
    pub const SPIRIT_BOARD: i32 = 0x6;
    pub const ADVENTURE: i32 = 0x7;

    pub const CLASSIC: i32 = 0x9;
    pub const MOB_SMASH: i32 = 0xa;
    pub const TRAINING: i32 = 0xb;

    pub const HOMERUN_SOLO: i32 = 0xd;
    pub const HOMERUN_CO_OP: i32 = 0xe;
    pub const HOMERUN_VERSUS: i32 = 0xf;
    pub const STAGE_BUILDER: i32 = 0x10;

    pub const ARENA: i32 = 0x13;

    pub const TIPS: i32 = 0x1b;
}

// extra lua_consts
pub const COLLISION_KIND_MASK_PARRY: smash::lib::LuaConst = smash::lib::LuaConst::new(0x80);