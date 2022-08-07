#!/usr/bin/python3
import shutil, os, sys, pkgutil, characters

if "help" in sys.argv or "--help" in sys.argv or "-h" in sys.argv:
  print("no arguments required for simple build. To build parts of the project"
    + " as a development reloadable plugin, use argument 'dev=mario,luigi,captain'"
    + " to specify which character crates to build into the reloadable dev plugin."
    + " to specify what mod folder name to build as, use 'name=hdr-pr' or 'name=hdr', etc.")
  print("For example:")
  print("\t./build.py debug dev=mario,luigi,captain\n")
  print("\t./build.py release dev=captain name=hdr-dev\n")
  exit(0)

is_dev_build = False
plugin_subpath = "skyline/plugins/"
development_subpath = "smashline/"
ryujinx_rom_path = "mods/contents/01006a800016e000/skyline/romfs"
switch_rom_path = "atmosphere/contents/01006a800016e000/romfs"

def install_with_ip(ip: str, is_dev: bool):
  plugin_subpath = "skyline/plugins/"
  development_subpath = "smashline/"
  switch_rom_path = "atmosphere/contents/01006a800016e000/romfs"

  if os.name == 'nt':
    if not is_dev:
      os.system('curl.exe -T ..\\plugin\\target\\aarch64-skyline-switch\\release\\libhdr.nro ftp://' + ip + ':5000/ultimate/mods/hdr-dev/plugin.nro')
    else:
      os.system('curl.exe -T ..\\plugin\\target\\development\\aarch64-skyline-switch\\release\\libhdr.nro ftp://' + ip + ':5000/' + switch_rom_path + '/' + development_subpath + 'development.nro')
      os.system('curl.exe -T ..\\plugin\\target\\standalone\\aarch64-skyline-switch\\release\\libhdr.nro ftp://' + ip + ':5000/ultimate/mods/hdr-dev/plugin.nro')
  else:
    if not is_dev:
      os.system('curl -T ../plugin/target/aarch64-skyline-switch/release/libhdr.nro ftp://' + ip + ':5000/ultimate/mods/hdr-dev/plugin.nro')
    else:
      os.system('curl -T ../plugin/target/development/aarch64-skyline-switch/release/libhdr.nro ftp://' + ip + ':5000/' + switch_rom_path + '/' + development_subpath + 'development.nro')
      os.system('curl -T ../plugin/target/standalone/aarch64-skyline-switch/release/libhdr.nro ftp://' + ip + ':5000/ultimate/mods/hdr-dev/plugin.nro')

# handle fallback exe on windows
def handle_fallback():
  if os.name == 'nt':
    print("windows build!")
    try:
      user_profile = os.environ['USERPROFILE']
    except:
      user_profile = ""

    try:
      rustup_home = os.environ['rustup_home']
    except:
      rustup_home = ""

    if user_profile:
      print("user profile: " + user_profile)
      fallback = os.path.join(user_profile, '.rustup', 'fallback', 'cargo.exe')
      print("checking for fallback cargo in: " + fallback)
      if os.path.exists(fallback):
        print("fallback found: " + fallback)
        os.remove(fallback)

    if rustup_home:
      print("rustup home: " + rustup_home)
      fallback = os.path.join(rustup_home, 'fallback', 'cargo.exe')
      print("checking for fallback cargo in: " + fallback)
      if os.path.exists(fallback):
        print("fallback found: " + fallback)
        os.remove(fallback)

handle_fallback()

characters = characters.characters

current_dir = os.getcwd()
os.chdir('..')

print("arguments: " + ' '.join(sys.argv))

allow_build_dev = True
if "nodev" in sys.argv:
  allow_build_dev = False

release_arg = "--release"
build_type = "release"
is_publish = False
if "release" in sys.argv or "--release" in sys.argv:
  release_arg = "--release"
  build_type = "release"
if "debug" in sys.argv or "--debug" in sys.argv:
  release_arg = ""
  build_type = "debug"
elif "publish" in sys.argv or "--publish" in sys.argv:
  release_arg = "--release"
  build_type = "release"
  is_publish = True

# check for version arguments
version = 'v1.69.420-dev'
for arg in sys.argv:
  if "version=" in arg:
    version = arg.split('=')[1]

# populate version file
version_file = os.path.join('plugin', 'hdr_version.txt')
print("checking for version file in " + version_file + "!" )

# deleting existing version file
if os.path.exists(version_file):
  os.remove(version_file)

# write the version file
with open(version_file, 'x') as version_handle:
  version_handle.write(version)


if is_publish:
  allow_dev_build = False

# if staging folder exists, delete it
if "build" in os.listdir('.'):
  shutil.rmtree('build')
os.mkdir('build')

# search for mod name plugin args
mod_name = "hdr-dev"
for arg in sys.argv:
  if "name=" in arg:
    mod_name = arg.split('=')[1]

switch_hdr_dir = "ultimate/mods/" + mod_name
ryujinx_hdr_dir = "sdcard/ultimate/mods/" + mod_name

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
      if char not in characters:
        print("fighter " + char + " does not exist! (are you using the ingame name for the character?) Valid names are:\n")
        for char_ok in characters:
          print(char_ok)
        exit()
      dev_characters.add(char)

if (is_dev_build and not is_publish):
    
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
  os.environ["CARGO_TARGET_DIR"] = os.path.join("target", "development")
  
  if allow_build_dev:
    print("release arg: " + release_arg)
    retval = pkgutil.build(release_arg, dev_args)
    
    print("subpath: " + development_subpath)
    print("type: " + build_type)
    pkgutil.collect_plugin("hdr-switch", os.path.join(switch_rom_path, development_subpath), build_type, "development.nro", "development")
    pkgutil.collect_plugin("hdr-ryujinx", os.path.join("sdcard", switch_rom_path, development_subpath), build_type, "development.nro", "development")

  # setup normal nro
  non_dev_characters = characters.copy()

  # remove any dev characters
  for char in dev_characters:
    non_dev_characters.remove(char)

  plugin_args = " --no-default-features "
  if len(non_dev_characters) > 0:
    # add each non dev character
    plugin_args += '--features="main_nro","add_status",'
    no_comma = True
    for arg in iter(non_dev_characters):
      if no_comma:
        plugin_args += '"' + arg + '"'
        no_comma = False
      else:
        plugin_args += ',"' + arg + '"'

  if not "dev-only" in sys.argv:
    # build the regular plugin with args
    handle_fallback()
    os.environ["CARGO_TARGET_DIR"] = os.path.join("target", "standalone")
    pkgutil.build(release_arg, plugin_args)

  # collect switch plugin
  pkgutil.collect_plugin("hdr-switch", 
    os.path.join(switch_hdr_dir), 
    build_type, "plugin.nro", "standalone")

    # collect switch romfs
  pkgutil.collect_romfs("hdr-switch", "", mod_name)

  # collect ryujinx plugin
  pkgutil.collect_plugin("hdr-ryujinx", 
    os.path.join(ryujinx_hdr_dir), 
    build_type, "plugin.nro", "standalone")
  
  # collect ryujinx romfs
  pkgutil.collect_romfs("hdr-ryujinx", "sdcard", mod_name)


else:
  if is_publish:
    feature_list = "--features=\"update\",\"main_nro\",\"add_status\""
  else:
    feature_list = "--features=\"main_nro\",\"add_status\""

  is_only = False

  for arg in sys.argv:
    if not "only" in arg:
      continue

    if not "=" in arg:
      print("only specified, but no character arguments given!\n please use 'dev=mario,luigi,samus' format")
      break

    is_only = True

    char_list = (arg.split('=')[1]).split(",")

    for char in char_list:
      if char not in characters:
        print("fighter " + char + " does not exist! (are you using the ingame name for the character?) Valid names are:\n")
        for char_ok in characters:
          print(char_ok)
        exit()
      feature_list += ",\"" + char + "\""

  # simple build
  if is_only:
    pkgutil.build(release_arg, "--no-default-features " + feature_list)
  else:
    pkgutil.build(release_arg, feature_list)

  # collect switch package
  pkgutil.collect_plugin("hdr-switch", 
    os.path.join(switch_hdr_dir), 
    build_type, "plugin.nro")

  # collect switch romfs
  pkgutil.collect_romfs("hdr-switch", "", mod_name)


  # collect ryujinx plugin
  pkgutil.collect_plugin("hdr-ryujinx", 
    os.path.join(ryujinx_hdr_dir), 
    build_type, "plugin.nro")
  
  # collect ryujinx romfs
  pkgutil.collect_romfs("hdr-ryujinx", "sdcard", mod_name)

os.chdir(current_dir)

for arg in sys.argv:
  if "ip" in arg:
    if not "=" in arg:
      print("ip specified but not ip argument given!")
      break
    install_with_ip(arg.split('=')[1], is_dev_build)
    break
