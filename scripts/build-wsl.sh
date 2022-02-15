#!/bin/bash
script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
repo_root=$(dirname "$script_dir")

app_data="$(wslpath $(cmd.exe /c echo %AppData%))"
ryujinx_path="${app_data::-1}/Ryujinx" # remove the carriage return because Windows and WSL are both stupid
mod_path="$ryujinx_path/mods/contents/01006a800016e000/skyline/romfs/skyline/plugins"
base_path="$ryujinx_path/sdcard/ultimate/mods/HDR-Base"

# do this before getting the rom files because this will update them
cd "$repo_root/plugin"
cargo skyline build --release
cp target/aarch64-skyline-switch/release/libhdr.nro "$mod_path/libhdr.nro"

files=$(find "$repo_root/romfs/build" -type f -name "*.prc" -print)
for file in $files;
do
    local_path=${file#"$repo_root/romfs/build"}
    full_rom_path=$base_path$local_path
    mkdir -p $(dirname "$full_rom_path")
    cp $file $full_rom_path
done