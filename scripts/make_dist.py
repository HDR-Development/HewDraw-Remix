#!/usr/bin/python3
import shutil, os, sys, pathlib

exec(open("build.py").read())

# TODO collect dependencies (skyline, smashline, arcropolis, etc)

# check if build exists
if not os.path.exists("../build"):
  print("ERROR: no build dir!")
  exit("build dir was missing!")

# if distribution folder exists, delete it
if "distributions" in os.listdir('..'):
  shutil.rmtree('../distributions')
os.mkdir('../distributions')

# zip each folder in the staging dir
packages = os.listdir("../build")
for package_name in packages:
  shutil.make_archive(package_name, 'zip', '../build/' + package_name)
  shutil.move(package_name + ".zip", "../distributions/" + package_name + ".zip")
