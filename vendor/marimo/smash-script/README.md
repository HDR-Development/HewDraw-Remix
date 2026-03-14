# `smash-script` and `lua-replace`
The `smash_script` crates is designed to utilize and work with the `liblua_replace.nro` dependency. It reimplements everything that `skyline-acmd` and `libacmd_hook.nro` do in much more efficient ways.

## Dependencies
The development of this tool required updating and changes to both `skyline` (`subsdk9`) and `nro-hook-plugin` (`libnro_hook.nro`). These will need to be updated, along with the installation of `liblua_replace.nro`, which can be found at https://github.com/marimo/lua-replace

## Replacement
Script replacement is done via the use of the `#[script(...)]` attribute and currently only supports fighters and weapons. Other types of scripts the game handles
(namely item scripts) are not yet implemented but are planned for future releases.

## Usage
Unlike `libacmd_hook` and `skyline-acmd`, the `smash-script` create does not support the use of the `acmd!` macro to generate valid replacement scripts. For more information on why, check the technical information.

### ACMD (AnimCMD) Scripts
Replacing scripts works for all of the animcmd script types: `game` (attack), `effect`, `sound`, and `expression` (misc.). A script can be created as follows:
```rust
use smash_script::script;
use smash_script::macros;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::sv_animcmd;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;

#[script( agent = "mariod", scripts = [ "game_specialn", "game_specialairn" ])]
fn double_pill(fighter: &mut L2CFighterCommon) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    println!("starting custom script!");
    sv_animcmd::frame(lua_state, 12.0);
    if macros::is_excute(fighter) {
        println!("running custom script!");
        ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, false, -1);
    }
    sv_animcmd::frame(lua_state, 20.0);
    if macros::is_excute(fighter) && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
        ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, false, -1);
    }
}

/* Rest of the plugin goes here */

pub fn install() {
    smash_script::replace_scripts!(double_pill);
}
```

Let's break down the above script. The `script` macro attribute takes two attributes: `agent` and `script(s)`.
* `agent` is supposed to be the name of the fighter or article which will be using this script. In the above example, this agent is `"mariod"`, or Dr. Mario. It also works for weapons, so `agent = "mariod_drcapsule"` would mean that we are replacing a script (or scripts) for Dr. Mario's capsule weapon.
* `script` is used for replacing a single script. It takes the name of the script (if you are familiar with `skyline-acmd`, this is the `animcmd` attribute). It's sibling attribute, `scripts`, is the same but instead takes an array of script names which it should replace. All of them get installed with the same call, and it is designed to ease replacing multiple scripts which appear to be the same (or need to be the same for a mod).

If you are familiar with `skyline-acmd`, you might be confused at something: why do I use a `println!` macro at the beginning of the function? Won't that print every frame? The short answer is: no. These scripts, just like vanilla, run **once**. This means you can use as many local variables as you want without fear of them getting replaced or reinitialized each frame.
#### "There Be Dragons": A note about using local variables
Due to the way exception handling and stack unwinding was implemented, local variables are *usable*, but I would strongly advocate not using local variables which allocate dynamic memory (heap) on creation and free it on their destruction (or drop), as these methods will never get called should the script be interrupted. 

### Once-per-frame Scripts
Another great feature of `acmd_hook` which has been reimplemented in `lua-replace` is that of once-per-frame fighter and weapon callbacks. There is a downside, though, which is that you have to match the fighter kind in *every single callback* that you use. These have been reimplemented to use a macro attribute and now directly replace the fighter's own system control function. The original function is still called immediately after the user's replace, just like `acmd_hook`.

Making a fighter/weapon specific once-per-frame hook is as simple as the following:
```rust
use smash_script::{ fighter_frame, weapon_frame };

#[fighter_frame( agent = FIGHTER_KIND_MARIOD )]
unsafe fn dr_mario_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let kind = smash::app::utility::get_kind(boma);
    println!("Calling once-per-frame with Dr. Mario: {:X}", kind);
}

#[weapon_frame( agent = WEAPON_KIND_MARIOD_DRCAPSULE )]
unsafe fn capsule_frame(fighter: &mut L2CFighterBase) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    let kind = smash::app::utility::get_kind(boma);
    println!("Calling once-per-frame with Dr. Mario\'s Capsule: {:X}", kind);
}

unsafe fn global_fighter_frame(fighter: &mut L2CFighterCommon) {
    // global fighter mods here
}

unsafe fn global_weapon_frame(fighter: &mut L2CFighterBase) {
    // global weapon mods here
}

/* Rest of plugin goes here */

pub fn install() {
    smash_script::replace_fighter_frames!(dr_mario_frame);
    smash_script::replace_weapon_frames!(capsule_frame);
    smash_script::add_fighter_frame_callbacks!(global_fighter_frame);
    smash_script::add_weapon_frame_callbacks!(global_weapon_frame);
}
```

The output of the above plugin while playing as Dr. Mario with any other fighters/weapons would be the following
```
Calling once-per-frame with Dr. Mario: 12
Calling once-per-frame with Dr. Mario: 12
Calling once-per-frame with Dr. Mario: 12
Calling once-per-frame with Dr. Mario: 12
Calling once-per-frame with Dr. Mario: 12
Calling once-per-frame with Dr. Mario: 12
Calling once-per-frame with Dr. Mario: 12
Calling once-per-frame with Dr. Mario: 12
Calling once-per-frame with Dr. Mario: 12
Calling once-per-frame with Dr. Mario's Capsule: D7
Calling once-per-frame with Dr. Mario: 12
Calling once-per-frame with Dr. Mario's Capsule: D7
Calling once-per-frame with Dr. Mario: 12
Calling once-per-frame with Dr. Mario's Capsule: D7
Calling once-per-frame with Dr. Mario: 12
Calling once-per-frame with Dr. Mario's Capsule: D7
Calling once-per-frame with Dr. Mario: 12
Calling once-per-frame with Dr. Mario's Capsule: D7
Calling once-per-frame with Dr. Mario: 12
Calling once-per-frame with Dr. Mario's Capsule: D7
```

You can only replace the once-per-frame script **once** (this also goes for regular AnimCMD script replacements). Any additional logic that needs to be separated should continue to use global callbacks, which work separately from once-per-frame script replacments. You can have as many global callbacks as you would like, but only one replacement script per fighter/weapon.

### Adding new scripts
Currently, an *extremely* experimental feature is the ability to add new scripts into the game. This is under active research by myself. You might ask why it is under research if `acmd_hook` already provides that functionality, and the answer is pretty simple:

`L2CAgentBase::call_coroutine` searches a table of hashes for a function. If it finds it, it gets called. If it doesn't, it doesn't get called. `libacmd_hook` intercepts this check with its own, and since the user provides their own functions, `call_coroutine` would get redirected to the users function. `liblua_replace` does not intercept this check and relies on supplying the game with the correct information it needs to call the function on its own. Currently, adding scripts with `lua_replace` will *only* work with fighters and will add the script to all of their `L2CFighterAnimcmd<>Common` agents. It is unknown when this will receive proper implementation.

The following is how to add a script (or multiple) to the game
```rust
#[script( agent = "koopa", scripts = ["game_appealhil", "game_appealhir"], new )]
```

## Technical Information
### Runtime complexity and performance increases
Currently, `acmd_hook` searches a vector which contains every single hook installed on every frame for every fighter. This is an O(n) complexity multiple times each frame during a match. While not an issue for smaller mods, larger mods that aim to reimplement or change the majority of fighters and scripts in the game begin to have noticeable performance dips when introducing more than 2 fighters to the screen.

`lua-replace` also has an O(n) complexity, however the performance is essentially identical to vanilla. During the match loading sequence, `lua-replace` checks the fighters being loaded (as well as the articles) and replaces their script with a sequential search of a function vector. Should this be an issue due to timing, this can be reworked to be faster but it shouldn't make a large difference. During the actual battle, if a script runs to completion there should be no difference in execution. If it gets interrupted, there might be a small difference in execution and timing, but it should be negligble. In total, the performance benefits will become more noticable the more mods you add to the game.
### Compatibility with `acmd_hook` script edits
`acmd_hook` hooks will be compatible, however currently two functions in the game get replaced by both plugins. It is unknown what will happen in this instance. Should this not be a problem, any `acmd_hook` script edits will take priority over `lua-replace` script replacements due to the nature of both plugins.
### Notable bugs from `skyline-acmd` that are no longer present
* Frame 1 modifications are now fully functional and can be expected to work 100% of the time, since we are as close to the original game's process as possible.
* The notorious `is_excute` bug no longer exists. This wasn't really a "bug" but rather a side-effect of the way that `acmd_hook` was implemented. Since the scripts ran once per frame, the `acmd!` macro did not use `sv_animcmd::frame` or `sv_animcmd::is_excute` but rather it's own implementation.
* While not a bug, `sv_animcmd` lua macros (i.e. `ATTACK`, `CATCH`, `ATK_HIT_ABS`, etc.) are done inline in each function and have their own defined functions in the `smash_script::macros` module. Feel free to implement new lua macros following the same format.
### Compatibility with `skyline-acmd`'s `acmd!` macro
Unfortunately, `lua-replace` and `smash-script` do not support the use of the `acmd!` macro. It is not my place (nor within my skillset) to reimplement this macro. It is up to the original author of the macro to decide if they want to modify it to supported this new style of script replacements. The `acmd!` macro's implementation was designed to be functional with scripts getting called once per frame, which these are not.

## "There Be Dragons": A Note of Caution
The most important feature that the `acmd_hook` and `skyline-acmd` provide over `lua_replace` and `smash-script` is a pretty crucial one: stability. The current implementaiton of `lua_replace` **will** break on the next update, and it is unsure if it will break on updates following that one. I beg of you: please do not anticipate this to be the glorious rise of true acmd and fall of old acmd. For the end user, there might not actually be that much difference, but for larger modpacks it will make a signifcant difference.

Please be careful when using this. 