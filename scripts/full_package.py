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

def download_and_extract(owner: str, repo: str, tag: str, asset: str):
    url = "https://github.com/" + owner + "/" + repo + "/releases/download/" + tag + "/" + asset
    print(url)
    urllib.request.urlretrieve(url, asset)
    with zipfile.ZipFile(asset, 'r') as zip_ref:
        zip_ref.extractall("package")
        sleep(1)
    os.remove(asset)

os.makedirs("package/atmosphere/contents/01006a800016e000/romfs/skyline/plugins/")

download_and_extract("HDR-Development", "HewDraw-Remix", hdr_version, "hdr-switch.zip")
download_and_extract("HDR-Development", "romfs-release", romfs_version, "romfs.zip")
download_and_extract("Raytwo", "ARCropolis", "v3.1.0", "release.zip")

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
