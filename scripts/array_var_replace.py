#!/usr/bin/python3.9
import shutil, os, sys, glob, characters

def help():
  print("usage: ./array_var_replace <old variable name> <new variable name>\nex:")
  print("\t./array_var_replace double_fireball common::DOUBLE_FIREBALL")

if "help" in sys.argv:
  help()

if not len(sys.argv) == 3:
  print("invalid arguments!\n")
  help()
  exit(1)

if not "::" in sys.argv[2]:
  print("No package given for new VarModule const! Please specify (for example: ./array_var_replace.py noknok_shell common::NOKNOK_SHELL)")
  exit(1)

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

index_values = [
  "id",
  "hdr::get_player_number(boma)"
]

flag_patterns = [
  ("{old_name}[{index_value}] = true;", "VarModule::on_flag(get_battle_object_from_accessor(boma), vars::{new_name});"),
  ("{old_name}[{index_value}] = false;", "VarModule::off_flag(get_battle_object_from_accessor(boma), vars::{new_name});"),
  ("if({old_name}[{index_value}])", "if VarModule::is_flag(get_battle_object_from_accessor(boma), vars::{new_name})"),
  ("({old_name}[{index_value}])", " VarModule::is_flag(get_battle_object_from_accessor(boma), vars::{new_name})"),
  ("(!{old_name}[{index_value}])", " !VarModule::is_flag(get_battle_object_from_accessor(boma), vars::{new_name})"),
  ("!{old_name}[{index_value}]", " !VarModule::is_flag(get_battle_object_from_accessor(boma), vars::{new_name})"),
  ("if {old_name}[{index_value}]", "if VarModule::is_flag(get_battle_object_from_accessor(boma), vars::{new_name})"),
]

int_patterns = [
  ("= {old_name}[{index_value}]", "= VarModule::get_int(get_battle_object_from_accessor(boma), vars::{new_name})"),
  ("={old_name}[{index_value}]", "= VarModule::get_int(get_battle_object_from_accessor(boma), vars::{new_name})"),
  ("<{old_name}[{index_value}]", "< VarModule::get_int(get_battle_object_from_accessor(boma), vars::{new_name})"),
  (">{old_name}[{index_value}]", "> VarModule::get_int(get_battle_object_from_accessor(boma), vars::{new_name})"),
  ("< {old_name}[{index_value}]", "< VarModule::get_int(get_battle_object_from_accessor(boma), vars::{new_name})"),
  ("> {old_name}[{index_value}]", "> VarModule::get_int(get_battle_object_from_accessor(boma), vars::{new_name})"),
  (", {old_name}[{index_value}]", ", VarModule::get_int(get_battle_object_from_accessor(boma), vars::{new_name})"),
  (",{old_name}[{index_value}]", ", VarModule::get_int(get_battle_object_from_accessor(boma), vars::{new_name})"),
  ("{old_name}[{index_value}]==", "VarModule::get_int(get_battle_object_from_accessor(boma), vars::{new_name}) =="),
  ("{old_name}[{index_value}] ==", "VarModule::get_int(get_battle_object_from_accessor(boma), vars::{new_name}) =="),
  ("{old_name}[{index_value}]>", "VarModule::get_int(get_battle_object_from_accessor(boma), vars::{new_name}) >"),
  ("{old_name}[{index_value}] >", "VarModule::get_int(get_battle_object_from_accessor(boma), vars::{new_name}) >"),
  ("{old_name}[{index_value}]<", "VarModule::get_int(get_battle_object_from_accessor(boma), vars::{new_name}) <"),
  ("{old_name}[{index_value}] <", "VarModule::get_int(get_battle_object_from_accessor(boma), vars::{new_name}) <"),
  ("({old_name}[{index_value}]", "(VarModule::get_int(get_battle_object_from_accessor(boma), vars::{new_name})"),

  ("{old_name}[{index_value}]= 0;", "VarModule::set_int(get_battle_object_from_accessor(boma), vars::{new_name}, 0);"),
  ("{old_name}[{index_value}] = 0;", "VarModule::set_int(get_battle_object_from_accessor(boma), vars::{new_name}, 0);"),
  ("{old_name}[{index_value}]= 1;", "VarModule::set_int(get_battle_object_from_accessor(boma), vars::{new_name}, 1);"),
  ("{old_name}[{index_value}] = 1;", "VarModule::set_int(get_battle_object_from_accessor(boma), vars::{new_name}, 1);"),

  ("{old_name}[{index_value}] += 1;", "VarModule::inc_int(get_battle_object_from_accessor(boma), vars::{new_name});"),
]

float_patterns = [
  ("= {old_name}[{index_value}]", "= VarModule::get_float(get_battle_object_from_accessor(boma), vars::{new_name})"),
  ("={old_name}[{index_value}]", "= VarModule::get_float(get_battle_object_from_accessor(boma), vars::{new_name})"),
  ("<{old_name}[{index_value}]", "< VarModule::get_float(get_battle_object_from_accessor(boma), vars::{new_name})"),
  (">{old_name}[{index_value}]", "> VarModule::get_float(get_battle_object_from_accessor(boma), vars::{new_name})"),
  ("< {old_name}[{index_value}]", "< VarModule::get_float(get_battle_object_from_accessor(boma), vars::{new_name})"),
  ("> {old_name}[{index_value}]", "> VarModule::get_float(get_battle_object_from_accessor(boma), vars::{new_name})"),
  (", {old_name}[{index_value}]", ", VarModule::get_float(get_battle_object_from_accessor(boma), vars::{new_name})"),
  (",{old_name}[{index_value}]", ", VarModule::get_float(get_battle_object_from_accessor(boma), vars::{new_name})"),
  ("{old_name}[{index_value}]==", "VarModule::get_float(get_battle_object_from_accessor(boma), vars::{new_name}) =="),
  ("{old_name}[{index_value}] ==", "VarModule::get_float(get_battle_object_from_accessor(boma), vars::{new_name}) =="),
  ("{old_name}[{index_value}]>", "VarModule::get_float(get_battle_object_from_accessor(boma), vars::{new_name}) >"),
  ("{old_name}[{index_value}] >", "VarModule::get_float(get_battle_object_from_accessor(boma), vars::{new_name}) >"),
  ("{old_name}[{index_value}]<", "VarModule::get_float(get_battle_object_from_accessor(boma), vars::{new_name}) <"),
  ("{old_name}[{index_value}] <", "VarModule::get_float(get_battle_object_from_accessor(boma), vars::{new_name}) <"),
  ("({old_name}[{index_value}]", "(VarModule::get_float(get_battle_object_from_accessor(boma), vars::{new_name})"),

  ("{old_name}[{index_value}]= 0.0;", "VarModule::set_float(get_battle_object_from_accessor(boma), vars::{new_name}, 0.0);"),
  ("{old_name}[{index_value}] = 0.0;", "VarModule::set_float(get_battle_object_from_accessor(boma), vars::{new_name}, 0.0);"),
  ("{old_name}[{index_value}]= 1.0;", "VarModule::set_float(get_battle_object_from_accessor(boma), vars::{new_name}, 1.0);"),
  ("{old_name}[{index_value}] = 1.0;", "VarModule::set_float(get_battle_object_from_accessor(boma), vars::{new_name}, 1.0);"),

  
  ("{old_name}[{index_value}] += 1.0;", "VarModule::add_float(get_battle_object_from_accessor(boma), vars::{new_name}, 1.0);"),

]

variable_type = input("what is the variable's type? (bool, int, float")

if variable_type is "bool":
  patterns = flag_patterns
elif variable_type is "int":
  patterns = int_patterns
elif variable_type is "float":
  patterns = float_patterns
else:
  print("Not a valid variable type! Please specify one of the options: [bool, int, float]")
  exit(1)

def variable_replace(file: str, old_var: str, new_var: str):
  for old, new in patterns:
    # print(old + ", " + new)
    for index_value in index_values:
      inplace_change(file, 
        old.replace("{old_name}", old_var).replace("{index_value}", index_value), 
        new.replace("{new_name}", new_var))

os.chdir("../fighters")

characters.characters.add("common")

for fighter in characters.characters:

  # get all files
  files = glob.glob("./" + fighter + "/**", recursive=True)

  for file in files:
    if os.path.isfile(file) and not "target" in file:
      # print(file)
      variable_replace(file, sys.argv[1], sys.argv[2])
        