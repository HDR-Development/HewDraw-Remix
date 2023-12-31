use rand::prelude::SliceRandom;
use rand::Rng;

static REGULAR_CHARA_HASHES: &[u64] = &[
    smash::hash40("ui_chara_bayonetta"),
    smash::hash40("ui_chara_captain"),
    smash::hash40("ui_chara_chrom"),
    smash::hash40("ui_chara_cloud"),
    smash::hash40("ui_chara_daisy"),
    smash::hash40("ui_chara_dedede"),
    smash::hash40("ui_chara_diddy"),
    smash::hash40("ui_chara_donkey"),
    smash::hash40("ui_chara_duckhunt"),
    smash::hash40("ui_chara_falco"),
    smash::hash40("ui_chara_fox"),
    smash::hash40("ui_chara_gamewatch"),
    smash::hash40("ui_chara_ganon"),
    smash::hash40("ui_chara_gaogaen"),
    smash::hash40("ui_chara_gekkouga"),
    smash::hash40("ui_chara_ice_climber"),
    smash::hash40("ui_chara_ike"),
    smash::hash40("ui_chara_inkling"),
    smash::hash40("ui_chara_kamui"),
    smash::hash40("ui_chara_ken"),
    smash::hash40("ui_chara_kirby"),
    smash::hash40("ui_chara_koopa"),
    smash::hash40("ui_chara_koopajr"),
    smash::hash40("ui_chara_krool"),
    smash::hash40("ui_chara_link"),
    smash::hash40("ui_chara_littlemac"),
    smash::hash40("ui_chara_lucario"),
    smash::hash40("ui_chara_lucas"),
    smash::hash40("ui_chara_lucina"),
    smash::hash40("ui_chara_luigi"),
    smash::hash40("ui_chara_mario"),
    smash::hash40("ui_chara_marth"),
    smash::hash40("ui_chara_metaknight"),
    smash::hash40("ui_chara_mewtwo"),
    smash::hash40("ui_chara_murabito"),
    smash::hash40("ui_chara_ness"),
    smash::hash40("ui_chara_pacman"),
    smash::hash40("ui_chara_palutena"),
    smash::hash40("ui_chara_peach"),
    smash::hash40("ui_chara_pichu"),
    smash::hash40("ui_chara_pikachu"),
    smash::hash40("ui_chara_pikmin"),
    smash::hash40("ui_chara_pit"),
    smash::hash40("ui_chara_pitb"),
    smash::hash40("ui_chara_ptrainer"),
    smash::hash40("ui_chara_purin"),
    smash::hash40("ui_chara_reflet"),
    smash::hash40("ui_chara_richter"),
    smash::hash40("ui_chara_ridley"),
    smash::hash40("ui_chara_robot"),
    smash::hash40("ui_chara_rockman"),
    smash::hash40("ui_chara_rosetta"),
    smash::hash40("ui_chara_roy"),
    smash::hash40("ui_chara_ryu"),
    smash::hash40("ui_chara_samus"),
    smash::hash40("ui_chara_samusd"),
    smash::hash40("ui_chara_sheik"),
    smash::hash40("ui_chara_shizue"),
    smash::hash40("ui_chara_shulk"),
    smash::hash40("ui_chara_simon"),
    smash::hash40("ui_chara_snake"),
    smash::hash40("ui_chara_sonic"),
    smash::hash40("ui_chara_szerosuit"),
    smash::hash40("ui_chara_toonlink"),
    smash::hash40("ui_chara_wario"),
    smash::hash40("ui_chara_wiifit"),
    smash::hash40("ui_chara_wolf"),
    smash::hash40("ui_chara_yoshi"),
    smash::hash40("ui_chara_younglink"),
    smash::hash40("ui_chara_zelda"),
    //smash::hash40("ui_chara_brave"),
    //smash::hash40("ui_chara_buddy"),
    //smash::hash40("ui_chara_demon"),
    //smash::hash40("ui_chara_dolly"),
    //smash::hash40("ui_chara_edge"),
    //smash::hash40("ui_chara_flame_first"),
    //smash::hash40("ui_chara_light_first"),
    //smash::hash40("ui_chara_jack"),
    //smash::hash40("ui_chara_master"),
    //smash::hash40("ui_chara_packun"),
    //smash::hash40("ui_chara_pickel"),
    //smash::hash40("ui_chara_tantan"),
    //smash::hash40("ui_chara_trail")
];

static PT_CHARA_HASHES: &[u64] = &[
    smash::hash40("ui_chara_pzenigame"),
    smash::hash40("ui_chara_plizardon"),
    smash::hash40("ui_chara_pfushigisou"),
];

static mut COUNTER: usize = 0;

static mut LAST_FIGHTER_FOUND: u64 = 0x0;
static mut LAST_FIGHTER2_FOUND: u64 = 0x0;

static mut WAS_RANDOM_SELECTION: bool = false;

static mut WAS_RANDOM: bool = false;
static mut IS_PRE_ENTRY: bool = false;

const HASH_MASK: u64 = 0xFF_FFFFFFFF;
const KEY_MASK: u64 = 0xFFFFFF_0000000000;

fn is_random(entry: u64) -> bool {
    (entry & HASH_MASK) == smash::hash40("ui_chara_random")
}

fn key(entry: u64) -> u64 {
    entry & KEY_MASK
}

#[skyline::hook(offset = 0x1a13780, inline)]
unsafe fn change_random_early(ctx: &mut skyline::hooks::InlineCtx) {
    let obj = *ctx.registers[23].x.as_ref() as *mut u64;
    let obj = *(obj as *mut *mut u64).add(1);
    println!("Entering change_random_early");
    let ignore_random = ninput::any::is_down_any(ninput::Buttons::ZL | ninput::Buttons::ZR);
    if ignore_random {
        println!("Ignoring the melee random selection!");
    }

    let main_chara = *obj.add(0x200 / 0x8);
    let sub_chara = *obj.add(0x208 / 0x8);

    if !ignore_random && (is_random(main_chara) || is_random(sub_chara)) {
        println!("The random pane was selected");

        let chara_hash = REGULAR_CHARA_HASHES.choose(&mut rand::thread_rng()).copied().unwrap_or(smash::hash40("ui_chara_random"));

        LAST_FIGHTER_FOUND = chara_hash | key(main_chara);
        LAST_FIGHTER2_FOUND = if chara_hash == smash::hash40("ui_chara_ptrainer") {
            PT_CHARA_HASHES.choose(&mut rand::thread_rng()).copied().unwrap_or(smash::hash40("ui_chara_random")) | key(sub_chara)
        } else {
            chara_hash | key(sub_chara)
        };

        println!("Main character: {:#x}", LAST_FIGHTER_FOUND);
        println!("Sub character: {:#x}", LAST_FIGHTER2_FOUND);
        
        println!("Setting x24 to {:#x}", *obj.add(0x200 / 0x8));
        *ctx.registers[24].x.as_mut() = LAST_FIGHTER_FOUND;
        WAS_RANDOM_SELECTION = true;
    } else {
        WAS_RANDOM_SELECTION = false;
    }

}

// only runs on random pane selected
#[skyline::hook(offset = 0x1a0ca40)]
unsafe fn decide_fighter(arg1: u64, arg2: u64, arg3: u64, arg4: u64) -> u64 {
    println!("Entering decide_fighter");
    if !WAS_RANDOM_SELECTION {
        println!("decide_fighter called when the selection was not random");
    }

    let p_main_chara = (arg1 as *mut u64).add(2);
    let p_sub_chara = (arg1 as *mut u64).add(3);

    if WAS_RANDOM_SELECTION && (is_random(*p_main_chara) || is_random(*p_sub_chara)) {
        *p_main_chara = LAST_FIGHTER_FOUND;
        *p_sub_chara = LAST_FIGHTER2_FOUND;
    }
    println!("Decided on fighter: {:#x}", *p_main_chara);

    WAS_RANDOM_SELECTION = false;
    println!("Cleared random selection flag");

    call_original!(arg1, arg2, arg3, arg4)
}

#[skyline::hook(offset = 0x1a1c030)]
unsafe fn copy_fighter_info2(dest: u64, src: u64) {
    let src_obj = *(src as *mut *mut u64).add(1);
    let src_obj = src_obj.add(0x1F0 / 8);
    println!("Entering copy_fighter_info2");

    if WAS_RANDOM_SELECTION {
        *(src_obj as *mut u32).add(8) = rand::thread_rng().gen::<u32>() % 8;

        println!("Randomly selected slot to be {}", *(src_obj as *mut u32).add(8));
    }
    call_original!(dest, src);
}

// #[skyline::hook(offset = 0x1842ec8, inline)]
// unsafe fn pre_entry_assign(ctx: &skyline::hooks::InlineCtx) {
    // let obj = *((*ctx.registers[1].x.as_ref() + 0x8) as *const u64);
    // let obj2 = (obj as *mut u64).add(0x1f0 / 0x8);
    // if (*obj2.add(2) & 0xFF_FFFFFFFF) == hash40("ui_chara_random") {
    //     let chara = *REGULAR_CHARA_HASHES.choose(&mut rand::thread_rng()).unwrap_or(&hash40("ui_chara_random"));
    //     let chara_2 = if chara == smash::hash40("ui_chara_ptrainer") {
    //         *PT_CHARA_HASHES.choose(&mut rand::thread_rng()).unwrap_or(&hash40("ui_chara_random"))
    //     } else {
    //         chara
    //     };
    //     *obj2.add(2) = (*obj2.add(2) & 0xFFFFFF_0000000000) | chara;
    //     *obj2.add(3) = (*obj2.add(3) & 0xFFFFFF_0000000000) | chara_2;
    //     println!("is random!");
    //     WAS_RANDOM = true;
    //     IS_PRE_ENTRY = true;
    // }
    // println!("HERE2");
// }

#[skyline::hook(offset = 0x1797ff8, inline)]
unsafe fn fix_chara_replace(ctx: &skyline::hooks::InlineCtx) {
    let ptr1 = *ctx.registers[0].x.as_ref() as *mut u64;
    let ptr2 = *ctx.registers[1].x.as_ref() as *mut u64;

    *ptr2.add(0x2) = *ptr1.add(0x2);
    *ptr2.add(0x3) = *ptr1.add(0x3);
    *ptr2.add(0x4) = *ptr1.add(0x4);
}

pub fn install() {
    skyline::install_hooks!(
        change_random_early,
        decide_fighter,
        copy_fighter_info2,
        fix_chara_replace
        // pre_entry_assign
    );

    unsafe {
        // skyline::patching::nop_data(0x1797ff8);
    }
}