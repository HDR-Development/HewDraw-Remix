#!/usr/bin/python3.9
import shutil, os, sys, glob, characters


def inplace_change(filename, old_string, new_string):
    # Safely read the input filename using 'with'
    with open(filename) as f:
        s = f.read()
        if old_string not in s:
            #print('"{old_string}" not found in {filename}.'.format(**locals()))
            return

    # Safely write the changed content, if found in the file
    with open(filename, 'w') as f:
        #print('Changing "{old_string}" to "{new_string}" in {filename}'.format(**locals()))
        s = s.replace(old_string, new_string)
        f.write(s)

def insert_text(filename, text:str):
  with open(filename, 'r+') as f:
    content = f.read()
    f.seek(0, 0)
    f.write(text.rstrip('\r\n') + '\n' + content)

flags = [
  ["FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N","Cat1::AttackN"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3","Cat1::AttackS3"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3","Cat1::AttackHi3"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3","Cat1::AttackLw3"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4","Cat1::AttackS4"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4","Cat1::AttackHi4"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4","Cat1::AttackLw4"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_N","Cat1::AttackAirN"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_F","Cat1::AttackAirF"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_B","Cat1::AttackAirB"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_HI","Cat1::AttackAirHi"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_LW","Cat1::AttackAirLw"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N","Cat1::SpecialN"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S","Cat1::SpecialS"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI","Cat1::SpecialHi"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW","Cat1::SpecialLw"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_WALK","Cat1::SpecialAny"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_DASH","Cat1::Walk"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_TURN","Cat1::Dash"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH","Cat1::Turn"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_JUMP","Cat1::TurnDash"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_JUMP_BUTTON","Cat1::Jump"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_AIR_ESCAPE","Cat1::JumpButton"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_SQUAT","Cat1::AirEscape"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE","Cat1::Squat"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F","Cat1::Escape"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B","Cat1::EscapeF"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_LEFT","Cat1::EscapeB"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_WALL_JUMP_RIGHT","Cat1::WallJumpLeft"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_CATCH","Cat1::WallJumpRight"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_NO_CMD","Cat1::Catch"],
  ["FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_ANY","Cat1::NoCmd"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L","Cat2::AppealSL"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R","Cat2::AppealSR"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_HI","Cat2::AppealHi"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_LW","Cat2::AppealLw"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_SMASH","Cat2::AppealSmash"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_ATTACK_DASH_ATTACK_HI4","Cat2::AttackDashAttackHi4"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP","Cat2::FallJump"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_DASH_ATTACK_S4","Cat2::DashAttackS4"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_DAMAGE_FALL_TO_FALL","Cat2::DamageFallToFall"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_DOWN_TO_DOWN_STAND_FB","Cat2::DownToDownStandFB"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_DOWN_TO_DOWN_STAND","Cat2::DownToDownStand"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_GUARD_TO_PASS","Cat2::GuardToPass"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_SQUAT_TO_SQUAT_F","Cat2::SquatToSquatF"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_SQUAT_TO_SQUAT_B","Cat2::SquatToSquatB"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_TURN_TO_ESCAPE_F","Cat2::TurnToEscapeF"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_TURN_TO_ESCAPE_B","Cat2::TurnToEscapeB"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_F","Cat2::StickEscapeF"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE_B","Cat2::StickEscapeB"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_STICK_ESCAPE","Cat2::StickEscape"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_SPECIAL_N_REVERSE_LR","Cat2::SpecialNReverseLR"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_THROW_F","Cat2::ThrowF"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B","Cat2::ThrowB"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI","Cat2::ThrowHi"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW","Cat2::ThrowLw"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_COMMON_GUARD","Cat2::CommonGuard"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_AIR_LASSO","Cat2::AirLasso"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_ATTACK_N2","Cat2::AttackN2"],
  ["FIGHTER_PAD_CMD_CAT2_FLAG_FINAL_REVERSE_LR","Cat2::FinalReverseLR"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_FB4","Cat3::ItemLightThrowFB4"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_HI4","Cat3::ItemLightThrowHi4"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_LW4","Cat3::ItemLightThrowLw4"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_HI","Cat3::ItemLightThrowHi"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_LW","Cat3::ItemLightThrowLw"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_DROP","Cat3::ItemLightDrop"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_FB","Cat3::ItemLightThrowFB"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_AIR_FB","Cat3::ItemLightThrowAirFB"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_AIR_FB4","Cat3::ItemLightThrowAirFB4"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_AIR_HI","Cat3::ItemLightThrowAirHi"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_AIR_HI4","Cat3::ItemLightThrowAirHi4"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_AIR_LW","Cat3::ItemLightThrowAirLw"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_AIR_LW4","Cat3::ItemLightThrowAirLw4"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_DROP_AIR","Cat3::ItemLightDropAir"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_HEAVY_THROW_FB","Cat3::ItemHeavyThrowFB"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_GET_AIR","Cat3::ItemGetAir"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_SPECIAL_S_SMASH","Cat3::SpecialSSmash"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_SPECIAL_S_SMASH_DASH","Cat3::SpecialSSmashDash"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW","Cat3::ItemLightThrow"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_AIR","Cat3::ItemLightThrowAir"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_4","Cat3::ItemLightThrow4"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_4_AIR","Cat3::ItemLightThrow4Air"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_ALL","Cat3::ItemLightThrowAll"],
  ["FIGHTER_PAD_CMD_CAT3_FLAG_ITEM_LIGHT_THROW_AIR_ALL","Cat3::ItemLightThrowAirAll"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N_COMMAND"," Cat4::SpecialNCommand"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND"," Cat4::SpecialN2Command"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_S_COMMAND"," Cat4::SpecialSCommand"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI_COMMAND"," Cat4::SpecialHICommand"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6"," Cat4::Command6N6"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_4N4"," Cat4::Command4N4"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_ATTACK_COMMAND1"," Cat4::AttackCommand1"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_HI2_COMMAND"," Cat4::SpecialHi2Command"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND"," Cat4::SuperSpecialCommand"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND"," Cat4::SuperSpecialRCommand"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND"," Cat4::SuperSpecial2Command"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND"," Cat4::SuperSpecial2RCommand"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623NB"," Cat4::Command623NB"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623STRICT"," Cat4::Command623Strict"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623ALONG"," Cat4::Command623ALong"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623BLONG"," Cat4::Command623BLong"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623A"," Cat4::Command623A"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_2"," Cat4::Command2"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_3"," Cat4::Command3"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_1"," Cat4::Command1"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6"," Cat4::Command6"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_4"," Cat4::Command4"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_8"," Cat4::Command8"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_9"," Cat4::Command9"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_7"," Cat4::Command7"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_6N6AB"," Cat4::Command6N6AB"],
  ["FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_323CATCH"," Cat4::Command323Catch"]

]

os.chdir("../fighters")

characters.characters.add("common")

for fighter in characters.characters:

  # get all files
  files = glob.glob("./" + fighter + "/**", recursive=True)

  for file in files:
    if os.path.isfile(file) and not "target" in file:
      # print(file)
      for entry in flags:
        # compare_mask(cat1, *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH)
        inplace_change(file, "compare_mask(cat1, *" + entry[0] + ")", "boma.is_cat_flag(" + entry[1] + ")")
        inplace_change(file, "compare_mask(cat2, *" + entry[0] + ")", "boma.is_cat_flag(" + entry[1] + ")")
        inplace_change(file, "compare_mask(cat3, *" + entry[0] + ")", "boma.is_cat_flag(" + entry[1] + ")")
        inplace_change(file, "compare_mask(cat4, *" + entry[0] + ")", "boma.is_cat_flag(" + entry[1] + ")")
        