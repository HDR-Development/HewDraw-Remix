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
    if line != "" and "Merge " in line and not "Merge branch 'dev'" in line and not "from HDR-Development/dev" in line:
        cleaned_lines.add(line.strip())


# show cleaned lines
print("merge lines:\n")
for line in cleaned_lines:
    print(line)

final_lines = dict()


for line in cleaned_lines:
    line_split = line.split(" ")
    commit_hash = line_split[0]
    pr_number = line_split[4]

    commit_text =  commit_hash + ' "' + pkgutil.run_command("git log -1 --pretty=format:%b " + commit_hash).strip() + '"'

    line_add = commit_hash + " : " + commit_text
    print("adding line: " + line_add)
    if commit_text.strip() == "":
        print("empty commit: " + line)
    else:
        final_lines[pr_number] = commit_text

file_output = ""
for pr_number,text in final_lines.items():
    file_output += "PR " + pr_number + ' -> ' + text + '\n\n'

print("\nfile output is:\n" + file_output)

with open('change_summary.txt', "w") as myfile:
    myfile.write(file_output)



os.remove('tmp')