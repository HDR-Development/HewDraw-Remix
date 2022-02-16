#!/usr/bin/python3
import shutil, os, sys, pkgutil

if len(sys.argv) <= 1 or len(sys.argv) > 2:
    exit("NO BRANCH REF NAME GIVEN! Invalid arguments.")

# determine branch name
branch_name = pkgutil.run_command("git rev-parse --abbrev-ref HEAD > tmp").strip()

print("current branch is " + branch_name)

# build the git command
git_command = "git log --oneline " + branch_name + "...origin/" + sys.argv[1]
print("git command is:\n" + git_command)
log_output = pkgutil.run_command(git_command)
print(log_output)

# get the cleaned lines of output
all_lines = log_output.split("\n")
cleaned_lines = set()
for line in all_lines:
    if line != "" and "Merge " in line and not "Merge branch 'dev'" in line:
        cleaned_lines.add(line.strip())

#cleaned_lines = reversed(cleaned_lines)

# show cleaned lines
print("merge lines:\n")
for line in cleaned_lines:
    print(line)

final_lines = dict()


for line in cleaned_lines:
    commit_hash = line.split(" ")[0]
    commit_text = pkgutil.run_command("git log -1 --pretty=format:%b " + commit_hash).strip()

    line_add = commit_hash + " : " + commit_text
    print("adding line: " + line_add)
    if commit_text.strip() == "":
        print("empty commit: " + line)
    else:
        final_lines[commit_hash] = commit_text

file_output = ""
for hash,text in final_lines.items():
    file_output += hash + ' -> "' + text + '"\n\n'

print("\nfile output is:\n" + file_output)

with open('change_summary.txt', "w") as myfile:
    myfile.write(file_output)



os.remove('tmp')