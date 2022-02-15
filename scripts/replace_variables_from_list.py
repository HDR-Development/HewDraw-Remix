#!/usr/bin/python3.9
import shutil, os, sys, glob, characters
from array_var_replace import replace_patterns


def inplace_change(filename, old_string, new_string) -> bool:
    # Safely read the input filename using 'with'
    with open(filename) as f:
        s = f.read()
        if old_string not in s:
            #print('"{old_string}" not found in {filename}.'.format(**locals()))
            return False

    # Safely write the changed content, if found in the file
    with open(filename, 'w') as f:
        #print('Changing "{old_string}" to "{new_string}" in {filename}'.format(**locals()))
        s = s.replace(old_string, new_string)
        f.write(s)
        return True

def insert_text(filename, text:str):
  with open(filename, 'r+') as f:
    content = f.read()
    f.seek(0, 0)
    f.write(text.rstrip('\r\n') + '\n' + content)

variables = [         
  ["ECB_Y_OFFSETS", "FLOAT"],
  ["DITCIT_SLIDING", "FLAG"],
  ["UP_SPECIAL_CANCEL", "FLAG"],
  ["SIDE_SPECIAL_CANCEL", "FLAG"],
  ["UP_SPECIAL_CANCEL_GROUNDED", "FLAG"],
  ["UP_SPECIAL_INTERRUPT", "FLAG"],
  ["UP_SPECIAL_INTERRUPT_AIRTIME", "FLAG"],
  ["SPECIAL_STALL", "FLAG"],
  ["SPECIAL_STALL_USED", "FLAG"],
  ["AIR_SPECIAL_USED", "FLAG"],
  ["FULL_HOP_AERIAL_BUFFERED", "FLAG"],
  ["B_TURNAROUND", "FLAG"],
  ["B_REVERSED", "FLAG"],
  ["SPECIAL_PROJECTILE_SPAWNED", "FLAG"],
  ["FLOAT_TIMER", "INT"],
  ["FLOAT_DURATION", "INT"],
  ["FLOAT_PAUSE_AERIAL", "FLAG"],
  ["AERIAL_NO_FLOAT", "FLAG"],
  ["FLOAT_STYLE", "INT"],
  ["IS_MOONWALK", "FLAG"],
  ["IS_MOONWALK_JUMP", "FLAG"],
  ["MOONWALK_JS_VEL", "FLOAT"],
  ["CURR_MOMENTUM", "FLOAT"],
  ["CURR_MOMENTUM_SPECIALS", "FLOAT"],
  ["JS_VEL", "FLOAT"],
  ["GROUND_VEL", "FLOAT"],
  ["BASE_DASH_SPEED", "FLOAT"],
  ["BASE_RUN_SPEED_MAX", "FLOAT"],
  ["RAR_LENIENCY", "FLOAT"],
  ["IRAR_WINDOW", "FLAG"],
  ["IRAR_JUMPSQUAT", "FLAG"],
  ["SH_MACRO", "FLAG"],
  ["JAB_DA_CHECKS", "FLAG"],
  ["TILT_CHECKS", "FLAG"],
  ["SMASH_CHECKS", "FLAG"],
  ["AERIAL_CHECKS", "FLAG"],
  ["SPECIAL_CHECKS", "FLAG"],
  ["FADC_CHECKS", "FLAG"],
  ["ATTACK_BOUNCED", "FLAG"],
  ["CAN_MAGIC_SERIES_CANCEL", "FLAG"],
  ["MAGIC_SERIES_CANCEL", "FLAG"],
  ["MAGIC_CANCEL_ADDITIONAL", "FLAG"],
  ["COMBO_SCALING", "FLAG"],
  ["REPEAT_INCREMENTED", "FLAG"],
  ["REPEAT_NUM_HI", "INT"],
  ["REPEAT_NUM_LW", "INT"],
  ["TETHER_LEDGE_REWIND", "FLAG"],
  ["TETHER_HOGGED", "FLAG"],
  ["LEDGE_OCCUPYING", "FLAG"],
  ["FOOTSTOOL_AIRDODGE_LOCKOUT", "FLAG"],
  ["METER_COUNTER", "INT"],
  ["METER_USED", "FLAG"],
  ["METER_LEVEL", "INT"],
  ["METER_GAIN_GLOW_TIMER", "INT"],
  ["METER_GAINED_CURRENT_STATUS", "FLAG"],
  ["METER_TO_GAIN", "INT"],
  ["EX_SPECIAL", "FLAG"],
  ["EX_SPECIAL_SCRIPTING", "INT"],
  ["GIMMICK_TIMER", "INT"],
  ["GIMMICK_READY_GLOW_TIMER", "INT"],
  ["IS_IN_HITSTUN", "FLAG"],
  ["HITSTUN_START", "FLAG"],
  ["IS_IN_TUMBLE", "FLAG"],
  ["TUMBLE_START", "FLAG"],
  ["TUMBLE_KB", "FLAG"],
  ["CAN_ESCAPE_TUMBLE", "FLAG"],
  ["HEAVY_AERIAL", "FLAG"],
  ["CHARGE_ATTACK_LEVEL", "FLOAT"],
  ["CHARGE_FINISHED", "FLAG"],
  ["DOUBLE_TRACTION_CHECK", "FLAG"],
  ["NEUTRAL_SPECIAL_TURN_COUNT", "INT"],
  ["NEUTRAL_SPECIAL_HIT", "FLAG"],
  ["UP_SPECIAL_HIT", "FLAG"],
  ["SIDE_SPECIAL_HIT", "FLAG"],
  ["DOWN_SPECIAL_HIT", "FLAG"],
  ["SOARING_SLASH_HIT", "FLAG"],
  ["CHROY_SWORD_TRAIL_EFFECT", "FLAG"],
  ["NOKNOK_SHELL", "FLAG"],
  ["AERIAL_COMMAND_RISING", "FLAG"],
  ["AERIAL_COMMAND_MOMENTUM_RESET", "FLAG"],
  ["AERIAL_COMMAND_RISEN", "FLAG"],
  ["SNAKE_GRENADE_COUNTER", "INT"],
  ["SPECIAL_AUTOCANCEL", "FLAG"],
  ["SHINESPARK_READY", "FLAG"],
  ["SHINESPARK_USED", "FLAG"],
  ["SPECIAL_WALL_JUMP", "FLAG"],
  ["ILLUSION_SHORTEN", "FLAG"],
  ["ILLUSION_SHORTENED", "FLAG"],
  ["KIRBY_STAR_ROD", "FLAG"],
  ["FINAL_CUTTER_HIT", "FLAG"],
  ["SPIN_ATTACK_LAND_CANCEL", "FLAG"],
  ["WITHDRAW_FRAME", "FLOAT"],
  ["SONIC_LIGHTSPEED_DASH_FRAME_COUNTER", "FLOAT"],
  ["SONIC_LIGHTSPEED_NO_JUMP", "FLAG"],
  ["SONIC_PULSE_HITBOX", "FLAG"],
  ["SONIC_BLAST_ATTACK", "FLAG"],
  ["SONIC_NO_AIRDODGE", "FLAG"],
  ["AIR_CROSS", "FLAG"],
  ["FIRE_KICK", "FLAG"],
  ["POWER_CHARGE_CANCEL", "FLAG"],
  ["SUPER_CANCEL", "FLAG"],
  ["EX_PART_1", "FLAG"],
  ["EX_PART_2", "FLAG"],
  ["EX_PART_3", "FLAG"],
  ["EX_PART_4", "FLAG"],
  ["EX_PART_5", "FLAG"],
  ["EX_SEQUENCE_NUMBER", "INT"],
  ["HEAVY_ATTACK", "FLAG"],
  ["DISABLE_SPECIAL_JC", "FLAG"],
  ["DOUBLE_JUMP_FRAME", "FLOAT"],
  ["DOUBLE_JUMP_TIMER", "FLOAT"],
  ["DOUBLE_JUMP_STOP", "FLAG"],
  ["DOUBLE_JUMP_CANCELED", "FLAG"],
  ["POPO_JC_GRAB", "FLAG"],
  ["CRITICAL_HIT", "FLAG"],
  ["GLIDE_TIMER", "FLOAT"],
  ["DISABLE_GALE_STRIKE", "FLAG"],
  ["SKYWARD_SLASH_DASH_HIT", "FLAG"]
]

current_consts = "pub const IS_HEAVY_ATTACK: i32 = 0x0;\n        pub const FIREBRAND_ACTIVATED: i32 = 0x1;\n        pub const DOUBLE_FIREBALL: i32 = 0x2;\n        pub const NOKNOK_SHELL: i32 = 0x3;\n        pub const IS_IN_HITSTUN: i32 = 0x4;\n        pub const CSTICK_OVERRIDE: i32 = 0x5;\n        pub const CSTICK_OVERRIDE_SECOND: i32 = 0x6;\n        pub const IS_TAP_JUMP: i32 = 0x7;\n        pub const OMNI_FLOAT: i32 = 0x8;\n        pub const SIDE_SPECIAL_CANCEL: i32 = 0x9;\n        pub const DISABLE_UP_SPECIAL_JUMP_REFRESH: i32 = 0xA;\n        pub const HITSTUN_START: i32 = 0xB;\n        pub const AERIAL_NO_FLOAT: i32 = 0xC;\n        pub const FLOAT_PAUSE_AERIAL: i32 = 0xD;\n        pub const SMASH_CHECKS: i32 = 0xE;\n        pub const TILT_CHECKS: i32 = 0xF;\n        pub const JAB_DA_CHECKS: i32 = 0x10;\n        pub const AERIAL_CHECKS: i32 = 0x11;\n        pub const SPECIAL_STALL_USED: i32 = 0x12;\n        pub const SPECIAL_STALL: i32 = 0x13;\n        pub const UP_SPECIAL_INTERRUPT: i32 = 0x14;\n        pub const ENABLE_AIR_ESCAPE_MAGNET: i32 = 0x15;\n        pub const UP_SPECIAL_INTERRUPT_AIRTIME: i32 = 0x16;\n        pub const DITCIT_SLIDING: i32 = 0x17;\n\n\n        // int\n        pub const LAST_ATTACK_RECEIVER_ENTRY_ID: i32 = 0x0;\n        pub const COSTUME_SLOT_NUMBER: i32 = 0x1;\n        pub const FLOAT_TIMER: i32 = 0x2;\n        pub const FLOAT_DURATION: i32 = 0x3;\n        pub const FLOAT_STYLE: i32 = 0x4;\n        pub const GIMMICK_READY_GLOW_TIMER: i32 = 0x5;\n\n\n        // float\n        /// var to store the damage that this fighter DEALT as the attacker.\n        pub const LAST_ATTACK_DAMAGE_DEALT: i32 = 0x0;\n        pub const CURRENT_MOMENTUM: i32 = 0x1;\n        pub const JUMPSQUAT_VELOCITY: i32 = 0x2;\n        pub const JUMP_SPEED_RATIO: i32 = 0x3;\n        pub const DOUBLE_JUMP_FRAME: i32 = 0x4;\n        pub const GROUND_VEL: i32 = 0x5;\n        pub const RAR_LENIENCY: i32 = 0x6;\n        pub const CURRENT_MOMENTUM_SPECIALS: i32 = 0x7;\n\n        // separator\n        // flag\n        pub const SPECIAL_PROJECTILE_SPAWNED: i32 = 0x50;\n        pub const UP_SPECIAL_CANCEL: i32 = 0x51;\n\n        // int\n        pub const GIMMICK_TIMER: i32 = 0x50;"

new_consts = set()

os.chdir("../fighters")
changed = 0

for entry in variables:
  #if entry[0] in current_consts:
  #  continue
  variable_name = entry[0]
  variable_type = entry[1].lower()

  variable_lowercase = variable_name.lower()
  just_changed = replace_patterns(variable_lowercase, "common::" + variable_name, variable_type)
  changed += just_changed
  if just_changed > 0 and not (entry[0], entry[1]) in new_consts:
    new_consts.add((entry[0], entry[1]))
    print(variable_name)

print("\nchanged: ")
print(changed)

int_consts = set()
float_consts = set()
flag_consts = set()

for const in new_consts:
  if const[1] == "FLAG":
    flag_consts.add(const[0])
  elif const[1] == "INT":
    int_consts.add(const[0])
  elif const[1] == "FLOAT":
    float_consts.add(const[0])
  else:
    exit("unknown const type!")
  
print("flag consts:")
for const in flag_consts:
  print(const)

print("int consts:")
for const in int_consts:
  print(const)

print("float consts:")
for const in float_consts:
  print(const)



      
        