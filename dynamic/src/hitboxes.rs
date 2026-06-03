pub use proc_hitbox::{
    CollisionPart,
    CollisionSituation,
    CollisionCategory,
    ShieldDamage,
    SoundLevel,
    CollisionSound,
    AttackRegion,
    SetOff,
    LrCheck,
    HitboxTemplate,
    HitboxData,
    create_hitbox,
};
pub use proc_hitbox_macro::{
    hitbox_templates,
    decl_hitbox,
    hitbox
};

// hitbox_templates
// the final result MUST include all of these:
    // part: 0,
    // fkb: 0,
    // hitlag: 1.0,
    // sdi: 1.0,
    // clank: SetOff::On,
    // facing: LrCheck::Pos,
    // set_weight: false,
    // shield_dmg: ShieldDamage::Add(0.0),
    // trip: 0.0,
    // rehit: 0,
    // reflectable: false,
    // absorbable: false,
    // flinchless: false,
    // disable_hitlag: false,
    // direct: true,
    // friendly_fire: false,
    // situation: CollisionSituation::GA,
    // category: CollisionCategory::all(),
    // hit_part: CollisionPart::all(),
    // effect: "collision_attr_normal",
    // hit_sound: CollisionSound::Punch,
    // sound_level: SoundLevel::M,
    // region: AttackRegion::Punch,

// these are also required, but should be included in the hitbox data itself:
    // id: 0, 
    // bone: "top",
    // dmg: 9.0,
    // angle: 361
    // kbg: 100,
    // bkb: 0,
    // size: 4.0,
    // x: 0.0,
    // y: 0.0,
    // z: 0.0,

// optional fields for capsule hitboxes:
    // x2: 0.0,
    // y2: 0.0,
    // z2: 0.0,

hitbox_templates! {
    pub BASE_HITBOX = {
        part: 0,
        fkb: 0,
        hitlag: 1.0,
        sdi: 1.0,
        clank: SetOff::On,
        facing: LrCheck::Pos,
        set_weight: false,
        shield_dmg: ShieldDamage::Add(0.0),
        trip: 0.0,
        rehit: 0,
        reflectable: false,
        absorbable: false,
        flinchless: false,
        disable_hitlag: false,
        direct: true,
        friendly_fire: false,
        situation: CollisionSituation::GA,
        category: CollisionCategory::all(),
        hit_part: CollisionPart::all(),
    };
    pub PHYSICAL_PROJECTILE_HITBOX = {
        extends: BASE_HITBOX,
        facing: LrCheck::Speed,
        reflectable: true,
        direct: false,
    };
    pub ENERGY_PROJECTILE_HITBOX = {
        extends: PHYSICAL_PROJECTILE_HITBOX,
        absorbable: true,
    };
    pub BASE_WINDBOX = {
        extends: BASE_HITBOX,
        clank: SetOff::Off,
        facing: LrCheck::F,
        flinchless: true,
        disable_hitlag: true,
        direct: false,
        category: CollisionCategory::all().difference(CollisionCategory::FLOOR),
        effect: "collision_attr_normal",
        sound_level: SoundLevel::S,
        hit_sound: CollisionSound::None,
        region: AttackRegion::None,
    };
    pub BASE_SEARCHBOX = {
        extends: BASE_WINDBOX,
        dmg: 0.0,
        angle: 361,
        kbg: 0,
        fkb: 0,
        bkb: 0,
        effect: "collision_attr_search",
    };
}