use crate::*;

use app::{
    WorkId,
    WorkType,
    WorkKind
};

/// Transition group for grounded specials.
/// 
/// The following transition terms are included in this group:
/// * `CONT_SPECIAL_N`
/// * `CONT_SPECIAL_S`
/// * `CONT_SPECIAL_HI`
/// * `CONT_SPECIAL_LW`
/// * `CONT_SPECIAL_N_COMMAND`
/// * `CONT_SPECIAL_N2_COMMAND`
/// * `CONT_SPECIAL_S_COMMAND`
/// * `CONT_SPECIAL_HI_COMMAND`
/// * `FINAL`
/// * `CONT_SPECIAL_LW_COMMAND`
/// * `CONT_SUPER_SPECIAL`
/// * `CONT_SUPER_SPECIAL2`
/// * `CONT_COMMAND_623NB`
pub const CHECK_GROUND_SPECIAL:  WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0x0);

/// Transition group for grounded item throws
/// 
/// The following transition terms are included in this group:
/// * `CONT_ITEM_THROW`
/// * `CONT_ITEM_THROW_FORCE`
pub const CHECK_GROUND_ITEM:     WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0x1);

/// Transition group for grounded catch
/// 
/// The following transition term is included in this group:
/// * `CONT_CATCH`
pub const CHECK_GROUND_CATCH:    WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0x2);

/// Transition group for grounded attacks
/// 
/// The following transition terms are included in this group:
/// * `CONT_ATTACK`
/// * `CONT_ATTACK_100`
/// * `CONT_ATTACK_S3`
/// * `CONT_ATTACK_HI3`
/// * `CONT_ATTACK_LW3`
/// * `CONT_ATTACK_S4_START`
/// * `CONT_ATTACK_HI4_START`
/// * `CONT_ATTACK_LW4_START`
/// * `CONT_ATTACK_COMMAND1`
/// * `CONT_ITEM_PICKUP_LIGHT`
/// * `CONT_ITEM_PICKUP_HEAVY`
/// * `CONT_ITEM_SWING_4`
/// * `CONT_ITEM_SWING_3`
/// * `CONT_ITEM_SWING`
/// * `CONT_ITEM_SHOOT`
/// * `CONT_ITEM_SHOOT_S3`
/// * `CONT_ITEM_SHOOT_S4`
/// * `CONT_COMMAND_623NB`
/// * `CONT_ATTACK_STAND`
/// * `CONT_ATTACK_SQUAT`
pub const CHECK_GROUND_ATTACK:   WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0x3);

/// Transition group for grounded dodges
/// 
/// The following transition terms are included in this group:
/// * `CONT_ESCAPE`
/// * `CONT_ESCAPE_F`
/// * `CONT_ESCAPE_B`
pub const CHECK_GROUND_ESCAPE:   WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0x4);

/// Transition group for shielding
/// 
/// The following transition term is included in this group:
/// * `CONT_GUARD_ON`
pub const CHECK_GROUND_GUARD:    WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0x5);

/// Transition group for grounded jumps
/// 
/// The following transition terms are included in this group:
/// * `CONT_JUMP_SQUAT`
/// * `CONT_JUMP_SQUAT_BUTTON`
pub const CHECK_GROUND_JUMP:     WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0x6);

/// Transition group for grounded movement options
/// 
/// The following transition terms are included in this group:
/// * `CONT_DASH`
/// * `CONT_TURN`
/// * `CONT_TURN_DASH`
/// * `CONT_SQUAT`
/// * `CONT_APPEAL_U`
/// * `CONT_APPEAL_S`
/// * `CONT_APPEAL_LW`
/// * `CONT_WALK`
pub const CHECK_GROUND:          WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0x7);

/// Transition group for landing
/// 
/// The following transition terms are included in this group:
/// * `LANDING`
/// * `LANDING_LIGHT`
pub const CHECK_AIR_LANDING:     WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0x8);

/// Transition group for grabbing ledge
/// 
/// The following transition term is included in this group:
/// * `CLIFF_CATCH`
pub const CHECK_AIR_CLIFF:       WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0x9);

/// Transition group for aerial specials
/// 
/// The following transition terms are included in this group:
/// * `CONT_SPECIAL_N`
/// * `CONT_SPECIAL_S`
/// * `CONT_SPECIAL_HI`
/// * `CONT_SPECIAL_LW`
/// * `CONT_SPECIAL_N_COMMAND`
/// * `CONT_SPECIAL_N2_COMMAND`
/// * `CONT_SPECIAL_S_COMMAND`
/// * `CONT_SPECIAL_HI_COMMAND`
/// * `FINAL`
/// * `CONT_SPECIAL_LW_COMMAND`
/// * `CONT_SUPER_SPECIAL`
/// * `CONT_SUPER_SPECIAL2`
pub const CHECK_AIR_SPECIAL:     WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0xA);

/// Transition group for aerial item throws
/// 
/// The following transition terms are included in this group:
/// * `CONT_ITEM_THROW`
/// * `CONT_ITEM_THROW_FORCE`
pub const CHECK_AIR_ITEM_THROW:  WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0xB);

/// Transition group for tethers
/// 
/// The following transition term is included in this group:
/// * `CONT_AIR_LASSO`
pub const CHECK_AIR_LASSO:       WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0xC);

/// Transition group for airdodges
/// 
/// The following transition term is included in this group:
/// * `CONT_ESCAPE_AIR`
pub const CHECK_AIR_ESCAPE:      WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0xD);

/// Transition group for aerials
/// 
/// The following transition terms are included in this group:
/// * `CONT_ATTACK_AIR`
/// * `CONT_ITEM_SHOOT_AIR`
pub const CHECK_AIR_ATTACK:      WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0xE);

/// Transition group for midair jumps
/// 
/// The following transition terms are included in this group:
/// * `CONT_TREAD_JUMP`
/// * `CONT_TREAD_JUMP_BUTTON`
/// * `CONT_TREAD_JUMP_NO_TRIGGER`
pub const CHECK_AIR_TREAD_JUMP:  WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0xF);

/// Transition group for wall interactions
/// 
/// The following transition terms are included in this group:
/// * `ATTACH_WALL`
/// * `WALL_JUMP`
pub const CHECK_AIR_WALL_JUMP:   WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0x10);

/// Transition group for jumping aerials
/// 
/// The following transition terms are included in this group:
/// * `CONT_JUMP_AERIAL`
/// * `CONT_JUMP_AERIAL_BUTTON`
/// * `CONT_FLY`
/// * `CONT_FLY_BUTTON`
/// * `CONT_FLY_NEXT`
pub const CHECK_AIR_JUMP_AERIAL: WorkId = WorkId::from_parts(WorkType::Int, WorkKind::TransitionGroup, 0x11);
