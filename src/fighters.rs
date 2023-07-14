
macro_rules! decl_init {
    ($($fighter:ident: $name:expr),*) => {
        pub fn install(is_runtime: bool) {
            #[cfg(not(feature = "no-engine"))]
            { engine::install(); }
            $(
                #[cfg(feature = $name)]
                { $fighter::install(is_runtime); }
            )*
        }
    }
}

macro_rules! decl_lazy_init {
    ($($fighter:ident: $name:expr),*) => {
        pub fn lazy_install() {
            #[cfg(not(feature = "no-engine"))]
            { engine::install(); }
            $(
                #[cfg(feature = $name)]
                { $fighter::delayed_install(); }
            )*
        }
    }
}

decl_init! {
    pitb: "pitb",
    eflame: "eflame",
    pfushigisou: "pfushigisou",
    purin: "purin",
    younglink: "younglink",
    plizardon: "plizardon",
    shulk: "shulk",
    chrom: "chrom",
    miifighter: "miifighter",
    gamewatch: "gamewatch",
    krool: "krool",
    samusd: "samusd",
    ryu: "ryu",
    rosetta: "rosetta",
    yoshi: "yoshi",
    lucas: "lucas",
    link: "link",
    lucario: "lucario",
    zelda: "zelda",
    kamui: "kamui",
    wolf: "wolf",
    nana: "nana",
    tantan: "tantan",
    koopag: "koopag",
    ridley: "ridley",
    gekkouga: "gekkouga",
    ike: "ike",
    roy: "roy",
    rockman: "rockman",
    peach: "peach",
    elight: "elight",
    gaogaen: "gaogaen",
    falco: "falco",
    ness: "ness",
    kirby: "kirby",
    pikachu: "pikachu",
    daisy: "daisy",
    sheik: "sheik",
    mariod: "mariod",
    luigi: "luigi",
    captain: "captain",
    mario: "mario",
    jack: "jack",
    palutena: "palutena",
    dolly: "dolly",
    pickel: "pickel",
    wiifit: "wiifit",
    pzenigame: "pzenigame",
    pichu: "pichu",
    pit: "pit",
    ken: "ken",
    brave: "brave",
    richter: "richter",
    miiswordsman: "miiswordsman",
    robin: "robin",
    master: "master",
    robot: "robot",
    szerosuit: "szerosuit",
    ganon: "ganon",
    wario: "wario",
    diddy: "diddy",
    shizue: "shizue",
    littlemac: "littlemac",
    pacman: "pacman",
    edge: "edge",
    marth: "marth",
    trail: "trail",
    mewtwo: "mewtwo",
    metaknight: "metaknight",
    toonlink: "toonlink",
    koopajr: "koopajr",
    sonic: "sonic",
    donkey: "donkey",
    inkling: "inkling",
    packun: "packun",
    miigunner: "miigunner",
    buddy: "buddy",
    pikmin: "pikmin",
    cloud: "cloud",
    fox: "fox",
    duckhunt: "duckhunt",
    lucina: "lucina",
    murabito: "murabito",
    bayonetta: "bayonetta",
    samus: "samus",
    koopa: "koopa",
    simon: "simon",
    popo: "popo",
    dedede: "dedede",
    snake: "snake",
    demon: "demon"
}

decl_lazy_init! {
    elight: "elight",
    ganon: "ganon",
    littlemac: "littlemac",
    wolf: "wolf",
    diddy: "diddy"
}