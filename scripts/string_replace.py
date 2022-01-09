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

patterns = [
  ("{old_name}[hdr::get_player_number(boma)] = true;", "VarModule::on_flag(fighter.battle_object, vars::{new_name})"),
  ("{old_name}[hdr::get_player_number(boma)] = false;", "VarModule::off_flag(fighter.battle_object, vars::{new_name})"),
  ("if({old_name}[hdr::get_player_number(boma)])", "if VarModule::is_flag(fighter.battle_object, vars::{new_name})"),
  ("({old_name}[hdr::get_player_number(boma)])", " VarModule::is_flag(fighter.battle_object, vars::{new_name})"),
  ("(!{old_name}[hdr::get_player_number(boma)])", " !VarModule::is_flag(fighter.battle_object, vars::{new_name})"),
  ("if {old_name}[hdr::get_player_number(boma)]", "if VarModule::is_flag(fighter.battle_object, vars::{new_name})")
]

def variable_replace(file: str, old_var: str, new_var: str):
  for old, new in patterns:
    inplace_change(file, old.replace("{old_name}", old_var), new.replace("{new_name}", new_var))

os.chdir("../fighters")



for fighter in characters.characters:

  # get all files
  files = glob.glob("./" + fighter + "/**", recursive=True)

  for file in files:
    if os.path.isfile(file):
      variable_replace(file, "noknok_shell", "common::NOKNOK_SHELL")
        