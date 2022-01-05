#!/usr/bin/python3.9
import shutil, os, sys, pathlib

current_dir = os.getcwd()

print("arguments: " + ' '.join(sys.argv))

build_type = "--release"
if "debug" in sys.argv:
  build_type = ""

# TODO special build commands for regular vs development plugins

build_command = "cargo skyline build " + build_type
print("BUILD COMMAND:")
print(build_command)

# build the plugin
os.chdir('../plugin')
os.system(build_command)
os.chdir('..')

# if staging folder exists, delete it
if "build" in os.listdir('.'):
  shutil.rmtree('build')
os.mkdir('build')

plugin_subpath = "romfs/skyline/plugins/"

ryujinx_plugin_path = "mods/contents/01006a800016e000/skyline/"
switch_plugin_path = "atmosphere/contents/01006a800016e000/"


def collect_plugin(package_name: str, context_path: str):
  print("COLLECTING " + package_name + " plugins!")
  # get regular nro, if it exists
  plugin_source = os.path.join('plugin/target/aarch64-skyline-switch/release/libhdr.nro')
  plugin_destination = os.path.join('build', package_name, context_path, plugin_subpath)
  pathlib.Path(plugin_destination).mkdir(parents=True)
  shutil.copy(
    os.path.join(plugin_source), 
    os.path.join(plugin_destination, "libhdr.nro"))
  
  # get development nro, if it exists

  return

def collect_romfs(package_name: str, context_path: str):
  print("COLLECTING " + package_name + " romfs!")
  romfs_source = os.path.join("romfs/build")
  romfs_destination = os.path.join("build", package_name, context_path, "ultimate/mods/hdr")
  shutil.copytree(
    os.path.join(romfs_source), 
    os.path.join(romfs_destination))

  return

collect_plugin("hdr-ryujinx", ryujinx_plugin_path)
collect_romfs("hdr-ryujinx", "sd")

collect_plugin("hdr-switch", switch_plugin_path)
collect_romfs("hdr-switch", "")

os.chdir(current_dir)