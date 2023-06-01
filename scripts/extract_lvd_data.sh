#!/bin/bash

TOP="$(pwd)"

ls -lart
unzip lvd_dumper.zip
ls * -lart
ls switch-package/sdcard/ultimate/mods/hdr-stages/stage/
mkdir lvd
cp switch-package/sdcard/ultimate/mods/hdr-stages/stage/ lvd -rf

find lvd -not -name '*.lvd' -type f | xargs rm
cd $TOP/publish
./Ultimate_LVD_data $TOP/lvd/stage
ls output
ls output/stages
cd output
zip -r lvd.zip stages
cp lvd.zip $TOP/artifacts