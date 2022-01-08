#!/usr/bin/python3.9
import shutil, os, sys, pkgutil, characters

if "help" in sys.argv or "-h" in sys.argv:
  print("no arguments required for simple build. To build parts of the project"
    + " as a development reloadable plugin, use argument 'dev=mario,luigi,captain'"
    + " to specify which character crates to build into the reloadable dev plugin.")
  print("For example:\n\t./build.py dev=mario,luigi,captain\n")
  exit(0)

characters = characters.characters

is_dev_build = False
plugin_subpath = "skyline/plugins/"
development_subpath = "smashline/"
ryujinx_rom_path = "mods/contents/01006a800016e000/skyline/romfs"
switch_rom_path = "atmosphere/contents/01006a800016e000/romfs"

current_dir = os.getcwd()
os.chdir('..')

print("arguments: " + ' '.join(sys.argv))

release_arg = "--release"
build_type = "release"
if "debug" in sys.argv:
  release_arg = ""
  build_type = "debug"


# if staging folder exists, delete it
if "build" in os.listdir('.'):
  shutil.rmtree('build')
os.mkdir('build')


# search for dev plugin args
dev_characters = set()
for arg in sys.argv:
  if "dev" in arg:

    # if theres no equals, break
    if not "=" in arg:
      print("dev specified, but no character arguments given!\n please use 'dev=mario,luigi,samus' format")
      break
    
    # set that this is a development build
    is_dev_build = True

    # get the list of characters
    char_list = (arg.split('=')[1]).split(",")

    # add each character to the set
    for char in char_list:
      dev_characters.add(char)

if (is_dev_build):
    
  # special build commands for regular vs development plugins
  dev_args = ""
  if len(dev_characters) > 0:
    # add all of the characters
    dev_args += ' --no-default-features --features="runtime"'
    for character in dev_characters:
      dev_args += ',"' + character + '"'
  else:
    print("ERROR: No character arguments given!")
  
  # build the dev plugin with args
  pkgutil.build(release_arg, dev_args)

  pkgutil.collect_plugin("hdr-switch", os.path.join(switch_rom_path, development_subpath), build_type, "development.nro")
  pkgutil.collect_plugin("hdr-ryujinx", os.path.join(ryujinx_rom_path, development_subpath), build_type, "development.nro")

  # setup normal nro
  non_dev_characters = characters.copy()

  # remove any dev characters
  for char in dev_characters:
    non_dev_characters.remove(char)

  plugin_args = " --no-default-features "
  if len(non_dev_characters) > 0:
    # add each non dev character
    plugin_args += "--features=" + non_dev_characters[0]
    for i in range(1, len(non_dev_characters)):
      plugin_args += ',"' + non_dev_characters[i] + '"'

  # build the regular plugin with args
  pkgutil.build(release_arg, plugin_args)

  # collect switch plugin
  pkgutil.collect_plugin("hdr-switch", 
    os.path.join(switch_rom_path, plugin_subpath), 
    build_type, "libhdr.nro")

    # collect switch romfs
  pkgutil.collect_romfs("hdr-switch", "")

  # collect ryujinx plugin
  pkgutil.collect_plugin("hdr-ryujinx", 
    os.path.join(ryujinx_rom_path, plugin_subpath), 
    build_type, "libhdr.nro")
  
  # collect ryujinx romfs
  pkgutil.collect_romfs("hdr-ryujinx", "sd")


else:
  # simple build
  pkgutil.build(release_arg, "")

  # collect switch package
  pkgutil.collect_plugin("hdr-switch", 
    os.path.join(switch_rom_path, plugin_subpath), 
    build_type, "libhdr.nro")

  # collect switch romfs
  pkgutil.collect_romfs("hdr-switch", "")


  # collect ryujinx plugin
  pkgutil.collect_plugin("hdr-ryujinx", 
    os.path.join(ryujinx_rom_path, plugin_subpath), 
    build_type, "libhdr.nro")
  
  # collect ryujinx romfs
  pkgutil.collect_romfs("hdr-ryujinx", "sd")

os.chdir(current_dir)


