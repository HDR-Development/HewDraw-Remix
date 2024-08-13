#!/bin/bash
script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
repo_root=$(dirname "$script_dir")
nx_plugin_path="/atmosphere/contents/01006A800016E000/romfs/skyline/plugins"
nx_mod_path="/ultimate/mods/HDR-Base"

# do this before getting the rom files because this will update them
cd "$repo_root/plugin"
cargo skyline build --release
curl -T target/aarch64-skyline-switch/release/libhdr.nro ftp://${1}:5000${nx_plugin_path}/libhdr.nro

files=$(find "$repo_root/romfs/build" -type f -name "*.prc" -print)
for file in $files;
do
    local_path=${file#"$repo_root/romfs/build"}
    parent_path=$(dirname "$local_path")

    # don't forget the end slash on the ftp path otherwise it makes it a file
    curl -T "$file" ftp://${1}:5000${nx_mod_path}${parent_path}/ --ftp-create-dirs
done