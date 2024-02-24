#!/usr/bin/python3
import shutil, os, sys, pkgutil

if len(sys.argv) <= 2 or len(sys.argv) > 3:
    print("args:")
    for arg in sys.argv:
        print(arg)
    exit("Invalid arguments! should be 'python3 get_build_type.py ref:<ref> prbase:<pr ref>'")


ref = (sys.argv[1] + " ").split(":")[1].strip()
pr_ref = (sys.argv[2] + " ").split(":")[1].strip()

if pr_ref == 'stable' or (ref == 'refs/heads/dev' and pr_ref == ''):
    print("nightly")
    exit()

if ref == 'refs/heads/stable':
    print("beta")
    exit()

if "refs/pull" in ref:
    print("pull_request")
    exit()

print("release")