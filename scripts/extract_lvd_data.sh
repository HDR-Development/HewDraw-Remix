#!/bin/bash

TOP="$(pwd)"

ls -lart
unzip lvd_dumper.zip
cd scripts
ls switch-package/
ls switch-package/ultimate/mods/hdr-stages/stage/
mkdir lvd
cp switch-package/ultimate/mods/hdr-stages/stage/ lvd -rf

find lvd -not -name '*.lvd' -type f | xargs rm
cd $TOP/publish
./Ultimate_LVD_data $TOP/scripts/lvd/stage
ls output
ls output/stages
cd output
zip -f lvd.zip stages
cp lvd.zip $TOP/scripts/artifacts