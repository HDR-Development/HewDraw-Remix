#!/usr/bin/python3
import shutil, os, sys, pathlib, zipfile

def create_diff(zip1: str, zip2: str, output_name: str):
    deleted_files = set()
    changed_files = set()
    deletions_text = "["
    with zipfile.ZipFile(zip1, 'r') as zip1:
        with zipfile.ZipFile(zip2, 'r') as zip2:
            # get the files from zip1 that are changed or deleted
            for info1 in zip1.infolist():
                try:
                    info2 = zip2.getinfo(info1.filename)
                    if info1.CRC != info2.CRC:
                        changed_files.add(info1.filename)
                except:
                    deleted_files.add(info1.filename)

            for info2 in zip2.infolist():
                try:
                    info1 = zip1.getinfo(info2.filename)
                except:
                    changed_files.add(info2.filename)

            for file in changed_files:
                print("changed file: " + file)
                

            for file in deleted_files:
                print("deleted file: " + file)
                deletions_text += "\"" + file + "\","
            deletions_text = deletions_text.strip(",")
            deletions_text += "]"

            # removing old diffs
            shutil.rmtree('diff', True)
            if (output_name + ".zip") in os.listdir('.'):
                os.remove(output_name + ".zip")

            for file in changed_files:
                zip2.extract(file, "diff")

    shutil.make_archive(output_name, 'zip', 'diff')

    
    with open('deletions.json', "w") as deletions_file:
        deletions_file.write(deletions_text)

if __name__ == '__main__':
    print("diffing zips")

    if "help" in sys.argv or "--help" in sys.argv or "-h" in sys.argv:
        print("compares two zip files, and generates a third zip containing the changed/new files, as well as a differences file for what was deleted")
        print("For example:")
        print("./diff_zips.py beta.zip release.zip")
        exit(0)

    if len(sys.argv) != 3:
        exit("invalid argument count!")

    create_diff(sys.argv[1], sys.argv[2], "diff.zip")