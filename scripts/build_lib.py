import pathlib, shutil, os

def collect_plugin(package_name: str, package_path: str, build_type: str, plugin_name: str):
  print("COLLECTING " + package_name + " plugins!")
  # get nro
  plugin_source = os.path.join("plugin/target/aarch64-skyline-switch/" + build_type + "/libhdr.nro")
  plugin_destination = os.path.join('build', package_name, package_path)
  pathlib.Path(plugin_destination).mkdir(parents=True)
  shutil.copy(
    os.path.join(plugin_source), 
    os.path.join(plugin_destination, plugin_name))
  
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

def build(build_type: str, dev_args: str):
  build_command = "cargo skyline build " + build_type + " " + dev_args
  print("BUILD COMMAND:")
  print(build_command)

  # build the plugin
  os.chdir('plugin')
  os.system(build_command)
  os.chdir('..')