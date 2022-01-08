#!/usr/bin/python3.9
import shutil, os, sys, pathlib, glob, characters

include_string = 'template = ["fighters/include-template"]\n'

output_string = ""

for character in characters.characters:
  output_string += include_string.replace("template", character)

print(output_string)