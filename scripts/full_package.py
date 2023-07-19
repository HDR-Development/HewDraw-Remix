from codecs import ignore_errors
from time import sleep
import urllib.request, shutil, os, sys
from urllib.request import urlopen
from shutil import copyfileobj
import zipfile
import hashlib
import glob
import hash_package


if "help" in sys.argv or "--help" in sys.argv or "-h" in sys.argv or len(sys.argv) != 3:
  print("provide arguments for, in order, HewDraw-Remix version and romfs version")
  exit(0)

hdr_version = sys.argv[1]
romfs_version = sys.argv[2]

shutil.rmtree("package", True)
if os.path.exists("switch-package.zip"):
    os.remove("switch-package.zip")
if os.path.exists("switch-package"):
    shutil.rmtree("switch-package")
os.mkdir("switch-package")

def download_and_extract(owner: str, repo: str, tag: str, asset: str, extract_directory = None):
    url = "https://github.com/" + owner + "/" + repo + "/releases/download/" + tag + "/" + asset

    # special case for packaging "latest"
    if tag == 'latest':
        url = "https://github.com/" + owner + "/" + repo + "/releases/latest/download/" + asset
        print("getting latest from url: " + url)
    else:
        print("getting release from url: " + url)

    urllib.request.urlretrieve(url, asset)
    with zipfile.ZipFile(asset, 'r') as zip_ref: 
        if extract_directory:
            extract_home = extract_directory
            os.makedirs("switch-package" + extract_home, exist_ok=True)
        else:
            extract_home = ""
        zip_ref.extractall("switch-package" + extract_home)
        sleep(1)
    os.remove(asset)

os.makedirs("switch-package/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/")

download_and_extract("HDR-Development", "HewDraw-Remix", hdr_version, "hdr-switch.zip")
download_and_extract("HDR-Development", "romfs-release", romfs_version, "romfs.zip")
download_and_extract("Raytwo", "ARCropolis", "latest", "release.zip")
download_and_extract("skyline-dev", "skyline", "beta", "skyline.zip", "/atmosphere/contents/01006A800016E000/")
download_and_extract("HDR-Development", "exlaunch", "latest", "HID-HDR.zip", "/atmosphere/contents/0100000000000013/exefs")

print("getting libnro_hook.nro")
urllib.request.urlretrieve("https://github.com/ultimate-research/nro-hook-plugin/releases/latest/download/libnro_hook.nro", "libnro_hook.nro")
shutil.move("libnro_hook.nro", "switch-package/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/")

print("getting libsmashline_hook.nro")
urllib.request.urlretrieve("https://github.com/blu-dev/smashline_hook/releases/download/2.0.0/libsmashline_hook.nro", "libsmashline_hook.nro")
shutil.move("libsmashline_hook.nro", "switch-package/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/")

print("getting hdr-launcher.nro")
urllib.request.urlretrieve("https://github.com/techyCoder81/hdr-launcher-react/releases/latest/download/hdr-launcher.nro", "hdr-launcher.nro")
shutil.move("hdr-launcher.nro", "switch-package/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/")


print("making switch-package.zip")
shutil.make_archive("switch-package", 'zip', 'switch-package')

print("creating hash files")
hash_package.hash_folder("switch-package", "content_hashes.txt")
hash_package.hash_folder_json("switch-package", "content_hashes.json")


# make a ryujinx package too
print("making ryujinx-package.zip")
os.remove("switch-package/atmosphere/contents/01006A800016E000/romfs/skyline/plugins/hdr-launcher.nro")
os.mkdir("switch-package/sdcard")
shutil.move("switch-package/atmosphere/", "switch-package/sdcard/")
shutil.move("switch-package/ultimate/", "switch-package/sdcard/")
shutil.make_archive("ryujinx-package", 'zip', 'switch-package')


# move the stuff to artifacts folder
if os.path.exists("artifacts"):
    shutil.rmtree("artifacts")
os.mkdir("artifacts")
shutil.move("switch-package.zip", "artifacts")
shutil.move("ryujinx-package.zip", "artifacts")
shutil.move("content_hashes.txt", "artifacts")
shutil.move("content_hashes.json", "artifacts")
