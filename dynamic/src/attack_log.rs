use crate::{consts::vars, VarModule};
use smash::app::BattleObject;

pub type StaleMoveQueue = AttackLog<4>;

#[inline]
pub const fn pack_player_seed(player_id: u32, seed: u32) -> i32 {
    let raw: u32 = ((player_id) << 16) | (seed);
    raw as i32
}

#[inline]
pub const fn unpack_player_id(packed: i32) -> u16 {
    ((packed as u32) >> 16) as u16
}

#[inline]
pub const fn unpack_seed(packed: i32) -> u16 {
    (packed as u32 & 0xFFFF) as u16
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct AttackPatternLogEntry {
    /// packed: hi16 = player_id, lo16 = per-player seed
    pub status_seed: i32,
    pub frame_entry: u32,
    pub attack_kind: u64,
}

impl AttackPatternLogEntry {
    #[inline]
    pub const fn new(status_seed: i32, frame_entry: u32, attack_kind: u64) -> Self {
        Self {
            status_seed,
            frame_entry,
            attack_kind,
        }
    }

    #[inline]
    pub const fn new_entry(player_id: u32, seed: u32, frame_entry: u32, attack_kind: u64) -> Self {
        Self {
            status_seed: pack_player_seed(player_id, seed),
            frame_entry,
            attack_kind,
        }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self {
            status_seed: 0,
            frame_entry: 0,
            attack_kind: 0,
        }
    }

    #[inline]
    pub const fn player_id(self) -> u16 {
        unpack_player_id(self.status_seed)
    }

    #[inline]
    pub const fn seed(self) -> u16 {
        unpack_seed(self.status_seed)
    }

    #[inline]
    pub const fn pack_slot0(self) -> u64 {
        // same-width cast
        let hi = self.status_seed as u32 as u64;
        let lo = self.frame_entry as u32 as u64;
        (hi << 32) | lo
    }

    #[inline]
    pub const fn unpack_slot0(slot0: u64) -> (i32, u32) {
        let hi = (slot0 >> 32) as u32 as i32;
        let lo = (slot0 & 0xFFFF_FFFF) as u32;
        (hi, lo)
    }
}

pub struct AttackLog<const N: usize>;

impl<const N: usize> AttackLog<N> {
    pub const FIRST_ENTRY: i32 = vars::common::instance::ATTACK_LOG_ENTRIES;
    pub const STRIDE: i32 = 2; // two u64 slots per entry

    #[inline]
    pub const fn base(i: usize) -> i32 {
        Self::FIRST_ENTRY + (i as i32) * Self::STRIDE
    }
    #[inline]
    pub const fn slot0(i: usize) -> i32 {
        Self::base(i)
    }
    #[inline]
    pub const fn slot1(i: usize) -> i32 {
        Self::base(i) + 1
    }

    #[inline]
    pub unsafe fn clear(object: *mut BattleObject) {
        for i in 0..N {
            VarModule::set_int64(object, Self::slot0(i), 0);
            VarModule::set_int64(object, Self::slot1(i), 0);
        }
    }

    #[inline]
    pub unsafe fn set(object: *mut BattleObject, i: usize, e: AttackPatternLogEntry) {
        VarModule::set_int64(object, Self::slot0(i), e.pack_slot0());
        VarModule::set_int64(object, Self::slot1(i), e.attack_kind);
    }

    #[inline]
    pub unsafe fn get(object: *mut BattleObject, i: usize) -> AttackPatternLogEntry {
        let s0 = VarModule::get_int64(object, Self::slot0(i));
        let attack_kind = VarModule::get_int64(object, Self::slot1(i));
        let (status_seed, frame_entry) = AttackPatternLogEntry::unpack_slot0(s0);
        AttackPatternLogEntry {
            status_seed,
            frame_entry,
            attack_kind,
        }
    }

    pub unsafe fn check_stale(object: *mut BattleObject, entry: AttackPatternLogEntry) -> bool {
        let entry_player = entry.player_id();
        let entry_seed = entry.seed();
        let entry_attack = entry.attack_kind;

        let mut oldest_index: usize = 0;
        let mut oldest_frame: u32 = u32::MAX;

        for i in 0..N {
            let e = Self::get(object, i);

            // Take first empty slot
            if e.frame_entry == 0 {
                Self::set(object, i, entry);
                return false;
            }

            // Track oldest entry
            if e.frame_entry < oldest_frame {
                oldest_frame = e.frame_entry;
                oldest_index = i;
            }

            // Check player
            if e.player_id() != entry_player {
                //println!("Skipping!");
                continue;
            }

            // Same attack seed, refresh frame counter
            if e.seed() == entry_seed {
                // refresh with new frame
                Self::set(object, i, entry);
                return false;
            }

            // New attack seed, check attack staleness
            if e.attack_kind == entry_attack {
                Self::set(object, i, entry);
                return true;
            }
        }
        // Seed and attack are new. Overwrite oldest.
        Self::set(object, oldest_index, entry);
        false
    }
}
