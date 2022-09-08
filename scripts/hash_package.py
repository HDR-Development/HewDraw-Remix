import os
import hashlib
import glob

# example:
# hash_folder("package", "content_hashes.txt")
# returns: the file name that was created
def hash_folder(folder_path: str, file_name: str):
    # create output file
    if os.path.exists(file_name):
        os.remove(file_name)

    original_dir = os.getcwd()
    os.chdir(folder_path)
    # get all files in the directory
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

    # build full output
    file_str = ""
    for line in all_hashes:
        file_str += line + "\n"

    # write output to file
    with open(file_name, 'w') as f:
        f.write(file_str)

# example:
# hash_folder_json("package", "content_hashes.json")
# returns: the file name that was created
def hash_folder_json(folder_path: str, file_name: str):
    # create output file
    if os.path.exists(file_name):
        os.remove(file_name)

    original_dir = os.getcwd()
    os.chdir(folder_path)
    # get all files in the directory
    files = glob.glob('**/*.*', recursive=True)
    os.chdir(original_dir)
    all_hashes = {}

    for file in files:
        # Open,close, read file and calculate MD5 on its contents 
        with open(os.path.join(folder_path + "/" + file), 'rb') as file_to_check:
            # read contents of the file
            data = file_to_check.read()    
            # pipe contents of the file through
            md5_returned = hashlib.md5(data).hexdigest()
            all_hashes[os.path.join("/", file).replace("\\", "/")] = md5_returned
            #print(os.path.join("/", file) + ":" + md5_returned)
    # build full output
    file_str = "["
    first = True
    for key in all_hashes:
        if (not first):
            file_str += ","
        first = False
        file_str += "{\"path\":\"" + key + "\",\"hash\":\"" + all_hashes[key] + "\"}"
    file_str += "]"

    # write output to file
    with open(file_name, 'w') as f:
        f.write(file_str)

