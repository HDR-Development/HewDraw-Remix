from codecs import ignore_errors
from time import sleep
import urllib.request, shutil, os, sys
from urllib.request import urlopen
from shutil import copyfileobj
import zipfile
import hashlib
import glob

# example:
# hash_folder("package", "content_hashes.txt")
# returns: the file name that was created
def hash_folder(folder_path: str, file_name: str):
    if os.path.exists(file_name):
        os.remove(file_name)
    original_dir = os.getcwd()
    os.chdir(folder_path)
    files = glob.glob('**/*.*', recursive=True)
    os.chdir(original_dir)
    all_hashes = []

    for file in files:
        # Open,close, read file and calculate MD5 on its contents 
        with open(os.path.join(folder_path + "/" + file), 'rb') as file_to_check:
            # read contents of the file
            data = file_to_check.read()    
            # pipe contents of the file through
            md5_returned = hashlib.md5(data).hexdigest()
            all_hashes.append(os.path.join("/", file) + ":" + md5_returned)
            #print(os.path.join("/", file) + ":" + md5_returned)

    file_str = ""
    for line in all_hashes:
        file_str += line + "\n"

    with open(file_name, 'w') as f:
        f.write(file_str)

