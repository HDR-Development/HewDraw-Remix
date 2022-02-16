#!/usr/bin/python3
import shutil, os, sys, pkgutil

# determine branch name
branch_name = pkgutil.run_command("git rev-parse --abbrev-ref HEAD > tmp").strip()

print("current branch is " + branch_name)

# build the git command
git_command = "git log --oneline " + branch_name + "...stable"
print("git command is:\n" + git_command)
log_output = pkgutil.run_command(git_command)
print(log_output)

# get the cleaned lines of output
all_lines = log_output.split("\n")
cleaned_lines = []
for line in all_lines:
    if line is not "" and "Merge pull request" not in line:
        cleaned_lines.append(line.strip())

# build the output file
file_output = ""
print("cleaned lines:\n")
for line in cleaned_lines:
    print(line)
    if len(line.split(")")) > 1:
        commit_text = line.split(")", 1)[1]
    else:
        commit_text = line.split(" ", 1)[1]
    file_output += line.split(" ")[0] + " : " + commit_text + "\n"

os.system("echo '" + file_output + "' > change_summary.txt")

os.remove('tmp')