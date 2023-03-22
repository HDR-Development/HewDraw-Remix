#[derive(Debug, Copy, Clone)]
#[repr(u32)]
enum EventId {
    SituationKindChanged = 0x0,
    StatusKindChanged = 0x1,
    AttackInflict = 0x2,
    /* Unknown = 0x3, */
    HitPause = 0x4,
    MotionKindChanged = 0x5,
    /* Unknown = 0x6, */
    WholeHitStatusChanged = 0x7,
    /* Unknown = 0x8, */
    ReceiveInflict = 0x9,
    LinkEvent = 0xA,
    ApplyDamage = 0xB, // Turned off by setting "No Damage" in Training
    UpdateTrainingModeCounter = 0xC, // Only run when "No Damage" is set in Training (or by an item ig)
    /* Unknown = 0xD, Knockback related */
    SearchCollision = 0xE,
    CallUserSearchCollision = 0xF,
    /* Unknown 0x10 */
    /* Unknown 0x11 */
    /* Unknown 0x12 */
    CheckShieldCollisionValidity = 0x13,
    ShieldCollisionHandled = 0x14,
    /* Unknown 0x15 */
    /* Unknown 0x16 */
    CheckReflectorCollisionValidity = 0x17,
    AbsorberAttackCollision = 0x18,
    CheckAbsorberCollisionValidity = 0x19,
    GrabInflict = 0x1A,
    /* Unknown 0x1B */
    TeamChangedEvent = 0x1C,
    /* Unknown 0x1D */
    /* Unkonwn 0x1E */
    PickupItem = 0x1F,
    ReleaseItem = 0x20,
    EquipItem = 0x21,
    UnequipItem = 0x22,
    UseTimer = 0x23,  // Might be more generic but I could only get it to proc on this
    UseTimer2 = 0x24, // Only saw this inside of UseTimer event
    DropItem = 0x25,
    /* Unknown 0x26 */
    /* Unknown 0x27 (has to do with item placement) */
    ShootItem = 0x28,
    /* Unknown 0x29 */
    FlipEvent = 0x2A, // param_motion.flip
    /* Unknown 0x2B */
    /* Unknown 0x2C */
    /* Unknown 0x2D */
    /* Unknown 0x2E */
    SetSlow = 0x2F,
    /* Unknown 0x30 (has to do with articlemodule) */
    ResetLogActionInfo = 0x30,
    /* Unknown 0x31 */
}

pub trait OnSituationKindChanged: Sized {
    fn on_situation_kind_changed(&mut self, prev: i32, new: i32);

    fn listen(&mut self, module_accessor: *mut smash::app::BattleObjectModuleAccessor) {
        let module_accessor = module_accessor as *mut smash_rs::app::BattleObjectModuleAccessor;
        unsafe {
            (*module_accessor)
                .event_manager()
                .get_event_listener_lists_mut(0)[EventId::SituationKindChanged as u32 as usize]
                .add_to_front(smash_rs::lib::EventListener::new(
                    handle_situation_kind_changed_event::<Self>,
                    self,
                ))
        }
    }
}

extern "C" fn handle_situation_kind_changed_event<S: Sized + OnSituationKindChanged>(
    event: &smash_rs::lib::BasicEvent,
    this: *mut u8,
) {
    let event = downcast::<SituationKindChangedEvent>(event, EventId::SituationKindChanged);
    unsafe {
        let this = &mut *(this as *mut S);
        this.on_situation_kind_changed(event.prev_situation_kind, event.new_situation_kind);
    }
}

pub trait OnStatusKindChanged: Sized {
    fn on_status_kind_changed(
        &mut self,
        prev: i32,
        new: i32,
        kinetic_type: i32,
        succeeds_bits: u32,
        ground_correct_kind: i32,
        ground_cliff_check_kind: i32,
    );

    fn listen(&mut self, module_accessor: *mut smash::app::BattleObjectModuleAccessor) {
        let module_accessor = module_accessor as *mut smash_rs::app::BattleObjectModuleAccessor;
        unsafe {
            (*module_accessor)
                .event_manager()
                .get_event_listener_lists_mut(0)[EventId::StatusKindChanged as u32 as usize]
                .add_to_front(smash_rs::lib::EventListener::new(
                    handle_status_kind_changed_event::<Self>,
                    self,
                ))
        }
    }
}

extern "C" fn handle_status_kind_changed_event<S: Sized + OnStatusKindChanged>(
    event: &smash_rs::lib::BasicEvent,
    this: *mut u8,
) {
    let event = downcast::<StatusKindChangedEvent>(event, EventId::StatusKindChanged);
    unsafe {
        let this = &mut *(this as *mut S);
        this.on_status_kind_changed(
            event.prev_status_kind,
            event.new_status_kind,
            event.kinetic_type,
            event.succeeds_bits,
            event.ground_correct_kind,
            event.ground_cliff_check_kind,
        )
    }
}

fn downcast<T>(event: &smash_rs::lib::BasicEvent, id: EventId) -> &T {
    if event.id != id as u32 {
        panic!(
            "Event called with invalid id. Expected {:?}, found {}",
            id, event.id
        );
    }

    unsafe { std::mem::transmute(event) }
}

#[repr(C)]
struct SituationKindChangedEvent {
    vtable: &'static (),
    id: u32,
    new_situation_kind: i32,
    prev_situation_kind: i32,
}

#[repr(C)]
struct StatusKindChangedEvent {
    vtable: &'static (),
    id: u32,
    new_status_kind: i32,
    prev_status_kind: i32,
    agent_status_data: *const u8,
    prev_agent_status_data: *const u8,
    kinetic_type: i32,
    succeeds_bits: u32,
    ground_correct_kind: i32,
    ground_cliff_check_kind: i32,
}
