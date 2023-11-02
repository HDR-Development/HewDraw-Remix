import pathlib, shutil, os, sys

def run_command(command: str) -> str:
  os.system(command + " > tmp")
  return open("tmp", "r").read()

def collect_plugin(package_name: str, package_path: str, build_type: str, plugin_name: str, build_subdir: str = None):
  print("COLLECTING " + package_name + " plugins!")
  # get nro
  if build_subdir != None:
    plugin_source = os.path.join("target/" + build_subdir + "/aarch64-skyline-switch/" + build_type + "/libhdr.nro")
  else:
    plugin_source = os.path.join("target/aarch64-skyline-switch/" + build_type + "/libhdr.nro")
    
  print("plugin source: " + plugin_source)

  plugin_destination = os.path.join('build', package_name, package_path)
  os.makedirs(plugin_destination, exist_ok=True)
  print("copying plugin from: " + plugin_source)
  print("copying plugin into: " + plugin_destination)
  shutil.copy(
    os.path.join(plugin_source), 
    os.path.join(plugin_destination, plugin_name))
  
  return

def collect_romfs(package_name: str, context_path: str, mod_name: str):
  print("COLLECTING " + package_name + " romfs!")
  romfs_source = os.path.join("romfs/build")
  romfs_destination = os.path.join("build", package_name, context_path, "ultimate/mods", mod_name)
  shutil.copytree(
    os.path.join(romfs_source), 
    os.path.join(romfs_destination),
    dirs_exist_ok=True)
  shutil.copyfile(os.path.join("romfs/config.json"), os.path.join(romfs_destination, "config.json"))
  shutil.copyfile(os.path.join("hdr_version.txt"), os.path.join(romfs_destination, "ui/hdr_version.txt"))
  return

## returns whether the build was successful
def build(build_type: str, dev_args: str, is_publish) -> bool:
  target = None
  if sys.platform == 'darwin':
    target = "aarch64-apple-darwin"
  elif sys.platform == 'linux':
    target = "x86_64-unknown-linux-gnu"
  elif sys.platform == 'win32':
    target = "x86_64-pc-windows-msvc"
  
  build_romfs_command = "RUSTFLAGS=\"--cfg skyline_std_v3\" SKYLINE_ADD_NRO_HEADER=1 cargo +nightly run --release -p build-tools -v -Z build-std=core,alloc,std,panic_abort --target " + target
  if "NO_RUST_NIGHTLY" in os.environ:
    build_romfs_command = "cargo run --release -p build-tool"
  print("BUILD ROMFS COMMAND:")
  print(build_romfs_command)

  os.system(build_romfs_command)

  build_command = "cargo skyline build " + build_type + " " + dev_args
  print("BUILD COMMAND:")
  print(build_command)

  # build the plugin
  retval = os.system(build_command)

  if retval == 0:
    return True
  else:
    exit("### build failed! ###")

# inform the user of their foolishness in running this file itself
if __name__ == '__main__':
    # Execute when the module is not initialized from an import statement.
    print("This is just a library you fool, don't run this lol.")