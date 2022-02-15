#!/usr/bin/python3.9
import shutil, os, sys, glob, characters

def help():
  print("usage: ./array_var_replace <old variable name> <new variable name>\nex:")
  print("\t./array_var_replace double_fireball common::DOUBLE_FIREBALL")


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

index_values = [
  "id",
  "WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;",
  "player_number",
  "get_player_number(boma)"
]

flag_patterns = [
  ("{old_name}[{index_value}] = true;", "VarModule::on_flag(boma.object(), vars::{new_name});"),
  ("{old_name}[{index_value}] = false;", "VarModule::off_flag(boma.object(), vars::{new_name});"),
  ("{old_name}[{index_value}] =true;", "VarModule::on_flag(boma.object(), vars::{new_name});"),
  ("{old_name}[{index_value}] =false;", "VarModule::off_flag(boma.object(), vars::{new_name});"),
  ("{old_name}[{index_value}]= true;", "VarModule::on_flag(boma.object(), vars::{new_name});"),
  ("{old_name}[{index_value}]= false;", "VarModule::off_flag(boma.object(), vars::{new_name});"),
  ("{old_name}[{index_value}]=true;", "VarModule::on_flag(boma.object(), vars::{new_name});"),
  ("{old_name}[{index_value}]=false;", "VarModule::off_flag(boma.object(), vars::{new_name});"),
  ("if({old_name}[{index_value}])", "if VarModule::is_flag(boma.object(), vars::{new_name})"),
  ("({old_name}[{index_value}])", " VarModule::is_flag(boma.object(), vars::{new_name})"),
  ("(!{old_name}[{index_value}])", " !VarModule::is_flag(boma.object(), vars::{new_name})"),
  ("!{old_name}[{index_value}]", " !VarModule::is_flag(boma.object(), vars::{new_name})"),
  ("if {old_name}[{index_value}]", "if VarModule::is_flag(boma.object(), vars::{new_name})"),
  ("{old_name}[{index_value}]", "VarModule::is_flag(boma.object(), vars::{new_name})"),
]

int_patterns = [
  ("= {old_name}[{index_value}]", "= VarModule::get_int(boma.object(), vars::{new_name})"),
  ("={old_name}[{index_value}]", "= VarModule::get_int(boma.object(), vars::{new_name})"),
  ("!={old_name}[{index_value}]", "!= VarModule::get_int(boma.object(), vars::{new_name})"),
  ("!= {old_name}[{index_value}]", "!= VarModule::get_int(boma.object(), vars::{new_name})"),
  ("<{old_name}[{index_value}]", "< VarModule::get_int(boma.object(), vars::{new_name})"),
  (">{old_name}[{index_value}]", "> VarModule::get_int(boma.object(), vars::{new_name})"),
  ("< {old_name}[{index_value}]", "< VarModule::get_int(boma.object(), vars::{new_name})"),
  ("> {old_name}[{index_value}]", "> VarModule::get_int(boma.object(), vars::{new_name})"),
  (", {old_name}[{index_value}]", ", VarModule::get_int(boma.object(), vars::{new_name})"),
  (",{old_name}[{index_value}]", ", VarModule::get_int(boma.object(), vars::{new_name})"),
  ("{old_name}[{index_value}]==", "VarModule::get_int(boma.object(), vars::{new_name}) =="),
  ("{old_name}[{index_value}] ==", "VarModule::get_int(boma.object(), vars::{new_name}) =="),
  ("{old_name}[{index_value}]!=", "VarModule::get_int(boma.object(), vars::{new_name}) !="),
  ("{old_name}[{index_value}] !=", "VarModule::get_int(boma.object(), vars::{new_name}) !="),
  ("{old_name}[{index_value}]>", "VarModule::get_int(boma.object(), vars::{new_name}) >"),
  ("{old_name}[{index_value}] >", "VarModule::get_int(boma.object(), vars::{new_name}) >"),
  ("{old_name}[{index_value}]<", "VarModule::get_int(boma.object(), vars::{new_name}) <"),
  ("{old_name}[{index_value}] <", "VarModule::get_int(boma.object(), vars::{new_name}) <"),
  ("({old_name}[{index_value}]", "(VarModule::get_int(boma.object(), vars::{new_name})"),

  ("{old_name}[{index_value}]= 0;", "VarModule::set_int(boma.object(), vars::{new_name}, 0);"),
  ("{old_name}[{index_value}] = 0;", "VarModule::set_int(boma.object(), vars::{new_name}, 0);"),
  ("{old_name}[{index_value}]= 1;", "VarModule::set_int(boma.object(), vars::{new_name}, 1);"),
  ("{old_name}[{index_value}] = 1;", "VarModule::set_int(boma.object(), vars::{new_name}, 1);"),

  ("{old_name}[{index_value}] += 1;", "VarModule::inc_int(boma.object(), vars::{new_name});"),

  # for staging setters that we cant automate
  ("{old_name}[{index_value}]=", "VarModule::set_int(boma.object(), vars::{new_name}, value_here) "),
  ("{old_name}[{index_value}] =", "VarModule::set_int(boma.object(), vars::{new_name}, value_here) "),
]

float_patterns = [
  ("= {old_name}[{index_value}]", "= VarModule::get_float(boma.object(), vars::{new_name})"),
  ("={old_name}[{index_value}]", "= VarModule::get_float(boma.object(), vars::{new_name})"),
  ("!= {old_name}[{index_value}]", "!= VarModule::get_float(boma.object(), vars::{new_name})"),
  ("!={old_name}[{index_value}]", "!= VarModule::get_float(boma.object(), vars::{new_name})"),
  ("<{old_name}[{index_value}]", "< VarModule::get_float(boma.object(), vars::{new_name})"),
  (">{old_name}[{index_value}]", "> VarModule::get_float(boma.object(), vars::{new_name})"),
  ("< {old_name}[{index_value}]", "< VarModule::get_float(boma.object(), vars::{new_name})"),
  ("> {old_name}[{index_value}]", "> VarModule::get_float(boma.object(), vars::{new_name})"),
  (", {old_name}[{index_value}]", ", VarModule::get_float(boma.object(), vars::{new_name})"),
  (",{old_name}[{index_value}]", ", VarModule::get_float(boma.object(), vars::{new_name})"),
  ("{old_name}[{index_value}]==", "VarModule::get_float(boma.object(), vars::{new_name}) =="),
  ("{old_name}[{index_value}] ==", "VarModule::get_float(boma.object(), vars::{new_name}) =="),
  ("{old_name}[{index_value}]!=", "VarModule::get_float(boma.object(), vars::{new_name}) !="),
  ("{old_name}[{index_value}] !=", "VarModule::get_float(boma.object(), vars::{new_name}) !="),
  ("{old_name}[{index_value}]>", "VarModule::get_float(boma.object(), vars::{new_name}) >"),
  ("{old_name}[{index_value}] >", "VarModule::get_float(boma.object(), vars::{new_name}) >"),
  ("{old_name}[{index_value}]<", "VarModule::get_float(boma.object(), vars::{new_name}) <"),
  ("{old_name}[{index_value}] <", "VarModule::get_float(boma.object(), vars::{new_name}) <"),
  ("({old_name}[{index_value}]", "(VarModule::get_float(boma.object(), vars::{new_name})"),

  ("{old_name}[{index_value}]= 0.0;", "VarModule::set_float(boma.object(), vars::{new_name}, 0.0);"),
  ("{old_name}[{index_value}] = 0.0;", "VarModule::set_float(boma.object(), vars::{new_name}, 0.0);"),
  ("{old_name}[{index_value}]= 1.0;", "VarModule::set_float(boma.object(), vars::{new_name}, 1.0);"),
  ("{old_name}[{index_value}] = 1.0;", "VarModule::set_float(boma.object(), vars::{new_name}, 1.0);"),

  ("{old_name}[{index_value}] += 1.0;", "VarModule::add_float(boma.object(), vars::{new_name}, 1.0);"),

  # for staging setters that we cant automate
  ("{old_name}[{index_value}]=", "VarModule::set_float(boma.object(), vars::{new_name}, value_here) "),
  ("{old_name}[{index_value}] =", "VarModule::set_float(boma.object(), vars::{new_name}, value_here) "),

]


def replace_patterns(old_var, new_var, var_type) -> int:

  if var_type == "flag":
    patterns = flag_patterns
  elif var_type == "int":
    patterns = int_patterns
  elif var_type == "float":
    patterns = float_patterns
  else:
    print("Not a valid variable type! Please specify one of the options: [flag, int, float]")
    exit(1)
  changed = 0

  def variable_replace(file: str, old_var: str, new_var: str) -> int:
    changed = 0
    for old, new in patterns:
      # print(old + ", " + new)
      for index_value in index_values:
        did_change = inplace_change(file, 
          old.replace("{old_name}", old_var).replace("{index_value}", index_value), 
          new.replace("{new_name}", new_var))
        if did_change:
          changed += 1
    return changed

  characters.characters.add("common")

  for fighter in characters.characters:

    # get all files
    files = glob.glob("./" + fighter + "/**", recursive=True)

    for file in files:
      if os.path.isfile(file) and not "target" in file:
        # print(file)
        changed += variable_replace(file, old_var, new_var)
  
  print("changed for " + new_var)
  print(changed)
  return changed

if __name__ == "__main__":

  os.chdir("../fighters")

  if "help" in sys.argv:
    help()

  if not len(sys.argv) == 3:
    print("invalid arguments!\n")
    help()
    exit(1)

  if not "::" in sys.argv[2]:
    print("No package given for new VarModule const! Please specify (for example: ./array_var_replace.py noknok_shell common::NOKNOK_SHELL)")
    exit(1)


  variable_type = input("what is the variable's type? (flag, int, float): ")

  replace_patterns(sys.argv[1], sys.argv[2], variable_type)