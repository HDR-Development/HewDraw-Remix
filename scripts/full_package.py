from codecs import ignore_errors
from time import sleep
import urllib.request, shutil, os, sys
from urllib.request import urlopen
from shutil import copyfileobj
import zipfile

if "help" in sys.argv or "--help" in sys.argv or "-h" in sys.argv or len(sys.argv) != 3:
  print("provide arguments for, in order, HewDraw-Remix version and romfs version")
  exit(0)

hdr_version = sys.argv[1]
romfs_version = sys.argv[2]

shutil.rmtree("package", True)
if os.path.exists("package.zip"):
    os.remove("package.zip")
os.mkdir("package")

def download_and_extract(owner: str, repo: str, tag: str, asset: str, extract_directory = None):
    url = "https://github.com/" + owner + "/" + repo + "/releases/download/" + tag + "/" + asset

    # special case for packaging "latest"
    if tag == 'latest':
        url = "https://github.com/" + owner + "/" + repo + "/releases/latest/download/" + asset
        print("getting latest from url: " + url)
    else:
        print("getting release from url: " + url)

    print(url)
    urllib.request.urlretrieve(url, asset)
    with zipfile.ZipFile(asset, 'r') as zip_ref: 
        if extract_directory:
            extract_home = extract_directory
            os.makedirs("package" + extract_home, exist_ok=True)
        else:
            extract_home = ""
        zip_ref.extractall("package" + extract_home)
        sleep(1)
    os.remove(asset)

os.makedirs("package/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/")

download_and_extract("HDR-Development", "HewDraw-Remix", hdr_version, "hdr-switch.zip")
download_and_extract("HDR-Development", "romfs-release", romfs_version, "romfs.zip")
download_and_extract("Raytwo", "ARCropolis", "v3.1.0", "release.zip")
download_and_extract("Raytwo", "ARCropolis", "v3.1.0", "release.zip")
download_and_extract("skyline-dev", "skyline", "beta", "skyline.zip", "/atmosphere/contents/01006A800016E000/")

#print("getting libnro_hook.nro")
#urllib.request.urlretrieve("https://github.com/ultimate-research/nro-hook-plugin/releases/download/v0.3.0/libnro_hook.nro", "libnro_hook.nro")
#shutil.move("libnro_hook.nro", "package/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/")

#print("getting libsmashline_hook.nro")
#urllib.request.urlretrieve("https://github.com/blu-dev/smashline_hook/releases/download/1.1.1/libsmashline_hook.nro", "libsmashline_hook.nro")
#shutil.move("libsmashline_hook.nro", "package/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/")

print("getting libsmashline_hook_development.nro")
urllib.request.urlretrieve("https://github.com/blu-dev/smashline_hook/releases/download/1.1.1/libsmashline_hook_development.nro", "libsmashline_hook_development.nro")
shutil.move("libsmashline_hook_development.nro", "package/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/")

print("making package.zip")
shutil.make_archive("package", 'zip', 'package')
