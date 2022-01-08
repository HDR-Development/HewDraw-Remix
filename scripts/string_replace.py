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

os.chdir("../fighters")

for fighter in characters.characters:

  # get all files
  files = glob.glob("./" + fighter + "/**", recursive=True)

  for file in files:
    if os.path.isfile(file):

      print(file)
      inplace_change(file, "use smash::app::{sv_system, sv_animcmd::{frame, wait}, self, lua_bind::*};\nuse smash::phx::Hash40;\nuse smash::lib::lua_const::*;\nuse smash::lua2cpp::L2CAgentBase;\nuse smash::app::utility::*;\nuse smash_script::*;\nuse smashline::*;\nuse smashline::*;\nuse smash_script::macros::*;\nuse crate::utils::hdr;\nuse crate::vars::*;", "use super::*;")