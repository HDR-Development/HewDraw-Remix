#!/usr/bin/python3
import shutil, os, sys, pkgutil

debug=False
if len(sys.argv) > 1:
    print("in debug mode!")
    debug=True

# build the git command to get the last thousand commits
git_command = "git log --oneline -n 1000"
if debug:
    print("git command is:\n" + git_command)
log_output = pkgutil.run_command(git_command)
#print(log_output)

log_lines = log_output.split("\n")
for line in log_lines:
    # if this is the entry for a beta pr
    if "Merge pull request #" in line and "from HDR-Development/dev" in line:
        commit_hash = line.split(" ", 1)[0]
        exit(commit_hash)