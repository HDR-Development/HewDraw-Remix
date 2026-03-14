bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct ItemStatus: u32 {
        const Have = 0x1;
        const PutBomb = 0x2;
        const Hammer = 0x4;
        const HammerThrowable = 0x8;
        const Superleaf = 0x10;
        const Rocketbelt = 0x20;
        const Specialflag = 0x40;
        const SpecialflagHoist = 0x80;
        const FirebarLong = 0x100;
        const FirebarMiddle = 0x200;
        const FirebarShort = 0x400;
        const Danger = 0x800;
        const HaveThrow = 0x1000;
        const HaveShoot = 0x2000;
        const HaveSwing = 0x4000;
        const HaveHeavy = 0x8000;
        const Curry = 0x10000;
        const Fireflower = 0x20000;
        const FireflowerFire = 0x40000;
        const Shooting = 0x80000;
        const Superstar = 0x100000;
    }
}

bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct SkillStatus: u32 {
        const MachStamp = 0x1;
        const HighSpeedDash = 0x2;
        const ControllerInvalid = 0x4;
        const HyperSmash = 0x8;
        const SuperarmorSmash = 0x10;
        const JustGuardUp = 0x20;
        const EatInvincible = 0x40;
        const EscapeUp = 0x80;
        const HaveItem = 0x100;
        const SuperArmor = 0x200;
        const SlowSuperArmor = 0x400;
        const JustShieldReflect = 0x800;
        const ShieldHeal = 0x1000;
    }
}

bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct Status: u32 {
        const Air = 0x1;
        const Midair = 0x2;
        const BuildMax = 0x4;
        const BuildUp = 0x8;
        const Rush = 0x10;
        const Gorogoro = 0x20;
        const Attention = 0x40;
        const WeaponOp = 0x80;
        const Final = 0x100;
        const FinalAct = 0x200;
        const CatchW = 0x400;
        const Dead = 0x800;
        const Loupe = 0x1000;
        const Invincible = 0x2000;
        const AtkCatch = 0x4000;
        const Passfall = 0x8000;
        const Weak = 0x10000;
        const DmgElec = 0x20000;
        const Reflect = 0x40000;
        const GorogoroNoCancel = 0x80000;
        const SpDir = 0x100000;
        const UnguardedHind = 0x200000;
        const Unguarded = 0x400000;
        const Combo = 0x800000;
        const NoJump = 0x1000000;
        const NoAttack = 0x2000000;
        const Suspend = 0x4000000;
        const Powerup = 0x8000000;
        const Cancelable = 0x10000000;
        const NoCounter = 0x20000000;
        const NotFreeActually = 0x40000000;
        const Active = 0x80000000;
    }
}

bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct ActionStatus: u32 {
        const TouchL = 0x1;
        const TouchR = 0x2;
        const TouchU = 0x4;
        const TouchD = 0x8;
        const CannotCatchCliff = 0x10;
        const Dive = 0x20;
        const Squat = 0x40;
        const EnableCancel = 0x80;
        const UnableCapture = 0x100;
        const UnableCliffXlu = 0x200;
        const UnableEscapeAir = 0x400;
        const UnableAttack = 0x800;
        const UnableSpecial = 0x1000;
        const UnableJump = 0x2000;
        const UnableShield = 0x4000;
    }
}

bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct WeakenStatus: u32 {
        const StickReverseX = 0x1;
        const StickReverseY = 0x2;
        const StickConfuseX = 0x4;
        const StickConfuseY = 0x8;
        const DogsBlindOwn = 0x10;
        const DogsBlindConfuse = 0x20;
        const TargetInvisible = 0x40;
    }
}

bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct SpiritsEventStatus: u32 {
        const Speedup = 0x1;
    }
}

bitflags! {
    #[repr(C)]
    #[allow(non_upper_case_globals)]
    pub struct CharacterStatus: u32 {
        const Mario_MantleHop = 0x1;
        const Mario_Pomp = 0x2;
        const Mariod_SpLwHop = 0x1;
        const Donkey_Headatk = 0x1;
        const Donkey_SpD = 0x2;
        const Donkey_LiftU = 0x4;
        const Yoshi_DisSpHi = 0x1;
        const Link_SpS = 0x1;
        const Link_SpLw = 0x2;
        const Pikachu_Kaminari = 0x1;
        const Kirby_Eat = 0x1;
        const Kirby_UseMetaknight = 0x2;
        const Kirby_UsePurin = 0x4;
        const Kirby_Stone = 0x8;
        const Kirby_FCutter = 0x10;
        const Kirby_Copy = 0x20;
        const Kirby_RockmanSpMetalblade = 0x40;
        const Kirby_PacmanFruits = 0x80;
        const Kirby_RyuHadouken = 0x100;
        const Kirby_RyuCommand236Step = 0x200;
        const Kirby_RyuCommand41236Step = 0x400;
        const Kirby_BraveSpN3Ready = 0x800;
        const Kirby_DollySpN = 0x1000;
        const Kirby_MasterSpNHold = 0x2000;
        const Kirby_PickelSpNBlock = 0x4000;
        const Kirby_DemonSpN = 0x8000;
        const Fox_Blaster = 0x1;
        const Fox_SpAirS = 0x2;
        const Ness_ThdWait = 0x1;
        const Koopa_DiveJump = 0x1;
        const Peach_Float = 0x1;
        const Peach_DisFloat = 0x2;
        const GwSt_DisSpHi = 0x1;
        const Gw_OilMax = 0x2;
        const Wario_BikeReady = 0x1;
        const Wario_GassMax = 0x2;
        const Wario_Bite = 0x4;
        const Meta_SpLw = 0x2;
        const Pikmin_Empty = 0x1;
        const Pikmin_Final = 0x2;
        const Pikmin_Heavy = 0x4;
        const Pikmin_DisSpHi = 0x8;
        const Diddy_DisSpS = 0x1;
        const Diddy_SpSCatch = 0x2;
        const Diddy_DisSpHi = 0x4;
        const Diddy_NoBanana = 0x8;
        const Diddy_DisSpAll = 0x10;
        const Dedede_Eat = 0x1;
        const Dedede_SpHi = 0x2;
        const Dedede_SpHiFall = 0x4;
        const Dedede_GordoSet = 0x8;
        const Ike_SpUNoCliff = 0x1;
        const Robot_GyroReady = 0x1;
        const Robot_BeamReady = 0x2;
        const Robot_BurnerLow = 0x4;
        const Zelda_Phantom = 0x1;
        const Zelda_SpDein = 0x2;
        const Sheik_SpAirD = 0x1;
        const Sonic_DisSpHi = 0x1;
        const Wiifit_SpS = 0x1;
        const Wiifit_Breathing = 0x2;
        const Wiifit_BreathingGauge = 0x4;
        const Wiifit_BreathEffect = 0x8;
        const Wiifit_BreathRestore = 0x10;
        const Murabito_EnableTakeoutOut = 0x1;
        const Murabito_TakeoutOutRangeS = 0x2;
        const Murabito_TakeoutOutRangeM = 0x4;
        const Murabito_TakeoutOutRange = 0x6;
        const Murabito_DisSpHi = 0x8;
        const Murabito_SpClayrocket = 0x10;
        const Murabito_SpSeed = 0x20;
        const Murabito_SpSprout = 0x40;
        const Murabito_SpTree = 0x80;
        const Murabito_SpNearSprout = 0x100;
        const Murabito_SpNearTree = 0x200;
        const Rockman_Leafshield = 0x1;
        const Rockman_SpRashcoil = 0x2;
        const Rockman_SpCrashbomb = 0x4;
        const Rockman_SpMetalblade = 0x8;
        const Rosetta_TicoFree = 0x1;
        const Rosetta_TicoDead = 0x2;
        const Rosetta_TicoMidair = 0x4;
        const Rosetta_SpLwCancel = 0x8;
        const Littlemac_KoGageMax = 0x1;
        const Littlemac_SpS = 0x2;
        const Reflet_SpNReady = 0x1;
        const Reflet_SpSReady = 0x2;
        const Reflet_SpHiReady = 0x4;
        const Reflet_SpLwReady = 0x8;
        const Reflet_ThunderSwordReady = 0x10;
        const Duckhunt_Clay = 0x1;
        const Duckhunt_Can = 0x2;
        const Duckhunt_Gunman = 0x4;
        const Pacman_Firehydrant = 0x1;
        const Pacman_Fruits = 0x2;
        const Pacman_DisSpS = 0x4;
        const Pacman_DisSpU = 0x8;
        const Szerosuit_Final = 0x1;
        const Szerosuit_SpAirLw = 0x2;
        const Szerosuit_SpAirLwKick = 0x4;
        const Koopajr_SpCanon = 0x1;
        const Koopajr_DisSpS = 0x2;
        const Koopajr_DisSpHi = 0x4;
        const Koopajr_SpMechakoopa = 0x8;
        const Lucario_Wavehigh = 0x1;
        const Shulk_MonadSelect = 0x1;
        const Shulk_MonadJump = 0x2;
        const Shulk_MonadSpeed = 0x4;
        const Shulk_MonadShield = 0x8;
        const Shulk_MonadBuster = 0x10;
        const Shulk_MonadSmash = 0x20;
        const Shulk_MonadEffected = 0x40;
        const Shulk_MonadJumpUnable = 0x80;
        const Shulk_MonadSpeedUnable = 0x100;
        const Shulk_MonadShieldUnable = 0x200;
        const Shulk_MonadBusterUnable = 0x400;
        const Shulk_MonadSmashUnable = 0x800;
        const Miifighter_SpLw2 = 0x1;
        const Miifighter_SpLw2Kick = 0x2;
        const Miiswordsman_SpS3 = 0x1;
        const Miigunner_SpStealth = 0x2;
        const Ryu_Hadouken = 0x1;
        const Ryu_SpAirNHop = 0x2;
        const Ryu_SpAirS = 0x4;
        const Ryu_Command236Step = 0x8;
        const Ryu_Command41236Step = 0x10;
        const Ryu_Command214Step = 0x20;
        const Ryu_Command623Step = 0x40;
        const Ken_Command632Step = 0x80;
        const Ryu_AttackCancel = 0x100;
        const Ryu_HitCancel = 0x200;
        const Bayonetta_SpAirS = 0x1;
        const Bayonetta_SpHi = 0x2;
        const Bayonetta_WitchTimeSuccess = 0x4;
        const Bayonetta_Final = 0x10;
        const Bayonetta_FinalMax = 0x20;
        const Kamui_SpS = 0x1;
        const Kamui_SpSWall = 0x2;
        const Popo_RubberJump = 0x1;
        const Popo_Single = 0x2;
        const Popo_SpHiCouple = 0x4;
        const Nana_RubberJump = 0x1;
        const Inkling_InkLow = 0x1;
        const Inkling_SpLw = 0x2;
        const Snake_SpN = 0x1;
        const Snake_SpSWorking = 0x2;
        const Snake_SpHi = 0x4;
        const Snake_SpLwReady = 0x8;
        const Snake_SpLwBomb = 0x10;
        const Snake_SpHiUsing = 0x20;
        const Krool_SpNSwallow = 0x1;
        const Krool_SpS = 0x2;
        const Simon_SpN = 0x1;
        const Simon_SpS = 0x2;
        const Simon_SpLw = 0x4;
        const Shizue_SpLwReady = 0x8;
        const Shizue_SpLwBomb = 0x10;
        const Gaogaen_SpS = 0x1;
        const Gaogaen_SpSLariat = 0x2;
        const Gaogaen_Revenge = 0x4;
        const Koopag_SpNLow = 0x1;
        const Pokemon_DisSpLw = 0x1;
        const Packun_SpN = 0x1;
        const Packun_SpS = 0x2;
        const Jack_DoyleExist = 0x1;
        const Jack_RebelGaugeMax = 0x2;
        const Jack_SpHiHop = 0x4;
        const Jack_SpUNoCliff = 0x8;
        const Jack_SpNNoEscape = 0x10;
        const Brave_SpN1Ready = 0x1;
        const Brave_SpN2Ready = 0x2;
        const Brave_SpN3Ready = 0x4;
        const Brave_SpS1Ready = 0x8;
        const Brave_SpS2Ready = 0x10;
        const Brave_SpS3Ready = 0x20;
        const Brave_SpHi1Ready = 0x40;
        const Brave_SpHi2Ready = 0x80;
        const Brave_SpHi3Ready = 0x100;
        const Brave_SpN1Hold = 0x200;
        const Brave_SpN2Hold = 0x400;
        const Brave_SpN3Hold = 0x800;
        const Brave_SpS1Hold = 0x1000;
        const Brave_SpS2Hold = 0x2000;
        const Brave_SpS3Hold = 0x4000;
        const Brave_SpHi1Hold = 0x8000;
        const Brave_SpHi2Hold = 0x10000;
        const Brave_SpHi3Hold = 0x20000;
        const Brave_SpLw = 0x40000;
        const Brave_SpeedUp = 0x80000;
        const Brave_SpLw11 = 0x100000;
        const Brave_SpLw16 = 0x200000;
        const Buddy_NoBomb = 0x1;
        const Buddy_SpS = 0x2;
        const Buddy_SpLwHop = 0x4;
        const Buddy_SpHi = 0x8;
        const Buddy_SpNTransform = 0x10;
        const Dolly_SpN = 0x1;
        const Dolly_SpS = 0x2;
        const Dolly_SpSuper = 0x4;
        const Dolly_Command236Step = 0x8;
        const Dolly_Command214Step = 0x10;
        const Dolly_Command623Step = 0x20;
        const Dolly_Command28Step = 0x40;
        const Dolly_Command236236Step = 0x80;
        const Dolly_Command21416Step = 0x100;
        const Dolly_Command214214Step = 0x200;
        const Dolly_Command23634Step = 0x400;
        const Dolly_EscapeAttack = 0x800;
        const Dolly_HitCancel = 0x1000;
        const Dolly_ReturnSpB = 0x2000;
        const Dolly_TurnSpB = 0x4000;
        const Master_SpNDisableCancel = 0x1;
        const Master_SpHiHop = 0x2;
        const Master_SpHiHop2 = 0x4;
        const Master_SpUNoCliff = 0x8;
        const Master_SpLwEnableTurn = 0x10;
        const Master_SpNHold = 0x20;
        const Tantan_LeftReady = 0x1;
        const Tantan_RightReady = 0x2;
        const Tantan_AppendAttack = 0x4;
        const Tantan_LeftRewind = 0x8;
        const Tantan_RightRewind = 0x10;
        const Tantan_Dragon = 0x20;
        const Tantan_SpHiHop = 0x40;
        const Tantan_SpUNoCliff = 0x80;
        const Tantan_SpLw = 0x100;
        const Tantan_AtkFConfirm = 0x200;
        const Pickel_AttackAirLw = 0x1;
        const Pickel_AttackAirLwCancelable = 0x2;
        const Pickel_AttackThrowLw = 0x4;
        const Pickel_SpNDigging = 0x8;
        const Pickel_SpNCraft = 0x10;
        const Pickel_SpNWorkBench = 0x20;
        const Pickel_SpNBlock = 0x40;
        const Pickel_OnSpNBlock = 0x80;
        const Pickel_SpF = 0x100;
        const Pickel_SpFRail = 0x200;
        const Pickel_SpFPoweredRail = 0x400;
        const Pickel_SpFTrolley = 0x800;
        const Pickel_SpFJump = 0x1000;
        const Pickel_SpU = 0x2000;
        const Pickel_SpUCancelable = 0x4000;
        const Pickel_SpUNoRise = 0x8000;
        const Pickel_SpD = 0x10000;
        const Pickel_SpDTntExist = 0x20000;
        const Pickel_SpDDisWire = 0x40000;
        const Eflame_DisSpLw = 0x1;
        const Eflame_NoSword = 0x2;
        const Elight_DisSpLw = 0x1;
        const Demon_SpN = 0x1;
        const Demon_SpF = 0x2;
        const Demon_SpU = 0x4;
        const Demon_Rage = 0x8;
        const Demon_DiveJump = 0x10;
        const Edge_SpWing = 0x1;
        const Trail_SpUAdd = 0x1;
    }
}
