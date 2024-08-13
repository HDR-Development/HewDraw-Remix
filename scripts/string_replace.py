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

def replace(fighter):
  # get all files
  files = glob.glob("./" + fighter + "/**", recursive=True)

  for file in files:
    if os.path.isfile(file) and not "target" in file and not "acmd" in file:
      if "opff" in file:
        print(file)
        if "common" in file:
          insert_text(file, "use crate::opff_import::*;")
        else:
          insert_text(file, "use common::opff_import::*;")
      else:
        print("no opff: " + file)

for fighter in characters.characters:
  replace(fighter)
  
replace("common")
