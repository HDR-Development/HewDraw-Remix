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

        // int
        pub const LAST_ATTACK_RECEIVER_ENTRY_ID: i32 = 0x0;

        // float
        /// var to store the damage that this fighter DEALT as the attacker.
        pub const LAST_ATTACK_DAMAGE_DEALT: i32 = 0x0;

        // separator
        // flag
        pub const SPECIAL_PROJECTILE_SPAWNED: i32 = 0x50;
        pub const UP_SPECIAL_CANCEL: i32 = 0x51;

        // int
        pub const GIMMICK_TIMER: i32 = 0x50;
        

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

    pub mod mario {
        // flags
        pub const FIREBRAND_SPAWNED: i32 = 0x1000;
        pub const DOUBLE_FIREBALL: i32 = 0x1001;
    }

    pub mod roy {
        // flags
        pub const TRAIL_EFFECT: i32 = 0x1000;
    }

    mod shotos {
        // flags
        pub const IS_USE_EX_SPECIAL: i32 = 0x1000;
        pub const IS_MAGIC_SERIES_CANCEL: i32 = 0x1001;
        pub const SHOULD_COMBOS_SCALE: i32 = 0x1002;

        // ints
        pub const REPEAT_COUNT_LW: i32 = 0x1000;
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

    pub mod gaogen {
        // floats
        pub const ANGLE_GRAB_STICK_Y: i32 = 0x1000;
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

    }

    pub mod miigunner {
        // floats
        pub const CHARGE_ATTACK_LEVEL: i32 = 0x1000;
        
        // flags
        pub const IS_CHARGE_FINISHED: i32 = 0x1000;
    }
}
