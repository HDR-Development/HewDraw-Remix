#!/usr/bin/python3
import shutil, os, sys, glob, characters

characters = characters.characters

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

for fighter in characters:
  if not os.path.exists(fighter):
    # make fighter folder
    shutil.copytree("template", fighter)
    
    # get all files
    files = glob.glob("./" + fighter + "/**", recursive=True)

    for file in files:
      print(file)
      if os.path.isfile(file):
        # edit the file with the right name
        inplace_change(file, "${template}", fighter)
        inplace_change(file, "${TEMPLATE}", fighter.upper())
